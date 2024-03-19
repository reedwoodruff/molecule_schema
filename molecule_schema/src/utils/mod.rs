use leptos::logging::log;
use serde_types::{
    constraint_schema::ConstraintSchema,
    primitives::{PrimitiveTypes, PrimitiveValues},
};

use self::reactive_types::RConstraintSchema;

pub mod operative_digest;
pub mod reactive_item;
pub mod reactive_types;
pub mod trait_impl_digest;

pub fn export_schema(schema: &RConstraintSchema<PrimitiveTypes, PrimitiveValues>) {
    let _path = std::path::Path::new("../../../constraint_schema/resources/schema.json");

    let converted: ConstraintSchema<PrimitiveTypes, PrimitiveValues> = schema.clone().into();
    let json = serde_json::to_string_pretty(&converted).unwrap();
    // std::fs::write(path, json).expect("Unable to write file");
    log! {"{}", json};
}

pub fn map_to_reactive_types(_schema: ConstraintSchema<PrimitiveTypes, PrimitiveValues>) {
    // let reactive_
}
