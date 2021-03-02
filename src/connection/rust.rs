use crate::{
    ConnectError,
    connection::{BufWithFds, Connection, RawEventAndSeqNumber, RequestConnection, ReplyOrError, SequenceNumber},
    cookie::{Cookie, CookieWithFds, VoidCookie},
    errors::{ConnectionError, ReplyOrIdError},
    protocol::xproto::Setup,
    rust_connection::RustConnection,
    utils::RawFdContainer,
    x11_utils::{ExtensionInformation, TryParse, TryParseFd},
};


#[allow(unreachable_pub)]
/// Buffer type used by `X11Connection`.
#[derive(Debug)]
pub struct Buffer {
    inner: Vec<u8>,
}

impl Buffer {
    fn from_raw(bytes: Vec<u8>) -> Buffer {
        Buffer { inner: bytes }
    }
}

impl AsRef<[u8]> for Buffer {
    fn as_ref(&self) -> &[u8] {
        self.inner.as_ref()
    }
}

#[allow(unreachable_pub)]
/// X11 connection.
///
/// This is backed by either `RustConnection`, or `XCBConnection` if
/// `allow-unsafe-code` feature is enabled.
#[derive(Debug)]
pub struct X11Connection {
    inner: RustConnection,
}

impl X11Connection {
    #[allow(dead_code)]
    pub(crate) fn connect(dpy_name: Option<&str>) -> Result<(X11Connection, usize), ConnectError> {
        let (inner, screen) = RustConnection::connect(dpy_name)?;
        Ok((
            X11Connection { inner },
            screen,
        ))
    }
}

impl RequestConnection for X11Connection {
    type Buf = Buffer;

    fn send_request_with_reply<R>(
        &self,
        bufs: &[std::io::IoSlice<'_>],
        fds: Vec<RawFdContainer>,
    ) -> Result<Cookie<'_, Self, R>, ConnectionError>
    where
        R: TryParse,
    {
        let cookie = self.inner.send_request_with_reply::<R>(bufs, fds)?;
        let cookie = Cookie::new(
            self,
            cookie.into_sequence_number(),
        );
        Ok(cookie)
    }

    fn send_request_with_reply_with_fds<R>(
        &self,
        bufs: &[std::io::IoSlice<'_>],
        fds: Vec<RawFdContainer>,
    ) -> Result<CookieWithFds<'_, Self, R>, ConnectionError>
    where
        R: TryParseFd,
    {
        let cookie = self.inner.send_request_with_reply_with_fds::<R>(bufs, fds)?;
        let cookie = CookieWithFds::new(
            self,
            cookie.sequence_number(),
        );
        Ok(cookie)
    }

    fn send_request_without_reply(
        &self,
        bufs: &[std::io::IoSlice<'_>],
        fds: Vec<RawFdContainer>,
    ) -> Result<VoidCookie<'_, Self>, ConnectionError> {
        let cookie = self.inner.send_request_without_reply(bufs, fds)?;
        let cookie = VoidCookie::new(
            self,
            cookie.sequence_number(),
        );
        Ok(cookie)
    }

    fn discard_reply(&self, sequence: SequenceNumber, kind: super::RequestKind, mode: super::DiscardMode) {
        self.inner.discard_reply(sequence, kind, mode)
    }

    fn prefetch_extension_information(
        &self,
        extension_name: &'static str,
    ) -> Result<(), ConnectionError> {
        self.inner.prefetch_extension_information(extension_name)
    }

    fn extension_information(
        &self,
        extension_name: &'static str,
    ) -> Result<Option<ExtensionInformation>, ConnectionError> {
        self.inner.extension_information(extension_name)
    }

    fn wait_for_reply_or_raw_error(
        &self,
        sequence: SequenceNumber,
    ) -> Result<ReplyOrError<Self::Buf>, ConnectionError> {
        let reply = self.inner.wait_for_reply_or_raw_error(sequence)?;
        match reply {
            ReplyOrError::Reply(r) => Ok(ReplyOrError::Reply(Buffer::from_raw(r))),
            ReplyOrError::Error(e) => Ok(ReplyOrError::Error(Buffer::from_raw(e))),
        }
    }

    fn wait_for_reply(
        &self,
        sequence: SequenceNumber,
    ) -> Result<Option<Self::Buf>, ConnectionError> {
        let reply = self.inner.wait_for_reply(sequence)?;
        Ok(reply.map(Buffer::from_raw))
    }

    fn wait_for_reply_with_fds_raw(
        &self,
        sequence: SequenceNumber,
    ) -> Result<ReplyOrError<BufWithFds<Self::Buf>, Self::Buf>, ConnectionError> {
        let reply = self.inner.wait_for_reply_with_fds_raw(sequence)?;
        match reply {
            ReplyOrError::Reply((r, f)) => Ok(ReplyOrError::Reply((Buffer::from_raw(r), f))),
            ReplyOrError::Error(e) => Ok(ReplyOrError::Error(Buffer::from_raw(e))),
        }
    }

    fn check_for_raw_error(
        &self,
        sequence: SequenceNumber,
    ) -> Result<Option<Self::Buf>, ConnectionError> {
        let raw = self.inner.check_for_raw_error(sequence)?;
        Ok(raw.map(Buffer::from_raw))
    }

    fn prefetch_maximum_request_bytes(&self) {
        self.inner.prefetch_maximum_request_bytes()
    }

    fn maximum_request_bytes(&self) -> usize {
        self.inner.maximum_request_bytes()
    }

    fn parse_error(&self, error: &[u8]) -> Result<crate::x11_utils::X11Error, crate::rust_connection::ParseError> {
        self.inner.parse_error(error)
    }

    fn parse_event(&self, event: &[u8]) -> Result<crate::protocol::Event, crate::rust_connection::ParseError> {
        self.inner.parse_event(event)
    }
}

impl Connection for X11Connection {
    fn wait_for_raw_event_with_sequence(
        &self,
    ) -> Result<RawEventAndSeqNumber<Self::Buf>, ConnectionError> {
        let (e, s) = self.inner.wait_for_raw_event_with_sequence()?;
        Ok((Buffer::from_raw(e), s))
    }

    fn poll_for_raw_event_with_sequence(
        &self,
    ) -> Result<Option<RawEventAndSeqNumber<Self::Buf>>, ConnectionError> {
        let event = self.inner.poll_for_raw_event_with_sequence()?;
        match event {
            Some((e, s)) => Ok(Some((Buffer::from_raw(e), s))),
            None => Ok(None),
        }
    }

    fn flush(&self) -> Result<(), ConnectionError> {
        self.inner.flush()
    }

    fn setup(&self) -> &Setup {
        self.inner.setup()
    }

    fn generate_id(&self) -> Result<u32, ReplyOrIdError> {
        self.inner.generate_id()
    }
}
