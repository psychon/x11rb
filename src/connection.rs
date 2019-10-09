use std::io::IoSlice;
use std::convert::{TryFrom, TryInto};
use std::marker::PhantomData;
use crate::utils::CSlice;
use crate::errors::{ParseError, ConnectionErrorOrX11Error};
use crate::generated::xproto::ListFontsWithInfoReply;

pub type SequenceNumber = u64;

pub trait Connection: Sized {
    fn send_request_with_reply<R>(&self, bufs: &[IoSlice]) -> Cookie<Self, R>
        where R: TryFrom<CSlice, Error=ParseError>;

    fn send_request_without_reply(&self, bufs: &[IoSlice]) -> SequenceNumber;

    fn discard_reply(&self, sequence: SequenceNumber);

    fn wait_for_reply(&self, sequence: SequenceNumber) -> Result<CSlice, ConnectionErrorOrX11Error>;
}

#[derive(Debug)]
pub struct Cookie<'a, C, R>
where C: Connection
{
    connection: &'a C,
    sequence_number: Option<SequenceNumber>,
    phantom: std::marker::PhantomData<R>
}

impl<C, R> Cookie<'_, C, R>
where R: TryFrom<CSlice, Error=ParseError>,
      C: Connection
{
    pub(crate) fn new(connection: &C, sequence_number: SequenceNumber) -> Cookie<C, R> {
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

impl<C, R> Drop for Cookie<'_, C, R>
where C: Connection
{
    fn drop(&mut self) {
        if let Some(number) = self.sequence_number {
            self.connection.discard_reply(number);
        }
    }
}

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
