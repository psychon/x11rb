//! This module contains the current mess that is error handling.

use crate::protocol::xproto::{SetupAuthenticate, SetupFailed};
use crate::x11_utils::X11Error;

/// An error occurred  while dynamically loading libxcb.
#[cfg(feature = "dl-libxcb")]
#[derive(Debug, Clone)]
pub enum LibxcbLoadError {
    /// Could not open the library. `load_libxcb` might try to load
    /// different library file names, so the vector contains the reason
    /// that caused each one to fail.
    ///
    /// The `OsString` is the library file name and the string is the
    /// reason.
    OpenLibError(Vec<(std::ffi::OsString, String)>),
    /// Could not get a symbol from the library. The byte vector is the
    /// symbol name and the string is the reason.
    GetSymbolError(Vec<u8>, String),
}

#[cfg(feature = "dl-libxcb")]
impl std::fmt::Display for LibxcbLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LibxcbLoadError::OpenLibError(libs_errors) => {
                f.write_str("failed to open libraries: ")?;
                for (i, (lib_name, e)) in libs_errors.iter().enumerate() {
                    if i != 0 {
                        f.write_str(", ")?;
                    }
                    write!(f, "{:?} ({})", lib_name, e)?;
                }
                Ok(())
            }
            LibxcbLoadError::GetSymbolError(symbol, e) => write!(
                f,
                "failed to get symbol \"{}\": {}",
                symbol
                    .iter()
                    .map(|&c| std::ascii::escape_default(c))
                    .flatten()
                    .map(char::from)
                    .collect::<String>(),
                e,
            ),
        }
    }
}

#[cfg(feature = "dl-libxcb")]
impl std::error::Error for LibxcbLoadError {}

/// An error occurred while parsing some data
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(
    not(feature = "I_need_rust_1_37_compatibility_but_know_that_enums_are_still_non_exhaustive"),
    non_exhaustive
)]
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
    /// When parsing the value of an enumeration, not all possible integer values have a defined
    /// meaning. This error occurs when an invalid value is encountered in this context.
    ///
    /// For example, `xproto` has an enumeration `Place` with possible values `OnTop` (0) and
    /// `OnBottom` (1). Any value other than 0 or 1 generates an `InvalidValue` when parsing.
    InvalidValue,

    /// Some file descriptors were expected, but none were received.
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

/// An error that occurred while connecting to an X11 server
#[derive(Debug)]
#[cfg_attr(
    not(feature = "I_need_rust_1_37_compatibility_but_know_that_enums_are_still_non_exhaustive"),
    non_exhaustive
)]
pub enum ConnectError {
    /// An unknown error occurred.
    ///
    /// One situation were this error is used when libxcb indicates an error that does not match
    /// any of the defined error conditions. Thus, libxcb is violating its own API (or new error
    /// cases were defined, but are not yet handled by x11rb)
    UnknownError,

    /// Error while parsing some data, see `ParseError`.
    ParseError(ParseError),

    /// Out of memory.
    ///
    /// This is `XCB_CONN_CLOSED_MEM_INSUFFICIENT`.
    InsufficientMemory,

    /// Error during parsing of display string.
    ///
    /// This is `XCB_CONN_CLOSSED_PARSE_ERR`.
    DisplayParsingError,

    /// Server does not have a screen matcing the display.
    ///
    /// This is `XCB_CONN_CLOSED_INVALID_SCREEN`.
    InvalidScreen,

    /// An I/O error occurred on the connection.
    IOError(std::io::Error),

    /// Invalid ID mask provided by the server.
    ///
    /// The value of `resource_id_mask` in the `Setup` provided by the server was zero.
    ZeroIDMask,

    /// The server rejected the connection with a `SetupAuthenticate` message.
    SetupAuthenticate(SetupAuthenticate),
    ///
    /// The server rejected the connection with a `SetupFailed` message.
    SetupFailed(SetupFailed),
}

impl std::error::Error for ConnectError {}

impl std::fmt::Display for ConnectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn display(
            f: &mut std::fmt::Formatter<'_>,
            prefix: &str,
            value: &[u8],
        ) -> std::fmt::Result {
            match std::str::from_utf8(value).ok() {
                Some(value) => write!(f, "{}: '{}'", prefix, value),
                None => write!(f, "{}: {:?} [message is not utf8]", prefix, value),
            }
        }
        match self {
            ConnectError::UnknownError => write!(f, "Unknown connection error"),
            ConnectError::InsufficientMemory => write!(f, "Insufficient memory"),
            ConnectError::DisplayParsingError => write!(f, "Display parsing error"),
            ConnectError::InvalidScreen => write!(f, "Invalid screen"),
            ConnectError::ParseError(err) => err.fmt(f),
            ConnectError::IOError(err) => err.fmt(f),
            ConnectError::ZeroIDMask => write!(f, "XID mask was zero"),
            ConnectError::SetupFailed(err) => display(f, "X11 setup failed", &err.reason),
            ConnectError::SetupAuthenticate(err) => {
                display(f, "X11 authentication failed", &err.reason)
            }
        }
    }
}

impl From<ParseError> for ConnectError {
    fn from(err: ParseError) -> Self {
        ConnectError::ParseError(err)
    }
}

