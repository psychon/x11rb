#![allow(dead_code)]

use std::error::Error;
use std::convert::{TryFrom, TryInto};
use std::io::{Cursor, IoSlice, Error as IOError, ErrorKind::{UnexpectedEof, InvalidData, Other}};
use std::marker::PhantomData;
use byteorder::{NativeEndian, ReadBytesExt};

// TODO: Make this nice
pub type SendFlags = u8;

const DISCARD_REPLY: SendFlags = 0b01;

type SequenceNumber = u64;

pub struct Connection(());

impl Connection {
    pub fn send_request_with_reply<R>(&self, _bufs: &[IoSlice], _flags: SendFlags) -> Result<Cookie<R>, Box<dyn Error>> {
        unimplemented!();
    }

    pub fn send_request_without_reply(&self, _bufs: &[IoSlice], _flags: SendFlags) -> Result<SequenceNumber, Box<dyn Error>> {
        unimplemented!();
    }

    fn discard_reply(&self, _sequence: SequenceNumber) {
        unimplemented!();
    }

    fn wait_for_reply(&self, _sequence: SequenceNumber) -> Result<Vec<u8>, ReplyError> {
        unimplemented!();
    }
}

pub struct Cookie<'a, R> {
    connection: &'a Connection,
    sequence_number: Option<SequenceNumber>,
    phantom: std::marker::PhantomData<R>
}

pub enum ReplyError {
    X11Error(GenericError),
    IOError(Box<dyn Error>)
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

impl<R> Cookie<'_, R>
where R: TryFrom<Vec<u8>, Error=ReplyError>
{
    pub fn wait(mut self) -> Result<R, ReplyError> {
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

trait Event {
    fn raw_bytes(&self) -> &[u8];
    fn to_raw_bytes(self) -> Vec<u8>;

    fn raw_response_type(&self) -> u8 {
        self.raw_bytes()[0]
    }

    fn response_type(&self) -> u8 {
        self.raw_response_type() & 0x7f
    }

    fn server_generated(&self) -> bool {
        self.raw_response_type() & 0x80 == 0
    }

    fn raw_sequence_number(&self) -> u16 {
        let bytes = self.raw_bytes();
        Cursor::new(&bytes[2..3]).read_u16::<NativeEndian>().unwrap()
    }
}

pub struct GenericEvent(Vec<u8>);

impl Event for GenericEvent {
    fn raw_bytes(&self) -> &[u8] {
        &self.0
    }

    fn to_raw_bytes(self) -> Vec<u8> {
        self.0
    }
}

const REPLY: u8 = 1;
const XGE_EVENT: u8 = 35;

impl TryFrom<Vec<u8>> for GenericEvent {
    type Error = Box<dyn Error>;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.len() < 32 {
            let err: IOError = UnexpectedEof.into();
            return Err(err.into())
        }
        let length_field = Cursor::new(&value[4..7]).read_u32::<NativeEndian>().unwrap();
        let other_error: IOError = Other.into();
        let other_error: Result<_, Self::Error> = Err(other_error.into());
        let length_field: usize = length_field.try_into().or(other_error)?;
        let actual_length = value.len();
        let event = GenericEvent(value);
        let expected_length = match event.response_type() {
            XGE_EVENT | REPLY => 32 + 4 * length_field,
            _ => 32
        };
        if actual_length != expected_length {
            let err: IOError = UnexpectedEof.into();
            return Err(err.into())
        }
        Ok(event)
    }
}

impl Into<Vec<u8>> for GenericEvent {
    fn into(self) -> Vec<u8> {
        self.to_raw_bytes()
    }
}

pub struct GenericError(Vec<u8>);

impl GenericError {
    pub fn error_code(&self) -> u8 {
        self.raw_bytes()[1]
    }
}

impl Event for GenericError {
    fn raw_bytes(&self) -> &[u8] {
        &self.0
    }

    fn to_raw_bytes(self) -> Vec<u8> {
        self.0
    }
}

impl TryFrom<Vec<u8>> for GenericError {
    type Error = Box<dyn Error>;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let event: GenericEvent = value.try_into()?;
        if event.response_type() != 0 {
            let err: IOError = InvalidData.into();
            return Err(err.into())
        }
        Ok(GenericError(event.to_raw_bytes()))
    }
}

impl Into<Vec<u8>> for GenericError {
    fn into(self) -> Vec<u8> {
        self.to_raw_bytes()
    }
}
