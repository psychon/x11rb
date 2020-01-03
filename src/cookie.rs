//! Cookies are handles to future replies or errors from the X11 server.

use std::convert::{TryFrom, TryInto};
use std::marker::PhantomData;
use crate::utils::{Buffer, RawFdContainer};
use crate::errors::{ParseError, ConnectionError, ConnectionErrorOrX11Error};
use crate::x11_utils::GenericError;
use crate::generated::xproto::ListFontsWithInfoReply;
use crate::connection::{RequestKind, DiscardMode, RequestConnection, SequenceNumber};

/// A handle to a possible error from the X11 server.
///
/// When sending a request for which no reply is expected, this library returns a `VoidCookie`.
/// This `VoidCookie` can then later be used to check if the X11 server sent an error.
#[derive(Debug)]
pub struct VoidCookie<'a, C>
where C: RequestConnection + ?Sized
{
    connection: &'a C,
    sequence_number: SequenceNumber,
}

impl<'a, C> VoidCookie<'a, C>
where C: RequestConnection + ?Sized
{
    /// Construct a new cookie.
    ///
    /// This function should only be used by implementations of
    /// `Connection::send_request_without_reply`.
    pub fn new(connection: &C, sequence_number: SequenceNumber) -> VoidCookie<C> {
        VoidCookie { connection, sequence_number }
    }

    /// Get the sequence number of the request that generated this cookie.
    pub fn sequence_number(&self) -> SequenceNumber {
        self.sequence_number
    }

    fn consume(self) -> (&'a C, SequenceNumber) {
        let result = (self.connection, self.sequence_number);
        std::mem::forget(self);
        result
    }

    /// Check if the original request caused an X11 error.
    pub fn check(self) -> Result<Option<GenericError>, ConnectionError> {
        let (connection, sequence) = self.consume();
        connection.check_for_error(sequence)
    }

    /// Ignore all errors to this request.
    ///
    /// Without calling this method, an error becomes available on the connection as an event after
    /// this cookie was dropped. This function causes errors to be ignored instead.
    pub fn ignore_error(self) {
        let (connection, sequence) = self.consume();
        connection.discard_reply(sequence, RequestKind::IsVoid, DiscardMode::DiscardReplyAndError)
    }
}

impl<C> Drop for VoidCookie<'_, C>
where C: RequestConnection + ?Sized
{
    fn drop(&mut self) {
        self.connection.discard_reply(self.sequence_number, RequestKind::IsVoid, DiscardMode::DiscardReply)
    }
}

/// Internal helper for a cookie with an response
#[derive(Debug)]
struct RawCookie<'a, C>
where C: RequestConnection + ?Sized
{
    connection: &'a C,
    sequence_number: SequenceNumber,
}

impl<C> RawCookie<'_, C>
where C: RequestConnection + ?Sized
{
    /// Construct a new raw cookie.
    ///
    /// This function should only be used by implementations of
    /// `RequestConnection::send_request_with_reply`.
    fn new(connection: &C, sequence_number: SequenceNumber) -> RawCookie<C> {
        RawCookie {
            connection,
            sequence_number,
        }
    }

    /// Consume this instance and get the contained sequence number out.
    fn into_sequence_number(self) -> SequenceNumber {
        let number = self.sequence_number;
        // Prevent drop() from running
        std::mem::forget(self);
        number
    }
}

impl<C> Drop for RawCookie<'_, C>
where C: RequestConnection + ?Sized
{
    fn drop(&mut self) {
        self.connection.discard_reply(self.sequence_number, RequestKind::HasResponse, DiscardMode::DiscardReply);
    }
}

/// A handle to a response from the X11 server.
///
/// When sending a request to the X11 server, this library returns a `Cookie`. This `Cookie` can
/// then later be used to get the response that the server sent.
#[derive(Debug)]
pub struct Cookie<'a, C, R>
where C: RequestConnection + ?Sized
{
    raw_cookie: RawCookie<'a, C>,
    phantom: PhantomData<R>
}