impl From<std::io::Error> for ConnectError {
    fn from(err: std::io::Error) -> Self {
        ConnectError::IOError(err)
    }
}

/// An error that occurred on an already established X11 connection
#[derive(Debug)]
#[cfg_attr(
    not(feature = "I_need_rust_1_37_compatibility_but_know_that_enums_are_still_non_exhaustive"),
    non_exhaustive
)]
pub enum ConnectionError {
    /// An unknown error occurred.
    ///
    /// One situation were this error is used when libxcb indicates an error that does not match
    /// any of the defined error conditions. Thus, libxcb is violating its own API (or new error
    /// cases were defined, but are not yet handled by x11rb)
    UnknownError,

    /// An X11 extension was not supported by the server.
    ///
    /// This corresponds to `XCB_CONN_CLOSED_EXT_NOTSUPPORTED`.
    UnsupportedExtension,

    /// A request larger than the maximum request length was sent.
    ///
    /// This corresponds to `XCB_CONN_CLOSED_REQ_LEN_EXCEED`.
    MaximumRequestLengthExceeded,

    /// File descriptor passing failed.
    ///
    /// This corresponds to `XCB_CONN_CLOSED_FDPASSING_FAILED`.
    FDPassingFailed,

    /// Error while parsing some data, see `ParseError`.
    ParseError(ParseError),

    /// Out of memory.
    ///
    /// This is `XCB_CONN_CLOSED_MEM_INSUFFICIENT`.
    InsufficientMemory,

    /// An I/O error occurred on the connection.
    IOError(std::io::Error),
}

impl std::error::Error for ConnectionError {}

impl std::fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectionError::UnknownError => write!(f, "Unknown connection error"),
            ConnectionError::UnsupportedExtension => write!(f, "Unsupported extension"),
            ConnectionError::InsufficientMemory => write!(f, "Insufficient memory"),
            ConnectionError::MaximumRequestLengthExceeded => {
                write!(f, "Maximum request length exceeded")
            }
            ConnectionError::FDPassingFailed => write!(f, "FD passing failed"),
            ConnectionError::ParseError(err) => err.fmt(f),
            ConnectionError::IOError(err) => err.fmt(f),
        }
    }
}

impl From<ParseError> for ConnectionError {
    fn from(err: ParseError) -> Self {
        ConnectionError::ParseError(err)
    }
}

impl From<std::io::Error> for ConnectionError {
    fn from(err: std::io::Error) -> Self {
        ConnectionError::IOError(err)
    }
}

/// An error that occurred with some request.
#[derive(Debug)]
pub enum ReplyError {
    /// Some error occurred on the X11 connection.
    ConnectionError(ConnectionError),
    /// The X11 server sent an error in response to the request.
    X11Error(X11Error),
}

impl std::error::Error for ReplyError {}

impl std::fmt::Display for ReplyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReplyError::ConnectionError(e) => write!(f, "{}", e),
            ReplyError::X11Error(e) => write!(f, "X11 error {:?}", e),
        }
    }
}

impl From<ParseError> for ReplyError {
    fn from(err: ParseError) -> Self {
        Self::from(ConnectionError::from(err))
    }
}

impl From<std::io::Error> for ReplyError {
    fn from(err: std::io::Error) -> Self {
        ConnectionError::from(err).into()
    }
}

impl From<ConnectionError> for ReplyError {
    fn from(err: ConnectionError) -> Self {
        Self::ConnectionError(err)
    }
}

impl From<X11Error> for ReplyError {
    fn from(err: X11Error) -> Self {
        Self::X11Error(err)
    }
}

/// An error caused by some request or by the exhaustion of IDs.
#[derive(Debug)]
pub enum ReplyOrIdError {
    /// All available IDs have been exhausted.
    IdsExhausted,
    /// Some error occurred on the X11 connection.
    ConnectionError(ConnectionError),
    /// The X11 server sent an error in response to a XC-MISC request.
    X11Error(X11Error),
}

impl std::fmt::Display for ReplyOrIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReplyOrIdError::IdsExhausted => f.write_str("X11 IDs have been exhausted"),
            ReplyOrIdError::ConnectionError(e) => write!(f, "{}", e),
            ReplyOrIdError::X11Error(e) => write!(f, "X11 error {:?}", e),
        }
    }
}

impl std::error::Error for ReplyOrIdError {}

impl From<ParseError> for ReplyOrIdError {
    fn from(err: ParseError) -> Self {
        ConnectionError::from(err).into()
    }
}

impl From<ConnectionError> for ReplyOrIdError {
    fn from(err: ConnectionError) -> Self {
        ReplyOrIdError::ConnectionError(err)
    }
}

impl From<X11Error> for ReplyOrIdError {
    fn from(err: X11Error) -> Self {
        ReplyOrIdError::X11Error(err)
    }
}

impl From<ReplyError> for ReplyOrIdError {
    fn from(err: ReplyError) -> Self {
        match err {
            ReplyError::ConnectionError(err) => ReplyOrIdError::ConnectionError(err),
            ReplyError::X11Error(err) => ReplyOrIdError::X11Error(err),
        }
    }
}
