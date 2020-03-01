//! X11 rust bindings.
//!
//! This library allows to interact with an X11 server from rust code. A connection to an X11
//! server is represented by an implementation of the `Connection` trait.
//!
//! The client can interact with the server by sending requests. The server can answer requests and
//! can also generate events.
//!
//! The examples that come with this library might be a good starting point for new users.
//!
//!
//! # Getting started with X11
//!
//! X11 is a big protocol. I would claim that most of it is not actually that complicated, but it
//! is still difficult to get into it. A good starting point might be some [libxcb
//! tutorial](https://www.x.org/releases/X11R7.7/doc/libxcb/tutorial/index.html). This tutorial
//! was adapted in this crate [as an
//! example](https://github.com/psychon/x11rb/blob/master/examples/tutorial.rs). A more in-depth
//! look at the X11 protocol can be gained from the [protocol reference
//! manual](https://www.x.org/releases/X11R7.6/doc/xproto/x11protocol.html), but this requires some
//! existing basic understanding of X11. If you want to figure out what some specific request does,
//! be sure to look it up in the specification!
//!
//! Most extensions can be understood by reading their specification. Most of them can be found
//! [here](https://www.x.org/releases/current/doc/index.html#protocol). For example, [the
//! specification of Composite
//! 0.4](https://www.x.org/releases/X11R7.5/doc/compositeproto/compositeproto.txt) consists of
//! about six pages of text.
//!
//! The notable exception is the X keyboard extension, which is documented in a [PDF file with 168
//! pages](https://www.x.org/releases/current/doc/kbproto/xkbproto.pdf) which I am never going to
//! read completely.
//!
//!
//! # Getting started with x11rb
//!
//! Most code in this code is automatically generated from an XML description of the protocol. This
//! is the same approach as taken by [libxcb](https://xcb.freedesktop.org/) (and in fact this uses
//! the same XML description). This means that if you know your way around X11, most things should
//! be obvious to you.
//!
//! For example, here is how to create a new window with x11rb:
//! ```no_run
//! use x11rb::connection::Connection;
//! use x11rb::generated::xproto::*;
//! use x11rb::errors::ReplyError;
//! use x11rb::COPY_DEPTH_FROM_PARENT;
//!
//! fn main() -> Result<(), ReplyError> {
//!     let (conn, screen_num) = x11rb::connect(None).unwrap();
//!     let screen = &conn.setup().roots[screen_num];
//!     let win_id = conn.generate_id();
//!     conn.create_window(COPY_DEPTH_FROM_PARENT, win_id, screen.root, 0, 0, 100, 100, 0, WindowClass::InputOutput,
//!                        0, &CreateWindowAux::new().background_pixel(screen.white_pixel))?;
//!     conn.map_window(win_id)?;
//!     conn.flush();
//!     loop {
//!         println!("Event: {:?}", conn.wait_for_event()?);
//!     }
//! }
//! ```
//! More examples can be found in the
//! [examples](https://github.com/psychon/x11rb/tree/master/examples) directory.

#![deny(missing_copy_implementations,
        missing_debug_implementations,
        //missing_docs,
        private_doc_tests,
        single_use_lifetimes,
        trivial_casts,
        trivial_numeric_casts,
        unreachable_pub,
        unused_extern_crates,
        unused_import_braces,
        unused_qualifications,
        unused_results,
        )]
#![cfg_attr(not(feature = "allow-unsafe-code"), forbid(unsafe_code))]

pub mod utils;
#[cfg(feature = "allow-unsafe-code")]
pub mod xcb_ffi;
#[macro_use]
pub mod x11_utils;
pub mod connection;
pub mod cookie;
pub mod errors;
pub mod extension_information;
pub mod rust_connection;

pub mod generated {
    include!(concat!(env!("OUT_DIR"), "/generated/mod.rs"));
}
pub mod wrapper;

use connection::Connection;
use errors::ConnectError;
use generated::xproto::{KEYSYM, TIMESTAMP};

/// Establish a new connection to an X11 server.
///
/// If a `dpy_name` is provided, it describes the display that should be connected to, for
/// example `127.0.0.1:1`. If no value is provided, the `$DISPLAY` environment variable is
/// used.
pub fn connect(
    dpy_name: Option<&str>,
) -> Result<(impl Connection + Send + Sync, usize), ConnectError> {
    #[cfg(feature = "allow-unsafe-code")]
    {
        let dpy_name = dpy_name
            .map(std::ffi::CString::new)
            .transpose()
            .map_err(|_| ConnectError::DisplayParsingError)?;
        let dpy_name = dpy_name.as_ref().map(|d| &**d);
        xcb_ffi::XCBConnection::connect(dpy_name)
    }
    #[cfg(not(feature = "allow-unsafe-code"))]
    {
        rust_connection::RustConnection::connect(dpy_name)
    }
}

/// The universal null resource or null atom parameter value for many core X requests
pub const NONE: u32 = 0;

/// This constant can be used for many parameters in `create_window`
pub const COPY_FROM_PARENT: u32 = 0;

/// This constant can be used for the depth parameter in `create_window`. It indicates to use the
/// parent window's depth.
pub const COPY_DEPTH_FROM_PARENT: u8 = 0;

/// This constant can be used for the class parameter in `create_window`. It indicates to use the
/// parent window's class.
pub const COPY_CLASS_FROM_PARENT: u16 = 0;

/// This constant can be used in most request that take a timestamp argument
pub const CURRENT_TIME: TIMESTAMP = 0;

/// This constant can be used to fill unused entries in `KEYSYM` tables
pub const NO_SYMBOL: KEYSYM = 0;
