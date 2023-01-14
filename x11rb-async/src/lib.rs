// This code is dual licensed under MIT OR Apache 2.0.

//! Asynchronous X11 rust bindings.

// -- Public Modules --

pub mod blocking;
pub mod connection;
pub mod rust_connection;

// -- Private Modules --

mod cookie;
mod util;

pub use cookie::{Cookie, CookieWithFds, VoidCookie};

#[doc(inline)]
pub use x11rb::{
    connection::{BufWithFds, RawEventAndSeqNumber, ReplyOrError, SequenceNumber},
    errors,
    protocol::{xproto::Setup, Event},
    utils::RawFdContainer,
    x11_utils,
};
