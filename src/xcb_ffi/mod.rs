//! A FFI-based connection to an X11 server, using libxcb.

#![allow(clippy::cast_ptr_alignment)] // FIXME: Remove this

use std::ptr::{null, null_mut};
use std::convert::{TryFrom, TryInto};
use std::ffi::CStr;
use std::io::IoSlice;
use std::ops::Deref;
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, RawFd};
use libc::c_void;
use crate::utils::{CSlice, Buffer, RawFdContainer};
use crate::x11_utils::{GenericError, GenericEvent};
use crate::errors::{ParseError, ConnectionError, ConnectionErrorOrX11Error};
use crate::connection::{RequestConnection, Connection, SequenceNumber, RequestKind, DiscardMode};
use crate::cookie::{VoidCookie, Cookie, CookieWithFds};
use crate::extension_information::ExtensionInformation;
use super::generated::xproto::{Setup, QueryExtensionReply};

mod pending_errors;
mod raw_ffi;

/// A connection to an X11 server.
///
/// This type wraps `*mut xcb_connection_t` that is provided by libxcb. It provides a rust
/// interface to this C library.
#[derive(Debug)]
pub struct XCBConnection {
    conn: raw_ffi::XCBConnectionWrapper,
    setup: Setup,
    ext_info: ExtensionInformation,
    errors: pending_errors::PendingErrors,
    should_drop: bool
}

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
                let conn = XCBConnection {
                    conn: raw_ffi::XCBConnectionWrapper(connection),
                    setup: Self::parse_setup(setup)?,
                    ext_info: Default::default(),
                    errors: Default::default(),
                    should_drop: true
                };
                Ok((conn, screen as usize))
            }
        }
    }

    /// Create a connection wrapper for a raw libxcb `xcb_connection_t`.
    ///
    /// It is the caller's responsibility to ensure the connection is valid and lives longer
    /// than the returned. `xcb_disconnect` is not called on drop.
    pub unsafe fn from_raw_xcb_connection(ptr: *mut c_void) -> Result<XCBConnection, ConnectionError> {
        let setup = raw_ffi::xcb_get_setup(ptr as *mut raw_ffi::xcb_connection_t);
        Ok(XCBConnection {
            conn: raw_ffi::XCBConnectionWrapper(ptr as *mut raw_ffi::xcb_connection_t),
            setup: Self::parse_setup(setup)?,
            ext_info: Default::default(),
            errors: Default::default(),
            should_drop: false
        })
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
        flags |= raw_ffi::send_request_flags::CHECKED;
        if reply_has_fds {
            flags |= raw_ffi::send_request_flags::REPLY_FDS;
        }

        // Convert the FDs into an array of ints. libxcb will close the FDs.
        let fds: Vec<_> = fds.into_iter().map(RawFdContainer::into_raw_fd).collect();

        let seqno = if fds.is_empty() {
            unsafe { raw_ffi::xcb_send_request64((self.conn).0, flags, &mut new_bufs[2], &protocol_request) }
        } else {
            let num_fds = fds.len().try_into().unwrap();
            let fds_ptr = fds.as_ptr();
            unsafe { raw_ffi::xcb_send_request_with_fds64((self.conn).0, flags, &mut new_bufs[2], &protocol_request, num_fds, fds_ptr) }
        };
        if seqno == 0 {
            unsafe { Err(XCBConnection::connection_error_from_connection((self.conn).0)) }
        } else {
            Ok(seqno)
        }
    }

    /// Check if the underlying XCB connection is in an error state.
    pub fn has_error(&self) -> Option<ConnectionError> {
        unsafe {
            let error = raw_ffi::xcb_connection_has_error((self.conn).0);
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
        (self.conn).0 as _
    }

    /// Check if a reply to the given request already received.
    ///
    /// Return Err(()) when the reply was not yet received. Returns Ok(None) when there can be no
    /// reply. Returns Ok(buffer) with the reply if there is one (this buffer can be an error or a
    /// reply).
    fn poll_for_reply(&self, sequence: SequenceNumber) -> Result<Option<Buffer>, ()> {
        unsafe {
            let mut reply = null_mut();
            let mut error = null_mut();
            let found = raw_ffi::xcb_poll_for_reply((self.conn).0, sequence as _, &mut reply, &mut error);
            if found == 0 {
                return Err(());
            }
            assert_eq!(found, 1);
            match (reply.is_null(), error.is_null()) {
                (true, true) => Ok(None),
                (true, false) => Ok(Some(XCBConnection::wrap_error(error as _))),
                (false, true) => Ok(Some(XCBConnection::wrap_reply(reply as _))),
                (false, false) => unreachable!()
            }
        }
    }

    unsafe fn wrap_reply(reply: *const u8) -> Buffer {
        let header = CSlice::new(reply, 32);

        let length_field = u32::from_ne_bytes(header[4..8].try_into().unwrap());
        let length_field: usize = length_field.try_into()
            .expect("usize should have at least 32 bits");

        let length = 32 + length_field * 4;
        Buffer::from_raw_parts(header.into_ptr(), length)
    }

    unsafe fn wrap_error(error: *const u8) -> Buffer {
         Buffer::from_raw_parts(error, 32)
    }

    unsafe fn wrap_event(event: *const u8) -> Result<GenericEvent, ParseError> {
        let mut length = 32;
        // The first byte contains the event type, check for XGE events
        if (*event & 0x7f) == super::generated::xproto::GE_GENERIC_EVENT {
            // Read the length field of the event to get its length
            let slice = std::slice::from_raw_parts(event, 8);
            let length_field = u32::from_ne_bytes([slice[4], slice[5], slice[6], slice[7]]);
            let length_field: usize = length_field.try_into()?;
            length += length_field * 4;
        }
        Ok(Buffer::from_raw_parts(event, length).try_into()?)
    }
}

