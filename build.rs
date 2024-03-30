use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=i2pd/libi2pd/api.cpp");
    // println!("cargo:rerun-if-changed=i2pd/libi2pd/api.h");

    let boost_include_path = PathBuf::from("/opt/homebrew/Cellar/boost/1.84.0_1/include");
    let zlib_inclue_path = PathBuf::from("/opt/homebrew/opt/zlib/include");
    let openssl = pkg_config::probe_library("openssl").unwrap();
    let openssl_include_paths = openssl.include_paths.iter().map(PathBuf::as_path);
    let mut include_paths: Vec<&Path> =
        vec![boost_include_path.as_path(), zlib_inclue_path.as_path()];
    include_paths.extend(openssl_include_paths);
    let _zlib = pkg_config::probe_library("zlib").unwrap();

    println!("cargo:rustc-link-search=native=./i2pd/build/");
    println!("cargo:rustc-link-search=native=/opt/homebrew/Cellar/boost/1.84.0_1/lib/");
    println!("cargo:rustc-link-lib=static=i2pd");
    println!("cargo:rustc-link-lib=static=boost_program_options");
    println!("cargo:rustc-link-lib=static=boost_filesystem");

    cxx_build::bridge("src/lib.rs")
        .std("c++17")
        .includes(include_paths)
        .compile("i2pd-sys");
}
