//! X11 resource manager library.
//!
//! The code in this module is only available when the `resource_manager` feature of the library is
//! enabled.

#![allow(missing_docs)] // FIXME remove this

use std::env::var_os;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use crate::connection::Connection;
use crate::errors::ReplyError;
use crate::protocol::xproto::{AtomEnum, ConnectionExt as _};

mod parser;
mod matcher;

/// Maximum nesting of #include directives, same value as Xlib uses
const MAX_INCLUSION_DEPTH: u8 = 100;

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
pub(crate) struct Entry {
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
        let cur_dir = Path::new(".");

        // 1. Try to load the RESOURCE_MANAGER property
        let mut entries = if let Some(db) = Self::new_from_resource_manager(conn)? {
            db.entries
        } else {
            let mut entries = Vec::new();
            if let Some(home) = var_os("HOME") {
                // 2. Otherwise, try to load $HOME/.Xresources
                let mut path = PathBuf::from(&home);
                path.push(".Xresources");
                let read_something = if let Ok(data) = std::fs::read(&path) {
                    parse_data_with_base_directory(&mut entries, &data, Path::new(&home), 0);
                    true
                } else {
                    false
                };
                let _ = path.pop();

                if !read_something {
                    // 3. Otherwise, try to load $HOME/.Xdefaults
                    path.push(".Xdefaults");
                    if let Ok(data) = std::fs::read(&path) {
                        parse_data_with_base_directory(&mut entries, &data, Path::new(&home), 0);
                    }
                    let _ = path.pop();
                }
            }
            entries
        };

        // 4. If XENVIRONMENT is specified, merge the database defined by that file
        if let Some(xenv) = var_os("XENVIRONMENT") {
            if let Ok(data) = std::fs::read(&xenv) {
                let base = Path::new(&xenv).parent().unwrap_or(cur_dir);
                parse_data_with_base_directory(&mut entries, &data, base, 0);
            }
        } else {
            let mut path = if let Some(home) = var_os("HOME") {
                PathBuf::from(home)
            } else {
                PathBuf::new()
            };
            let mut file = std::ffi::OsString::from(".Xdefaults-");
            file.push(gethostname::gethostname());
            path.push(file);
            if let Ok(data) = std::fs::read(&path) {
                let base = path.parent().unwrap_or(cur_dir);
                parse_data_with_base_directory(&mut entries, &data, base, 0);
            }
        }

        Ok(Self { entries })
    }

    pub fn new_from_resource_manager(conn: impl Connection) -> Result<Option<Self>, ReplyError> {
        let max_length = 100_000_000; // This is what Xlib does, so it must be correct (tm)
        let window = conn.setup().roots[0].root;
        let property = conn.
            get_property(false, window, AtomEnum::RESOURCE_MANAGER, AtomEnum::STRING, 0, max_length)?
            .reply()?;
        if property.format == 8 && !property.value.is_empty() {
            Ok(Some(Self::new_from_data(&property.value)))
        } else {
            Ok(None)
        }
    }

    pub fn new_from_data(data: &[u8]) -> Self {
        let mut entries = Vec::new();
        parse_data_with_base_directory(&mut entries, data, Path::new("."), 0);
        Self { entries }
    }

    pub fn new_from_data_with_base_directory(data: &[u8], base_path: impl AsRef<Path>) -> Self {
        Self::new_from_data_with_base_directory_impl(data, base_path.as_ref())
    }

    fn new_from_data_with_base_directory_impl(data: &[u8], base_path: &Path) -> Self {
        let mut entries = Vec::new();
        parse_data_with_base_directory(&mut entries, data, base_path, 0);
        Self { entries }
    }

    pub fn get_bytes(&self, resource_name: &str, resource_class: &str) -> Option<&[u8]> {
        matcher::match_entry(&self.entries, resource_name, resource_class)
    }

    pub fn get_string(&self, resource_name: &str, resource_class: &str) -> Option<&str> {
        std::str::from_utf8(self.get_bytes(resource_name, resource_class)?).ok()
    }

    pub fn get_bool(&self, resource_name: &str, resource_class: &str) -> Option<bool> {
        to_bool(self.get_string(resource_name, resource_class)?)
    }

    pub fn get_value<T>(&self, resource_name: &str, resource_class: &str) -> Result<Option<T>, T::Err>
        where T: FromStr
    {
        self.get_string(resource_name, resource_class).map(T::from_str).transpose()
    }
}

fn parse_data_with_base_directory(result: &mut Vec<Entry>, data: &[u8], base_path: &Path, depth: u8) {
    if depth > MAX_INCLUSION_DEPTH {
        return;
    }
    parser::parse_database(data, result, |path, entries| {
        // Construct the name of the file to include
        if let Ok(path) = std::str::from_utf8(path) {
            let mut path_buf = PathBuf::from(base_path);
            path_buf.push(path);

            // Read the file contents
            if let Ok(new_data) = std::fs::read(&path_buf) {
                // Parse the file contents with the new base path
                let new_base = path_buf.parent().unwrap_or(base_path);
                parse_data_with_base_directory(entries, &new_data, new_base, depth + 1);
            }
        }
    });
}

fn to_bool(data: &str) -> Option<bool> {
    if let Ok(num) = i64::from_str(data) {
        return Some(num != 0);
    }
    match data.to_lowercase().as_bytes() {
        b"true" => Some(true),
        b"on" => Some(true),
        b"yes" => Some(true),
        b"false" => Some(false),
        b"off" => Some(false),
        b"no" => Some(false),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use super::{Database, to_bool};

    #[test]
    fn test_bool_true() {
        let data = [
            "1",
            "10",
            "true",
            "TRUE",
            "on",
            "ON",
            "yes",
            "YES",
        ];
        for input in &data {
            assert_eq!(Some(true), to_bool(input));
        }
    }

    #[test]
    fn test_bool_false() {
        let data = [
            "0",
            "false",
            "FALSE",
            "off",
            "OFF",
            "no",
            "NO",
        ];
        for input in &data {
            assert_eq!(Some(false), to_bool(input));
        }
    }

    #[test]
    fn test_bool_none() {
        let data = ["", "abc"];
        for input in &data {
            assert_eq!(None, to_bool(input));
        }
    }

    #[test]
    fn test_parse_i32_fail() {
        let db = Database::new_from_data(b"a:");
        assert_eq!(db.get_string("a", "a"), Some(""));
        assert!(db.get_value::<i32>("a", "a").is_err());
    }

    #[test]
    fn test_parse_i32_success() {
        let data = [
            (&b"a: 0"[..], 0),
            (b"a: 1", 1),
            (b"a: -1", -1),
            (b"a: 100", 100),
        ];
        for (input, expected) in data.iter() {
            let db = Database::new_from_data(input);
            let result = db.get_value::<i32>("a", "a");
            assert_eq!(result.unwrap().unwrap(), *expected);
        }
    }
}
