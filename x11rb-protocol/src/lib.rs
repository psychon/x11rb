//! TODO: Docs

#![forbid(unsafe_code)]

use std::borrow::Cow;

pub mod connection;
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

/// Number type used for referring to things that were sent to the server in responses from the
/// server.
///
/// Each request sent to the X11 server is implicitly assigned a monotonically increasing sequence
/// number. Replies, events, and errors contain the sequence number of the last request that the
/// server received. This allows to map replies to their requests and to figure out which request
/// caused an error.
pub type SequenceNumber = u64;

/// The raw bytes of an event and its sequence number.
pub type RawEventAndSeqNumber<B> = (B, SequenceNumber);

/// Variants describing which responses to a request should be discarded.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DiscardMode {
    /// Only discard the actual reply. Errors go to the main loop.
    DiscardReply,
    /// Ignore any kind of response that this request generates.
    DiscardReplyAndError,
}
