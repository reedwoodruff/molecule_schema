use proc_macro::TokenStream;

use quote::quote;

use base_types::constraint_schema::*;
use base_types::constraint_schema_item::ConstraintSchemaItem;
use base_types::primitives::*;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Result as SynResult, Token, Type,
};

use crate::utils::{get_primitive_type, get_variant_builder_name, get_variant_name};

pub(crate) fn generate_operative_streams(
    instantiable: Box<
        &dyn ConstraintSchemaItem<TTypes = PrimitiveTypes, TValues = PrimitiveValues>,
    >,
    constraint_schema: &ConstraintSchema<PrimitiveTypes, PrimitiveValues>,
) -> proc_macro2::TokenStream {
    let mut field_names = Vec::<syn::Ident>::new();
    let mut field_names_setters = Vec::<syn::Ident>::new();
    let mut field_values = Vec::<proc_macro2::TokenStream>::new();
    let mut initial_values = Vec::<proc_macro2::TokenStream>::new();
    let struct_name = get_variant_name(&instantiable);
    let struct_builder_name = get_variant_builder_name(&instantiable);
    let item_trait_stream = crate::generate_trait_impl_streams::generate_trait_impl_streams(
        &instantiable,
        constraint_schema,
    );

    let reference_template_id = instantiable.get_template_id();
    let (constraint_schema_tag_name, constraint_schema_tag_id) = (
        instantiable.get_tag().name.clone(),
        instantiable.get_tag().id,
    );
    let reference_template = constraint_schema
        .clone()
        .template_library
        .get(reference_template_id)
        .cloned()
        .expect("instantiable must be based on a constraint object");

    let field_digest = instantiable
        .get_locked_fields_digest(constraint_schema)
        .unwrap();
    let unfulfilled_fields = field_digest.get_unfulfilled_fields();
    let unfulfilled_field_names = unfulfilled_fields
        .iter()
        .map(|field| syn::Ident::new(&*field.tag.name, proc_macro2::Span::call_site()))
        .collect::<Vec<_>>();
    let unfulfilled_field_value_types = unfulfilled_fields
        .iter()
        .map(|field| get_primitive_type(&field.value_type))
        .collect::<Vec<_>>();

    quote! {
        pub struct #struct_name {
            #(#unfulfilled_field_names: #unfulfilled_field_value_types,)*
        }
        #[derive(validator::Validate, Default)]
        pub struct #struct_builder_name {
            #(#[validate(required)] #unfulfilled_field_names: Option<#unfulfilled_field_value_types>,)*

        }
        impl base_types::traits::Finalizable<#struct_name> for #struct_builder_name {
            fn finalize(&self) -> Result<#struct_name, validator::ValidationErrors> {
                <Self as validator::Validate>::validate(self)?;
                Ok(#struct_name {
                    #(#unfulfilled_field_names: self.#unfulfilled_field_names.as_ref().unwrap().clone(),)*
                })
            }
        }
    }
}
