//! A FFI-based connection to an X11 server, using libxcb.

use std::ptr::{null, null_mut};
use std::convert::{TryFrom, TryInto};
use std::ffi::CStr;
use std::io::IoSlice;
use std::ops::Deref;
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, RawFd};
use libc::c_void;
use crate::utils::{CSlice, Buffer, RawFdContainer};
use crate::x11_utils::{GenericError, GenericEvent, Event};
use crate::errors::{ParseError, ConnectionError, ConnectionErrorOrX11Error};
use crate::connection::{Connection, Cookie, CookieWithFds, SequenceNumber, ExtensionInformation};
use super::generated::xproto::{Setup, QueryExtensionReply};

/// A connection to an X11 server.
///
/// This type wraps `*mut xcb_connection_t` that is provided by libxcb. It provides a rust
/// interface to this C library.
#[derive(Debug)]
pub struct XCBConnection(raw_ffi::XCBConnectionWrapper, Setup, ExtensionInformation);

impl XCBConnection {
    unsafe fn connection_error_from_connection(c: *const raw_ffi::xcb_connection_t) -> ConnectionError {
        Self::connection_error_from_c_error(raw_ffi::xcb_connection_has_error(c))
    }

    fn connection_error_from_c_error(error: i32) -> ConnectionError {
        use crate::xcb_ffi::raw_ffi::connection_errors::*;

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

    /// Establish a new connection to an X11 server.
    ///
    /// If a `dpy_name` is provided, it describes the display that should be connected to, for
    /// example `127.0.0.1:1`. If no value is provided, the `$DISPLAY` environment variable is
    /// used.
    pub fn connect(dpy_name: Option<&CStr>) -> Result<(XCBConnection, usize), ConnectionError>  {
        use libc::c_int;
        unsafe {
            let mut screen: c_int = 0;
            let dpy_ptr = dpy_name.map_or(null(), |s| s.as_ptr());
            let connection = raw_ffi::xcb_connect(dpy_ptr, &mut screen);
            let error = raw_ffi::xcb_connection_has_error(connection);
            if error != 0 {
                raw_ffi::xcb_disconnect(connection);
                Err(Self::connection_error_from_c_error(error.try_into().or(Err(ConnectionError::UnknownError))?))
            } else {
                let setup = raw_ffi::xcb_get_setup(connection);
                let conn = XCBConnection(raw_ffi::XCBConnectionWrapper(connection), Self::parse_setup(setup)?, Default::default());
                Ok((conn, screen as usize))
            }
        }
    }

    unsafe fn parse_setup(setup: *const u8) -> Result<Setup, ParseError> {
        use std::slice::from_raw_parts;

        // We know that the setup information has at least eight bytes.
        // Use a slice instead of Buffer::CSlice since we must not free() the xcb_setup_t that libxcb owns.
        let wrapper = from_raw_parts(setup, 8);

        // The length field is in the last two bytes
        let length = u16::from_ne_bytes([wrapper[6], wrapper[7]]);

        // The length is in four-byte-units after the known header
        let length = length * 4 + 8;

        let slice = from_raw_parts(wrapper.as_ptr(), length.try_into()?);
        let result = Setup::try_from(&*slice)?;

        Ok(result)
    }

    fn send_request(&self, bufs: &[IoSlice], fds: Vec<RawFdContainer>, has_reply: bool, reply_has_fds: bool) -> Result<SequenceNumber, ConnectionError> {
        // For this, we derefence the IoSlices, add two new entries, and create new IoSlices.
        let mut new_bufs = Vec::with_capacity(2 + bufs.len());

        // XCB wants to access bufs[-1] and bufs[-2], so we need to add two empty items in front.
        new_bufs.push(&[][..]);
        new_bufs.push(&[][..]);

        // Add the actual request buffers
        let mut storage = Default::default();
        new_bufs.extend(self.compute_length_field(bufs, &mut storage)?.iter().map(Deref::deref));

        // Now wrap the buffers with IoSlice
        let mut new_bufs = new_bufs.into_iter().map(IoSlice::new).collect::<Vec<_>>();

        // Set up the information that libxcb needs
        let protocol_request = raw_ffi::xcb_protocol_request_t {
            count: bufs.len(),
            ext: null_mut(), // Not needed since we always use raw
            opcode: 0,
            isvoid: if has_reply { 0 } else { 1 }
        };
        let mut flags = raw_ffi::send_request_flags::RAW;
        assert!(has_reply || !reply_has_fds);
        if has_reply {
            flags |= raw_ffi::send_request_flags::CHECKED;
        }
        if reply_has_fds {
            flags |= raw_ffi::send_request_flags::REPLY_FDS;
        }

        // Convert the FDs into an array of ints. libxcb will close the FDs.
        let fds: Vec<_> = fds.into_iter().map(RawFdContainer::into_raw_fd).collect();

        let seqno = if fds.is_empty() {
            unsafe { raw_ffi::xcb_send_request64((self.0).0, flags, &mut new_bufs[2], &protocol_request) }
        } else {
            let num_fds = fds.len().try_into().unwrap();
            let fds_ptr = fds.as_ptr();
            unsafe { raw_ffi::xcb_send_request_with_fds64((self.0).0, flags, &mut new_bufs[2], &protocol_request, num_fds, fds_ptr) }
        };
        if seqno == 0 {
            unsafe { Err(XCBConnection::connection_error_from_connection((self.0).0)) }
        } else {
            Ok(seqno)
        }
    }

    /// Check if the underlying XCB connection is in an error state.
    pub fn has_error(&self) -> Option<ConnectionError> {
        unsafe {
            let error = raw_ffi::xcb_connection_has_error((self.0).0);
            if error == 0 {
                None
            } else {
                Some(Self::connection_error_from_c_error(error))
            }
        }
    }

    /// Get access to the raw libxcb `xcb_connection_t`.
    ///
    /// The returned pointer is valid for as long as the original object was not dropped. No
    /// ownerhsip is transferred.
    pub fn get_raw_xcb_connection(&self) -> *mut c_void {
        (self.0).0 as _
    }
}

impl Connection for XCBConnection {
    fn send_request_with_reply<R>(&self, bufs: &[IoSlice], fds: Vec<RawFdContainer>) -> Result<Cookie<Self, R>, ConnectionError>
        where R: TryFrom<Buffer, Error=ParseError>
    {
        Ok(Cookie::new(self, self.send_request(bufs, fds, true, false)?))
    }

