extern crate pkg_config;

use std::env;
use std::fs::{create_dir, read_dir};
use std::io::Result;
use std::path::{Path, PathBuf};
use std::process::Command;

fn create_dir_if_not_exist(dir: &PathBuf) -> Result<()> {
    let result = create_dir(dir);
    if let Err(ref e) = result {
        if e.kind() == std::io::ErrorKind::AlreadyExists {
            return Ok(());
        }
    }
    result
}

#[cfg(not(feature = "vendor-xcb-proto"))]
fn get_paths() -> (String, String) {
    let pythondir = pkg_config::get_variable("xcb-proto", "pythondir").unwrap();
    let includedir = pkg_config::get_variable("xcb-proto", "xcbincludedir").unwrap();
    (pythondir, includedir)
}

#[cfg(feature = "vendor-xcb-proto")]
fn get_paths() -> (String, String) {
    let dir = "xcbproto-1.13-6-ge79f6b0/".to_string();
    let pythondir = dir.clone();
    let includedir = dir + "src";
    (pythondir, includedir)
}

// Returns a list of files in `dir` whose name ends with `end`.
fn list_files_with_ending(dir: impl AsRef<Path>, end: &str) -> Vec<PathBuf> {
    read_dir(dir.as_ref())
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.to_string_lossy().ends_with(end))
        .collect()
}

fn main() {
    println!("cargo:rerun-if-changed=rs_code_generator.py");
    for py_file in list_files_with_ending("code_generator_helpers", ".py") {
        println!("cargo:rerun-if-changed={}", py_file.to_str().unwrap());
    }
    for py_file in list_files_with_ending("xcbproto-1.13-6-ge79f6b0/xcbgen", ".py") {
        println!("cargo:rerun-if-changed={}", py_file.to_str().unwrap());
    }
    for xml_file in list_files_with_ending("xcbproto-1.13-6-ge79f6b0/src", ".xml") {
        println!("cargo:rerun-if-changed={}", xml_file.to_str().unwrap());
    }

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_path = out_path.join("generated");
    create_dir_if_not_exist(&out_path).unwrap();
    let out_path = out_path.to_str().unwrap();
    let (pythondir, includedir) = get_paths();
    let status = Command::new("python")
        .args(&[
            "rs_code_generator.py",
            "-p",
            &pythondir,
            "-i",
            &includedir,
            "-o",
            out_path,
            "mod",
        ])
        .status()
        .unwrap();
    assert!(status.success());
}
