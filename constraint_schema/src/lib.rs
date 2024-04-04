use base_types::{
    constraint_schema::ConstraintSchema,
    primitives::{PrimitiveTypes, PrimitiveValues},
};
use proc_macro::TokenStream;

#[proc_macro]
pub fn constraint_schema(_input: TokenStream) -> proc_macro::TokenStream {
    let data = std::fs::read_to_string("constraint_schema/resources/schema.json");
    let data = data.expect("schema json must be present");
    let _constraint_schema_generated: ConstraintSchema<PrimitiveTypes, PrimitiveValues> =
        serde_json::from_str::<ConstraintSchema<PrimitiveTypes, PrimitiveValues>>(&data)
            .expect("json should be formatted correctly");

    print!("{}", data);
    quote::quote! {
        const SCHEMA_JSON: &str = #data;
         let constraint_schema_generated: base_types::constraint_schema::ConstraintSchema<base_types::primitives::PrimitiveTypes, base_types::primitives::PrimitiveValues> =
        serde_json::from_str(SCHEMA_JSON).unwrap();
    }.into()
}
