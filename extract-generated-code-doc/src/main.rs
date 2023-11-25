#![deny(
    rust_2018_idioms,
    trivial_numeric_casts,
    unsafe_code,
    unreachable_pub,
    unused_import_braces,
    unused_must_use,
    unused_qualifications
)]
#![forbid(unsafe_code)]

use std::io::Error as IoError;
use std::path::Path;

pub mod doc;

/// Representation of a file split on empty lines
#[derive(Debug)]
pub struct Sections(Vec<String>);

impl Sections {
    fn new_from_empty_lines(input: &str) -> Self {
        let mut vec = Vec::new();
        let mut previous = None;
        for entry in input.split("\n\n").map(String::from) {
            if let Some(prev) = previous {
                // If the new entry starts with indentation, it belongs to the previous section
                if Some(&b' ') == entry.as_bytes().first() {
                    previous = Some(format!("{}\n\n{}", prev, entry));
                } else {
                    vec.push(prev);
                    previous = Some(entry);
                }
            } else {
                previous = Some(entry);
            }
        }
        vec.extend(previous);
        Self(vec)
    }

    fn new_from_trait(input: &str) -> Self {
        let mut result = input
            .split(" }")
            .map(|s| format!("    {} }}", s.trim_start()))
            .collect::<Vec<_>>();
        if let Some(first) = result.first_mut() {
            if let Some((_, rest)) =
                first.split_once("pub trait ConnectionExt: RequestConnection {\n")
            {
                *first = rest.to_string();
            }
        }
        Self(result)
    }

    fn get_range_by_index(&self, range: std::ops::RangeTo<usize>) -> String {
        self.0[range].join("\n\n")
    }

    fn get_by_needle(&self, needle: &str) -> &str {
        self.0
            .iter()
            .find(|text| text.contains(needle))
            .unwrap_or_else(|| panic!("Could not find text by needle '{needle}'"))
    }
}

fn load_sections(path: &Path) -> Result<Sections, IoError> {
    Ok(Sections::new_from_empty_lines(&std::fs::read_to_string(
        path,
    )?))
}

fn main2() -> Result<u8, IoError> {
    let args: Vec<_> = std::env::args_os().collect();
    if args.len() != 4 {
        eprintln!("USAGE:");
        eprintln!(
            "    {} <OUTPUT_FILE> <PROTO_XPROTO_FILE> <X11RB_XPROTO_FILE>",
            args[0].to_string_lossy()
        );
        return Ok(1);
    }
    let output_file = Path::new(&args[1]);
    let proto_xproto = load_sections(Path::new(&args[2]))?;
    let x11rb_xproto = load_sections(Path::new(&args[3]))?;

    let output = doc::generate(&proto_xproto, &x11rb_xproto);
    std::fs::write(output_file, output)?;

    Ok(0)
}

fn main() -> Result<(), IoError> {
    std::process::exit(i32::from(main2()?));
}
