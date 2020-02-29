//! Some wrappers around the generated code to simplify use.

use std::convert::TryInto;
use std::marker::PhantomData;

use super::connection::Connection;
use super::cookie::{Cookie, VoidCookie};
use super::errors::{ConnectionError, ConnectionErrorOrX11Error};
use super::generated::xproto::{ConnectionExt as XProtoConnectionExt, InternAtomReply, ATOM};
use super::x11_utils::TryParse;

/// Iterator implementation used by `GetPropertyReply`.
///
/// This is the actual type returned by `GetPropertyReply::value8`, `GetPropertyReply::value16`,
/// and `GetPropertyReply::value32`. This type needs to be public due to Rust's visibility rules.
#[derive(Debug, Clone)]
pub struct PropertyIterator<'a, T>(&'a [u8], PhantomData<T>);

impl<'a, T> PropertyIterator<'a, T> {
    pub(crate) fn new(value: &'a [u8]) -> Self {
        PropertyIterator(value, PhantomData)
    }
}

impl<T> Iterator for PropertyIterator<'_, T>
where
    T: TryParse,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match T::try_parse(self.0) {
            Ok((value, remaining)) => {
                self.0 = remaining;
                Some(value)
            }
            Err(_) => {
                self.0 = &[];
                None
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.0.len() / std::mem::size_of::<T>();
        (size, Some(size))
    }
}

impl<T: TryParse> std::iter::FusedIterator for PropertyIterator<'_, T> {}

/// Extension trait that simplifies API use
pub trait ConnectionExt: XProtoConnectionExt {
    /// Change a property on a window with format 8.
    fn change_property8<A>(
        &self,
        mode: A,
        window: u32,
        property: u32,
        type_: u32,
        data: &[u8],
    ) -> Result<VoidCookie<Self>, ConnectionError>
    where
        A: Into<u8>,
    {
        self.change_property(
            mode,
            window,
            property,
            type_,
            8,
            data.len().try_into()?,
            data,
        )
    }

    /// Change a property on a window with format 16.
    fn change_property16<A>(
        &self,
        mode: A,
        window: u32,
        property: u32,
        type_: u32,
        data: &[u16],
    ) -> Result<VoidCookie<Self>, ConnectionError>
    where
        A: Into<u8>,
    {
        let mut data_u8 = Vec::with_capacity(data.len() * 2);
        for item in data {
            data_u8.extend(&item.to_ne_bytes());
        }
        self.change_property(
            mode,
            window,
            property,
            type_,
            16,
            data.len().try_into()?,
            &data_u8,
        )
    }

    /// Change a property on a window with format 32.
    fn change_property32<A>(
        &self,
        mode: A,
        window: u32,
        property: u32,
        type_: u32,
        data: &[u32],
    ) -> Result<VoidCookie<Self>, ConnectionError>
    where
        A: Into<u8>,
    {
        let mut data_u8 = Vec::with_capacity(data.len() * 4);
        for item in data {
            data_u8.extend(&item.to_ne_bytes());
        }
        self.change_property(
            mode,
            window,
            property,
            type_,
            32,
            data.len().try_into()?,
            &data_u8,
        )
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

/// A type allowing to lazily query atoms.
///
/// To avoid round-trips, X11 clients should not send requests and then synchronously wait for the
/// reply. This is especially true for atoms, because a typical application will need many atoms.
/// Doing one round-trip for each atom can be quite slow. This type represents an atom that is
/// lazily resolved on its first use. Thus, this type hides the involved latency and simplifies
/// code.
#[derive(Debug)]
pub enum LazyAtom<'c, C: Connection> {
    Pending(Cookie<'c, C, InternAtomReply>),
    Resolved(ATOM),
    Errored,
}

impl<'c, C: Connection> LazyAtom<'c, C> {
    /// Create a new LazyAtom by sending an `InternAtom` request.
    ///
    /// The meaning of the arguments is identical to xproto's `InternAtom` request.
    pub fn new(conn: &'c C, only_if_exists: bool, name: &[u8]) -> Self {
        match conn.intern_atom(only_if_exists, name) {
            Ok(cookie) => LazyAtom::Pending(cookie),
            Err(_) => LazyAtom::Errored,
        }
    }

    /// Create a new LazyAtom for the given resolved atom.
    ///
    /// This function just wraps an existing atom. The resulting LazyAtom will always return
    /// `Ok(atom)` from `atom()`.
    pub fn new_for_atom(atom: ATOM) -> Self {
        LazyAtom::Resolved(atom)
    }

    /// Get the atom that is contained in this type.
    ///
    /// This function gets the answer from the X11 server if it was not yet fetched. It returns the atom value.
    pub fn atom(&mut self) -> Result<ATOM, ConnectionErrorOrX11Error> {
        match self {
            LazyAtom::Pending(_) => {
                // We need to move the cookie out of self to call reply()
                if let LazyAtom::Pending(cookie) = std::mem::replace(self, LazyAtom::Resolved(0)) {
                    // Now get the reply and replace self again with the correct value
                    let reply = cookie.reply().map(|reply| reply.atom);
                    *self = match reply {
                        Ok(atom) => LazyAtom::Resolved(atom),
                        Err(_) => LazyAtom::Errored,
                    };
                    reply
                } else {
                    unreachable!()
                }
            }
            LazyAtom::Resolved(result) => Ok(*result),
            LazyAtom::Errored => Err(ConnectionError::UnknownError.into()),
        }
    }
}
