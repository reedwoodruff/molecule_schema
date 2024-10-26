use base_types::common::Uid;
use base_types::constraint_schema::{ConstraintSchema, LibraryOperative, OperativeSlot};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use base_types::constraint_schema_item::ConstraintSchemaItem;
use base_types::primitives::*;

// pub(crate) fn concat_unique_element(
//     element: &Box<&dyn ConstraintSchemaItem<TTypes = PrimitiveTypes, TValues = PrimitiveValues>>,
// ) -> String {
//     element.get_tag().name.clone()
// }
pub(crate) fn get_operative_variant_name(operative_name: &str) -> syn::Ident {
    syn::Ident::new(operative_name, proc_macro2::Span::call_site())
}
// pub(crate) fn get_variant_builder_name(
//     element: &Box<&dyn ConstraintSchemaItem<TTypes = PrimitiveTypes, TValues = PrimitiveValues>>,
// ) -> syn::Ident {
//     syn::Ident::new(
//         &(concat_unique_element(element) + "Builder"),
//         proc_macro2::Span::call_site(),
//     )
// }
// pub(crate) fn get_operative_wrapped_name(operative_name: &str) -> syn::Ident {
//     syn::Ident::new(
//         &format!(
//             "base_types::post_generation::reactive::RGSOWrapper<{}, Schema>",
//             get_operative_variant_name(operative_name).to_string()
//         ),
//         proc_macro2::Span::call_site(),
//     )
// }
pub(crate) fn get_operative_wrapped_name(operative_name: &str) -> TokenStream {
    let op_name = get_operative_variant_name(operative_name);

    quote! {RGSOConcrete<#op_name, Schema>}
}

pub(crate) fn get_template_get_field_fn_name(field_name: &str) -> syn::Ident {
    syn::Ident::new(
        &format!("get_{}_field", field_name.to_lowercase()),
        proc_macro2::Span::call_site(),
    )
}
pub(crate) fn get_template_get_field_trait_name(template_name: &str) -> syn::Ident {
    syn::Ident::new(
        &format!("Get{}Fields", template_name),
        proc_macro2::Span::call_site(),
    )
}
pub(crate) fn get_template_get_slot_fn_name(slot_name: &str) -> syn::Ident {
    syn::Ident::new(
        &format!("get_{}_slot", slot_name.to_lowercase()),
        proc_macro2::Span::call_site(),
    )
}
pub(crate) fn get_template_get_slot_fn_name_id_only(field_name: &str) -> syn::Ident {
    syn::Ident::new(
        &format!("get_{}_slot_ids", field_name.to_lowercase()),
        proc_macro2::Span::call_site(),
    )
}
pub(crate) fn get_template_get_slots_trait_name(template_name: &str) -> syn::Ident {
    syn::Ident::new(
        &format!("Get{}Slots", template_name),
        proc_macro2::Span::call_site(),
    )
}
pub(crate) fn get_template_slot_enum_name(
    constraint_schema: &ConstraintSchema<PrimitiveTypes, PrimitiveValues>,
    slot: &OperativeSlot,
) -> TokenStream {
    match &slot.operative_descriptor {
        base_types::constraint_schema::OperativeVariants::LibraryOperative(op_id) => {
            get_operative_subclass_enum_name(constraint_schema, op_id)
        }
        base_types::constraint_schema::OperativeVariants::TraitOperative(trait_op) => {
            get_slot_trait_enum_name(constraint_schema, &trait_op.trait_ids).into_token_stream()
        }
    }
}
pub(crate) fn get_operative_subclass_enum_name(
    constraint_schema: &ConstraintSchema<PrimitiveTypes, PrimitiveValues>,
    operative_id: &Uid,
) -> TokenStream {
    let operative = constraint_schema
        .operative_library
        .get(operative_id)
        .unwrap();
    let name = &operative.tag.name;
    if get_all_subclasses(constraint_schema, operative_id).len() <= 1 {
        get_operative_wrapped_name(name)
    } else {
        syn::Ident::new(
            &format!("{}Subclasses", name),
            proc_macro2::Span::call_site(),
        )
        .into_token_stream()
    }
}
pub(crate) fn get_all_slots_enum_name(template_name: &str, slot_name: &str) -> syn::Ident {
    syn::Ident::new(
        &format!("{}{}", slot_name, template_name,),
        proc_macro2::Span::call_site(),
    )
}
pub(crate) fn get_all_subclasses(
    constraint_schema: &ConstraintSchema<PrimitiveTypes, PrimitiveValues>,
    operative_id: &Uid,
) -> Vec<LibraryOperative<PrimitiveTypes, PrimitiveValues>> {
    constraint_schema
        .operative_library
        .values()
        .filter_map(|op| {
            if op.check_ancestry(constraint_schema, operative_id) {
                Some(op)
            } else {
                None
            }
        })
        .cloned()
        .collect()
}
pub(crate) fn get_all_superclasses(
    constraint_schema: &ConstraintSchema<PrimitiveTypes, PrimitiveValues>,
    operative_id: &Uid,
) -> Vec<LibraryOperative<PrimitiveTypes, PrimitiveValues>> {
    let root = constraint_schema
        .operative_library
        .get(operative_id)
        .unwrap();
    let mut next_to_check = root.get_parent_operative_id();
    let mut all_supers = vec![];
    while let Some(next_id) = next_to_check {
        let super_el = constraint_schema.operative_library.get(next_id).unwrap();
        all_supers.push(super_el.clone());
        next_to_check = super_el.get_parent_operative_id();
    }
    all_supers
}
pub fn get_slot_trait_enum_name(
    constraint_schema: &ConstraintSchema<PrimitiveTypes, PrimitiveValues>,
    trait_operative: &[Uid],
) -> syn::Ident {
    let mut names = Vec::from(trait_operative);
    names.sort();
    let names = names
        .iter()
        .map(|trait_id| {
            constraint_schema
                .traits
                .get(trait_id)
                .unwrap()
                .tag
                .name
                .clone()
        })
        .collect::<Vec<_>>()
        .join("");

    syn::Ident::new(&format!("{}Traits", names), proc_macro2::Span::call_site())
}

pub fn get_all_operatives_which_implement_trait_set(
    schema: &ConstraintSchema<PrimitiveTypes, PrimitiveValues>,
    trait_ids: &[Uid],
) -> Vec<LibraryOperative<PrimitiveTypes, PrimitiveValues>> {
    let mut return_val = schema
        .operative_library
        .values()
        .filter_map(|op| {
            let trait_digest = op.get_trait_impl_digest(schema).trait_impls;
            if trait_ids
                .iter()
                .all(|required_trait_id| trait_digest.contains_key(required_trait_id))
            {
                Some(op)
            } else {
                None
            }
        })
        .cloned()
        .collect::<Vec<_>>();
    return_val.sort_by(|a, b| a.tag.id.cmp(&b.tag.id));
    return_val
}

pub(crate) fn get_primitive_type(ty: &PrimitiveTypes) -> proc_macro2::TokenStream {
    match ty {
        PrimitiveTypes::String => quote! {String},
        PrimitiveTypes::Int => quote! {u32},
        // PrimitiveTypes::Float => quote! {f32},
        PrimitiveTypes::Bool => quote! {bool},
        // PrimitiveTypes::Char => quote! {char},
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
        // PrimitiveValues::Float(val) => quote! {#val},
        PrimitiveValues::String(val) => quote! {#val},
        PrimitiveValues::Bool(val) => quote! {#val},
        // PrimitiveValues::Char(val) => quote! {#val},
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
pub(crate) fn get_primitive_value_enum_variant_name(
    ty: &PrimitiveTypes,
) -> proc_macro2::TokenStream {
    match ty {
        PrimitiveTypes::Int => quote! {Int},
        // PrimitiveTypes::Float => quote! {#val},
        PrimitiveTypes::String => quote! {String},
        PrimitiveTypes::Bool => quote! {Bool},
        // PrimitiveTypes::Char => quote! {#val},
        _ => quote! {todo!()}, // PrimitiveTypes::Option => {
                               //     quote! {Option}
                               // }
                               // PrimitiveTypes::List => {
                               //     quote! {List}
                               // }
    }
}

pub(crate) fn capitalize_first_letter(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}
