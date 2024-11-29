use first_generation_compiler::generate_crate;
fn main() {
    generate_crate(
        "/home/reed/development/molecule_schema/resources/recursive_schema.json",
        "/home/reed/development/molecule_schema",
        Some("/home/reed/development/molecule_schema/resources/initial_schema_editor_data.json"),
        // None,
        Some("neo4j_pipeline_generated_toolkit"),
    );
}
