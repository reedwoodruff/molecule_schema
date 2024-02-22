use proc_macro::TokenStream;
use serde_types::{
    constraint_schema::ConstraintSchema,
    primitives::{PrimitiveTypes, PrimitiveValues},
};

#[proc_macro]
pub fn constraint_schema(input: TokenStream) -> proc_macro::TokenStream {
    let data = std::fs::read_to_string("constraint_schema/resources/schema.json");
    let data = data.expect("schema json must be present");
    let constraint_schema_generated: ConstraintSchema<PrimitiveTypes, PrimitiveValues> =
        serde_json::from_str::<ConstraintSchema<PrimitiveTypes, PrimitiveValues>>(&data)
            .expect("json should be formatted correctly");

    print!("{}", data);
    quote::quote! {
        const SCHEMA_JSON: &str = #data;
         let constraint_schema_generated: serde_types::constraint_schema::ConstraintSchema<serde_types::primitives::PrimitiveTypes, serde_types::primitives::PrimitiveValues> =
        serde_json::from_str(SCHEMA_JSON).unwrap();
    }.into()
}
