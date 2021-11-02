use std::path::PathBuf;
use bindgen::Builder;

fn main() {
    println!("cargo:rustc-link-lib=qrencode");
    println!("cargo:rerun-if-changed=src/wrapper.h");

    let bindings = Builder::default()
        .header("src/wrapper.h")
        .generate()
        .expect("Failed to generate bindings!");
    
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs")).expect("Failed to write bindings!");
}