//! This module contains the current mess that is error handling.

/// An error occurred while parsing some data
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ParseError {
    /// Not enough data was provided.
    InsufficientData,

    /// A value did not fit.
    ///
    /// This error can e.g. happen when a value that was received from the X11 server does not fit
    /// into an `usize`.
    ConversionFailed,

    /// The value of an expression could not be computed.
    ///
    /// As an example, the length of the data in `xproto`'s `GetPropertyReply` is described by
    /// `value_len * (format / 8)`. The multiplication could cause an overflow, which would be
    /// represented by this error.
    InvalidExpression,

    /// A value was outside of its valid range.
    ///
    /// There are two kinds of situations where this error can happen:
    ///
    /// 1. The protocol was violated and a nonsensical value was found.
    /// 2. The user of the API called the wrong parsing function.
    ///
    /// Examples for the first kind of error:
    ///
    /// - One of a set of values should be present (a `<switch>` in xcb-proto-speak), but none of
    ///   the `<cases>` matched. This can e.g. happen when parsing
    ///   [`crate::protocol::xinput::InputInfo`].
    /// - Parsing a request with a length field that is too small for the request header to fit.
    ///
    /// Examples for the second kind of error:
    ///
    /// - Parsing an X11 error with `response_type != 0`.
    /// - Parsing an X11 reply with `response_type != 1`.
    /// - Parsing an X11 request with the wrong value for its `minor_opcode`.
    InvalidValue,

    /// Some file descriptors were expected, but not enough were received.
    MissingFileDescriptors,
}

impl std::error::Error for ParseError {}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InsufficientData => write!(f, "Insufficient data was provided"),
            ParseError::ConversionFailed => {
                write!(f, "A value conversion failed due to out of range data")
            }
            ParseError::InvalidExpression => write!(
                f,
                "An expression could not be computed, e.g. due to overflow"
            ),
            ParseError::InvalidValue => {
                write!(f, "A value could not be parsed into an enumeration")
            }
            ParseError::MissingFileDescriptors => write!(f, "Missing file descriptors"),
        }
    }
}
