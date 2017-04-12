extern crate bindgen;
extern crate glob;

use glob::glob;

use std::env;
use std::path::{Path, PathBuf};

fn generate_bindings(qh_path: PathBuf) {
    let out_dir = env::var("OUT_DIR").unwrap();
    let _ = bindgen::builder()
        .no_unstable_rust()
        .header(qh_path.into_os_string().into_string().unwrap())
        .generate().unwrap()
        .write_to_file(Path::new(&out_dir).join("quantum.rs"));
}

fn main() {
    println!("cargo:rustc-link-lib=quantum");
    println!("cargo:rustc-link-lib=gomp");

    let mut patterns = Vec::new();
    patterns.push("/usr/include/**/quantum.h".to_string());
    patterns.push("/usr/local/include/**/quantum.h".to_string());

    match env::var("LIBQUANTUM_INCLUDE") {
        Ok(path) => patterns.push(path), 
        Err(_) => (),
    }

    for pattern in patterns.iter() {
        for entry in glob(pattern).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => { generate_bindings(path); return; },
                Err(e) => println!("{:?}", e),
            }
        }
    }
    panic!("Failed to find dependency 'quantum.h'!");
}
