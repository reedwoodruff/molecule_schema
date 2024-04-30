use base_types::{
    constraint_schema::ConstraintSchema,
    primitives::{PrimitiveTypes, PrimitiveValues},
};
use leptos::*;
use molecule_schema::components::app::App;

pub fn main() {
    let data = include_str!("/home/reed/dev/molecule_schema/resources/schema.json");

    let constraint_schema_generated: ConstraintSchema<PrimitiveTypes, PrimitiveValues> =
        serde_json::from_str::<ConstraintSchema<PrimitiveTypes, PrimitiveValues>>(&data)
            .expect("json should be formatted correctly");
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App schema=constraint_schema_generated/> });
}
