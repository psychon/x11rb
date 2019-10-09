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

pub mod generated {
    include!(concat!(env!("OUT_DIR"), "/generated/mod.rs"));
}
