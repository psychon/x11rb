extern crate libc;

use std::ptr::{null, null_mut};
use std::ops::Deref;
use std::marker::PhantomData;
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::io::{IoSlice, Error as IOError, ErrorKind::Other};
use std::mem::forget;
use libc::free;

pub type SequenceNumber = u64;

pub struct Connection(*mut raw_ffi::xcb_connection_t);

pub enum ConnectionError {
    ConnectionError,
    UnsupportedExtension,
    InsufficientMemory,
    MaximumRequestLengthExceeded,
    DisplayParsingError,
    InvalidScreen,
    FDPassingFailed,
    UnknownError
}

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
    fn from_c_error(error: u32) -> ConnectionError {
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
    pub fn new() -> Result<Connection, ConnectionError>  {
        unsafe {
            let connection = Connection(raw_ffi::xcb_connect(null(), null_mut()));
            let error = raw_ffi::xcb_connection_has_error(connection.0);
            if error != 0 {
                Err(ConnectionError::from_c_error(error.try_into().or(Err(ConnectionError::UnknownError))?))
            } else {
                Ok(connection)
            }
        }
    }

    pub fn flush(&self) {
        unsafe { raw_ffi::xcb_flush(self.0); }
    }

    pub fn send_request_with_reply<R>(&self, bufs: &[IoSlice]) -> Result<Cookie<R>, Box<dyn Error>> {
        Ok(Cookie::new(self, self.send_request(bufs, true)?))
    }

    pub fn send_request_without_reply(&self, bufs: &[IoSlice]) -> Result<SequenceNumber, Box<dyn Error>> {
        self.send_request(bufs, false)
    }

    fn send_request(&self, bufs: &[IoSlice], has_reply: bool) -> Result<SequenceNumber, Box<dyn Error>> {
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
        let sequence = unsafe {
            raw_ffi::xcb_send_request64(self.0, flags, bufs.as_ptr(), &protocol_request)
        };
        Ok(sequence)
    }

    fn discard_reply(&self, sequence: SequenceNumber) {
        unsafe {
            raw_ffi::xcb_discard_reply64(self.0, sequence);
        }
    }

    fn wait_for_reply(&self, sequence: SequenceNumber) -> Result<CSlice, Box<dyn Error>> {
        unsafe {
            let mut error = null_mut();
            let reply = raw_ffi::xcb_wait_for_reply64(self.0, sequence, &mut error);
            assert!(reply == null_mut() || error == null_mut());
            if reply != null_mut() {
                let header = CSlice::new(reply as _, 32);

                let length_field = u32::from_ne_bytes(header[4..8].try_into().unwrap());
                let other_error: IOError = Other.into();
                let other_error: Result<_, Box<dyn Error>> = Err(other_error.into());
                let length_field: usize = length_field.try_into().or(other_error)?;

                let length = 32 + length_field * 4;
                Ok(CSlice::new(header.into_ptr(), length))
            } else {
                let error = CSlice::new(error as _, 32);

                // The return type is incorrect currently and I am lazy
                let _ = error;
                unimplemented!();
            }
        }
    }

    pub fn wait_for_event(&self) -> Result<CSlice, Box<dyn Error>> {
        unsafe {
            let event = raw_ffi::xcb_wait_for_event(self.0);
            assert_ne!(35, (*event).response_type); // FIXME: XGE events may have sizes > 32
            Ok(CSlice::new(event as _, 32))
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

pub struct Cookie<'a, R> {
    connection: &'a Connection,
    sequence_number: Option<SequenceNumber>,
    phantom: std::marker::PhantomData<R>
}

impl<R> Cookie<'_, R> {
    fn new(connection: &Connection, sequence_number: SequenceNumber) -> Cookie<R> {
        Cookie {
            connection,
            sequence_number: Some(sequence_number),
            phantom: PhantomData
        }
    }
}

impl<'a, R> Cookie<'_, R>
where R: TryFrom<CSlice, Error=Box<dyn Error>>,
{
    pub fn reply(mut self) -> Result<R, Box<dyn Error>> {
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

    #[allow(non_camel_case_types)]
    #[repr(C)]
    pub struct xcb_generic_error_t {
        pub response_type: u8,
        pub error_code: u8,
        pub sequence: u16,
        pub resource_id: u32,
        pub minor_code: u16,
        pub major_code: u8,
        pub pad0: u8,
        pub pad: [u32; 5],
        pub full_sequence: u32
    }

    #[allow(non_camel_case_types)]
    #[repr(C)]
    pub struct xcb_generic_event_t {
        pub response_type: u8,
        pub pad0: u8,
        pub sequence: u16,
        pub pad: [u32; 7],
        pub full_sequence: u32
    }

    pub mod connection_errors {
        pub const ERROR: u32 = 1;
        pub const EXT_NOTSUPPORTED: u32 = 2;
        pub const MEM_INSUFFICIENT: u32 = 3;
        pub const REQ_LEN_EXCEED: u32 = 4;
        pub const PARSE_ERR: u32 = 5;
        pub const INVALID_SCREEN: u32 = 6;
        pub const FDPASSING_FAILED: u32 = 7;
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
        pub fn xcb_wait_for_reply64(c: *const xcb_connection_t, request: u64, e: *mut * mut xcb_generic_error_t) -> *mut c_void;
        pub fn xcb_wait_for_event(c: *const xcb_connection_t) -> *mut xcb_generic_event_t;
        pub fn xcb_flush(c: *const xcb_connection_t) -> c_int;
    }
}
