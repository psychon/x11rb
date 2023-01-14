// This code is dual licensed under MIT OR Apache 2.0.

//! Cookies!

use x11rb::connection::RequestKind;
use x11rb_protocol::DiscardMode;

use crate::connection::{Connection, RequestConnection};
use crate::errors::{ReplyError, ConnectionError};
use crate::x11_utils::{TryParse, TryParseFd};
use crate::{ReplyOrError, SequenceNumber, BufWithFds};
use std::marker::PhantomData;
use std::mem;

/// A cookie for a request without a reply.
pub struct VoidCookie<'conn, C: RequestConnection + ?Sized> {
    conn: &'conn C,
    sequence: SequenceNumber,
}

impl<'conn, C: Connection + ?Sized> VoidCookie<'conn, C> {
    /// Create a new cookie from its raw parts.
    pub fn new(conn: &'conn C, sequence: SequenceNumber) -> Self {
        Self { conn, sequence }
    }

    /// Get the sequence number of this cookie.
    pub fn sequence_number(&self) -> SequenceNumber {
        self.sequence
    }

    /// Check if this request caused an X11 error.
    pub async fn check(self) -> Result<(), ReplyError> {
        let (conn, seq) = self.consume();
        match conn.check_for_raw_error(seq).await? {
            Some(e) => Err(conn.parse_error(e.as_ref())?.into()),
            None => Ok(()),
        }
    }

    /// Ignore errors associated with this request.
    pub fn ignore_error(self) {
        let (conn, seq) = self.consume();
        conn.discard_reply(seq, RequestKind::IsVoid, DiscardMode::DiscardReplyAndError);
    }

    /// Eat the cookie and return the connection.
    fn consume(self) -> (&'conn C, SequenceNumber) {
        let res = (self.conn, self.sequence);
        mem::forget(self);
        res
    }
}

impl<'conn, C: RequestConnection + ?Sized> Drop for VoidCookie<'conn, C> {
    fn drop(&mut self) {
        self.conn.discard_reply(
            self.sequence,
            RequestKind::IsVoid,
            DiscardMode::DiscardReply,
        );
    }
}

/// Helper for cookies that hold a reply.
struct RawCookie<'a, C: RequestConnection + ?Sized> {
    conn: &'a C,
    sequence: SequenceNumber,
}

impl<'a, C: RequestConnection + ?Sized> RawCookie<'a, C> {
    fn new(conn: &'a C, sequence: SequenceNumber) -> Self {
        Self { conn, sequence }
    }

    fn consume(self) -> (&'a C, SequenceNumber) {
        let res = (self.conn, self.sequence);
        mem::forget(self);
        res
    }
}

impl<'a, C: RequestConnection + ?Sized> Drop for RawCookie<'a, C> {
    fn drop(&mut self) {
        self.conn.discard_reply(
            self.sequence,
            RequestKind::HasResponse,
            DiscardMode::DiscardReply,
        );
    }
}

/// A cookie for a request that has a reply.
pub struct Cookie<'conn, C: RequestConnection + ?Sized, R> {
    raw: RawCookie<'conn, C>,
    capture: PhantomData<R>,
}

impl<'conn, C: Connection + ?Sized, R: TryParse> Cookie<'conn, C, R> {
    /// Create a new cookie from its raw parts.
    pub fn new(conn: &'conn C, sequence: SequenceNumber) -> Self {
        Self {
            raw: RawCookie::new(conn, sequence),
            capture: PhantomData,
        }
    }

    /// Get the sequence number of this cookie.
    pub fn sequence_number(&self) -> SequenceNumber {
        self.raw.sequence
    }

    /// Get the raw reply that the server sent.
    pub async fn raw_reply(self) -> Result<C::Buf, ReplyError> {
        let (conn, seq) = self.raw.consume();

        // Wait for the reply
        let reply_or_error = conn.wait_for_reply_or_raw_error(seq).await?;

        // Check for errors
        match reply_or_error {
            ReplyOrError::Reply(reply) => Ok(reply),
            ReplyOrError::Error(error) => Err(conn.parse_error(error.as_ref())?.into()),
        }
    }

    /// Get the reply, but have errors handled as events.
    pub async fn raw_reply_unchecked(self) -> Result<Option<C::Buf>, ConnectionError> {
        let (conn, seq) = self.raw.consume();

        // Wait for the reply
        conn.wait_for_reply(seq).await
    }

    /// Get the reply that the server sent.
    pub async fn reply(self) -> Result<R, ReplyError> {
        let buf = self.raw_reply().await?;

        // Parse the reply
        let (reply, _) = R::try_parse(buf.as_ref())?;
        Ok(reply)
    } 

    /// Get the reply, but have errors handled as events.
    pub async fn reply_unchecked(self) -> Result<Option<R>, ConnectionError> {
        let buf = self.raw_reply_unchecked().await?;

        // Parse the reply
        let reply = buf.map(|buf| R::try_parse(buf.as_ref()).unwrap().0);
        Ok(reply)
    }
}

/// A cookie for a request that has a reply containing file descriptors.
pub struct CookieWithFds<'conn, C: RequestConnection + ?Sized, R> {
    raw: RawCookie<'conn, C>,
    capture: PhantomData<R>,
}

impl<'conn, C: Connection + ?Sized, R: TryParseFd> CookieWithFds<'conn, C, R> {
    /// Create a new cookie from its raw parts.
    pub fn new(conn: &'conn C, sequence: SequenceNumber) -> Self {
        Self {
            raw: RawCookie::new(conn, sequence),
            capture: PhantomData,
        }
    }

    /// Get the sequence number of this cookie.
    pub fn sequence_number(&self) -> SequenceNumber {
        self.raw.sequence
    }

    /// Get the raw reply that the server sent.
    pub async fn raw_reply(self) -> Result<BufWithFds<C::Buf>, ReplyError> {
        let (conn, seq) = self.raw.consume();

        // Wait for the reply
        let reply_or_error = conn.wait_for_reply_with_fds_raw(seq).await?;

        // Check for errors
        match reply_or_error {
            ReplyOrError::Reply(reply) => Ok(reply),
            ReplyOrError::Error(error) => Err(conn.parse_error(error.as_ref())?.into()),
        }
    }

    /// Get the reply that the server sent.
    pub async fn reply(self) -> Result<R, ReplyError> {
        let (buf, mut fds) = self.raw_reply().await?;

        // Parse the reply
        let (reply, _) = R::try_parse_fd(buf.as_ref(), &mut fds)?;
        Ok(reply)
    }
}
