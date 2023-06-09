use std::env;
use std::path::{Path, PathBuf};
use glob::glob;

fn main() {
    let manifest_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let include_dir = Path::new(&manifest_dir).join("cld3");
    cxx_build::CFG.exported_header_dirs.push(&include_dir);

    let cxx_files = glob(format!("{}/*.cc", include_dir.to_string_lossy()).as_str())
        .unwrap()
        .filter_map(Result::ok)
        .collect::<Vec<PathBuf>>();

    cxx_build::bridge("src/lib.rs")
        .file("cld3.cc")
        .files(cxx_files)
        .flag_if_supported("-std=c++17")
        .flag("-Wno-unused-parameter")
        .flag("-Wno-implicit-fallthrough")
        .compile("cxx-cld3");

    println!("cargo:rustc-link-lib=m");
    println!("cargo:rustc-link-lib=protobuf");

    println!("cargo:rerun-if-changed=cld3/cld3.cc");
    println!("cargo:rerun-if-changed=cld3/cld3.h");
    println!("cargo:rerun-if-changed=src/lib.rs");
}
