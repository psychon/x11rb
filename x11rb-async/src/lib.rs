// This code is dual licensed under MIT OR Apache 2.0.

//! Asynchronous X11 rust bindings.

// -- Public Modules --

pub mod blocking;
pub mod connection;
#[allow(clippy::type_complexity)]
#[rustfmt::skip]
pub mod protocol;
pub mod rust_connection;

// -- Private Modules --

mod cookie;

pub use cookie::{Cookie, CookieWithFds, VoidCookie};

#[doc(inline)]
pub use x11rb::{
    connection::{BufWithFds, RawEventAndSeqNumber, ReplyOrError, SequenceNumber},
    errors,
    protocol::{xproto::Setup, Event},
    utils::RawFdContainer,
    x11_utils,
};

mod utils {
    pub use crate::RawFdContainer;
}
