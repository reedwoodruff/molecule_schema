use std::path::Path;

use base_types::{
    constraint_schema::ConstraintSchema,
    primitives::{PrimitiveTypes, PrimitiveValues},
};
#[test]
fn should_generate_enum() {
    // type_map!(String);
    // Types::String;
    // Values::String(String::from("hello"));
    let test2: ConstraintSchema<PrimitiveTypes, PrimitiveValues> =
        // constraint_schema::constraint_schema(
        //     "/home/reed/dev/molecule_schema/resources/schema.json",
        // );
        constraint_schema::constraint_schema!();
}
