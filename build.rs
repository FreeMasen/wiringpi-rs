extern crate bindgen;

use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let bindings = bindgen::Builder::default()
                .header("wiringPi.h")
                .generate()
                .expect("Unable to generate bindgen");
    bindings.write_to_file(Path::new(&out_dir).join("bindings.rs"))
                        .expect("Unable to write bindings");
}