impl RequestConnection for XCBConnection {
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

    fn send_request_without_reply(&self, bufs: &[IoSlice], fds: Vec<RawFdContainer>) -> Result<VoidCookie<Self>, ConnectionError> {
        Ok(VoidCookie::new(self, self.send_request(bufs, fds, false, false)?))
    }

    fn discard_reply(&self, sequence: SequenceNumber, _kind: RequestKind, mode: DiscardMode) {
        match mode {
            DiscardMode::DiscardReplyAndError => unsafe {
                // libxcb can throw away everything for us
                raw_ffi::xcb_discard_reply64((self.conn).0, sequence);
            },
            // We have to check for errors ourselves
            DiscardMode::DiscardReply => self.errors.discard_reply(sequence)
        }
    }

    fn extension_information(&self, extension_name: &'static str) -> Option<QueryExtensionReply> {
        self.ext_info.extension_information(self, extension_name)
    }

    fn wait_for_reply_or_error(&self, sequence: SequenceNumber) -> Result<Buffer, ConnectionErrorOrX11Error> {
        unsafe {
            let mut error = null_mut();
            let reply = raw_ffi::xcb_wait_for_reply64((self.conn).0, sequence, &mut error);

            // At least one of these pointers must be NULL.
            assert!(reply.is_null() || error.is_null());

            // If both pointers are NULL, the xcb connection must be in an error state
            if reply.is_null() && error.is_null() {
                return Err(Self::connection_error_from_connection((self.conn).0).into());
            }

            if !reply.is_null() {
                Ok(XCBConnection::wrap_reply(reply as _))
            } else {
                let error: GenericError = XCBConnection::wrap_error(error as _).try_into()?;
                Err(error.into())
            }
        }
    }

    fn wait_for_reply(&self, sequence: SequenceNumber) -> Result<Option<Buffer>, ConnectionError> {
        use ConnectionErrorOrX11Error::*;
        match self.wait_for_reply_or_error(sequence) {
            Ok(buffer) => Ok(Some(buffer)),
            Err(err) => match err {
                ConnectionError(err) => Err(err),
                X11Error(err) => {
                    self.errors.append_error(err);
                    Ok(None)
                }
            }
        }
    }

