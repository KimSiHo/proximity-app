use std::env;
use std::path::PathBuf;

fn main() {
    let header = "protocol/include/deepstream_proximity_protocol.h";

    println!("cargo:rerun-if-changed={header}");

    let bindings = bindgen::Builder::default()
        .header(header)
        .generate()
        .expect("failed to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("protocol.rs"))
        .expect("failed to write bindings");
}