    fn send_request_with_reply_with_fds<R>(&self, bufs: &[IoSlice], fds: Vec<RawFdContainer>) -> Result<CookieWithFds<Self, R>, ConnectionError>
        where R: TryFrom<(Buffer, Vec<RawFdContainer>), Error=ParseError>
    {
        Ok(CookieWithFds::new(self, self.send_request(bufs, fds, true, true)?))
    }

    fn send_request_without_reply(&self, bufs: &[IoSlice], fds: Vec<RawFdContainer>) -> Result<SequenceNumber, ConnectionError> {
        self.send_request(bufs, fds, false, false)
    }

    fn discard_reply(&self, sequence: SequenceNumber) {
        unsafe {
            raw_ffi::xcb_discard_reply64((self.0).0, sequence);
        }
    }

    fn extension_information(&self, extension_name: &'static str) -> Option<&QueryExtensionReply> {
        self.2.extension_information(self, extension_name)
    }

    fn wait_for_reply(&self, sequence: SequenceNumber) -> Result<Buffer, ConnectionErrorOrX11Error> {
        unsafe {
            let mut error = null_mut();
            let reply = raw_ffi::xcb_wait_for_reply64((self.0).0, sequence, &mut error);

            // At least one of these pointers must be NULL.
            assert!(reply == null_mut() || error == null_mut());

            // If both pointers are NULL, the xcb connection must be in an error state
            if reply == null_mut() && error == null_mut() {
                Err(Self::connection_error_from_connection((self.0).0))?
            }

            if reply != null_mut() {
                let header = CSlice::new(reply as _, 32);

                let length_field = u32::from_ne_bytes(header[4..8].try_into().unwrap());
                let length_field: usize = length_field.try_into()?;

                let length = 32 + length_field * 4;
                Ok(Buffer::from_raw_parts(header.into_ptr(), length))
            } else {
                let error: GenericError = Buffer::from_raw_parts(error as _, 32).try_into()?;
                Err(error.into())
            }
        }
    }

    #[cfg(unix)]
    fn wait_for_reply_with_fds(&self, sequence: SequenceNumber) -> Result<(Buffer, Vec<RawFdContainer>), ConnectionErrorOrX11Error> {
        let buffer = self.wait_for_reply(sequence)?;

        // Get a pointer to the array of integers where libxcb saved the FD numbers
        let fd_ptr = match &buffer {
            &Buffer::Vec(_) => unreachable!(), // wait_for_reply() always returns a CSlice
            &Buffer::CSlice(ref slice) => {
                // libxcb saves the list of FDs after the data of the reply
                (unsafe { slice.as_ptr().add(slice.len()) }) as *const RawFd
            }
        };

        // The number of FDs is in the second byte (= buffer[1]) in all replies.
        let vector = unsafe { std::slice::from_raw_parts(fd_ptr, buffer[1] as usize) };
        let vector = vector.iter().map(|&fd| RawFdContainer::new(fd)).collect();

        Ok((buffer, vector))
    }

    #[cfg(not(unix))]
    fn wait_for_reply_with_fds(&self, _sequence: SequenceNumber) -> Result<(Buffer, Vec<RawFdContainer>), ConnectionErrorOrX11Error> {
        unimplemented!("FD passing is currently only implemented on Unix-like systems")
    }

