use base_types::{
    constraint_schema::ConstraintSchema,
    primitives::{PrimitiveTypes, PrimitiveValues},
};
use proc_macro::TokenStream;
use syn::parse_macro_input;

// #[macro_use]
// extern crate lazy_static;

#[proc_macro]
pub fn constraint_schema(input: TokenStream) -> proc_macro::TokenStream {
    // let static_str = parse_macro_input!(input as syn::LitStr);
    let data = std::fs::read_to_string("/home/reed/dev/molecule_schema/resources/schema.json");
    let data = data.expect("schema json must be present");
    let _constraint_schema_generated: ConstraintSchema<PrimitiveTypes, PrimitiveValues> =
        serde_json::from_str::<ConstraintSchema<PrimitiveTypes, PrimitiveValues>>(&data)
            .expect("json should be formatted correctly");

    print!("{}", data);
    quote::quote! {
        // const SCHEMA_JSON: &str = #data;


        // lazy_static!{
        // pub static ref constraint_schema_generated: base_types::constraint_schema::ConstraintSchema<base_types::primitives::PrimitiveTypes, base_types::primitives::PrimitiveValues> =
        // serde_json::from_str(SCHEMA_JSON).unwrap();
        // }
        serde_json::from_str(#data).unwrap()
    }
    .into()
}
