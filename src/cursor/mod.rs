//! Utility functions for working with X11 cursors

mod parse_cursor;
mod find_cursor;

use std::fs::File;

pub fn foo() -> *const () {
    return parse_cursor::parse_cursor::<File> as _;
}
pub fn bar() -> *const() {
    return find_cursor::find_cursor as _;
}
