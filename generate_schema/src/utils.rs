

use quote::quote;


use base_types::constraint_schema_item::ConstraintSchemaItem;
use base_types::primitives::*;

pub(crate) fn concat_unique_element(
    element: &Box<&dyn ConstraintSchemaItem<TTypes = PrimitiveTypes, TValues = PrimitiveValues>>,
) -> String {
    element.get_tag().name.clone()
}
pub(crate) fn get_variant_name(
    element: &Box<&dyn ConstraintSchemaItem<TTypes = PrimitiveTypes, TValues = PrimitiveValues>>,
) -> syn::Ident {
    syn::Ident::new(
        &concat_unique_element(element),
        proc_macro2::Span::call_site(),
    )
}
pub(crate) fn get_variant_builder_name(
    element: &Box<&dyn ConstraintSchemaItem<TTypes = PrimitiveTypes, TValues = PrimitiveValues>>,
) -> syn::Ident {
    syn::Ident::new(
        &(concat_unique_element(element) + "Builder"),
        proc_macro2::Span::call_site(),
    )
}
pub(crate) fn get_primitive_type(ty: &PrimitiveTypes) -> proc_macro2::TokenStream {
    match ty {
        PrimitiveTypes::String => quote! {String},
        PrimitiveTypes::Int => quote! {u32},
        PrimitiveTypes::Float => quote! {f32},
        PrimitiveTypes::Bool => quote! {bool},
        PrimitiveTypes::Char => quote! {char},
        PrimitiveTypes::Option(inner) => {
            let inner = get_primitive_type(inner);
            quote! {Option<#inner>}
        }
        PrimitiveTypes::List(inner) => {
            let inner = get_primitive_type(inner);
            quote! {Vec<#inner>}
        }
        _ => panic!("Not a PrimitiveType"),
    }
}
pub(crate) fn get_primitive_value(ty: &PrimitiveValues) -> proc_macro2::TokenStream {
    match ty {
        PrimitiveValues::Int(val) => quote! {#val},
        PrimitiveValues::Float(val) => quote! {#val},
        PrimitiveValues::String(val) => quote! {#val},
        PrimitiveValues::Bool(val) => quote! {#val},
        PrimitiveValues::Char(val) => quote! {#val},
        PrimitiveValues::Option(val) => {
            if let Some(present_value) = val.as_ref() {
                let inner = get_primitive_value(present_value);
                quote! {#inner}
            } else {
                quote! {None}
            }
        }
        PrimitiveValues::List(val) => {
            let inner = val.iter().map(get_primitive_value);
            quote! {vec![#(#inner)*]}
        }
    }
}
