extern crate pkg_config;

use std::process::Command;
use std::path::PathBuf;
use std::env;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_path = out_path.join("bindings.rs");
    let out_path = out_path.to_str().unwrap();
    let pythondir = pkg_config::get_variable("xcb-proto", "pythondir").unwrap();
    let includedir = pkg_config::get_variable("xcb-proto", "xcbincludedir").unwrap();
    let status = Command::new("python")
        .args(&["rs_code_generator.py", "-p", &pythondir, "-i", &includedir, "-o", out_path])
        .status()
        .unwrap();
    assert!(status.success());
}