    fn wait_for_event(&self) -> Result<GenericEvent, ConnectionError> {
        unsafe {
            let event = raw_ffi::xcb_wait_for_event((self.0).0);
            if event.is_null() {
                return Err(Self::connection_error_from_connection((self.0).0));
            }
            let generic_event: GenericEvent = Buffer::from_raw_parts(event as _, 32).try_into()?;
            assert_ne!(35, generic_event.response_type()); // FIXME: XGE events may have sizes > 32
            Ok(generic_event)
        }
    }

    fn poll_for_event(&self) -> Result<Option<GenericEvent>, ConnectionError> {
        unsafe {
            let event = raw_ffi::xcb_poll_for_event((self.0).0);
            if event.is_null() {
                let err = raw_ffi::xcb_connection_has_error((self.0).0);
                if err == 0 {
                    return Ok(None);
                } else {
                    return Err(Self::connection_error_from_c_error(err));
                }
            }
            let generic_event: GenericEvent = Buffer::from_raw_parts(event as _, 32).try_into()?;
            assert_ne!(35, generic_event.response_type()); // FIXME: XGE events may have sizes > 32
            Ok(Some(generic_event))
        }
    }

    fn flush(&self) {
        // xcb_flush() returns 0 if the connection is in (or just entered) an error state, else 1.
        // Adding a Result<(), ConnectionError> as a return value here would be too noisy, I think,
        // so just ignore this return value.
        let _ = unsafe { raw_ffi::xcb_flush((self.0).0) };
    }

    fn generate_id(&self) -> u32 {
        unsafe { raw_ffi::xcb_generate_id((self.0).0) }
    }

    fn setup(&self) -> &Setup {
        &self.1
    }

    fn maximum_request_bytes(&self) -> usize {
        4 * unsafe { raw_ffi::xcb_get_maximum_request_length((self.0).0) as usize }
    }
}

impl Drop for XCBConnection {
    fn drop(&mut self) {
        unsafe {
            raw_ffi::xcb_disconnect((self.0).0 as *mut raw_ffi::xcb_connection_t);
        }
    }
}

#[cfg(unix)]
impl AsRawFd for XCBConnection {
    fn as_raw_fd(&self) -> RawFd {
        unsafe {
            raw_ffi::xcb_get_file_descriptor((self.0).0)
        }
    }
}

mod raw_ffi {
    use libc::{c_void, c_int, c_char, c_uint};
    use std::io::IoSlice;

    #[allow(non_camel_case_types)]
    #[repr(C)]
    pub(crate) struct xcb_connection_t {
        _unused: [u8; 0]
    }

    #[derive(Debug)]
    pub(crate) struct XCBConnectionWrapper(pub(crate) *const xcb_connection_t);

    // libxcb is fully thread-safe (well, except for xcb_disconnect()), so the following is
    // actually fine and safe:
    unsafe impl Send for XCBConnectionWrapper {}
    unsafe impl Sync for XCBConnectionWrapper {}

    #[allow(non_camel_case_types)]
    #[repr(C)]
    pub(crate) struct xcb_extension_t {
        pub(crate) name: *const c_char,
        pub(crate) global_id: c_int
    }

    #[allow(non_camel_case_types)]
    #[repr(C)]
    pub(crate) struct xcb_protocol_request_t {
        pub(crate) count: usize,
        pub(crate) ext: *mut xcb_extension_t,
        pub(crate) opcode: u8,
        pub(crate) isvoid: u8
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

    #[link(name = "xcb")]
    extern {
        pub(crate) fn xcb_connect(displayname: *const c_char, screenp: *mut c_int ) -> *mut xcb_connection_t;
        pub(crate) fn xcb_disconnect(c: *mut xcb_connection_t);
        pub(crate) fn xcb_connection_has_error(c: *const xcb_connection_t) -> c_int;
        pub(crate) fn xcb_send_request64(c: *const xcb_connection_t, flags: c_int, vector: *mut IoSlice, request: *const xcb_protocol_request_t) -> u64;
        pub(crate) fn xcb_send_request_with_fds64(c: *const xcb_connection_t, flags: c_int, vector: *mut IoSlice, request: *const xcb_protocol_request_t, num_fds: c_uint, fds: *const c_int) -> u64;
        pub(crate) fn xcb_discard_reply64(c: *const xcb_connection_t, sequence: u64);
        pub(crate) fn xcb_wait_for_reply64(c: *const xcb_connection_t, request: u64, e: *mut * mut c_void) -> *mut c_void;
        pub(crate) fn xcb_wait_for_event(c: *const xcb_connection_t) -> *mut c_void;
        pub(crate) fn xcb_poll_for_event(c: *const xcb_connection_t) -> *mut c_void;
        pub(crate) fn xcb_flush(c: *const xcb_connection_t) -> c_int;
        pub(crate) fn xcb_generate_id(c: *const xcb_connection_t) -> u32;
        pub(crate) fn xcb_get_setup(c: *const xcb_connection_t) -> *const u8;
        #[cfg(unix)]
        pub(crate) fn xcb_get_file_descriptor(c: *const xcb_connection_t) -> c_int;
        pub(crate) fn xcb_get_maximum_request_length(c: *const xcb_connection_t) -> u32;
    }
}
