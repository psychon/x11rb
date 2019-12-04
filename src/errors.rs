//! This module contains the current mess that is error handling.

use std::error::Error;

use crate::x11_utils::GenericError;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ParseError {
    ParseError
}

impl Error for ParseError {
}

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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ConnectionError {
    UnknownError,
    ConnectionError,
    UnsupportedExtension,
    InsufficientMemory,
    MaximumRequestLengthExceeded,
    DisplayParsingError,
    InvalidScreen,
    FDPassingFailed,
    ParseError,
}

impl Error for ConnectionError {
}

impl std::fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConnectionError::UnknownError => write!(f, "Unknown connection error"),
            ConnectionError::ConnectionError => write!(f, "Error with the underlying connection"),
            ConnectionError::UnsupportedExtension => write!(f, "Unsupported extension"),
            ConnectionError::InsufficientMemory => write!(f, "Insufficient memory"),
            ConnectionError::MaximumRequestLengthExceeded => write!(f, "Maximum request length exceeded"),
            ConnectionError::DisplayParsingError => write!(f, "Display parsing error"),
            ConnectionError::InvalidScreen => write!(f, "Invalid screen"),
            ConnectionError::FDPassingFailed => write!(f, "FD passing failed"),
            ConnectionError::ParseError => write!(f, "Parsing error"),
        }
    }
}

impl From<ParseError> for ConnectionError {
    fn from(err: ParseError) -> Self {
        match err {
            ParseError::ParseError => ConnectionError::ParseError
        }
    }
}

impl From<std::num::TryFromIntError> for ConnectionError {
    fn from(err: std::num::TryFromIntError) -> Self {
        Self::from(ParseError::from(err))
    }
}

#[derive(Debug, Clone)]
pub enum ConnectionErrorOrX11Error {
    ConnectionError(ConnectionError),
    X11Error(GenericError)
}

impl Error for ConnectionErrorOrX11Error {
}

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
