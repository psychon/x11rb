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

use std::io::{Read as _, Write as _};
use std::path::{Path, PathBuf};

mod generator;

#[derive(Debug)]
enum Error {
    FileOpenFailed {
        path: PathBuf,
        error: std::io::Error,
    },
    FileReadFailed {
        path: PathBuf,
        error: std::io::Error,
    },
    FileWriteFailed {
        path: PathBuf,
        error: std::io::Error,
    },
    DirOpenFailed {
        path: PathBuf,
        error: std::io::Error,
    },
    DirReadFailed {
        path: PathBuf,
        error: std::io::Error,
    },
    FileIsNotUtf8 {
        path: PathBuf,
        error: std::str::Utf8Error,
    },
    XmlParseFailed {
        path: PathBuf,
        error: roxmltree::Error,
    },
    XcbParseFailed {
        path: PathBuf,
        error: xcbgen::ParseError,
    },
    XcbResolveFailed {
        error: xcbgen::ResolveError,
    },
}

fn read_file(path: &Path) -> Result<Vec<u8>, Error> {
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .write(false)
        .open(path)
        .map_err(|e| Error::FileOpenFailed {
            path: path.to_path_buf(),
            error: e,
        })?;

    let mut buf = Vec::new();
    file.read_to_end(&mut buf)
        .map_err(|e| Error::FileReadFailed {
            path: path.to_path_buf(),
            error: e,
        })?;
    Ok(buf)
}

fn list_xmls(dir_path: &Path) -> Result<Vec<PathBuf>, Error> {
    let mut files = Vec::new();
    let dir_reader = std::fs::read_dir(dir_path).map_err(|e| Error::DirOpenFailed {
        path: dir_path.to_path_buf(),
        error: e,
    })?;
    for entry in dir_reader {
        let entry = entry.map_err(|e| Error::DirReadFailed {
            path: dir_path.to_path_buf(),
            error: e,
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
    let file_bytes = read_file(path)?;
    let file_string = String::from_utf8(file_bytes).map_err(|e| Error::FileIsNotUtf8 {
        path: path.to_path_buf(),
        error: e.utf8_error(),
    })?;
    let xml_doc = roxmltree::Document::parse(&file_string).map_err(|e| Error::XmlParseFailed {
        path: path.to_path_buf(),
        error: e,
    })?;
    parser
        .parse_namespace(xml_doc.root().first_element_child().unwrap())
        .map_err(|e| Error::XcbParseFailed {
            path: path.to_path_buf(),
            error: e,
        })?;
    Ok(())
}

/// Writes `data` to `file_path` if the file does not exist or
/// its current contents are different. This avoids updating the timestamps
/// if the contents have not changed.
fn replace_file_if_different(file_path: &Path, data: &[u8]) -> Result<(), Error> {
    if file_path.exists() {
        let existing_data = read_file(file_path)?;
        if existing_data == data {
            return Ok(());
        }
    }

    let mut file = std::fs::OpenOptions::new()
        .read(false)
        .write(true)
        .truncate(true)
        .create(true)
        .open(file_path)
        .map_err(|e| Error::FileOpenFailed {
            path: file_path.to_path_buf(),
            error: e,
        })?;

    file.write_all(data).map_err(|e| Error::FileWriteFailed {
        path: file_path.to_path_buf(),
        error: e,
    })?;
    file.flush().map_err(|e| Error::FileWriteFailed {
        path: file_path.to_path_buf(),
        error: e,
    })?;

    Ok(())
}

fn main2() -> Result<u8, Error> {
    let args_defs = clap::App::new("x11rb generator")
        .arg(
            clap::Arg::with_name("input")
                .short("i")
                .value_name("INPUT_DIR")
                .help("Input directory")
                .required(true),
        )
        .arg(
            clap::Arg::with_name("output")
                .short("o")
                .value_name("OUTPUT_DIR")
                .help("Output directory")
                .required(true),
        );

    let arg_matches = match args_defs.get_matches_safe() {
        Ok(arg_matches) => arg_matches,
        Err(e) => {
            eprintln!("{}", e);
            return Ok(1);
        }
    };

    let input_dir_path = Path::new(arg_matches.value_of_os("input").unwrap());
    let output_dir_path = Path::new(arg_matches.value_of_os("output").unwrap());

    let xml_files = list_xmls(input_dir_path)?;
    let module = xcbgen::defs::Module::new();
    let mut parser = xcbgen::Parser::new(module.clone());
    for file_path in xml_files.iter() {
        println!("Loading {:?}", file_path);
        load_namespace(file_path, &mut parser)?;
    }

    //eprintln!("{:#?}", module);
    println!("{} XMLs loaded", module.namespaces.borrow().len());

    xcbgen::resolve(&module).map_err(|e| Error::XcbResolveFailed { error: e })?;
    println!("Resolved successfully");

    let generated = generator::generate(&module);
    for (file_name, file_data) in generated.iter() {
        let mut file_path = PathBuf::from(output_dir_path);
        file_path.push(file_name);
        replace_file_if_different(&file_path, file_data.as_bytes())?;
    }
    println!("Code generated successfully");

    Ok(0)
}

fn main() {
    let exit_code = main2().unwrap();
    std::process::exit(i32::from(exit_code));
}
