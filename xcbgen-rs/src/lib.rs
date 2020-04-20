#![deny(
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unreachable_pub,
    unused_import_braces,
    unused_must_use,
    unused_qualifications
)]
#![forbid(unsafe_code)]

pub mod defs;
mod parser;
mod resolver;

pub use parser::{ParseError, Parser};
pub use resolver::{resolve, ResolveError};
