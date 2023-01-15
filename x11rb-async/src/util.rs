// This code is dual licensed under MIT OR Apache 2.0.

//! Utilities.

use crate::errors::{ConnectionError, ReplyError, ReplyOrIdError};
use std::io;

pub(crate) trait AsIoError {
    fn as_io_error(&self) -> Option<&io::Error>;
    fn from_io_error(e: io::Error) -> Self;
}

impl AsIoError for io::Error {
    fn as_io_error(&self) -> Option<&io::Error> {
        Some(self)
    }

    fn from_io_error(e: io::Error) -> Self {
        e
    }
}

impl AsIoError for ConnectionError {
    fn as_io_error(&self) -> Option<&io::Error> {
        match self {
            Self::IoError(ref e) => Some(e),
            _ => None,
        }
    }

    fn from_io_error(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

impl AsIoError for ReplyError {
    fn as_io_error(&self) -> Option<&io::Error> {
        match self {
            Self::ConnectionError(ConnectionError::IoError(ref e)) => Some(e),
            _ => None,
        }
    }

    fn from_io_error(e: io::Error) -> Self {
        Self::ConnectionError(ConnectionError::IoError(e))
    }
}

impl AsIoError for ReplyOrIdError {
    fn as_io_error(&self) -> Option<&io::Error> {
        match self {
            Self::ConnectionError(ConnectionError::IoError(ref e)) => Some(e),
            _ => None,
        }
    }

    fn from_io_error(e: io::Error) -> Self {
        Self::ConnectionError(ConnectionError::IoError(e))
    }
}
