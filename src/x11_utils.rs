use std::marker::PhantomData;
use std::convert::{TryFrom, TryInto};

use crate::utils::CSlice;
use crate::xcb_ffi::Connection;
use crate::errors::{ParseError, ConnectionErrorOrX11Error};

pub type SequenceNumber = u64;

#[derive(Debug)]
pub struct Cookie<'a, R> {
    connection: &'a Connection,
    sequence_number: Option<SequenceNumber>,
    phantom: std::marker::PhantomData<R>
}

impl<R> Cookie<'_, R>
where R: TryFrom<CSlice, Error=ParseError>
{
    pub(crate) fn new(connection: &Connection, sequence_number: SequenceNumber) -> Cookie<R> {
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

impl<R> Drop for Cookie<'_, R> {
    fn drop(&mut self) {
        if let Some(number) = self.sequence_number {
            self.connection.discard_reply(number);
        }
    }
}

pub trait Event {
    fn raw_bytes(&self) -> &[u8];

    fn raw_response_type(&self) -> u8 {
        self.raw_bytes()[0]
    }

    fn response_type(&self) -> u8 {
        self.raw_response_type() & 0x7f
    }

    fn server_generated(&self) -> bool {
        self.raw_response_type() & 0x80 == 0
    }

    fn raw_sequence_number(&self) -> Option<u16> {
        use crate::generated::xproto::KEYMAP_NOTIFY_EVENT;
        match self.response_type() {
            KEYMAP_NOTIFY_EVENT => None,
            _ => {
                let bytes = self.raw_bytes();
                Some(u16::from_ne_bytes([bytes[2], bytes[3]]))
            }
        }
    }
}

#[derive(Debug)]
pub struct GenericEvent(CSlice);

impl Event for GenericEvent {
    fn raw_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl Into<CSlice> for GenericEvent {
    fn into(self) -> CSlice {
        self.0
    }
}

const REPLY: u8 = 1;

impl TryFrom<CSlice> for GenericEvent {
    type Error = ParseError;

    fn try_from(value: CSlice) -> Result<Self, Self::Error> {
        use super::generated::xproto::GE_GENERIC_EVENT;
        if value.len() < 32 {
            return Err(ParseError::ParseError);
        }
        let length_field = u32::from_ne_bytes([value[4], value[5], value[6], value[7]]);
        let length_field: usize = length_field.try_into()?;
        let actual_length = value.len();
        let event = GenericEvent(value);
        let expected_length = match event.response_type() {
            GE_GENERIC_EVENT | REPLY => 32 + 4 * length_field,
            _ => 32
        };
        if actual_length != expected_length {
            return Err(ParseError::ParseError);
        }
        Ok(event)
    }
}

#[derive(Debug)]
pub struct GenericError(CSlice);

impl GenericError {
    pub fn error_code(&self) -> u8 {
        self.raw_bytes()[1]
    }
}

impl Event for GenericError {
    fn raw_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl Into<CSlice> for GenericError {
    fn into(self) -> CSlice {
        self.0
    }
}

impl TryFrom<GenericEvent> for GenericError {
    type Error = ParseError;

    fn try_from(event: GenericEvent) -> Result<Self, Self::Error> {
        if event.response_type() != 0 {
            return Err(ParseError::ParseError)
        }
        Ok(GenericError(event.into()))
    }
}

impl TryFrom<CSlice> for GenericError {
    type Error = ParseError;

    fn try_from(value: CSlice) -> Result<Self, Self::Error> {
        let event: GenericEvent = value.try_into()?;
        event.try_into()
    }
}

pub trait TryParse: Sized {
    fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError>;
}

macro_rules! implement_try_parse {
    ($t:ty: [$($indicies: expr),*]) => {
        impl TryParse for $t {
            fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
                let len = std::mem::size_of::<$t>();
                if value.len() < len {
                    Err(ParseError::ParseError)
                } else {
                    Ok((<$t>::from_ne_bytes([ $(value[$indicies],)* ]), &value[len..]))
                }
            }
        }
    }
}

implement_try_parse!(u8: [0]);
implement_try_parse!(i8: [0]);
implement_try_parse!(u16: [0, 1]);
implement_try_parse!(i16: [0, 1]);
implement_try_parse!(u32: [0, 1, 2, 3]);
implement_try_parse!(i32: [0, 1, 2, 3]);

#[macro_export]
macro_rules! bitmask_binop {
    ($t:ty, $u:ty) => {
        impl std::ops::BitOr for $t {
            type Output = $u;
            fn bitor(self, other: Self) -> Self::Output {
                Into::<Self::Output>::into(self) | Into::<Self::Output>::into(other)
            }
        }
        impl std::ops::BitOr<$u> for $t {
            type Output = $u;
            fn bitor(self, other: $u) -> Self::Output {
                Into::<Self::Output>::into(self) | other
            }
        }
        impl std::ops::BitOr<$t> for $u {
            type Output = $u;
            fn bitor(self, other: $t) -> Self::Output {
                self | Into::<Self::Output>::into(other)
            }
        }
        impl std::ops::BitOrAssign<$t> for $u {
            fn bitor_assign(&mut self, other: $t) {
                *self |= Into::<Self>::into(other)
            }
        }
    }
}
