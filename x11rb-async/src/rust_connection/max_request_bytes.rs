// This code is dual licensed under MIT OR Apache 2.0.

//! Wrapper for calling `maximum_request_bytes` in `compute_length_field.`

use std::io::IoSlice;

use x11rb::connection::{RequestConnection, RequestKind};
use x11rb::cookie::{Cookie, CookieWithFds, VoidCookie};
use x11rb::errors::{ConnectionError, ParseError, ReplyError};
use x11rb::protocol::Event;
use x11rb::x11_utils::{TryParse, TryParseFd, X11Error};
use x11rb_protocol::{DiscardMode, RawFdContainer, SequenceNumber};

/// The maximum number of bytes that can be sent in a single request.
pub(super) struct MaxBytesConnWrapper(usize);

impl MaxBytesConnWrapper {
    /// Create a new `MaxBytesConnWrapper` that returns the given value.
    pub(super) fn new(max_bytes: usize) -> Self {
        Self(max_bytes)
    }
}

impl RequestConnection for MaxBytesConnWrapper {
    type Buf = Vec<u8>;

    fn check_for_error(&self, _sequence: SequenceNumber) -> Result<(), ReplyError> {
        unimplemented!()
    }

    fn check_for_raw_error(
        &self,
        _sequence: SequenceNumber,
    ) -> Result<Option<Self::Buf>, ConnectionError> {
        unimplemented!()
    }

    fn discard_reply(&self, _sequence: SequenceNumber, _kind: RequestKind, _mode: DiscardMode) {
        unimplemented!()
    }

    fn extension_information(
        &self,
        _extension_name: &'static str,
    ) -> Result<Option<x11rb::x11_utils::ExtensionInformation>, ConnectionError> {
        unimplemented!()
    }

    fn prefetch_extension_information(
        &self,
        _extension_name: &'static str,
    ) -> Result<(), ConnectionError> {
        unimplemented!()
    }

    fn maximum_request_bytes(&self) -> usize {
        self.0
    }

    fn prefetch_maximum_request_bytes(&self) {}

    fn parse_error(&self, _error: &[u8]) -> Result<X11Error, ParseError> {
        unimplemented!()
    }

    fn parse_event(&self, _event: &[u8]) -> Result<Event, ParseError> {
        unimplemented!()
    }

    fn send_request_with_reply<R>(
        &self,
        _bufs: &[IoSlice<'_>],
        _fds: Vec<RawFdContainer>,
    ) -> Result<Cookie<'_, Self, R>, ConnectionError>
    where
        R: TryParse,
    {
        unimplemented!()
    }

    fn send_request_with_reply_with_fds<R>(
        &self,
        _bufs: &[IoSlice<'_>],
        _fds: Vec<RawFdContainer>,
    ) -> Result<CookieWithFds<'_, Self, R>, ConnectionError>
    where
        R: TryParseFd,
    {
        unimplemented!()
    }

    fn send_request_without_reply(
        &self,
        _bufs: &[IoSlice<'_>],
        _fds: Vec<RawFdContainer>,
    ) -> Result<VoidCookie<'_, Self>, ConnectionError> {
        unimplemented!()
    }

    fn wait_for_reply(
        &self,
        _sequence: SequenceNumber,
    ) -> Result<Option<Self::Buf>, ConnectionError> {
        unimplemented!()
    }

    fn wait_for_reply_or_raw_error(
        &self,
        _sequence: SequenceNumber,
    ) -> Result<x11rb::connection::ReplyOrError<Self::Buf>, ConnectionError> {
        unimplemented!()
    }

    fn wait_for_reply_with_fds_raw(
        &self,
        _sequence: SequenceNumber,
    ) -> Result<
        x11rb::connection::ReplyOrError<x11rb::connection::BufWithFds<Self::Buf>, Self::Buf>,
        ConnectionError,
    > {
        unimplemented!()
    }
}
