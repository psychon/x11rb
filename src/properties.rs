//! Utility functions for working with X11 properties

use crate::connection::RequestConnection;
use crate::cookie::Cookie;
use crate::errors::{ConnectionError, ParseError, ReplyError};
use crate::xproto::{AtomEnum, GetPropertyReply, Window, get_property};

/// A cookie for getting a window's `WM_CLASS` property.
#[derive(Debug)]
pub struct WmClassCookie<'a, Conn: RequestConnection + ?Sized>(Cookie<'a, Conn, GetPropertyReply>);

impl<'a, Conn> WmClassCookie<'a, Conn>
where Conn: RequestConnection + ?Sized
{
    /// Send a `GetProperty` request for the `WM_CLASS` property of the given window
    pub fn new(conn: &'a Conn, window: Window) -> Result<Self, ConnectionError> {
        Ok(WmClassCookie(get_property(
            conn,
            false,
            window,
            AtomEnum::WM_CLASS.into(),
            AtomEnum::STRING.into(),
            0,
            2048,
        )?))
    }

    /// Get the reply that the server sent.
    pub fn reply(self) -> Result<WmClass, ReplyError<Conn::Buf>> {
        Ok(WmClass::new(self.0.reply()?)?)
    }

    /// Get the reply that the server sent, but have errors handled as events.
    pub fn reply_unchecked(self) -> Result<Option<WmClass>, ConnectionError> {
        self.0.reply_unchecked()?
            .map(|r| WmClass::new(r))
            .transpose()
            .map_err(Into::into)
    }
}

/// The value of a window's `WM_CLASS` property.
#[derive(Debug)]
pub struct WmClass(GetPropertyReply, usize);

impl WmClass {
    /// Send a `GetProperty` request for the `WM_CLASS` property of the given window
    pub fn get<C: RequestConnection>(conn: &C, window: Window) -> Result<WmClassCookie<'_, C>, ConnectionError> {
        WmClassCookie::new(conn, window)
    }

    /// Construct a new `WmClass` instance from a `GetPropertyReply`.
    ///
    /// The original `GetProperty` request must have been for a `WM_CLASS` property for this
    /// function to return sensible results.
    pub fn new(reply: GetPropertyReply) -> Result<Self, ParseError> {
        if reply.type_ != AtomEnum::STRING.into() || reply.format != 8 {
            return Err(ParseError::ParseError);
        }
        // Find the first zero byte in the value
        let offset = reply.value
            .iter()
            .position(|&v| v == 0)
            .unwrap_or_else(|| reply.value.len());
        Ok(WmClass(reply, offset))
    }

    /// Get the instance contained in this `WM_CLASS` property
    pub fn instance(&self) -> &[u8] {
        &self.0.value[0..self.1]
    }

    /// Get the class contained in this `WM_CLASS` property
    pub fn class(&self) -> &[u8] {
        let start = self.1 + 1;
        if start >= self.0.value.len() {
            return &[];
        };
        let end = if self.0.value.last() == Some(&0) {
            self.0.value.len() - 1
        } else {
            self.0.value.len()
        };
        &self.0.value[start..end]
    }
}

#[cfg(test)]
mod test {
    use std::convert::TryInto;

    use crate::xproto::{AtomEnum, GetPropertyReply};
    use super::WmClass;

    fn get_property_reply(value: &[u8]) -> GetPropertyReply {
        GetPropertyReply {
            response_type: 1,
            format: 8,
            sequence: 0,
            length: 0,
            type_: AtomEnum::STRING.into(),
            bytes_after: 0,
            value_len: value.len().try_into().unwrap(),
            value: value.to_vec(),
        }
    }

    #[test]
    fn test_wm_class() {
        for (input, instance, class) in &[
            (&b""[..], &b""[..], &b""[..]),
            (b"\0", b"", b""),
            (b"\0\0", b"", b""),
            (b"\0\0\0", b"", b"\0"),
            (b"Hello World", b"Hello World", b""),
            (b"Hello World\0", b"Hello World", b""),
            (b"Hello\0World\0", b"Hello", b"World"),
            (b"Hello\0World", b"Hello", b"World"),
            (b"Hello\0World\0Good\0Day", b"Hello", b"World\0Good\0Day"),
        ] {
            let wm_class = WmClass::new(get_property_reply(input)).unwrap();
            assert_eq!((wm_class.instance(), wm_class.class()), (*instance, *class));
        }
    }
}
