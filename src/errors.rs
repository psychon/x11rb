//! This module contains the current mess that is error handling.

use std::error::Error;

use crate::x11_utils::GenericError;

#[derive(Debug, Copy, Clone)]
pub enum ParseError {
    ParseError
}

impl Error for ParseError {
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "FIXME")
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

#[derive(Debug, Copy, Clone)]
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
        write!(f, "FIXME")
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

#[derive(Debug)]
pub enum ConnectionErrorOrX11Error {
    ConnectionError(ConnectionError),
    X11Error(GenericError)
}

impl Error for ConnectionErrorOrX11Error {
}

impl std::fmt::Display for ConnectionErrorOrX11Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "FIXME")
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
