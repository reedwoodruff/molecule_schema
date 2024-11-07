use std::{env, fs, path::Path, process::Command};

// use first_generation_compiler::generate_concrete_schema_reactive;
use first_generation_compiler::generate_crate;
fn main() {
    generate_crate(
        "/home/reed/development/molecule_schema/resources/recursive_schema.json",
        "/home/reed/development/molecule_schema",
        Some("/home/reed/development/molecule_schema/resources/initial_schema_editor_data.json"),
        Some("second_generation_compiler_generated_toolkit"),
    );
}
