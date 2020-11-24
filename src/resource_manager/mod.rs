//! X11 resource manager library.
//!
//! The code in this module is only available when the `resource_manager` feature of the library is
//! enabled.

#![allow(missing_docs)] // FIXME remove this

use crate::connection::Connection;
use crate::errors::ReplyError;

mod parser;
mod matcher;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Binding {
    Tight,
    Loose,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Component {
    Normal(String), // Actually just a-z, A-Z, 0-9 and _ or - is allowed
    Wildcard,
}

#[derive(Debug, Clone, PartialEq)]
struct Entry {
    components: Vec<(Binding, Component)>,
    value: Vec<u8>,
}

/// A resource database.
#[derive(Debug, Default, Clone)]
pub struct Database {
    entries: Vec<Entry>,
}

impl Database {
    pub fn new_from_default(conn: impl Connection) -> Result<Self, ReplyError> {
        let _ = conn;
        todo!()
    }

    pub fn new_from_resource_manager_property(conn: impl Connection) -> Result<Self, ReplyError> {
        let _ = conn;
        todo!()
    }

    pub fn new_from_data(data: &[u8]) -> Self {
        let _ = data;
        todo!()
    }

    pub fn new_from_file(todo: ()) -> Self {
        // I think it would be better to replace this with "from reader"? Or remove it completely?
        let _ = todo;
        todo!()
    }

    pub fn to_string(&self) -> String {
        let _ = self;
        todo!()
    }

    pub fn combine(&self, target: &mut Self, overwrite: bool) {
        let _ = (target, overwrite);
        todo!()
    }

    pub fn put_resource(&mut self, resource: &str, value: &[u8]) {
        let _ = (resource, value);
        todo!()
    }

    pub fn put_resource_line(&mut self, line: &[u8]) {
        let _ = line;
        todo!()
    }

    pub fn get_bytes(&self, resource_name: &str, resource_class: &str) -> &[u8] {
        let _ = (resource_name, resource_class);
        todo!()
    }

    pub fn get_string(&self, resource_name: &str, resource_class: &str) -> &str {
        let _ = (resource_name, resource_class);
        todo!()
    }

    pub fn get_bool(&self, resource_name: &str, resource_class: &str) -> bool {
        let _ = (resource_name, resource_class);
        todo!()
    }

    pub fn get_value<T>(&self, resource_name: &str, resource_class: &str) -> Result<T, T::Err>
        where T: std::str::FromStr
    {
        let _ = (resource_name, resource_class);
        T::from_str(self.get_string(resource_name, resource_class))
    }
}

// API: [u8] or str?
// It seems that the value is "a bunch of bytes" (with one extra rule - no zero bytes, but
// everything else is possible via escape sequences). The resource itself is less than ascii.

// TODO: Add an API so that no file I/O is done
