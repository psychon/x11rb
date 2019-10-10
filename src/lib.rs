//! X11 rust bindings.
//!
//! This library allows to interact with an X11 server from rust code. A connection to an X11
//! server is represented by an implementation of the `Connection` trait.
//!
//! The client can interact with the server by sending requests. The server can answer requests and
//! can also generate events.
//!
//! The examples that come with this library might be a good starting point for new users.

#![deny(//missing_copy_implementations,
        //missing_debug_implementations,
        //missing_docs,
        //private_doc_tests,
        //single_use_lifetimes,
        trivial_casts,
        trivial_numeric_casts,
        //unreachable_pub
        )]

extern crate libc;

pub mod xcb_ffi;
pub mod utils;
#[macro_use]
pub mod x11_utils;
pub mod errors;
pub mod connection;
pub mod rust_connection;

pub mod generated {
    include!(concat!(env!("OUT_DIR"), "/generated/mod.rs"));
}
