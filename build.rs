use std::env;
use std::ffi::OsStr;
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

fn get_paths() -> (PathBuf, PathBuf) {
    let dir = Path::new("xcbproto-1.13-6-ge79f6b0");
    let pythondir = dir.to_path_buf();
    let includedir = dir.join("src");
    (pythondir, includedir)
}

// Returns a list of files in `dir` whose extension is `ext`.
fn list_files_with_extension(dir: impl AsRef<Path>, ext: impl AsRef<OsStr>) -> Vec<PathBuf> {
    read_dir(dir.as_ref())
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension() == Some(ext.as_ref()))
        .collect()
}

fn main() {
    let mut out_path = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    out_path.push("generated");
    create_dir_if_not_exist(&out_path).unwrap();
    let (pythondir, includedir) = get_paths();

    println!("cargo:rerun-if-changed=rs_code_generator.py");
    for py_file in list_files_with_extension("code_generator_helpers", "py") {
        println!("cargo:rerun-if-changed={}", py_file.to_str().unwrap());
    }
    for py_file in list_files_with_extension(pythondir.join("xcbgen"), "py") {
        println!("cargo:rerun-if-changed={}", py_file.to_str().unwrap());
    }
    for xml_file in list_files_with_extension(&includedir, "xml") {
        println!("cargo:rerun-if-changed={}", xml_file.to_str().unwrap());
    }

    let status = Command::new("python")
        .arg("rs_code_generator.py")
        .arg("-p")
        .arg(&pythondir)
        .arg("-i")
        .arg(&includedir)
        .arg("-o")
        .arg(&out_path)
        .arg("mod")
        .status()
        .unwrap();
    assert!(status.success());
}
