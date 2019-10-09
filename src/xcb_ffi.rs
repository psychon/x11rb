extern crate libc;

use std::ptr::{null, null_mut};
use std::ops::Deref;
use std::marker::PhantomData;
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::ffi::CStr;
use std::io::IoSlice;
use std::mem::forget;
use libc::free;
use super::generated::xproto::Setup;

pub type SequenceNumber = u64;

#[derive(Debug)]
pub struct Connection(*mut raw_ffi::xcb_connection_t, Setup);

#[derive(Debug)]
pub enum ParseError {
    ParseError
}

impl Error for ParseError {
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "FIXME")
    }
}

impl From<std::convert::Infallible> for ParseError {
    fn from(_: std::convert::Infallible) -> Self {
        unreachable!()
    }
}

impl From<std::num::TryFromIntError> for ParseError {
    fn from(_: std::num::TryFromIntError) -> Self {
        ParseError::ParseError
    }
}

#[derive(Debug)]
pub enum ConnectionError {
    UnknownError,
    ConnectionError,
    UnsupportedExtension,
    InsufficientMemory,
    MaximumRequestLengthExceeded,
    DisplayParsingError,
    InvalidScreen,
    FDPassingFailed,
    ParseError,
}

impl Error for ConnectionError {
}

impl std::fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "FIXME")
    }
}

impl From<ParseError> for ConnectionError {
    fn from(err: ParseError) -> Self {
        match err {
            ParseError::ParseError => ConnectionError::ParseError
        }
    }
}

impl From<std::num::TryFromIntError> for ConnectionError {
    fn from(err: std::num::TryFromIntError) -> Self {
        Self::from(ParseError::from(err))
    }
}

#[derive(Debug)]
pub enum ConnectionErrorOrX11Error {
    ConnectionError(ConnectionError),
    X11Error(GenericError)
}

impl Error for ConnectionErrorOrX11Error {
}

impl std::fmt::Display for ConnectionErrorOrX11Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "FIXME")
    }
}

impl From<ParseError> for ConnectionErrorOrX11Error {
    fn from(err: ParseError) -> Self {
        Self::from(ConnectionError::from(err))
    }
}

impl From<std::num::TryFromIntError> for ConnectionErrorOrX11Error {
    fn from(err: std::num::TryFromIntError) -> Self {
        Self::from(ParseError::from(err))
    }
}

impl From<ConnectionError> for ConnectionErrorOrX11Error {
    fn from(err: ConnectionError) -> Self {
        Self::ConnectionError(err)
    }
}

impl From<GenericError> for ConnectionErrorOrX11Error {
    fn from(err: GenericError) -> Self {
        Self::X11Error(err)
    }
}

#[derive(Debug)]
pub struct CSlice {
    slice: &'static [u8]
}
impl CSlice {
    unsafe fn new(ptr: *const u8, len: usize) -> CSlice {
        let slice = std::slice::from_raw_parts(ptr, len);
        CSlice{ slice }
    }

    fn into_ptr(self) -> *const u8 {
        let ptr = self.slice.as_ptr();
        forget(self);
        ptr
    }
}
impl Deref for CSlice {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.slice
    }
}
impl Drop for CSlice {
    fn drop(&mut self) {
        unsafe { free(self.slice.as_ptr() as _) }
    }
}

impl ConnectionError {
    unsafe fn from_connection(c: *const raw_ffi::xcb_connection_t) -> ConnectionError {
        Self::from_c_error(raw_ffi::xcb_connection_has_error(c))
    }

    fn from_c_error(error: i32) -> ConnectionError {
        use raw_ffi::connection_errors::*;

        assert_ne!(error, 0);
        match error {
            ERROR => ConnectionError::ConnectionError,
            EXT_NOTSUPPORTED => ConnectionError::UnsupportedExtension,
            MEM_INSUFFICIENT => ConnectionError::InsufficientMemory,
            REQ_LEN_EXCEED => ConnectionError::MaximumRequestLengthExceeded,
            PARSE_ERR => ConnectionError::DisplayParsingError,
            INVALID_SCREEN => ConnectionError::InvalidScreen,
            FDPASSING_FAILED => ConnectionError::FDPassingFailed,
            _ => ConnectionError::UnknownError
        }
    }
}

impl Connection {
    pub fn connect(dpy_name: Option<&CStr>) -> Result<(Connection, usize), ConnectionError>  {
        use libc::c_int;
        unsafe {
            let mut screen: c_int = 0;
            let dpy_ptr = dpy_name.map_or(null(), |s| s.as_ptr());
            let connection = raw_ffi::xcb_connect(dpy_ptr, &mut screen);
            let error = raw_ffi::xcb_connection_has_error(connection);
            if error != 0 {
                raw_ffi::xcb_disconnect(connection);
                Err(ConnectionError::from_c_error(error.try_into().or(Err(ConnectionError::UnknownError))?))
            } else {
                let setup = raw_ffi::xcb_get_setup(connection);
                Ok((Connection(connection, Self::parse_setup(setup)?), screen as usize))
            }
        }
    }

