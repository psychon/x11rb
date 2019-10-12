//! Some wrappers around the generated code to simplify use.

use std::convert::TryInto;

use super::generated::xproto::change_property;
use super::connection::{Connection, SequenceNumber};
use super::errors::ConnectionError;

/// Change a property on a window with format 8.
pub fn change_property8<C: Connection, A>(c: &C, mode: A, window: u32, property: u32, type_: u32, data: &[u8])
-> Result<SequenceNumber, ConnectionError>
where A: Into<u8>
{
    change_property(c, mode, window, property, type_, 8, data.len().try_into()?, data)
}

/// Change a property on a window with format 16.
pub fn change_property16<C: Connection, A>(c: &C, mode: A, window: u32, property: u32, type_: u32, data: &[u16])
-> Result<SequenceNumber, ConnectionError>
where A: Into<u8>
{
    let mut data_u8 = Vec::with_capacity(data.len() * 2);
    for item in data {
        data_u8.extend(&item.to_ne_bytes());
    }
    change_property(c, mode, window, property, type_, 16, data.len().try_into()?, &data_u8)
}

/// Change a property on a window with format 32.
pub fn change_property32<C: Connection, A>(c: &C, mode: A, window: u32, property: u32, type_: u32, data: &[u32])
-> Result<SequenceNumber, ConnectionError>
where A: Into<u8>
{
    let mut data_u8 = Vec::with_capacity(data.len() * 4);
    for item in data {
        data_u8.extend(&item.to_ne_bytes());
    }
    change_property(c, mode, window, property, type_, 32, data.len().try_into()?, &data_u8)
}
