// The build constants emitted by this script are NOT public API.

fn main() {
    // Get the current Rust version.
    let rust_version = Version::get();

    // If we in a version prior to Rust 1.61, we can't use the "target_has_atomic" feature.
    //
    // This feature was stabilized in Rust 1.60, but some nightlies will claim to be 1.60
    // but not have the feature. Rust 1.61 lets us safely target nightlies without this feature.
    //
    // We use a negative implementation of the cfg guard, rather than emitting, say, a
    // "x11rb_has_target_has_atomic" cfg guard. This is because some non-Cargo build systems
    // will not run the build script. In this case, it is best to assume that we are targeting
    // the latest stable version of Rust, where this feature is available. In this case, the absence
    // of this cfg guard will indicate the available feature.
    //
    // TODO(notgull): Once the MSRV is raised to 1.60 or higher, this entire build script can be deleted.
    if rust_version.major < 1 || (rust_version.major == 1 && rust_version.minor < 61) {
        println!("cargo:rustc-cfg=x11rb_no_target_has_atomic");
    }
}

struct Version {
    major: usize,
    minor: usize,
}

impl Version {
    fn get() -> Self {
        let rustc = std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());
        let output = std::process::Command::new(rustc)
            .args(["--version", "--verbose"])
            .output()
            .expect("failed to execute rustc");

        let stdout = std::str::from_utf8(&output.stdout).expect("rustc stdout is not utf8");

        // Find the line with the "release" tag
        let release = stdout
            .lines()
            .find_map(|line| line.strip_prefix("release: "))
            .expect("rustc output does not contain release tag");

        // Strip off the extra channel info.
        let release = release.split('-').next().unwrap();

        // Split into semver components.
        let mut release = release.splitn(3, '.');
        let major = release.next().unwrap().parse().unwrap();
        let minor = release.next().unwrap().parse().unwrap();

        Version { major, minor }
    }
}
