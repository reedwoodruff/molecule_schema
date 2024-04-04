use crate::{
    constraint_schema::ConstraintSchema,
    primitives::{PrimitiveTypes, PrimitiveValues},
};

#[cfg(feature = "serde")]
pub fn print_schema(schema: &ConstraintSchema<PrimitiveTypes, PrimitiveValues>) {
    let _path = std::path::Path::new("../../../constraint_schema/resources/schema.json");

    // let converted: ConstraintSchema<PrimitiveTypes, PrimitiveValues> = schema.clone().into();
    let json = serde_json::to_string_pretty(&schema).unwrap();
    // std::fs::write(path, json).expect("Unable to write file");
    println!("{}", json);
}

pub fn map_to_reactive_types(_schema: ConstraintSchema<PrimitiveTypes, PrimitiveValues>) {
    // let reactive_
}
