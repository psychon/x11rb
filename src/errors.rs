//! This module contains the current mess that is error handling.

use std::error::Error;

use crate::generated::xproto::{SetupAuthenticate, SetupFailed};
use crate::x11_utils::GenericError;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ParseError {
    ParseError,
}

impl Error for ParseError {}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error while parsing (not enough data?)")
    }
}

impl From<std::convert::Infallible> for ParseError {
    fn from(_: std::convert::Infallible) -> Self {
        unreachable!()
    }
}

impl From<std::num::TryFromIntError> for ParseError {
    fn from(_: std::num::TryFromIntError) -> Self {
        ParseError::ParseError
    }
}

/// An error that occurred while connecting to an X11 server
#[derive(Debug)]
pub enum ConnectError {
    UnknownError,
    ParseError,

    // Errors from XCB
    ConnectionError,
    InsufficientMemory,
    DisplayParsingError,
    InvalidScreen,

    // Errors from RustConnection
    IOError(std::io::Error),
    ZeroIDMask,
    SetupAuthenticate(SetupAuthenticate),
    SetupFailed(SetupFailed),
}

impl Error for ConnectError {}

impl std::fmt::Display for ConnectError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        fn display(f: &mut std::fmt::Formatter, prefix: &str, value: &[u8]) -> std::fmt::Result {
            match std::str::from_utf8(value).ok() {
                Some(value) => write!(f, "{}: '{}'", prefix, value),
                None => write!(f, "{}: {:?} [message is not utf8]", prefix, value),
            }
        }
        match self {
            ConnectError::UnknownError => write!(f, "Unknown connection error"),
            ConnectError::ConnectionError => write!(f, "Error with the underlying connection"),
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
    ConnectionError,
    UnsupportedExtension,
    MaximumRequestLengthExceeded,
    FDPassingFailed,
    ParseError,

    // Errors from XCB
    InsufficientMemory,

    // Errors from RustConnection
    IOError(std::io::Error),
}

impl Error for ConnectionError {}

impl std::fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConnectionError::UnknownError => write!(f, "Unknown connection error"),
            ConnectionError::ConnectionError => write!(f, "Error with the underlying connection"),
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

impl From<std::num::TryFromIntError> for ConnectionError {
    fn from(err: std::num::TryFromIntError) -> Self {
        Self::from(ParseError::from(err))
    }
}

impl From<std::io::Error> for ConnectionError {
    fn from(err: std::io::Error) -> Self {
        ConnectionError::IOError(err)
    }
}

#[derive(Debug)]
pub enum ConnectionErrorOrX11Error {
    ConnectionError(ConnectionError),
    X11Error(GenericError),
}

impl Error for ConnectionErrorOrX11Error {}

impl std::fmt::Display for ConnectionErrorOrX11Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConnectionErrorOrX11Error::ConnectionError(e) => write!(f, "{}", e),
            ConnectionErrorOrX11Error::X11Error(e) => write!(f, "X11 error {:?}", e),
        }
    }
}

impl From<ParseError> for ConnectionErrorOrX11Error {
    fn from(err: ParseError) -> Self {
        Self::from(ConnectionError::from(err))
    }
}

impl From<std::num::TryFromIntError> for ConnectionErrorOrX11Error {
    fn from(err: std::num::TryFromIntError) -> Self {
        Self::from(ParseError::from(err))
    }
}

impl From<std::io::Error> for ConnectionErrorOrX11Error {
    fn from(err: std::io::Error) -> Self {
        ConnectionError::from(err).into()
    }
}

impl From<ConnectionError> for ConnectionErrorOrX11Error {
    fn from(err: ConnectionError) -> Self {
        Self::ConnectionError(err)
    }
}

impl From<GenericError> for ConnectionErrorOrX11Error {
    fn from(err: GenericError) -> Self {
        Self::X11Error(err)
    }
}
