extern crate pkg_config;

use std::process::Command;
use std::path::PathBuf;
use std::fs::create_dir;
use std::io::Result;
use std::env;

fn create_dir_if_not_exist(dir: &PathBuf) -> Result<()> {
    let result = create_dir(dir);
    if let Err(ref e) = result {
        if e.kind() == std::io::ErrorKind::AlreadyExists {
            return Ok(())
        }
    }
    result
}

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_path = out_path.join("generated");
    create_dir_if_not_exist(&out_path).unwrap();
    let out_path = out_path.to_str().unwrap();
    let pythondir = pkg_config::get_variable("xcb-proto", "pythondir").unwrap();
    let includedir = pkg_config::get_variable("xcb-proto", "xcbincludedir").unwrap();
    let status = Command::new("python")
        .args(&["rs_code_generator.py", "-p", &pythondir, "-i", &includedir, "-o", out_path, "mod"])
        .status()
        .unwrap();
    assert!(status.success());
}
