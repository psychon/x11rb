//! TODO: Docs

#![forbid(unsafe_code)]

use std::borrow::Cow;

#[macro_use]
pub mod x11_utils;
pub mod errors;
#[rustfmt::skip]
pub mod protocol;
pub mod utils;
pub mod wrapper;

// Used to avoid too-complex types.
/// A combination of a buffer and a list of file descriptors.
pub type BufWithFds<B> = (B, Vec<utils::RawFdContainer>);
/// A buffer that is logically continuous, but presented in a number of pieces.
pub type PiecewiseBuf<'a> = Vec<Cow<'a, [u8]>>;