impl<C, R> Cookie<'_, C, R>
where R: TryFrom<Buffer, Error=ParseError>,
      C: RequestConnection + ?Sized
{
    /// Construct a new cookie.
    ///
    /// This function should only be used by implementations of
    /// `RequestConnection::send_request_with_reply`.
    pub fn new(connection: &C, sequence_number: SequenceNumber) -> Cookie<C, R> {
        Cookie {
            raw_cookie: RawCookie::new(connection, sequence_number),
            phantom: PhantomData
        }
    }

    /// Get the sequence number of the request that generated this cookie.
    pub fn sequence_number(&self) -> SequenceNumber {
        self.raw_cookie.sequence_number
    }

    /// Get the raw reply that the server sent.
    pub fn raw_reply(self) -> Result<Buffer, ConnectionErrorOrX11Error> {
        let conn = self.raw_cookie.connection;
        Ok(conn.wait_for_reply_or_error(self.raw_cookie.into_sequence_number())?)
    }

    /// Get the raw reply that the server sent, but have errors handled as events.
    pub fn raw_reply_unchecked(self) -> Result<Option<Buffer>, ConnectionError> {
        let conn = self.raw_cookie.connection;
        Ok(conn.wait_for_reply(self.raw_cookie.into_sequence_number())?)
    }

    /// Get the reply that the server sent.
    pub fn reply(self) -> Result<R, ConnectionErrorOrX11Error> {
        Ok(self.raw_reply()?.try_into()?)
    }

    /// Get the reply that the server sent, but have errors handled as events.
    pub fn reply_unchecked(self) -> Result<Option<R>, ConnectionError> {
        self.raw_reply_unchecked()?
            .map(TryInto::try_into)
            .transpose()
            .map_err(Into::into)
    }

    /// Discard all responses to the request this cookie represents, even errors.
    ///
    /// Without this function, errors are treated as events after the cookie is dropped.
    pub fn discard_reply_and_errors(self) {
        let conn = self.raw_cookie.connection;
        conn.discard_reply(self.raw_cookie.into_sequence_number(), RequestKind::HasResponse, DiscardMode::DiscardReplyAndError)
    }
}

/// A handle to a response containing `RawFd` from the X11 server.
///
/// When sending a request to the X11 server, this library returns a `Cookie`. This `Cookie` can
/// then later be used to get the response that the server sent.
///
/// This variant of `Cookie` represents a response that can contain `RawFd`s.
#[derive(Debug)]
pub struct CookieWithFds<'a, C, R>
where C: RequestConnection + ?Sized
{
    raw_cookie: RawCookie<'a, C>,
    phantom: PhantomData<R>
}

impl<C, R> CookieWithFds<'_, C, R>
where R: TryFrom<(Buffer, Vec<RawFdContainer>), Error=ParseError>,
      C: RequestConnection + ?Sized
{
    /// Construct a new cookie.
    ///
    /// This function should only be used by implementations of
    /// `RequestConnection::send_request_with_reply`.
    pub fn new(connection: &C, sequence_number: SequenceNumber) -> CookieWithFds<C, R> {
        CookieWithFds {
            raw_cookie: RawCookie::new(connection, sequence_number),
            phantom: PhantomData
        }
    }

    /// Get the sequence number of the request that generated this cookie.
    pub fn sequence_number(&self) -> SequenceNumber {
        self.raw_cookie.sequence_number
    }

    /// Get the raw reply that the server sent.
    pub fn raw_reply(self) -> Result<(Buffer, Vec<RawFdContainer>), ConnectionErrorOrX11Error> {
        let conn = self.raw_cookie.connection;
        Ok(conn.wait_for_reply_with_fds(self.raw_cookie.into_sequence_number())?)
    }

    /// Get the reply that the server sent.
    pub fn reply(self) -> Result<R, ConnectionErrorOrX11Error> {
        Ok(self.raw_reply()?.try_into()?)
    }
}

/// A handle to the replies to a `ListFontsWithInfo` request.
///
/// `ListFontsWithInfo` generated more than one reply, but `Cookie` only allows getting one reply.
/// This structure implements `Iterator` and allows to get all the replies.
#[derive(Debug)]
pub struct ListFontsWithInfoCookie<'a, C: RequestConnection + ?Sized>(Option<RawCookie<'a, C>>);

impl<C> ListFontsWithInfoCookie<'_, C>
where C: RequestConnection + ?Sized
{
    pub(crate) fn new(cookie: Cookie<C, ListFontsWithInfoReply>) -> ListFontsWithInfoCookie<C> {
        ListFontsWithInfoCookie(Some(cookie.raw_cookie))
    }

    /// Get the sequence number of the request that generated this cookie.
    pub fn sequence_number(&self) -> Option<SequenceNumber> {
        self.0.as_ref().map(|x| x.sequence_number)
    }
}

impl<C> Iterator for ListFontsWithInfoCookie<'_, C>
where C: RequestConnection + ?Sized
{
    type Item = Result<ListFontsWithInfoReply, ConnectionErrorOrX11Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let cookie = match self.0.take() {
            None => return None,
            Some(cookie) => cookie
        };
        let reply = cookie.connection.wait_for_reply_or_error(cookie.sequence_number);
        let reply = match reply {
            Err(e) => return Some(Err(e)),
            Ok(v) => v
        };
        let reply: Result<ListFontsWithInfoReply, ParseError> = reply.try_into();
        let reply = reply.map_err(ConnectionErrorOrX11Error::from);
        if reply.is_ok() {
            // Is this an indicator that no more replies follow?
            if !reply.as_ref().unwrap().name.is_empty() {
                self.0 = Some(cookie);
            } else {
                return None
            }
        }
        Some(reply)
    }
}
