//! Utility functions for working with X11 cursors

mod parse_cursor;

pub(crate) use parse_cursor::parse_cursor;

use std::fs::File;

pub fn foo() -> *const () {
    return parse_cursor::<File> as _;
}
