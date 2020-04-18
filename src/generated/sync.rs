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
use crate::x11_utils::{Serialize, TryParse};
use crate::connection::RequestConnection;
#[allow(unused_imports)]
use crate::cookie::{Cookie, CookieWithFds, VoidCookie};
use crate::errors::{ConnectionError, ParseError};
#[allow(unused_imports)]
use crate::x11_utils::GenericEvent;
#[allow(unused_imports)]
use crate::x11_utils::GenericError;
#[allow(unused_imports)]
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
    fn serialize(&self) -> Self::Bytes {
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
        let (name, remaining) = crate::x11_utils::parse_list::<u8>(remaining, name_len as usize)?;
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
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(14);
        self.counter.serialize_into(bytes);
        self.resolution.serialize_into(bytes);
        let name_len = self.name.len() as u16;
        name_len.serialize_into(bytes);
        self.name.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
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
    fn serialize(&self) -> Self::Bytes {
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
    fn serialize(&self) -> Self::Bytes {
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
impl CounterError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
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
impl<B: AsRef<[u8]>> From<GenericError<B>> for CounterError {
    fn from(value: GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericError<B>> for CounterError {
    fn from(value: &GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&CounterError> for [u8; 32] {
    fn from(input: &CounterError) -> Self {
        let response_type = input.response_type.serialize();
        let error_code = input.error_code.serialize();
        let sequence = input.sequence.serialize();
        let bad_counter = input.bad_counter.serialize();
        let minor_opcode = input.minor_opcode.serialize();
        let major_opcode = input.major_opcode.serialize();
        [
            response_type[0], error_code[0], sequence[0], sequence[1], bad_counter[0], bad_counter[1], bad_counter[2], bad_counter[3],
            minor_opcode[0], minor_opcode[1], major_opcode[0], /* trailing padding */ 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
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
impl AlarmError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
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
impl<B: AsRef<[u8]>> From<GenericError<B>> for AlarmError {
    fn from(value: GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericError<B>> for AlarmError {
    fn from(value: &GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&AlarmError> for [u8; 32] {
    fn from(input: &AlarmError) -> Self {
        let response_type = input.response_type.serialize();
        let error_code = input.error_code.serialize();
        let sequence = input.sequence.serialize();
        let bad_alarm = input.bad_alarm.serialize();
        let minor_opcode = input.minor_opcode.serialize();
        let major_opcode = input.major_opcode.serialize();
        [
            response_type[0], error_code[0], sequence[0], sequence[1], bad_alarm[0], bad_alarm[1], bad_alarm[2], bad_alarm[3],
            minor_opcode[0], minor_opcode[1], major_opcode[0], /* trailing padding */ 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<AlarmError> for [u8; 32] {
    fn from(input: AlarmError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the Initialize request
pub const INITIALIZE_REQUEST: u8 = 0;
pub fn initialize<Conn>(conn: &Conn, desired_major_version: u8, desired_minor_version: u8) -> Result<Cookie<'_, Conn, InitializeReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let desired_major_version_bytes = desired_major_version.serialize();
    let desired_minor_version_bytes = desired_minor_version.serialize();
    let request0 = [
        extension_information.major_opcode,
        INITIALIZE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        desired_major_version_bytes[0],
        desired_minor_version_bytes[0],
        0 /* trailing padding */,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InitializeReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: u8,
    pub minor_version: u8,
}
impl InitializeReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
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

/// Opcode for the ListSystemCounters request
pub const LIST_SYSTEM_COUNTERS_REQUEST: u8 = 1;
pub fn list_system_counters<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, ListSystemCountersReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (4) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let request0 = [
        extension_information.major_opcode,
        LIST_SYSTEM_COUNTERS_REQUEST,
        length_bytes[0],
        length_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListSystemCountersReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub counters: Vec<Systemcounter>,
}
impl ListSystemCountersReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (counters_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (counters, remaining) = crate::x11_utils::parse_list::<Systemcounter>(remaining, counters_len as usize)?;
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

/// Opcode for the CreateCounter request
pub const CREATE_COUNTER_REQUEST: u8 = 2;
pub fn create_counter<Conn>(conn: &Conn, id: Counter, initial_value: Int64) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let id_bytes = id.serialize();
    let initial_value_bytes = initial_value.serialize();
    let request0 = [
        extension_information.major_opcode,
        CREATE_COUNTER_REQUEST,
        length_bytes[0],
        length_bytes[1],
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
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the DestroyCounter request
pub const DESTROY_COUNTER_REQUEST: u8 = 6;
pub fn destroy_counter<Conn>(conn: &Conn, counter: Counter) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let counter_bytes = counter.serialize();
    let request0 = [
        extension_information.major_opcode,
        DESTROY_COUNTER_REQUEST,
        length_bytes[0],
        length_bytes[1],
        counter_bytes[0],
        counter_bytes[1],
        counter_bytes[2],
        counter_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the QueryCounter request
pub const QUERY_COUNTER_REQUEST: u8 = 5;
pub fn query_counter<Conn>(conn: &Conn, counter: Counter) -> Result<Cookie<'_, Conn, QueryCounterReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let counter_bytes = counter.serialize();
    let request0 = [
        extension_information.major_opcode,
        QUERY_COUNTER_REQUEST,
        length_bytes[0],
        length_bytes[1],
        counter_bytes[0],
        counter_bytes[1],
        counter_bytes[2],
        counter_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryCounterReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub counter_value: Int64,
}
impl QueryCounterReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
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

/// Opcode for the Await request
pub const AWAIT_REQUEST: u8 = 7;
pub fn await_<'c, Conn>(conn: &'c Conn, wait_list: &[Waitcondition]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (4 + 28 * wait_list.len() + 3) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let wait_list_bytes = wait_list.serialize();
    let request0 = [
        extension_information.major_opcode,
        AWAIT_REQUEST,
        length_bytes[0],
        length_bytes[1],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&wait_list_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&wait_list_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the ChangeCounter request
pub const CHANGE_COUNTER_REQUEST: u8 = 4;
pub fn change_counter<Conn>(conn: &Conn, counter: Counter, amount: Int64) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let counter_bytes = counter.serialize();
    let amount_bytes = amount.serialize();
    let request0 = [
        extension_information.major_opcode,
        CHANGE_COUNTER_REQUEST,
        length_bytes[0],
        length_bytes[1],
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
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the SetCounter request
pub const SET_COUNTER_REQUEST: u8 = 3;
pub fn set_counter<Conn>(conn: &Conn, counter: Counter, value: Int64) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let counter_bytes = counter.serialize();
    let value_bytes = value.serialize();
    let request0 = [
        extension_information.major_opcode,
        SET_COUNTER_REQUEST,
        length_bytes[0],
        length_bytes[1],
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
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the CreateAlarm request
pub const CREATE_ALARM_REQUEST: u8 = 8;
/// Auxiliary and optional information for the create_alarm function.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CreateAlarmAux {
    pub counter: Option<Counter>,
    pub value_type: Option<u32>,
    pub value: Option<Int64>,
    pub test_type: Option<u32>,
    pub delta: Option<Int64>,
    pub events: Option<u32>,
}
impl CreateAlarmAux {
    /// Create a new instance with all fields unset / not present.
    pub fn new() -> Self {
        Default::default()
    }
    fn value_mask(&self) -> u32 {
        let mut mask = 0;
        if self.counter.is_some() {
            mask |= u32::from(CA::Counter);
        }
        if self.value_type.is_some() {
            mask |= u32::from(CA::ValueType);
        }
        if self.value.is_some() {
            mask |= u32::from(CA::Value);
        }
        if self.test_type.is_some() {
            mask |= u32::from(CA::TestType);
        }
        if self.delta.is_some() {
            mask |= u32::from(CA::Delta);
        }
        if self.events.is_some() {
            mask |= u32::from(CA::Events);
        }
        mask
    }
    /// Set the counter field of this structure.
    pub fn counter<I>(mut self, value: I) -> Self where I: Into<Option<Counter>> {
        self.counter = value.into();
        self
    }
    /// Set the valueType field of this structure.
    pub fn value_type<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.value_type = value.into();
        self
    }
    /// Set the value field of this structure.
    pub fn value<I>(mut self, value: I) -> Self where I: Into<Option<Int64>> {
        self.value = value.into();
        self
    }
    /// Set the testType field of this structure.
    pub fn test_type<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.test_type = value.into();
        self
    }
    /// Set the delta field of this structure.
    pub fn delta<I>(mut self, value: I) -> Self where I: Into<Option<Int64>> {
        self.delta = value.into();
        self
    }
    /// Set the events field of this structure.
    pub fn events<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.events = value.into();
        self
    }
}
impl Serialize for CreateAlarmAux {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        if let Some(ref value) = self.counter {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.value_type {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.value {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.test_type {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.delta {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.events {
            value.serialize_into(bytes);
        }
    }
}
pub fn create_alarm<'c, Conn>(conn: &'c Conn, id: Alarm, value_list: &CreateAlarmAux) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let value_mask = value_list.value_mask();
    let value_list_bytes = value_list.serialize();
    let length: usize = (12 + value_list_bytes.len() + 3) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let id_bytes = id.serialize();
    let value_mask_bytes = value_mask.serialize();
    let request0 = [
        extension_information.major_opcode,
        CREATE_ALARM_REQUEST,
        length_bytes[0],
        length_bytes[1],
        id_bytes[0],
        id_bytes[1],
        id_bytes[2],
        id_bytes[3],
        value_mask_bytes[0],
        value_mask_bytes[1],
        value_mask_bytes[2],
        value_mask_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&value_list_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&value_list_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the ChangeAlarm request
pub const CHANGE_ALARM_REQUEST: u8 = 9;
/// Auxiliary and optional information for the change_alarm function.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ChangeAlarmAux {
    pub counter: Option<Counter>,
    pub value_type: Option<u32>,
    pub value: Option<Int64>,
    pub test_type: Option<u32>,
    pub delta: Option<Int64>,
    pub events: Option<u32>,
}
impl ChangeAlarmAux {
    /// Create a new instance with all fields unset / not present.
    pub fn new() -> Self {
        Default::default()
    }
    fn value_mask(&self) -> u32 {
        let mut mask = 0;
        if self.counter.is_some() {
            mask |= u32::from(CA::Counter);
        }
        if self.value_type.is_some() {
            mask |= u32::from(CA::ValueType);
        }
        if self.value.is_some() {
            mask |= u32::from(CA::Value);
        }
        if self.test_type.is_some() {
            mask |= u32::from(CA::TestType);
        }
        if self.delta.is_some() {
            mask |= u32::from(CA::Delta);
        }
        if self.events.is_some() {
            mask |= u32::from(CA::Events);
        }
        mask
    }
    /// Set the counter field of this structure.
    pub fn counter<I>(mut self, value: I) -> Self where I: Into<Option<Counter>> {
        self.counter = value.into();
        self
    }
    /// Set the valueType field of this structure.
    pub fn value_type<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.value_type = value.into();
        self
    }
    /// Set the value field of this structure.
    pub fn value<I>(mut self, value: I) -> Self where I: Into<Option<Int64>> {
        self.value = value.into();
        self
    }
    /// Set the testType field of this structure.
    pub fn test_type<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.test_type = value.into();
        self
    }
    /// Set the delta field of this structure.
    pub fn delta<I>(mut self, value: I) -> Self where I: Into<Option<Int64>> {
        self.delta = value.into();
        self
    }
    /// Set the events field of this structure.
    pub fn events<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.events = value.into();
        self
    }
}
impl Serialize for ChangeAlarmAux {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        if let Some(ref value) = self.counter {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.value_type {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.value {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.test_type {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.delta {
            value.serialize_into(bytes);
        }
        if let Some(ref value) = self.events {
            value.serialize_into(bytes);
        }
    }
}
pub fn change_alarm<'c, Conn>(conn: &'c Conn, id: Alarm, value_list: &ChangeAlarmAux) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let value_mask = value_list.value_mask();
    let value_list_bytes = value_list.serialize();
    let length: usize = (12 + value_list_bytes.len() + 3) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let id_bytes = id.serialize();
    let value_mask_bytes = value_mask.serialize();
    let request0 = [
        extension_information.major_opcode,
        CHANGE_ALARM_REQUEST,
        length_bytes[0],
        length_bytes[1],
        id_bytes[0],
        id_bytes[1],
        id_bytes[2],
        id_bytes[3],
        value_mask_bytes[0],
        value_mask_bytes[1],
        value_mask_bytes[2],
        value_mask_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&value_list_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&value_list_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the DestroyAlarm request
pub const DESTROY_ALARM_REQUEST: u8 = 11;
pub fn destroy_alarm<Conn>(conn: &Conn, alarm: Alarm) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let alarm_bytes = alarm.serialize();
    let request0 = [
        extension_information.major_opcode,
        DESTROY_ALARM_REQUEST,
        length_bytes[0],
        length_bytes[1],
        alarm_bytes[0],
        alarm_bytes[1],
        alarm_bytes[2],
        alarm_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the QueryAlarm request
pub const QUERY_ALARM_REQUEST: u8 = 10;
pub fn query_alarm<Conn>(conn: &Conn, alarm: Alarm) -> Result<Cookie<'_, Conn, QueryAlarmReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let alarm_bytes = alarm.serialize();
    let request0 = [
        extension_information.major_opcode,
        QUERY_ALARM_REQUEST,
        length_bytes[0],
        length_bytes[1],
        alarm_bytes[0],
        alarm_bytes[1],
        alarm_bytes[2],
        alarm_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
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
impl QueryAlarmReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
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

/// Opcode for the SetPriority request
pub const SET_PRIORITY_REQUEST: u8 = 12;
pub fn set_priority<Conn>(conn: &Conn, id: u32, priority: i32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (12) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let id_bytes = id.serialize();
    let priority_bytes = priority.serialize();
    let request0 = [
        extension_information.major_opcode,
        SET_PRIORITY_REQUEST,
        length_bytes[0],
        length_bytes[1],
        id_bytes[0],
        id_bytes[1],
        id_bytes[2],
        id_bytes[3],
        priority_bytes[0],
        priority_bytes[1],
        priority_bytes[2],
        priority_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the GetPriority request
pub const GET_PRIORITY_REQUEST: u8 = 13;
pub fn get_priority<Conn>(conn: &Conn, id: u32) -> Result<Cookie<'_, Conn, GetPriorityReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let id_bytes = id.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_PRIORITY_REQUEST,
        length_bytes[0],
        length_bytes[1],
        id_bytes[0],
        id_bytes[1],
        id_bytes[2],
        id_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetPriorityReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub priority: i32,
}
impl GetPriorityReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
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

/// Opcode for the CreateFence request
pub const CREATE_FENCE_REQUEST: u8 = 14;
pub fn create_fence<Conn>(conn: &Conn, drawable: xproto::Drawable, fence: Fence, initially_triggered: bool) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let drawable_bytes = drawable.serialize();
    let fence_bytes = fence.serialize();
    let initially_triggered_bytes = (initially_triggered as u8).serialize();
    let request0 = [
        extension_information.major_opcode,
        CREATE_FENCE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        drawable_bytes[0],
        drawable_bytes[1],
        drawable_bytes[2],
        drawable_bytes[3],
        fence_bytes[0],
        fence_bytes[1],
        fence_bytes[2],
        fence_bytes[3],
        initially_triggered_bytes[0],
        0 /* trailing padding */,
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the TriggerFence request
pub const TRIGGER_FENCE_REQUEST: u8 = 15;
pub fn trigger_fence<Conn>(conn: &Conn, fence: Fence) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let fence_bytes = fence.serialize();
    let request0 = [
        extension_information.major_opcode,
        TRIGGER_FENCE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        fence_bytes[0],
        fence_bytes[1],
        fence_bytes[2],
        fence_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the ResetFence request
pub const RESET_FENCE_REQUEST: u8 = 16;
pub fn reset_fence<Conn>(conn: &Conn, fence: Fence) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let fence_bytes = fence.serialize();
    let request0 = [
        extension_information.major_opcode,
        RESET_FENCE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        fence_bytes[0],
        fence_bytes[1],
        fence_bytes[2],
        fence_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the DestroyFence request
pub const DESTROY_FENCE_REQUEST: u8 = 17;
pub fn destroy_fence<Conn>(conn: &Conn, fence: Fence) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let fence_bytes = fence.serialize();
    let request0 = [
        extension_information.major_opcode,
        DESTROY_FENCE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        fence_bytes[0],
        fence_bytes[1],
        fence_bytes[2],
        fence_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the QueryFence request
pub const QUERY_FENCE_REQUEST: u8 = 18;
pub fn query_fence<Conn>(conn: &Conn, fence: Fence) -> Result<Cookie<'_, Conn, QueryFenceReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let fence_bytes = fence.serialize();
    let request0 = [
        extension_information.major_opcode,
        QUERY_FENCE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        fence_bytes[0],
        fence_bytes[1],
        fence_bytes[2],
        fence_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryFenceReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub triggered: bool,
}
impl QueryFenceReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
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

/// Opcode for the AwaitFence request
pub const AWAIT_FENCE_REQUEST: u8 = 19;
pub fn await_fence<'c, Conn>(conn: &'c Conn, fence_list: &[Fence]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (4 + 4 * fence_list.len() + 3) / 4;
    let length_bytes = u16::try_from(length).unwrap_or(0).serialize();
    let fence_list_bytes = fence_list.serialize();
    let request0 = [
        extension_information.major_opcode,
        AWAIT_FENCE_REQUEST,
        length_bytes[0],
        length_bytes[1],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&fence_list_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&fence_list_bytes), IoSlice::new(&padding1)], Vec::new())?)
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
impl CounterNotifyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
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
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for CounterNotifyEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for CounterNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&CounterNotifyEvent> for [u8; 32] {
    fn from(input: &CounterNotifyEvent) -> Self {
        let response_type = input.response_type.serialize();
        let kind = input.kind.serialize();
        let sequence = input.sequence.serialize();
        let counter = input.counter.serialize();
        let wait_value = input.wait_value.serialize();
        let counter_value = input.counter_value.serialize();
        let timestamp = input.timestamp.serialize();
        let count = input.count.serialize();
        let destroyed = input.destroyed.serialize();
        [
            response_type[0], kind[0], sequence[0], sequence[1], counter[0], counter[1], counter[2], counter[3],
            wait_value[0], wait_value[1], wait_value[2], wait_value[3], wait_value[4], wait_value[5], wait_value[6], wait_value[7],
            counter_value[0], counter_value[1], counter_value[2], counter_value[3], counter_value[4], counter_value[5], counter_value[6], counter_value[7],
            timestamp[0], timestamp[1], timestamp[2], timestamp[3], count[0], count[1], destroyed[0], 0
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
impl AlarmNotifyEvent {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
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
impl<B: AsRef<[u8]>> TryFrom<GenericEvent<B>> for AlarmNotifyEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl<B: AsRef<[u8]>> TryFrom<&GenericEvent<B>> for AlarmNotifyEvent {
    type Error = ParseError;
    fn try_from(value: &GenericEvent<B>) -> Result<Self, Self::Error> {
        Self::try_from(value.raw_bytes())
    }
}
impl From<&AlarmNotifyEvent> for [u8; 32] {
    fn from(input: &AlarmNotifyEvent) -> Self {
        let response_type = input.response_type.serialize();
        let kind = input.kind.serialize();
        let sequence = input.sequence.serialize();
        let alarm = input.alarm.serialize();
        let counter_value = input.counter_value.serialize();
        let alarm_value = input.alarm_value.serialize();
        let timestamp = input.timestamp.serialize();
        let state = u8::from(input.state).serialize();
        [
            response_type[0], kind[0], sequence[0], sequence[1], alarm[0], alarm[1], alarm[2], alarm[3],
            counter_value[0], counter_value[1], counter_value[2], counter_value[3], counter_value[4], counter_value[5], counter_value[6], counter_value[7],
            alarm_value[0], alarm_value[1], alarm_value[2], alarm_value[3], alarm_value[4], alarm_value[5], alarm_value[6], alarm_value[7],
            timestamp[0], timestamp[1], timestamp[2], timestamp[3], state[0], 0, 0, 0
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

    fn sync_await_<'c>(&'c self, wait_list: &[Waitcondition]) -> Result<VoidCookie<'c, Self>, ConnectionError>
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

    fn sync_create_alarm<'c>(&'c self, id: Alarm, value_list: &CreateAlarmAux) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        create_alarm(self, id, value_list)
    }

    fn sync_change_alarm<'c>(&'c self, id: Alarm, value_list: &ChangeAlarmAux) -> Result<VoidCookie<'c, Self>, ConnectionError>
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

    fn sync_await_fence<'c>(&'c self, fence_list: &[Fence]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        await_fence(self, fence_list)
    }

}
impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
