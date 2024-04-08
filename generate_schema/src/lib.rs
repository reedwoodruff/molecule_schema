use proc_macro::TokenStream;

use quote::quote;

use base_types::constraint_schema::*;
use base_types::constraint_schema_item::ConstraintSchemaItem;
use base_types::primitives::*;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated, Result as SynResult, Token, Type,
};

mod generate_operative_streams;
    mod generate_trait_impl_streams;
mod utils;

#[proc_macro]
pub fn generate_concrete_schema(input: TokenStream) -> TokenStream {
    let graph_environment = syn::parse_macro_input!(input as syn::Expr); 

    constraint_schema::constraint_schema!();

    // The goal here is as follows:
    // 1. Map the constraint objects to individual structs which have:
    //      - The same structure as defined in the field constraints
    //      - A constructor function which ensures that all constraints are met (edge and field)
    //      - Some reference to the internal structure of the template (maybe just reference to the
    //      constraint_schema id )
    //      - Helper methods for adding and removing edges (but not mandatory ones)
    //  2. Create an enum with a variant for each struct

    let trait_definition_streams = constraint_schema_generated.traits.values().map(| trait_def| {
        let trait_name = syn::Ident::new(&trait_def.tag.name, proc_macro2::Span::call_site());
        let fn_streams = trait_def.methods.values().map(|method_def| {
            let method_name = syn::Ident::new(&method_def.tag.name, proc_macro2::Span::call_site());
            let return_type = utils::get_primitive_type(&method_def.return_type);    
            quote! {
                fn #method_name(&self, env: &dyn base_types::traits::GraphEnvironment<base_types::primitives::PrimitiveTypes, base_types::primitives::PrimitiveValues>) -> std::borrow::Cow<#return_type>;
            }
        });
        quote! {
            pub trait #trait_name {
                #(#fn_streams)*
            }
        }
    });

    let reference_constraint_schema: ConstraintSchema<PrimitiveTypes, PrimitiveValues> = constraint_schema_generated.clone();


    // let template_streams = constraint_schema_generated.template_library.values().map(|el| {
    //     generate_operative_streams::generate_operative_streams(Box::new(el), &reference_constraint_schema)
    // }).collect::<Vec<_>>();
    let library_operative_streams = constraint_schema_generated.operative_library.values().map(|el| {
        generate_operative_streams::generate_operative_streams(Box::new(el), &reference_constraint_schema)
    }).collect::<Vec<_>>();
    let instance_streams = constraint_schema_generated.instance_library.values().map(|el| {
        generate_operative_streams::generate_operative_streams(Box::new(el), &reference_constraint_schema)
    })
    .collect::<Vec<_>>();

    quote! {
        use base_types::traits::GSO;
        // Helper trait, private to your module
        trait IsGraphEnvironment {}

        // Implement IsMyTrait for all T that implement MyTrait
        impl<T> IsGraphEnvironment for T where T: base_types::traits::GraphEnvironment<base_types::primitives::PrimitiveTypes, base_types::primitives::PrimitiveValues> {}
        let _check: &dyn IsGraphEnvironment = &#graph_environment;

        // const SCHEMA_JSON: &str = #data;
        #(#trait_definition_streams)*
        // #(#template_streams)*
        #(#library_operative_streams)*
        #(#instance_streams)*

            
    }
    .into()
}


