//! Some wrappers around the generated code to simplify use.

use std::convert::TryInto;

use super::generated::xproto::ConnectionExt as XProtoConnectionExt;
use super::connection::VoidCookie;
use super::errors::{ConnectionError, ConnectionErrorOrX11Error};

/// Extension trait that simplifies API use
pub trait ConnectionExt: XProtoConnectionExt {
    /// Change a property on a window with format 8.
    fn change_property8<A>(&self, mode: A, window: u32, property: u32, type_: u32, data: &[u8])
    -> Result<VoidCookie<Self>, ConnectionError>
    where A: Into<u8>
    {
        self.change_property(mode, window, property, type_, 8, data.len().try_into()?, data)
    }

    /// Change a property on a window with format 16.
    fn change_property16<A>(&self, mode: A, window: u32, property: u32, type_: u32, data: &[u16])
    -> Result<VoidCookie<Self>, ConnectionError>
    where A: Into<u8>
    {
        let mut data_u8 = Vec::with_capacity(data.len() * 2);
        for item in data {
            data_u8.extend(&item.to_ne_bytes());
        }
        self.change_property(mode, window, property, type_, 16, data.len().try_into()?, &data_u8)
    }

    /// Change a property on a window with format 32.
    fn change_property32<A>(&self, mode: A, window: u32, property: u32, type_: u32, data: &[u32])
    -> Result<VoidCookie<Self>, ConnectionError>
    where A: Into<u8>
    {
        let mut data_u8 = Vec::with_capacity(data.len() * 4);
        for item in data {
            data_u8.extend(&item.to_ne_bytes());
        }
        self.change_property(mode, window, property, type_, 32, data.len().try_into()?, &data_u8)
    }

    /// Synchronise with the X11 server.
    ///
    /// This function synchronises with the X11 server. This means that all requests that are still
    /// in the output buffer are sent to the server. Then, we wait until the X11 server processed
    /// all requests.
    fn sync(&self) -> Result<(), ConnectionErrorOrX11Error> {
        // When a new request is generated, it is appended to the output buffer. Thus, this causes
        // all previous requests to be sent.
        // The X11 server is single-threaded and processes requests in-order. Thus, it will only
        // reply to our GetInputFocus after everything before was processed.
        self.get_input_focus()?.reply().and(Ok(()))
    }
}
impl<C: XProtoConnectionExt + ?Sized> ConnectionExt for C {}
