// This code is dual licensed under MIT OR Apache 2.0.

//! Asynchronous X11 rust bindings.

// -- Public Modules --

pub mod blocking;
pub mod connection;
#[allow(clippy::type_complexity)]
#[rustfmt::skip]
pub mod protocol;
pub mod rust_connection;

#[doc(inline)]
pub use x11rb::{errors, x11_utils};

#[doc(inline)]
pub use x11rb_protocol::SequenceNumber;

// -- Private Modules --

mod cookie;

pub use cookie::{Cookie, CookieWithFds, VoidCookie};

pub mod utils {
    pub use x11rb::utils::RawFdContainer;
}
