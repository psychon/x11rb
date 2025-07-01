#![deny(
    rust_2018_idioms,
    trivial_numeric_casts,
    unsafe_code,
    unreachable_pub,
    unused,
    unused_qualifications
)]
#![forbid(unsafe_code)]
// This crate is not shipped to users and does not follow our MSRV
#![allow(clippy::incompatible_msrv)]

use std::path::{Path, PathBuf};
use std::process::ExitCode;

mod generator;

#[derive(Debug)]
enum Error {
    FileReadFailed {
        _path: PathBuf,
        _error: std::io::Error,
    },
    FileWriteFailed {
        _path: PathBuf,
        _error: std::io::Error,
    },
    DirOpenFailed {
        _path: PathBuf,
        _error: std::io::Error,
    },
    DirReadFailed {
        _path: PathBuf,
        _error: std::io::Error,
    },
    FileIsNotUtf8 {
        _path: PathBuf,
        _error: std::str::Utf8Error,
    },
    XmlParseFailed {
        _path: PathBuf,
        _error: roxmltree::Error,
    },
    XcbParseFailed {
        _path: PathBuf,
        _error: xcbgen::ParseError,
    },
    XcbResolveFailed {
        _error: xcbgen::ResolveError,
    },
}

fn list_xmls(dir_path: &Path) -> Result<Vec<PathBuf>, Error> {
    let mut files = Vec::new();
    let dir_reader = std::fs::read_dir(dir_path).map_err(|e| Error::DirOpenFailed {
        _path: dir_path.to_path_buf(),
        _error: e,
    })?;
    for entry in dir_reader {
        let entry = entry.map_err(|e| Error::DirReadFailed {
            _path: dir_path.to_path_buf(),
            _error: e,
        })?;
        let file_path = entry.path();
        if file_path.extension() == Some(std::ffi::OsStr::new("xml")) {
            files.push(file_path);
        }
    }
    files.sort();
    Ok(files)
}

fn load_namespace(path: &Path, parser: &mut xcbgen::Parser) -> Result<(), Error> {
    let file_bytes = std::fs::read(path).map_err(|e| Error::FileReadFailed {
        _path: path.to_path_buf(),
        _error: e,
    })?;
    let file_string = String::from_utf8(file_bytes).map_err(|e| Error::FileIsNotUtf8 {
        _path: path.to_path_buf(),
        _error: e.utf8_error(),
    })?;
    let xml_doc = roxmltree::Document::parse(&file_string).map_err(|e| Error::XmlParseFailed {
        _path: path.to_path_buf(),
        _error: e,
    })?;
    parser
        .parse_namespace(xml_doc.root().first_element_child().unwrap())
        .map_err(|e| Error::XcbParseFailed {
            _path: path.to_path_buf(),
            _error: e,
        })?;
    Ok(())
}

/// Writes `data` to `file_path` if the file does not exist or
/// its current contents are different. This avoids updating the timestamps
/// if the contents have not changed.
fn replace_file_if_different(file_path: &Path, data: &[u8]) -> Result<(), Error> {
    if file_path.exists() {
        let existing_data = std::fs::read(file_path).map_err(|e| Error::FileReadFailed {
            _path: file_path.to_path_buf(),
            _error: e,
        })?;
        if existing_data == data {
            return Ok(());
        }
    }

    std::fs::write(file_path, data).map_err(|e| Error::FileWriteFailed {
        _path: file_path.to_path_buf(),
        _error: e,
    })?;

    Ok(())
}

fn main() -> Result<ExitCode, Error> {
    let args: Vec<_> = std::env::args_os().collect();
    if args.len() != 5 {
        eprintln!("USAGE:");
        eprintln!(
            "    {} <INPUT_DIR> <PROTO_OUTPUT_DIR> <X11RB_OUTPUT_DIR> <ASYNC_OUTPUT_DIR>",
            args[0].to_string_lossy()
        );
        return Ok(ExitCode::FAILURE);
    }
    let input_dir_path = Path::new(&args[1]);
    let proto_output_dir_path = Path::new(&args[2]);
    let x11rb_output_dir_path = Path::new(&args[3]);
    let async_output_dir_path = Path::new(&args[4]);

    let xml_files = list_xmls(input_dir_path)?;
    let module = xcbgen::defs::Module::new();
    let mut parser = xcbgen::Parser::new(module.clone());
    for file_path in xml_files.iter() {
        println!("Loading {file_path:?}");
        load_namespace(file_path, &mut parser)?;
    }

    //eprintln!("{:#?}", module);
    println!("{} XMLs loaded", module.namespaces.borrow().len());

    xcbgen::resolve(&module).map_err(|e| Error::XcbResolveFailed { _error: e })?;
    println!("Resolved successfully");

    let generated = generator::generate(&module);
    for generated in generated.iter() {
        let mut proto_file_path = PathBuf::from(proto_output_dir_path);
        let mut x11rb_file_path = PathBuf::from(x11rb_output_dir_path);
        proto_file_path.push(&generated.file_name);
        x11rb_file_path.push(&generated.file_name);
        let async_file_path = async_output_dir_path.join(&generated.file_name);
        replace_file_if_different(&proto_file_path, generated.proto.as_bytes())?;
        replace_file_if_different(&x11rb_file_path, generated.x11rb.as_bytes())?;
        replace_file_if_different(&async_file_path, generated.async_.as_bytes())?;
    }
    println!("Code generated successfully");

    Ok(ExitCode::SUCCESS)
}
