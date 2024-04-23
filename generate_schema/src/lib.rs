use proc_macro::TokenStream;

use quote::quote;

use base_types::constraint_schema::*;
use base_types::constraint_schema_item::ConstraintSchemaItem;
use base_types::primitives::*;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated, Result as SynResult, Token, Type,
};

use crate::utils::get_variant_name;

mod generate_operative_streams;
    mod generate_trait_impl_streams;
mod utils;

#[proc_macro]
pub fn generate_concrete_schema(input: TokenStream) -> TokenStream {
    // let graph_environment = syn::parse_macro_input!(input as syn::Expr); 

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
                fn #method_name(&self, 
                    env: &dyn base_types::traits::GraphEnvironment<Types=base_types::primitives::PrimitiveTypes, Values=base_types::primitives::PrimitiveValues, Schema = Schema>
                    ) -> std::borrow::Cow<#return_type>;
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

    let all_lib_op_names = constraint_schema_generated.operative_library.values().map(|el| {
        get_variant_name(&Box::new(el))
    }).collect::<Vec<_>>();



    quote! {
        use base_types::utils::IntoPrimitiveValue;
        use base_types::traits::GSO;
        use base_types::{traits as bt};
        use validator::Validate;
        use base_types::traits::Buildable;

        #(#trait_definition_streams)*
        // #(#template_streams)*
        #(#library_operative_streams)*
        #(#instance_streams)*

        #[derive(Debug, Clone)]
        pub enum Schema {
            #(#all_lib_op_names(bt::GSOWrapper<#all_lib_op_names, Schema>),)*
        }


        impl bt::GSO for Schema {
            type Schema = Self;
            fn get_constraint_schema_operative_tag(&self) -> std::rc::Rc<base_types::common::Tag> {
                match &self {
                #(Self::#all_lib_op_names(item) => item.get_constraint_schema_operative_tag(),)*
                _ => panic!(),
                }
            }
            fn get_id(&self) -> &base_types::common::Uid {
                match self {
                    #(Self::#all_lib_op_names(item) => item.get_id(),)*
                    _ => panic!(),
                }
            }
            fn get_constraint_schema_template_tag(&self) -> std::rc::Rc<base_types::common::Tag> {
                match self {
                    #(Self::#all_lib_op_names(item) => item.get_constraint_schema_template_tag(),)*
                    _ => panic!(),
                }
            }
            fn get_slots(&self) -> &HashMap<Uid, bt::ActiveSlot>{
                match self {
                    #(Self::#all_lib_op_names(item) => item.get_slots(),)*
                    _ => panic!(),
                }
            }
            fn get_parent_slots(&self) -> &Vec<bt::SlotRef>{
                match self {
                    #(Self::#all_lib_op_names(item) => item.get_parent_slots(),)*
                    _ => panic!(),
                }
            }
            fn add_parent_slot(&mut self, slot_ref: &bt::SlotRef) ->  &mut Self {
                match self {
                    #(Self::#all_lib_op_names(item) => {item.add_parent_slot(slot_ref); self},)*
                    _ => panic!(),
                }
            }
            fn remove_child_from_slot(&mut self, slot_ref: &bt::SlotRef) -> &mut Self{
                match self {
                    #(Self::#all_lib_op_names(item) => {item.remove_child_from_slot(slot_ref); self},)*
                    _ => panic!(),
                }
            }
            fn remove_parent(&mut self, parent_id: &base_types::common::Uid, slot_id: Option<&base_types::common::Uid>) -> &mut Self {
                match self {
                    #(Self::#all_lib_op_names(item) => {item.remove_parent(parent_id, slot_id);self},)*
                    _ => panic!(),
                }
            }
            fn set_history(&mut self, history: Option<bt::HistoryStack<Self>>) {
                match self {
                    #(Self::#all_lib_op_names(item) => item.set_history(history),)*
                    _ => panic!(),
                }
            }
            fn add_child_to_slot(&mut self, slot_ref: &bt::SlotRef) -> &mut Self {
                match self {
                    #(Self::#all_lib_op_names(item) => {item.add_child_to_slot(slot_ref); self},)*
                    _ => panic!(),
                }
            }
            
        }

            
    }
    .into()
}


