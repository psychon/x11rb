//! This module contains the current mess that is error handling.

use crate::protocol::xproto::{SetupAuthenticate, SetupFailed};
use crate::protocol::Error;

/// An error occurred while parsing some data
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// Error while parsing some data
    ParseError,
}

impl std::error::Error for ParseError {}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error while parsing (not enough data?)")
    }
}

/// An error that occurred while connecting to an X11 server
#[derive(Debug)]
pub enum ConnectError {
    UnknownError,

    /// Error while parsing some data, see `ParseError`.
    ParseError,

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
            ConnectError::ParseError => write!(f, "Parsing error"),
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
        match err {
            ParseError::ParseError => ConnectError::ParseError,
        }
    }
}

impl From<std::io::Error> for ConnectError {
    fn from(err: std::io::Error) -> Self {
        ConnectError::IOError(err)
    }
}

/// An error that occurred on an already established X11 connection
#[derive(Debug)]
pub enum ConnectionError {
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
    ParseError,

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
            ConnectionError::ParseError => write!(f, "Parsing error"),
            ConnectionError::IOError(err) => err.fmt(f),
        }
    }
}

impl From<ParseError> for ConnectionError {
    fn from(err: ParseError) -> Self {
        match err {
            ParseError::ParseError => ConnectionError::ParseError,
        }
    }
}

impl From<std::io::Error> for ConnectionError {
    fn from(err: std::io::Error) -> Self {
        ConnectionError::IOError(err)
    }
}

/// An error that occurred with some request.
#[derive(Debug)]
pub enum ReplyError<B: AsRef<[u8]> + std::fmt::Debug> {
    /// Some error occurred on the X11 connection.
    ConnectionError(ConnectionError),
    /// The X11 server sent an error in response to the request.
    X11Error(Error<B>),
}

impl<B: AsRef<[u8]> + std::fmt::Debug> std::error::Error for ReplyError<B> {}

impl<B: AsRef<[u8]> + std::fmt::Debug> std::fmt::Display for ReplyError<B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReplyError::ConnectionError(e) => write!(f, "{}", e),
            ReplyError::X11Error(e) => write!(f, "X11 error {:?}", e),
        }
    }
}

impl<B: AsRef<[u8]> + std::fmt::Debug> From<ParseError> for ReplyError<B> {
    fn from(err: ParseError) -> Self {
        Self::from(ConnectionError::from(err))
    }
}

impl<B: AsRef<[u8]> + std::fmt::Debug> From<std::io::Error> for ReplyError<B> {
    fn from(err: std::io::Error) -> Self {
        ConnectionError::from(err).into()
    }
}

impl<B: AsRef<[u8]> + std::fmt::Debug> From<ConnectionError> for ReplyError<B> {
    fn from(err: ConnectionError) -> Self {
        Self::ConnectionError(err)
    }
}

impl<B: AsRef<[u8]> + std::fmt::Debug> From<Error<B>> for ReplyError<B> {
    fn from(err: Error<B>) -> Self {
        Self::X11Error(err)
    }
}

/// An error caused by some request or by the exhaustion of IDs.
#[derive(Debug)]
pub enum ReplyOrIdError<B: AsRef<[u8]> + std::fmt::Debug> {
    /// All available IDs have been exhausted.
    IdsExhausted,
    /// Some error occurred on the X11 connection.
    ConnectionError(ConnectionError),
    /// The X11 server sent an error in response to a XC-MISC request.
    X11Error(Error<B>),
}

impl<B: AsRef<[u8]> + std::fmt::Debug> std::fmt::Display for ReplyOrIdError<B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReplyOrIdError::IdsExhausted => f.write_str("X11 IDs have been exhausted"),
            ReplyOrIdError::ConnectionError(e) => write!(f, "{}", e),
            ReplyOrIdError::X11Error(e) => write!(f, "X11 error {:?}", e),
        }
    }
}

impl<B: AsRef<[u8]> + std::fmt::Debug> std::error::Error for ReplyOrIdError<B> {}

impl<B: AsRef<[u8]> + std::fmt::Debug> From<ConnectionError> for ReplyOrIdError<B> {
    fn from(err: ConnectionError) -> Self {
        ReplyOrIdError::ConnectionError(err)
    }
}

impl<B: AsRef<[u8]> + std::fmt::Debug> From<Error<B>> for ReplyOrIdError<B> {
    fn from(err: Error<B>) -> Self {
        ReplyOrIdError::X11Error(err)
    }
}

impl<B: AsRef<[u8]> + std::fmt::Debug> From<ReplyError<B>> for ReplyOrIdError<B> {
    fn from(err: ReplyError<B>) -> Self {
        match err {
            ReplyError::ConnectionError(err) => ReplyOrIdError::ConnectionError(err),
            ReplyError::X11Error(err) => ReplyOrIdError::X11Error(err),
        }
    }
}