    unsafe fn parse_setup(setup: *const u8) -> Result<Setup, ParseError> {
        // We know that the setup information has at least eight bytes
        let wrapper = CSlice::new(setup, 8);

        // The length field is in the last two bytes
        let length = u16::from_ne_bytes([(*wrapper)[6], (*wrapper)[7]]);

        // The length is in four-byte-units after the known header
        let length = length * 4 + 8;

        let slice = CSlice::new(wrapper.into_ptr(), length.try_into()?);
        let result = Setup::try_from(&*slice)?;

        // We must not free() xcb_setup_t that libxcb owns
        forget(slice);

        Ok(result)
    }

    pub fn setup(&self) -> &Setup {
        &self.1
    }

    pub fn generate_id(&self) -> u32 {
        unsafe { raw_ffi::xcb_generate_id(self.0) }
    }

    pub fn flush(&self) {
        unsafe { raw_ffi::xcb_flush(self.0); }
    }

    pub fn send_request_with_reply<R>(&self, bufs: &[IoSlice]) -> Cookie<R>
        where R: TryFrom<CSlice, Error=ParseError>
    {
        Cookie::new(self, self.send_request(bufs, true))
    }

    pub fn send_request_without_reply(&self, bufs: &[IoSlice]) -> SequenceNumber {
        self.send_request(bufs, false)
    }

    fn send_request(&self, bufs: &[IoSlice], has_reply: bool) -> SequenceNumber {
        let protocol_request = raw_ffi::xcb_protocol_request_t {
            count: bufs.len(),
            ext: null_mut(),
            opcode: 0,
            isvoid: if has_reply { 0 } else { 1 }
        };
        let mut flags = raw_ffi::send_request_flags::RAW;
        if has_reply {
            flags |= raw_ffi::send_request_flags::CHECKED;
        }
        // FIXME: xcb wants to be able to access bufs[-1] and bufs[-2]
        unsafe {
            raw_ffi::xcb_send_request64(self.0, flags, bufs.as_ptr(), &protocol_request)
        }
    }

    fn discard_reply(&self, sequence: SequenceNumber) {
        unsafe {
            raw_ffi::xcb_discard_reply64(self.0, sequence);
        }
    }

    fn wait_for_reply(&self, sequence: SequenceNumber) -> Result<CSlice, ConnectionErrorOrX11Error> {
        unsafe {
            let mut error = null_mut();
            let reply = raw_ffi::xcb_wait_for_reply64(self.0, sequence, &mut error);
            assert!(reply == null_mut() || error == null_mut());
            if reply != null_mut() {
                let header = CSlice::new(reply as _, 32);

                let length_field = u32::from_ne_bytes(header[4..8].try_into().unwrap());
                let length_field: usize = length_field.try_into()?;

                let length = 32 + length_field * 4;
                Ok(CSlice::new(header.into_ptr(), length))
            } else {
                let error: GenericError = CSlice::new(error as _, 32).try_into()?;
                Err(error.into())
            }
        }
    }

    pub fn wait_for_event(&self) -> Result<GenericEvent, ConnectionError> {
        unsafe {
            let event = raw_ffi::xcb_wait_for_event(self.0);
            if event.is_null() {
                return Err(ConnectionError::from_connection(self.0));
            }
            let generic_event: Result<GenericEvent, _> = CSlice::new(event as _, 32).try_into();
            let generic_event = generic_event.or(Err(ConnectionError::UnknownError))?; // FIXME: error type
            assert_ne!(35, generic_event.response_type()); // FIXME: XGE events may have sizes > 32
            Ok(generic_event)
        }
    }

    pub fn has_error(&self) -> Option<ConnectionError> {
        unsafe {
            let error = raw_ffi::xcb_connection_has_error(self.0);
            if error == 0 {
                None
            } else {
                Some(ConnectionError::from_c_error(error))
            }
        }
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        unsafe {
            raw_ffi::xcb_disconnect(self.0);
        }
    }
}

#[derive(Debug)]
pub struct Cookie<'a, R> {
    connection: &'a Connection,
    sequence_number: Option<SequenceNumber>,
    phantom: std::marker::PhantomData<R>
}

impl<R> Cookie<'_, R>
where R: TryFrom<CSlice, Error=ParseError>
{
    fn new(connection: &Connection, sequence_number: SequenceNumber) -> Cookie<R> {
        Cookie {
            connection,
            sequence_number: Some(sequence_number),
            phantom: PhantomData
        }
    }

    pub fn reply(mut self) -> Result<R, ConnectionErrorOrX11Error> {
        let reply = self.connection.wait_for_reply(self.sequence_number.take().unwrap())?;
        Ok(reply.try_into()?)
    }
}

impl<R> Drop for Cookie<'_, R> {
    fn drop(&mut self) {
        if let Some(number) = self.sequence_number {
            self.connection.discard_reply(number);
        }
    }
}

pub trait Event {
    fn raw_bytes(&self) -> &[u8];

    fn raw_response_type(&self) -> u8 {
        self.raw_bytes()[0]
    }