    fn check_for_error(&self, sequence: SequenceNumber) -> Result<Option<GenericError>, ConnectionError> {
        let cookie = raw_ffi::xcb_void_cookie_t { sequence: sequence as _ };
        let error = unsafe { raw_ffi::xcb_request_check((self.conn).0, cookie) };
        if error.is_null() {
            Ok(None)
        } else {
            unsafe { Ok(Some(XCBConnection::wrap_error(error as _).try_into()?)) }
        }
    }

    #[cfg(unix)]
    fn wait_for_reply_with_fds(&self, sequence: SequenceNumber) -> Result<(Buffer, Vec<RawFdContainer>), ConnectionErrorOrX11Error> {
        let buffer = self.wait_for_reply_or_error(sequence)?;

        // Get a pointer to the array of integers where libxcb saved the FD numbers
        let fd_ptr = match buffer {
            Buffer::Vec(_) => unreachable!(), // wait_for_reply() always returns a CSlice
            Buffer::CSlice(ref slice) => {
                // libxcb saves the list of FDs after the data of the reply. Since the reply's
                // length is encoded in "number of 4 bytes block", the following pointer is aligned
                // correctly (if malloc() returned an alloced chunk, which it does).
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

    fn maximum_request_bytes(&self) -> usize {
        4 * unsafe { raw_ffi::xcb_get_maximum_request_length((self.conn).0) as usize }
    }
}

impl Connection for XCBConnection {
    fn wait_for_event(&self) -> Result<GenericEvent, ConnectionError> {
        if let Some(error) = self.errors.get(self) {
            return Ok(error.into());
        }
        unsafe {
            let event = raw_ffi::xcb_wait_for_event((self.conn).0);
            if event.is_null() {
                return Err(Self::connection_error_from_connection((self.conn).0));
            }
            Ok(XCBConnection::wrap_event(event as _)?)
        }
    }

    fn poll_for_event(&self) -> Result<Option<GenericEvent>, ConnectionError> {
        if let Some(error) = self.errors.get(self) {
            return Ok(Some(error.into()));
        }
        unsafe {
            let event = raw_ffi::xcb_poll_for_event((self.conn).0);
            if event.is_null() {
                let err = raw_ffi::xcb_connection_has_error((self.conn).0);
                if err == 0 {
                    return Ok(None);
                } else {
                    return Err(Self::connection_error_from_c_error(err));
                }
            }
            Ok(Some(XCBConnection::wrap_event(event as _)?))
        }
    }

    fn flush(&self) {
        // xcb_flush() returns 0 if the connection is in (or just entered) an error state, else 1.
        // Adding a Result<(), ConnectionError> as a return value here would be too noisy, I think,
        // so just ignore this return value.
        let _ = unsafe { raw_ffi::xcb_flush((self.conn).0) };
    }

    fn generate_id(&self) -> u32 {
        unsafe { raw_ffi::xcb_generate_id((self.conn).0) }
    }

    fn setup(&self) -> &Setup {
        &self.setup
    }
}

impl Drop for XCBConnection {
    fn drop(&mut self) {
        if self.should_drop {
            unsafe {
                raw_ffi::xcb_disconnect((self.conn).0 as *mut raw_ffi::xcb_connection_t);
            }
        }
    }
}

#[cfg(unix)]
impl AsRawFd for XCBConnection {
    fn as_raw_fd(&self) -> RawFd {
        unsafe {
            raw_ffi::xcb_get_file_descriptor((self.conn).0)
        }
    }
}

#[cfg(test)]
mod test {
    use std::ffi::CString;
    use super::{XCBConnection, ConnectionError};

    #[test]
    fn xcb_connect_smoke_test() -> Result<(), ConnectionError> {
        // in cfg(test), raw_ffi does not call XCB, but instead uses a mock. This test calls into
        // that mock and tests a bit of XCBConnection.

        let str = CString::new("display name").unwrap();
        let (_conn, screen) = XCBConnection::connect(Some(&str)).expect("Failed to 'connect'");
        assert_eq!(screen, 0);

        Ok(())
    }
}
