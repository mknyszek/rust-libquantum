extern crate bindgen;

use std::env;
use std::path::Path;

fn main() {
    println!("cargo:rustc-link-lib=quantum");
    println!("cargo:rustc-link-lib=gomp");
  
    let out_dir = env::var("OUT_DIR").unwrap();
    let _ = bindgen::builder()
        .no_unstable_rust()
        .header("/usr/include/quantum.h")
        .generate().unwrap()
        .write_to_file(Path::new(&out_dir).join("quantum.rs"));
}
