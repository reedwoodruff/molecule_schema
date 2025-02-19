use first_generation_compiler::generate_crate;
// use generate_schema_reactive::inner_generate_crate;
fn main() {
    // let schema_location = Path::new("../resources/recursive_schema.json");
    // let out_dir = env::var_os("OUT_DIR").unwrap();
    // let dest_path = Path::new(&out_dir).join("recursive_schema.rs");

    // let final_output = generate_concrete_schema_reactive(schema_location);
    // fs::write(&dest_path, final_output).unwrap();
    // // Format the generated file with rustfmt
    // let status = Command::new("rustfmt")
    //     .arg(&dest_path)
    //     .status()
    //     .expect("Failed to run rustfmt");

    // if !status.success() {
    //     panic!("rustfmt failed with status: {:?}", status);
    // }
    generate_crate(
        "/home/reed/development/molecule_schema/resources/recursive_schema.json",
        "/home/reed/development/molecule_schema",
        Some("/home/reed/development/molecule_schema/resources/initial_schema_editor_data.json"),
        // None,
        Some("schema_editor_generated_toolkit"),
    );
}
