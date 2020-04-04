use std::ptr::NonNull;

#[cfg(not(all(test, unix)))]
use libc::c_void;
#[cfg(unix)]
pub(crate) use libc::iovec;
use libc::{c_char, c_int, c_uint};

// As defined in xcb_windefs.h
#[cfg(not(unix))]
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub(crate) struct iovec {
    iov_base: *mut c_void,
    iov_len: c_int,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct xcb_connection_t {
    _unused: [u8; 0],
}

#[derive(Debug)]
pub(crate) struct XCBConnectionWrapper {
    ptr: NonNull<xcb_connection_t>,
    should_drop: bool,
}

// libxcb is fully thread-safe (well, except for xcb_disconnect()), so the following is
// actually fine and safe:
unsafe impl Send for XCBConnectionWrapper {}
unsafe impl Sync for XCBConnectionWrapper {}

impl Drop for XCBConnectionWrapper {
    fn drop(&mut self) {
        if self.should_drop {
            unsafe {
                xcb_disconnect(self.ptr.as_ptr());
            }
        }
    }
}

impl XCBConnectionWrapper {
    pub(crate) unsafe fn new(ptr: *mut xcb_connection_t, should_drop: bool) -> Self {
        Self {
            ptr: NonNull::new_unchecked(ptr),
            should_drop,
        }
    }

    pub(crate) fn as_ptr(&self) -> *mut xcb_connection_t {
        self.ptr.as_ptr()
    }
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct xcb_extension_t {
    pub(crate) name: *const c_char,
    pub(crate) global_id: c_int,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct xcb_void_cookie_t {
    pub(crate) sequence: c_uint,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct xcb_protocol_request_t {
    pub(crate) count: usize,
    pub(crate) ext: *mut xcb_extension_t,
    pub(crate) opcode: u8,
    pub(crate) isvoid: u8,
}

pub(crate) mod connection_errors {
    pub(crate) const ERROR: i32 = 1;
    pub(crate) const EXT_NOTSUPPORTED: i32 = 2;
    pub(crate) const MEM_INSUFFICIENT: i32 = 3;
    pub(crate) const REQ_LEN_EXCEED: i32 = 4;
    pub(crate) const PARSE_ERR: i32 = 5;
    pub(crate) const INVALID_SCREEN: i32 = 6;
    pub(crate) const FDPASSING_FAILED: i32 = 7;
}

pub(crate) mod send_request_flags {
    use libc::c_int;

    pub(crate) const CHECKED: c_int = 1;
    pub(crate) const RAW: c_int = 2;
    //pub(crate) const DISCARD_REPLY: c_int = 4;
    pub(crate) const REPLY_FDS: c_int = 8;
}

#[cfg(not(test))]
#[link(name = "xcb")]
extern "C" {
    pub(crate) fn xcb_connect(
        displayname: *const c_char,
        screenp: *mut c_int,
    ) -> *mut xcb_connection_t;
    pub(crate) fn xcb_disconnect(c: *mut xcb_connection_t);
    pub(crate) fn xcb_connection_has_error(c: *const xcb_connection_t) -> c_int;
    pub(crate) fn xcb_send_request64(
        c: *const xcb_connection_t,
        flags: c_int,
        vector: *mut iovec,
        request: *const xcb_protocol_request_t,
    ) -> u64;
    pub(crate) fn xcb_send_request_with_fds64(
        c: *const xcb_connection_t,
        flags: c_int,
        vector: *mut iovec,
        request: *const xcb_protocol_request_t,
        num_fds: c_uint,
        fds: *mut c_int,
    ) -> u64;
    pub(crate) fn xcb_discard_reply64(c: *const xcb_connection_t, sequence: u64);
    pub(crate) fn xcb_wait_for_reply64(
        c: *const xcb_connection_t,
        request: u64,
        e: *mut *mut c_void,
    ) -> *mut c_void;
    pub(crate) fn xcb_poll_for_reply(
        c: *const xcb_connection_t,
        request: c_uint,
        reply: *mut *mut c_void,
        e: *mut *mut c_void,
    ) -> c_int;
    pub(crate) fn xcb_request_check(
        c: *const xcb_connection_t,
        void_cookie: xcb_void_cookie_t,
    ) -> *mut c_void;
    pub(crate) fn xcb_wait_for_event(c: *const xcb_connection_t) -> *mut c_void;
    pub(crate) fn xcb_poll_for_event(c: *const xcb_connection_t) -> *mut c_void;
    pub(crate) fn xcb_flush(c: *const xcb_connection_t) -> c_int;
    pub(crate) fn xcb_generate_id(c: *const xcb_connection_t) -> u32;
    pub(crate) fn xcb_get_setup(c: *const xcb_connection_t) -> *const u8;
    #[cfg(unix)]
    pub(crate) fn xcb_get_file_descriptor(c: *const xcb_connection_t) -> c_int;
    pub(crate) fn xcb_get_maximum_request_length(c: *const xcb_connection_t) -> u32;
    pub(crate) fn xcb_prefetch_maximum_request_length(c: *const xcb_connection_t);
}

#[cfg(test)]
mod mock {
    use std::ffi::CStr;

    use libc::{c_char, c_int, c_uint, c_void};

    use super::{iovec, xcb_connection_t, xcb_protocol_request_t, xcb_void_cookie_t};
    use crate::x11_utils::Serialize;
    use crate::xproto::{ImageOrder, Setup};

    #[repr(C)]
    struct ConnectionMock {
        error: c_int,
        setup: Vec<u8>,
    }

    pub(crate) unsafe fn xcb_connect(
        displayname: *const c_char,
        screenp: *mut c_int,
    ) -> *mut xcb_connection_t {
        // Test that the provided displayname is correct
        if CStr::from_ptr(displayname).to_str().unwrap() != "display name" {
            panic!("Did not get the expected displayname");
        }
        std::ptr::write(screenp, 0);

        let length_field = 10;
        let setup = Setup {
            status: 0,
            protocol_major_version: 0,
            protocol_minor_version: 0,
            length: length_field,
            release_number: 0,
            resource_id_base: 0,
            resource_id_mask: 0,
            motion_buffer_size: 0,
            maximum_request_length: 0,
            image_byte_order: ImageOrder::LSBFirst,
            bitmap_format_bit_order: ImageOrder::LSBFirst,
            bitmap_format_scanline_unit: 0,
            bitmap_format_scanline_pad: 0,
            min_keycode: 0,
            max_keycode: 0,
            vendor: Default::default(),
            pixmap_formats: Default::default(),
            roots: Default::default(),
        };
        let setup = setup.serialize();
        assert_eq!(setup.len(), 4 * length_field as usize);

        let mock = ConnectionMock { error: 0, setup };
        Box::into_raw(Box::new(mock)) as _
    }

    pub(crate) unsafe fn xcb_disconnect(c: *mut xcb_connection_t) {
        // The pointer is suitable aligned since our xcb_connect() mock above created it
        #[allow(clippy::cast_ptr_alignment)]
        let _ = Box::from_raw(c as *mut ConnectionMock);
    }

    pub(crate) unsafe fn xcb_connection_has_error(c: *const xcb_connection_t) -> c_int {
        // The pointer is suitable aligned since our xcb_connect() mock above created it
        #[allow(clippy::cast_ptr_alignment)]
        (*(c as *const ConnectionMock)).error
    }

    pub(crate) unsafe fn xcb_send_request64(
        _c: *const xcb_connection_t,
        _flags: c_int,
        _vector: *mut iovec,
        _request: *const xcb_protocol_request_t,
    ) -> u64 {
        unimplemented!();
    }

    pub(crate) unsafe fn xcb_send_request_with_fds64(
        _c: *const xcb_connection_t,
        _flags: c_int,
        _vector: *mut iovec,
        _request: *const xcb_protocol_request_t,
        _num_fds: c_uint,
        _fds: *mut c_int,
    ) -> u64 {
        unimplemented!();
    }

    pub(crate) unsafe fn xcb_discard_reply64(_c: *const xcb_connection_t, _sequence: u64) {
        unimplemented!();
    }

    pub(crate) unsafe fn xcb_wait_for_reply64(
        _c: *const xcb_connection_t,
        _request: u64,
        _e: *mut *mut c_void,
    ) -> *mut c_void {
        unimplemented!();
    }

    pub(crate) unsafe fn xcb_poll_for_reply(
        _c: *const xcb_connection_t,
        _request: c_uint,
        _reply: *mut *mut c_void,
        _e: *mut *mut c_void,
    ) -> c_int {
        unimplemented!();
    }

    pub(crate) unsafe fn xcb_request_check(
        _c: *const xcb_connection_t,
        _void_cookie: xcb_void_cookie_t,
    ) -> *mut c_void {
        unimplemented!();
    }

    pub(crate) unsafe fn xcb_wait_for_event(_c: *const xcb_connection_t) -> *mut c_void {
        unimplemented!();
    }

    pub(crate) unsafe fn xcb_poll_for_event(_c: *const xcb_connection_t) -> *mut c_void {
        unimplemented!();
    }

    pub(crate) unsafe fn xcb_flush(_c: *const xcb_connection_t) -> c_int {
        unimplemented!();
    }

    pub(crate) unsafe fn xcb_generate_id(_c: *const xcb_connection_t) -> u32 {
        unimplemented!();
    }

    pub(crate) unsafe fn xcb_get_setup(c: *const xcb_connection_t) -> *const u8 {
        // The pointer is suitable aligned since our xcb_connect() mock above created it
        #[allow(clippy::cast_ptr_alignment)]
        (*(c as *const ConnectionMock)).setup.as_ptr()
    }

    #[cfg(unix)]
    pub(crate) unsafe fn xcb_get_file_descriptor(_c: *const xcb_connection_t) -> c_int {
        unimplemented!();
    }

    pub(crate) unsafe fn xcb_get_maximum_request_length(_c: *const xcb_connection_t) -> u32 {
        unimplemented!();
    }

    pub(crate) unsafe fn xcb_prefetch_maximum_request_length(_c: *const xcb_connection_t) {
        unimplemented!();
    }
}

#[cfg(test)]
pub(crate) use mock::*;