    fn response_type(&self) -> u8 {
        self.raw_response_type() & 0x7f
    }

    fn server_generated(&self) -> bool {
        self.raw_response_type() & 0x80 == 0
    }

    fn raw_sequence_number(&self) -> Option<u16> {
        use crate::generated::xproto::KEYMAP_NOTIFY_EVENT;
        match self.response_type() {
            KEYMAP_NOTIFY_EVENT => None,
            _ => {
                let bytes = self.raw_bytes();
                Some(u16::from_ne_bytes([bytes[2], bytes[3]]))
            }
        }
    }
}

#[derive(Debug)]
pub struct GenericEvent(CSlice);

impl Event for GenericEvent {
    fn raw_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl Into<CSlice> for GenericEvent {
    fn into(self) -> CSlice {
        self.0
    }
}

// FIXME: Have these (or at least one of them?) generated
const REPLY: u8 = 1;
const XGE_EVENT: u8 = 35;

impl TryFrom<CSlice> for GenericEvent {
    type Error = ParseError;

    fn try_from(value: CSlice) -> Result<Self, Self::Error> {
        if value.len() < 32 {
            return Err(ParseError::ParseError);
        }
        let length_field = u32::from_ne_bytes([value[4], value[5], value[6], value[7]]);
        let length_field: usize = length_field.try_into()?;
        let actual_length = value.len();
        let event = GenericEvent(value);
        let expected_length = match event.response_type() {
            XGE_EVENT | REPLY => 32 + 4 * length_field,
            _ => 32
        };
        if actual_length != expected_length {
            return Err(ParseError::ParseError);
        }
        Ok(event)
    }
}

#[derive(Debug)]
pub struct GenericError(CSlice);

impl GenericError {
    pub fn error_code(&self) -> u8 {
        self.raw_bytes()[1]
    }
}

impl Event for GenericError {
    fn raw_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl Into<CSlice> for GenericError {
    fn into(self) -> CSlice {
        self.0
    }
}

impl TryFrom<GenericEvent> for GenericError {
    type Error = ParseError;

    fn try_from(event: GenericEvent) -> Result<Self, Self::Error> {
        if event.response_type() != 0 {
            return Err(ParseError::ParseError)
        }
        Ok(GenericError(event.into()))
    }
}

impl TryFrom<CSlice> for GenericError {
    type Error = ParseError;

    fn try_from(value: CSlice) -> Result<Self, Self::Error> {
        let event: GenericEvent = value.try_into()?;
        event.try_into()
    }
}

mod raw_ffi {
    use libc::{c_void, c_int, c_char};
    use std::io::IoSlice;

    #[allow(non_camel_case_types)]
    #[repr(C)]
    pub struct xcb_connection_t {
        _unused: [u8; 0]
    }

    #[allow(non_camel_case_types)]
    #[repr(C)]
    pub struct xcb_extension_t {
        pub name: *const c_char,
        pub global_id: c_int
    }

    #[allow(non_camel_case_types)]
    #[repr(C)]
    pub struct xcb_protocol_request_t {
        pub count: usize,
        pub ext: *mut xcb_extension_t,
        pub opcode: u8,
        pub isvoid: u8
    }

    pub mod connection_errors {
        pub const ERROR: i32 = 1;
        pub const EXT_NOTSUPPORTED: i32 = 2;
        pub const MEM_INSUFFICIENT: i32 = 3;
        pub const REQ_LEN_EXCEED: i32 = 4;
        pub const PARSE_ERR: i32 = 5;
        pub const INVALID_SCREEN: i32 = 6;
        pub const FDPASSING_FAILED: i32 = 7;
    }

    pub mod send_request_flags {
        use libc::c_int;

        pub const CHECKED: c_int = 1;
        pub const RAW: c_int = 2;
        //pub const DISCARD_REPLY: c_int = 4;
        //pub const REPLY_FDS: c_int = 8;
    }

    #[link(name = "xcb")]
    extern {
        pub fn xcb_connect(displayname: *const c_char, screenp: *mut c_int ) -> *mut xcb_connection_t;
        pub fn xcb_disconnect(c: *mut xcb_connection_t);
        pub fn xcb_connection_has_error(c: *const xcb_connection_t) -> c_int;
        pub fn xcb_send_request64(c: *const xcb_connection_t, flags: c_int, vector: *const IoSlice, request: *const xcb_protocol_request_t) -> u64;
        pub fn xcb_discard_reply64(c: *const xcb_connection_t, sequence: u64);
        pub fn xcb_wait_for_reply64(c: *const xcb_connection_t, request: u64, e: *mut * mut c_void) -> *mut c_void;
        pub fn xcb_wait_for_event(c: *const xcb_connection_t) -> *mut c_void;
        pub fn xcb_flush(c: *const xcb_connection_t) -> c_int;
        pub fn xcb_generate_id(c: *const xcb_connection_t) -> u32;
        pub fn xcb_get_setup(c: *const xcb_connection_t) -> *const u8;
    }
}
