#![allow(clippy::single_match_else)]
use rustc_tools_util::VersionInfo;

#[test]
fn check_that_clippy_lints_has_the_same_version_as_clippy() {
    let clippy_meta = cargo_metadata::MetadataCommand::new()
        .no_deps()
        .exec()
        .expect("could not obtain cargo metadata");
    std::env::set_current_dir(std::env::current_dir().unwrap().join("clippy_lints")).unwrap();
    let clippy_lints_meta = cargo_metadata::MetadataCommand::new()
        .no_deps()
        .exec()
        .expect("could not obtain cargo metadata");
    assert_eq!(clippy_lints_meta.packages[0].version, clippy_meta.packages[0].version);
    for package in &clippy_meta.packages[0].dependencies {
        if package.name == "clippy_lints" {
            assert!(package.req.matches(&clippy_lints_meta.packages[0].version));
            return;
        }
    }
}

#[test]
fn check_that_clippy_has_the_same_major_version_as_rustc() {
    let clippy_version = rustc_tools_util::get_version_info!();
    let clippy_major = clippy_version.major;
    let clippy_minor = clippy_version.minor;
    let clippy_patch = clippy_version.patch;

    // get the rustc version from cargo
    // this way the rust-toolchain file version is honored
    let rustc_version = String::from_utf8(
        std::process::Command::new("cargo")
            .arg("--version")
            .output()
            .expect("failed to run 'cargo --version'")
            .stdout,
    )
    .unwrap();
    // extract "1 50 0" from "cargo 1.50.0-nightly (a3c2627fb 2020-12-14)"
    let vsplit: Vec<&str> = rustc_version
        .split(' ')
        .nth(1)
        .unwrap()
        .split('-')
        .next()
        .unwrap()
        .split('.')
        .collect();
    match vsplit.as_slice() {
        [rustc_major, rustc_minor, _rustc_patch] => {
            // clippy 0.1.50 should correspond to rustc 1.50.0
            dbg!(&rustc_version);
            dbg!(&clippy_version);
            assert_eq!(clippy_major, 0); // this will probably stay the same for a long time
            assert_eq!(
                clippy_minor.to_string(),
                *rustc_major,
                "clippy minor version does not equal rustc major version"
            );
            assert_eq!(
                clippy_patch.to_string(),
                *rustc_minor,
                "clippy patch version does not equal rustc minor version"
            );
            // do not check rustc_patch because when a stable-patch-release is made (like 1.50.2),
            // we don't want our tests failing suddenly
        },
        _ => {
            dbg!(vsplit);
            panic!("Failed to parse rustc version");
        },
    };
}
