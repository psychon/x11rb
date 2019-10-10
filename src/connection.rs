//! Generic connection-related types and definitions.
//!
//! This module contains the `Connection` trait and related definitions. The code in this module is
//! used by each concrete implementation of the X11 protocol.

use std::io::IoSlice;
use std::convert::{TryFrom, TryInto};
use std::marker::PhantomData;
use crate::utils::Buffer;
use crate::errors::{ParseError, ConnectionError, ConnectionErrorOrX11Error};
use crate::x11_utils::GenericEvent;
use crate::generated::xproto::{Setup, ListFontsWithInfoReply};

/// Number type used for referring to things that were sent to the server in responses from the
/// server.
///
/// Each request sent to the X11 server is implicitly assigned a monotonically increasing sequence
/// number. Replies, events, and errors contain the sequence number of the last request that the
/// server received. This allows to map replies to their requests and to figure out which request
/// caused an error.
pub type SequenceNumber = u64;

/// A connection to an X11 server.
pub trait Connection: Sized {
    /// Send a request with a reply to the server.
    ///
    /// The `bufs` parameter describes the raw bytes that should be sent. The returned cookie
    /// allows to get the response.
    ///
    /// Users of this library will most likely not want to use this function directly. Instead, the
    /// generated code will take the supplied arguments, construct byte buffers, and call this
    /// method.
    fn send_request_with_reply<R>(&self, bufs: &[IoSlice]) -> Cookie<Self, R>
        where R: TryFrom<Buffer, Error=ParseError>;

    /// Send a request without a reply to the server.
    ///
    /// The `bufs` parameter describes the raw bytes that should be sent. The sequence number of
    /// the request is returned, but most likely not useful to users.
    ///
    /// Users of this library will most likely not want to use this function directly. Instead, the
    /// generated code will take the supplied arguments, construct byte buffers, and call this
    /// method.
    fn send_request_without_reply(&self, bufs: &[IoSlice]) -> SequenceNumber;

    /// A reply to an error should be discarded.
    ///
    /// This method is automatically called by the `Drop` implementation on `Cookie` so that any
    /// replies that are received later can be ignored.
    ///
    /// Users of this library will most likely not want to use this function directly.
    fn discard_reply(&self, sequence: SequenceNumber);

    /// Wait for the reply to a request.
    ///
    /// The given sequence number identifies the request for which replies are expected.
    ///
    /// Users of this library will most likely not want to use this function directly.
    fn wait_for_reply(&self, sequence: SequenceNumber) -> Result<Buffer, ConnectionErrorOrX11Error>;

    /// Wait for a new event from the X11 server.
    fn wait_for_event(&self) -> Result<GenericEvent, ConnectionError>;

    /// Send all pending requests to the server.
    ///
    /// Implementations of this trait may buffer requests for batched sending. When this method is
    /// called, all pending requests are sent.
    ///
    /// You do not have to call this method before `wait_for_reply()`. If the request you want to
    /// wait for was not yet sent, it will be sent by `wait_for_reply()`.
    fn flush(&self);

    /// Get the setup information sent by the X11 server.
    ///
    /// The setup information contains X11 server, for example the window id of the root window.
    fn setup(&self) -> &Setup;

    /// Generate a new X11 identifier.
    ///
    /// This method can, for example, be used for creating a new window. First, this method is
    /// called to generate an identifier. Next, `generated::xproto::create_window` can be called to
    /// actually create the window.
    fn generate_id(&self) -> u32;
}

/// A handle to a response from the X11 server.
///
/// When sending a request to the X11 server, this library returns a `Cookie`. This `Cookie` can
/// then later be used to get the response that the server sent.
#[derive(Debug)]
pub struct Cookie<'a, C, R>
where C: Connection
{
    connection: &'a C,
    sequence_number: Option<SequenceNumber>,
    phantom: std::marker::PhantomData<R>
}

impl<C, R> Cookie<'_, C, R>
where R: TryFrom<Buffer, Error=ParseError>,
      C: Connection
{
    pub(crate) fn new(connection: &C, sequence_number: SequenceNumber) -> Cookie<C, R> {
        Cookie {
            connection,
            sequence_number: Some(sequence_number),
            phantom: PhantomData
        }
    }

    /// Get the reply that the server sent.
    pub fn reply(mut self) -> Result<R, ConnectionErrorOrX11Error> {
        let reply = self.connection.wait_for_reply(self.sequence_number.take().unwrap())?;
        Ok(reply.try_into()?)
    }
}

impl<C, R> Drop for Cookie<'_, C, R>
where C: Connection
{
    fn drop(&mut self) {
        if let Some(number) = self.sequence_number {
            self.connection.discard_reply(number);
        }
    }
}

/// A handle to the replies to a `ListFontsWithInfo` request.
///
/// `ListFontsWithInfo` generated more than one reply, but `Cookie` only allows getting one reply.
/// This structure implements `Iterator` and allows to get all the replies.
pub struct ListFontsWithInfoCookie<'a, C: Connection>(Cookie<'a, C, ListFontsWithInfoReply>);

impl<C> ListFontsWithInfoCookie<'_, C>
where C: Connection
{
    pub(crate) fn new(cookie: Cookie<C, ListFontsWithInfoReply>) -> ListFontsWithInfoCookie<C> {
        ListFontsWithInfoCookie(cookie)
    }
}

impl<C> Iterator for ListFontsWithInfoCookie<'_, C>
where C: Connection
{
    type Item = Result<ListFontsWithInfoReply, ConnectionErrorOrX11Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let sequence = match self.0.sequence_number.take() {
            None => return None,
            Some(sequence) => sequence
        };
        let reply = self.0.connection.wait_for_reply(sequence);
        let reply = match reply {
            Err(e) => return Some(Err(e)),
            Ok(v) => v
        };
        let reply: Result<ListFontsWithInfoReply, ParseError> = reply.try_into();
        let reply = reply.map_err(ConnectionErrorOrX11Error::from);
        if reply.is_ok() {
            if !reply.as_ref().unwrap().name.is_empty() {
                self.0.sequence_number = Some(sequence);
            } else {
                return None
            }
        }
        Some(reply)
    }
}
