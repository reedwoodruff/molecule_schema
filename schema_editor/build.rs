use std::{env, fs, path::Path, process::Command};

use generate_schema_reactive::generate_concrete_schema_reactive;
fn main() {
    let schema_location = Path::new("../resources/recursive_schema.json");
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("recursive_schema.rs");

    let final_output = generate_concrete_schema_reactive(schema_location);
    fs::write(&dest_path, final_output).unwrap();
    // Format the generated file with rustfmt
    let status = Command::new("rustfmt")
        .arg(&dest_path)
        .status()
        .expect("Failed to run rustfmt");

    if !status.success() {
        panic!("rustfmt failed with status: {:?}", status);
    }

    println!("cargo::rerun-if-changed=../molecule_schema/generate_schema_reactive/src");
    println!("cargo::rerun-if-changed=../molecule_schema/base_types/src");
    println!("cargo::rerun-if-changed=../molecule_schema/base_types/src/post_generation");
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=../molecule_schema/resources/recursive_schema.json");
}
