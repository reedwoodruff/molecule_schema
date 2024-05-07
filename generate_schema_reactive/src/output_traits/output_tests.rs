use super::*;
#[test]
fn test() {
    let schema_location =
        std::path::Path::new("/home/reed/dev/molecule_schema/resources/schema.json");
    let raw_json_data = std::fs::read_to_string(schema_location.to_str().unwrap());
    let raw_json_data = raw_json_data.expect("schema json must be present");
    let constraint_schema_generated: ConstraintSchema<PrimitiveTypes, PrimitiveValues> =
        serde_json::from_str(&raw_json_data).expect("Schema formatted incorrectly");
}
