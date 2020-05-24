// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `Sync` X11 extension.

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ALARMSTATE {
    Active = 0,
    Inactive = 1,
    Destroyed = 2,
}
impl From<ALARMSTATE> for u8 {
    fn from(input: ALARMSTATE) -> Self {
        match input {
            ALARMSTATE::Active => 0,
            ALARMSTATE::Inactive => 1,
            ALARMSTATE::Destroyed => 2,
        }
    }
}
impl From<ALARMSTATE> for Option<u8> {
    fn from(input: ALARMSTATE) -> Self {
        Some(u8::from(input))
    }
}
impl From<ALARMSTATE> for u16 {
    fn from(input: ALARMSTATE) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ALARMSTATE> for Option<u16> {
    fn from(input: ALARMSTATE) -> Self {
        Some(u16::from(input))
    }
}
impl From<ALARMSTATE> for u32 {
    fn from(input: ALARMSTATE) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ALARMSTATE> for Option<u32> {
    fn from(input: ALARMSTATE) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for ALARMSTATE {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ALARMSTATE::Active),
            1 => Ok(ALARMSTATE::Inactive),
            2 => Ok(ALARMSTATE::Destroyed),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for ALARMSTATE {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for ALARMSTATE {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

pub type Counter = u32;

pub type Fence = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TESTTYPE {
    PositiveTransition = 0,
    NegativeTransition = 1,
    PositiveComparison = 2,
    NegativeComparison = 3,
}
impl From<TESTTYPE> for u8 {
    fn from(input: TESTTYPE) -> Self {
        match input {
            TESTTYPE::PositiveTransition => 0,
            TESTTYPE::NegativeTransition => 1,
            TESTTYPE::PositiveComparison => 2,
            TESTTYPE::NegativeComparison => 3,
        }
    }
}
impl From<TESTTYPE> for Option<u8> {
    fn from(input: TESTTYPE) -> Self {
        Some(u8::from(input))
    }
}
impl From<TESTTYPE> for u16 {
    fn from(input: TESTTYPE) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<TESTTYPE> for Option<u16> {
    fn from(input: TESTTYPE) -> Self {
        Some(u16::from(input))
    }
}
impl From<TESTTYPE> for u32 {
    fn from(input: TESTTYPE) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<TESTTYPE> for Option<u32> {
    fn from(input: TESTTYPE) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for TESTTYPE {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TESTTYPE::PositiveTransition),
            1 => Ok(TESTTYPE::NegativeTransition),
            2 => Ok(TESTTYPE::PositiveComparison),
            3 => Ok(TESTTYPE::NegativeComparison),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for TESTTYPE {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for TESTTYPE {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum VALUETYPE {
    Absolute = 0,
    Relative = 1,
}
impl From<VALUETYPE> for bool {
    fn from(input: VALUETYPE) -> Self {
        match input {
            VALUETYPE::Absolute => false,
            VALUETYPE::Relative => true,
        }
    }
}
impl From<VALUETYPE> for u8 {
    fn from(input: VALUETYPE) -> Self {
        match input {
            VALUETYPE::Absolute => 0,
            VALUETYPE::Relative => 1,
        }
    }
}
impl From<VALUETYPE> for Option<u8> {
    fn from(input: VALUETYPE) -> Self {
        Some(u8::from(input))
    }
}
impl From<VALUETYPE> for u16 {
    fn from(input: VALUETYPE) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<VALUETYPE> for Option<u16> {
    fn from(input: VALUETYPE) -> Self {
        Some(u16::from(input))
    }
}
impl From<VALUETYPE> for u32 {
    fn from(input: VALUETYPE) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<VALUETYPE> for Option<u32> {
    fn from(input: VALUETYPE) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for VALUETYPE {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(VALUETYPE::Absolute),
            1 => Ok(VALUETYPE::Relative),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for VALUETYPE {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for VALUETYPE {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CA {
    Counter = 1 << 0,
    ValueType = 1 << 1,
    Value = 1 << 2,
    TestType = 1 << 3,
    Delta = 1 << 4,
    Events = 1 << 5,
}
impl From<CA> for u8 {
    fn from(input: CA) -> Self {
        match input {
            CA::Counter => 1 << 0,
            CA::ValueType => 1 << 1,
            CA::Value => 1 << 2,
            CA::TestType => 1 << 3,
            CA::Delta => 1 << 4,
            CA::Events => 1 << 5,
        }
    }
}
impl From<CA> for Option<u8> {
    fn from(input: CA) -> Self {
        Some(u8::from(input))
    }
}
impl From<CA> for u16 {
    fn from(input: CA) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<CA> for Option<u16> {
    fn from(input: CA) -> Self {
        Some(u16::from(input))
    }
}
impl From<CA> for u32 {
    fn from(input: CA) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<CA> for Option<u32> {
    fn from(input: CA) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for CA {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(CA::Counter),
            2 => Ok(CA::ValueType),
            4 => Ok(CA::Value),
            8 => Ok(CA::TestType),
            16 => Ok(CA::Delta),
            32 => Ok(CA::Events),
            _ => Err(ParseError::ParseError),
        }
    }
}
impl TryFrom<u16> for CA {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for CA {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
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
impl TryFrom<&[u8]> for Int64 {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
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
        let (name, remaining) = crate::x11_utils::parse_u8_list(remaining, name_len.try_into().or(Err(ParseError::ParseError))?)?;
        let name = name.to_vec();
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::ParseError)?;
        let result = Systemcounter { counter, resolution, name };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Systemcounter {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
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
        let wait_type = wait_type.try_into()?;
        let test_type = test_type.try_into()?;
        let result = Trigger { counter, wait_type, wait_value, test_type };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Trigger {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
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
impl TryFrom<&[u8]> for Waitcondition {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CounterError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_counter: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}
impl TryParse for CounterError {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (bad_counter, remaining) = u32::try_parse(remaining)?;
        let (minor_opcode, remaining) = u16::try_parse(remaining)?;
        let (major_opcode, remaining) = u8::try_parse(remaining)?;
        let result = CounterError { response_type, error_code, sequence, bad_counter, minor_opcode, major_opcode };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for CounterError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl From<&CounterError> for [u8; 32] {
    fn from(input: &CounterError) -> Self {
        let response_type_bytes = input.response_type.serialize();
        let error_code_bytes = input.error_code.serialize();
        let sequence_bytes = input.sequence.serialize();
        let bad_counter_bytes = input.bad_counter.serialize();
        let minor_opcode_bytes = input.minor_opcode.serialize();
        let major_opcode_bytes = input.major_opcode.serialize();
        [
            response_type_bytes[0],
            error_code_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            bad_counter_bytes[0],
            bad_counter_bytes[1],
            bad_counter_bytes[2],
            bad_counter_bytes[3],
            minor_opcode_bytes[0],
            minor_opcode_bytes[1],
            major_opcode_bytes[0],
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
        ]
    }
}
impl From<CounterError> for [u8; 32] {
    fn from(input: CounterError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the Alarm error
pub const ALARM_ERROR: u8 = 1;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AlarmError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_alarm: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}
impl TryParse for AlarmError {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (bad_alarm, remaining) = u32::try_parse(remaining)?;
        let (minor_opcode, remaining) = u16::try_parse(remaining)?;
        let (major_opcode, remaining) = u8::try_parse(remaining)?;
        let result = AlarmError { response_type, error_code, sequence, bad_alarm, minor_opcode, major_opcode };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for AlarmError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl From<&AlarmError> for [u8; 32] {
    fn from(input: &AlarmError) -> Self {
        let response_type_bytes = input.response_type.serialize();
        let error_code_bytes = input.error_code.serialize();
        let sequence_bytes = input.sequence.serialize();
        let bad_alarm_bytes = input.bad_alarm.serialize();
        let minor_opcode_bytes = input.minor_opcode.serialize();
        let major_opcode_bytes = input.major_opcode.serialize();
        [
            response_type_bytes[0],
            error_code_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            bad_alarm_bytes[0],
            bad_alarm_bytes[1],
            bad_alarm_bytes[2],
            bad_alarm_bytes[3],
            minor_opcode_bytes[0],
            minor_opcode_bytes[1],
            major_opcode_bytes[0],
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
        ]
    }
}
impl From<AlarmError> for [u8; 32] {
    fn from(input: AlarmError) -> Self {
        Self::from(&input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InitializeRequest {
    pub desired_major_version: u8,
    pub desired_minor_version: u8,
}
impl InitializeRequest {
    /// Opcode for the Initialize request
    pub const fn opcode() -> u8 { 0 }
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
            Self::opcode(),
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
}
pub fn initialize<Conn>(conn: &Conn, desired_major_version: u8, desired_minor_version: u8) -> Result<Cookie<'_, Conn, InitializeReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = InitializeRequest {
        desired_major_version,
        desired_minor_version,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InitializeReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: u8,
    pub minor_version: u8,
}
impl TryParse for InitializeReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (major_version, remaining) = u8::try_parse(remaining)?;
        let (minor_version, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(22..).ok_or(ParseError::ParseError)?;
        let result = InitializeReply { response_type, sequence, length, major_version, minor_version };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for InitializeReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ListSystemCountersRequest;
impl ListSystemCountersRequest {
    /// Opcode for the ListSystemCounters request
    pub const fn opcode() -> u8 { 1 }
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
            Self::opcode(),
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn list_system_counters<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, ListSystemCountersReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ListSystemCountersRequest;
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListSystemCountersReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub counters: Vec<Systemcounter>,
}
impl TryParse for ListSystemCountersReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (counters_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (counters, remaining) = crate::x11_utils::parse_list::<Systemcounter>(remaining, counters_len.try_into().or(Err(ParseError::ParseError))?)?;
        let result = ListSystemCountersReply { response_type, sequence, length, counters };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ListSystemCountersReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateCounterRequest {
    pub id: Counter,
    pub initial_value: Int64,
}
impl CreateCounterRequest {
    /// Opcode for the CreateCounter request
    pub const fn opcode() -> u8 { 2 }
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
            Self::opcode(),
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
}
pub fn create_counter<Conn>(conn: &Conn, id: Counter, initial_value: Int64) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateCounterRequest {
        id,
        initial_value,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DestroyCounterRequest {
    pub counter: Counter,
}
impl DestroyCounterRequest {
    /// Opcode for the DestroyCounter request
    pub const fn opcode() -> u8 { 6 }
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
            Self::opcode(),
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
}
pub fn destroy_counter<Conn>(conn: &Conn, counter: Counter) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DestroyCounterRequest {
        counter,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryCounterRequest {
    pub counter: Counter,
}
impl QueryCounterRequest {
    /// Opcode for the QueryCounter request
    pub const fn opcode() -> u8 { 5 }
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
            Self::opcode(),
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
}
pub fn query_counter<Conn>(conn: &Conn, counter: Counter) -> Result<Cookie<'_, Conn, QueryCounterReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryCounterRequest {
        counter,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryCounterReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub counter_value: Int64,
}
impl TryParse for QueryCounterReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (counter_value, remaining) = Int64::try_parse(remaining)?;
        let result = QueryCounterReply { response_type, sequence, length, counter_value };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryCounterReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AwaitRequest<'input> {
    pub wait_list: &'input [Waitcondition],
}
impl<'input> AwaitRequest<'input> {
    /// Opcode for the Await request
    pub const fn opcode() -> u8 { 7 }
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
            Self::opcode(),
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
}
pub fn await_<'c, 'input, Conn>(conn: &'c Conn, wait_list: &'input [Waitcondition]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = AwaitRequest {
        wait_list,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChangeCounterRequest {
    pub counter: Counter,
    pub amount: Int64,
}
impl ChangeCounterRequest {
    /// Opcode for the ChangeCounter request
    pub const fn opcode() -> u8 { 4 }
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
            Self::opcode(),
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
}
pub fn change_counter<Conn>(conn: &Conn, counter: Counter, amount: Int64) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ChangeCounterRequest {
        counter,
        amount,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetCounterRequest {
    pub counter: Counter,
    pub value: Int64,
}
impl SetCounterRequest {
    /// Opcode for the SetCounter request
    pub const fn opcode() -> u8 { 3 }
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
            Self::opcode(),
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
}
pub fn set_counter<Conn>(conn: &Conn, counter: Counter, value: Int64) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetCounterRequest {
        counter,
        value,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
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
#[allow(dead_code, unused_variables)]
impl CreateAlarmAux {
    fn serialize(&self, value_mask: u32) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result, value_mask);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>, value_mask: u32) {
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
            expr_value |= u32::from(CA::Counter);
        }
        if self.value_type.is_some() {
            expr_value |= u32::from(CA::ValueType);
        }
        if self.value.is_some() {
            expr_value |= u32::from(CA::Value);
        }
        if self.test_type.is_some() {
            expr_value |= u32::from(CA::TestType);
        }
        if self.delta.is_some() {
            expr_value |= u32::from(CA::Delta);
        }
        if self.events.is_some() {
            expr_value |= u32::from(CA::Events);
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateAlarmRequest<'input> {
    pub id: Alarm,
    pub value_list: &'input CreateAlarmAux,
}
impl<'input> CreateAlarmRequest<'input> {
    /// Opcode for the CreateAlarm request
    pub const fn opcode() -> u8 { 8 }
    /// Serialize this request into bytes for the provided connection
    fn serialize<Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let id_bytes = self.id.serialize();
        let value_mask = u32::try_from(self.value_list.switch_expr()).unwrap();
        let value_mask_bytes = value_mask.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            Self::opcode(),
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
}
pub fn create_alarm<'c, 'input, Conn>(conn: &'c Conn, id: Alarm, value_list: &'input CreateAlarmAux) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateAlarmRequest {
        id,
        value_list,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
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
#[allow(dead_code, unused_variables)]
impl ChangeAlarmAux {
    fn serialize(&self, value_mask: u32) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result, value_mask);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>, value_mask: u32) {
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
            expr_value |= u32::from(CA::Counter);
        }
        if self.value_type.is_some() {
            expr_value |= u32::from(CA::ValueType);
        }
        if self.value.is_some() {
            expr_value |= u32::from(CA::Value);
        }
        if self.test_type.is_some() {
            expr_value |= u32::from(CA::TestType);
        }
        if self.delta.is_some() {
            expr_value |= u32::from(CA::Delta);
        }
        if self.events.is_some() {
            expr_value |= u32::from(CA::Events);
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChangeAlarmRequest<'input> {
    pub id: Alarm,
    pub value_list: &'input ChangeAlarmAux,
}
impl<'input> ChangeAlarmRequest<'input> {
    /// Opcode for the ChangeAlarm request
    pub const fn opcode() -> u8 { 9 }
    /// Serialize this request into bytes for the provided connection
    fn serialize<Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let id_bytes = self.id.serialize();
        let value_mask = u32::try_from(self.value_list.switch_expr()).unwrap();
        let value_mask_bytes = value_mask.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            Self::opcode(),
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
}
pub fn change_alarm<'c, 'input, Conn>(conn: &'c Conn, id: Alarm, value_list: &'input ChangeAlarmAux) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ChangeAlarmRequest {
        id,
        value_list,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DestroyAlarmRequest {
    pub alarm: Alarm,
}
impl DestroyAlarmRequest {
    /// Opcode for the DestroyAlarm request
    pub const fn opcode() -> u8 { 11 }
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
            Self::opcode(),
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
}
pub fn destroy_alarm<Conn>(conn: &Conn, alarm: Alarm) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DestroyAlarmRequest {
        alarm,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryAlarmRequest {
    pub alarm: Alarm,
}
impl QueryAlarmRequest {
    /// Opcode for the QueryAlarm request
    pub const fn opcode() -> u8 { 10 }
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
            Self::opcode(),
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
}
pub fn query_alarm<Conn>(conn: &Conn, alarm: Alarm) -> Result<Cookie<'_, Conn, QueryAlarmReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryAlarmRequest {
        alarm,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryAlarmReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub trigger: Trigger,
    pub delta: Int64,
    pub events: bool,
    pub state: ALARMSTATE,
}
impl TryParse for QueryAlarmReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (trigger, remaining) = Trigger::try_parse(remaining)?;
        let (delta, remaining) = Int64::try_parse(remaining)?;
        let (events, remaining) = bool::try_parse(remaining)?;
        let (state, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let state = state.try_into()?;
        let result = QueryAlarmReply { response_type, sequence, length, trigger, delta, events, state };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryAlarmReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetPriorityRequest {
    pub id: u32,
    pub priority: i32,
}
impl SetPriorityRequest {
    /// Opcode for the SetPriority request
    pub const fn opcode() -> u8 { 12 }
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
            Self::opcode(),
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
}
pub fn set_priority<Conn>(conn: &Conn, id: u32, priority: i32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetPriorityRequest {
        id,
        priority,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetPriorityRequest {
    pub id: u32,
}
impl GetPriorityRequest {
    /// Opcode for the GetPriority request
    pub const fn opcode() -> u8 { 13 }
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
            Self::opcode(),
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
}
pub fn get_priority<Conn>(conn: &Conn, id: u32) -> Result<Cookie<'_, Conn, GetPriorityReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetPriorityRequest {
        id,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetPriorityReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub priority: i32,
}
impl TryParse for GetPriorityReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (priority, remaining) = i32::try_parse(remaining)?;
        let result = GetPriorityReply { response_type, sequence, length, priority };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetPriorityReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateFenceRequest {
    pub drawable: xproto::Drawable,
    pub fence: Fence,
    pub initially_triggered: bool,
}
impl CreateFenceRequest {
    /// Opcode for the CreateFence request
    pub const fn opcode() -> u8 { 14 }
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
            Self::opcode(),
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
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TriggerFenceRequest {
    pub fence: Fence,
}
impl TriggerFenceRequest {
    /// Opcode for the TriggerFence request
    pub const fn opcode() -> u8 { 15 }
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
            Self::opcode(),
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
}
pub fn trigger_fence<Conn>(conn: &Conn, fence: Fence) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = TriggerFenceRequest {
        fence,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResetFenceRequest {
    pub fence: Fence,
}
impl ResetFenceRequest {
    /// Opcode for the ResetFence request
    pub const fn opcode() -> u8 { 16 }
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
            Self::opcode(),
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
}
pub fn reset_fence<Conn>(conn: &Conn, fence: Fence) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ResetFenceRequest {
        fence,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DestroyFenceRequest {
    pub fence: Fence,
}
impl DestroyFenceRequest {
    /// Opcode for the DestroyFence request
    pub const fn opcode() -> u8 { 17 }
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
            Self::opcode(),
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
}
pub fn destroy_fence<Conn>(conn: &Conn, fence: Fence) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DestroyFenceRequest {
        fence,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryFenceRequest {
    pub fence: Fence,
}
impl QueryFenceRequest {
    /// Opcode for the QueryFence request
    pub const fn opcode() -> u8 { 18 }
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
            Self::opcode(),
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
}
pub fn query_fence<Conn>(conn: &Conn, fence: Fence) -> Result<Cookie<'_, Conn, QueryFenceReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryFenceRequest {
        fence,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryFenceReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub triggered: bool,
}
impl TryParse for QueryFenceReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (triggered, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(23..).ok_or(ParseError::ParseError)?;
        let result = QueryFenceReply { response_type, sequence, length, triggered };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryFenceReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AwaitFenceRequest<'input> {
    pub fence_list: &'input [Fence],
}
impl<'input> AwaitFenceRequest<'input> {
    /// Opcode for the AwaitFence request
    pub const fn opcode() -> u8 { 19 }
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
            Self::opcode(),
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
}
pub fn await_fence<'c, 'input, Conn>(conn: &'c Conn, fence_list: &'input [Fence]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = AwaitFenceRequest {
        fence_list,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
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
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (kind, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (counter, remaining) = Counter::try_parse(remaining)?;
        let (wait_value, remaining) = Int64::try_parse(remaining)?;
        let (counter_value, remaining) = Int64::try_parse(remaining)?;
        let (timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (count, remaining) = u16::try_parse(remaining)?;
        let (destroyed, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let result = CounterNotifyEvent { response_type, kind, sequence, counter, wait_value, counter_value, timestamp, count, destroyed };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for CounterNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
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
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (kind, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (alarm, remaining) = Alarm::try_parse(remaining)?;
        let (counter_value, remaining) = Int64::try_parse(remaining)?;
        let (alarm_value, remaining) = Int64::try_parse(remaining)?;
        let (timestamp, remaining) = xproto::Timestamp::try_parse(remaining)?;
        let (state, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(3..).ok_or(ParseError::ParseError)?;
        let state = state.try_into()?;
        let result = AlarmNotifyEvent { response_type, kind, sequence, alarm, counter_value, alarm_value, timestamp, state };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for AlarmNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
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
