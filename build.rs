extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
  /// Tell cargo to tell rustc to link quantum.h
  println!("cargo:rustc-link-lib=quantum");

  let bindings = bindgen::Builder::default()
  .header("header.h")
  .generate()
  .expect("Unable to generate bindings");

  // Write the bindings to the $OUT_DIR/bindings.rs file.
  let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
  bindings
    .write_to_file(out_path.join("quantum.rs"))
    .expect("Couldn't write bindings!");
}