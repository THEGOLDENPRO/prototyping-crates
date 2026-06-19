use std::{env::var_os, fs::File, io::Write, path::PathBuf};

fn main() {
    // just supporting the rp2040 atm, will expand support later
    let memory_x_contents = include_bytes!("memory/rp2040.x");

    let output_path = PathBuf::from(var_os("OUT_DIR").unwrap());

    let mut memory_file = File::create(output_path.join("memory.x"))
        .unwrap();

    memory_file.write_all(memory_x_contents).unwrap();

    println!("cargo:rustc-link-search={}", output_path.display());
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=memory.x");
}