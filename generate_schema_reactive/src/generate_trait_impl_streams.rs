use proc_macro2::Ident;
use quote::quote;

use base_types::constraint_schema::*;
use base_types::constraint_schema_item::ConstraintSchemaItem;
use base_types::primitives::*;

use crate::utils;

pub(crate) fn generate_trait_impl_streams(
    instantiable: &Box<
        &dyn ConstraintSchemaItem<TTypes = PrimitiveTypes, TValues = PrimitiveValues>,
    >,
    constraint_schema: &ConstraintSchema<PrimitiveTypes, PrimitiveValues>,
) -> proc_macro2::TokenStream {
    let instantiable_name = crate::get_operative_variant_name(&instantiable.get_tag().name);
    let trait_impl_digest = instantiable.get_trait_impl_digest(constraint_schema);

    let trait_impl_stream = trait_impl_digest
        .trait_impls
        .iter()
        .map(|(trait_id, trait_impl)| {
            let trait_def = &constraint_schema.traits[trait_id];
            let trait_name = syn::Ident::new(&trait_def.tag.name, proc_macro2::Span::call_site());
            let fn_streams = trait_def.methods.values().map(|method_def| {
                let method_name =
                    syn::Ident::new(&method_def.tag.name, proc_macro2::Span::call_site());
                let return_type = utils::get_primitive_type(&method_def.return_type);
                let method_impl = &trait_impl.trait_impl[&method_def.tag.id];
                let inner_method_stream = method_impl.iter().map(|method_impl_part| {
                match method_impl_part {
                    TraitMethodImplPath::Field(field_id) => {
                        // let field_name = &instantiable.get_locked_fields_digest(constraint_schema).unwrap().field_constraints[field_id].tag.name;
                        // let field_ident = Ident::new(field_name, proc_macro2::Span::call_site());
                        quote!{
                            // self.data.#field_ident
                            match self.fields.get(&#field_id).unwrap().get() {

                                base_types::primitives::PrimitiveValues::#return_type(val) => val,
                                _ => panic!()
                            }
                        }
                    },
                    TraitMethodImplPath::TraitMethod { trait_id: _, trait_method_id: _ } => todo!(),
                    TraitMethodImplPath::Constituent(_) => todo!(),
                }
            });

                quote! {
                fn #method_name(&self,) -> #return_type {
                        #(#inner_method_stream)*

                    }
                }
            });
            quote! {
                impl #trait_name for RGSOWrapper<#instantiable_name, Schema> {
                    #(#fn_streams)*
                }
            }
        });
    quote! {
        #(#trait_impl_stream)*
    }

    // let mut rolling_trait_impl_list = instantiable.get_local_trait_impls().clone();
    // let ancestor_trait_impls = instantiable
    //     .get_trait_impl_digest(&constraint_schema)
    //     .get_ancestors_trait_impls()
    //     .iter()
    //     .map(|(trait_id, related_impl)| (*trait_id, related_impl.trait_impl.clone()));
    // rolling_trait_impl_list.extend(ancestor_trait_impls);
    //
    // let trait_streams = rolling_trait_impl_list.iter().map(|(trait_id, method_impls)| {
    //         let trait_def = constraint_schema.traits.get(trait_id).expect("trait must exist");
    //         let trait_name = syn::Ident::new(&trait_def.tag.name, proc_macro2::Span::call_site());
    //
    //         let method_impl_streams = method_impls.iter().map(|(method_id, impl_path)| {
    //             let mut method_impl_stream = quote! {panic!("No terminating path for impl implementation")};
    //
    //
    //             println!("on trait: {}", trait_id );
    //             println!("expected method: {}", method_id );
    //             println!("trait_def: {:?}", trait_def);
    //             let method_def = trait_def.methods.values().find(|method| &method.tag.id == method_id).expect("method must exist");
    //             let method_name = syn::Ident::new(&method_def.tag.name, proc_macro2::Span::call_site());
    //             let method_return_type = crate::get_primitive_type(&method_def.return_type);
    //
    //             let mut prepend_call: proc_macro2::TokenStream = quote!{};
    //             let mut last_item_id = instantiable.get_tag().id;
    //             let mut last_item_type = instantiable.get_constraint_schema_instantiable_type();
    //
    //             for (path_index, path_item) in impl_path.iter().enumerate() {
    //                 match path_item {
    //                     base_types::constraint_schema::TraitMethodImplPath::Field  (field_id) => {
    //                         match last_item_type {
    //                             base_types::constraint_schema::ConstraintSchemaItemType::Template => {
    //                                 let template = constraint_schema.template_library.get(&last_item_id).expect("constraint object must exist");
    //                                 println!("expected: {}", field_id);
    //                                 let field_constraint = template.field_constraints.iter().find(|constraint| &constraint.tag.id == field_id).expect("field must exist");
    //                                 let field_name = syn::Ident::new(&field_constraint.tag.name, proc_macro2::Span::call_site());
    //                                 method_impl_stream = quote!{ std::borrow::Cow::Borrowed(&self.#field_name) };
    //                             }
    //                             base_types::constraint_schema::ConstraintSchemaItemType::Instance => {
    //                                 let instance_ref = constraint_schema.instance_library.get(&last_item_id).expect("instance must exist");
    //                                 println!("expected: {}", field_id);
    //                                 let fulfilled_constraint = instance_ref.data.iter().find(|field| &field.tag.id == field_id).expect("field must exist");
    //                                 let value = get_primitive_value(&fulfilled_constraint.value);
    //                                 method_impl_stream = quote! { std::borrow::Cow::Owned(#value) };
    //                             }
    //                             base_types::constraint_schema::ConstraintSchemaItemType::Operative => {
    //                                 let operative_ref = constraint_schema.operative_library.get(&last_item_id).expect("operative must exist");
    //                                 let fulfilled_constraint = operative_ref.locked_fields.iter().find(|field| &field.tag.id == field_id);
    //                                 if let Some(fulfilled_constraint) = fulfilled_constraint {
    //                                     let value = get_primitive_value(&fulfilled_constraint.value);
    //                                     method_impl_stream = quote! { std::borrow::Cow::Owned(#value) };
    //                                 } else {
    //                                     let variant_name = get_variant_name(&Box::new(operative_ref));
    //                                     let variant_name_string = variant_name.to_string();
    //                                     let _prepend_call_string = prepend_call.to_string();
    //                                     let template = constraint_schema.template_library.get(&operative_ref.template_id).expect("constraint object must exist");
    //                                     let field_constraint = template.field_constraints.iter().find(|constraint| &constraint.tag.id == field_id).expect("field must exist");
    //                                     let field_name = syn::Ident::new(&field_constraint.tag.name, proc_macro2::Span::call_site());
    //                                     method_impl_stream = quote!{ std::borrow::Cow::Owned(match &#prepend_call {
    //                                         // Schema::Person(person) => person.#field_name.clone(),
    //                                         Schema::#variant_name(item) => item.#field_name.clone(),
    //                                         _ => {
    //                                             println!("prepend_call: {:?}", #prepend_call);
    //                                             println!("attempted variant_name: {:?}", #variant_name_string);
    //                                             panic!("no variant name");
    //
    //                                         }
    //                                         }) };
    //                                 }
    //                             }
    //                         };
    //                     }
    //                     base_types::constraint_schema::TraitMethodImplPath::TraitMethod { trait_id, trait_method_id } => {
    //                         let inner_trait_def = constraint_schema.traits.get(&trait_id).expect("trait must exist");
    //                         println!("running here!");
    //                         let inner_method_def = inner_trait_def.methods.values().find(|method| &method.tag.id == trait_method_id).expect("method must exist");
    //                         let inner_method_name = syn::Ident::new(&inner_method_def.tag.name, proc_macro2::Span::call_site());
    //
    //                         match last_item_type {
    //                             base_types::constraint_schema::ConstraintSchemaItemType::Template => {
    //                                 let _template = constraint_schema.template_library.get(&last_item_id).expect("constraint object must exist");
    //
    //                                 method_impl_stream = quote!{ self.#inner_method_name(graph_environment) };
    //                             }
    //                             base_types::constraint_schema::ConstraintSchemaItemType::Instance => {
    //                                 // Not sure how to do this yet -- the instances being
    //                                 // referenced are in the library and are not GSO's, but rather
    //                                 // Instantiable objects. So they don't have real trait impls at
    //                                 // this point.
    //                             }
    //                             base_types::constraint_schema::ConstraintSchemaItemType::Operative => {
    //                                 // let operative_ref = constraint_schema.operative_library.get(&last_item_id).expect("operative must exist");
    //
    //                                 method_impl_stream = quote!{ graph_environment.get_element(#prepend_call).expect("element must exist").#inner_method_name() };
    //                             }
    //                         }
    //                     }
    //                     // base_types::constraint_schema::TraitMethodImplPath::TraitOperativeConstituent{trait_operative_id, trait_id, trait_method_id} => {
    //                     //     let inner_trait_def = constraint_schema.traits.get(trait_id).expect("trait must exist");
    //                     //     println!("running here too");
    //                     //     let inner_method_def = inner_trait_def.methods.iter().find(|method| &method.tag.id == trait_method_id).expect("method must exist");
    //                     //     let inner_method_name = syn::Ident::new(&inner_method_def.tag.name, proc_macro2::Span::call_site());
    //                     //     let variants_which_impl_trait = constraint_schema.template_library.iter().filter(
    //                     //         |(_co_id, co)| {
    //                     //             co.trait_impls.contains_key(trait_id)
    //                     //         }
    //                     //     ).map(|(_co_id, co)| {
    //                     //             get_variant_name(&Box::new(co))
    //                     //     }).collect::<Vec<_>>();
    //                     //
    //                     //     println!("variants which impl trait: {:?}", variants_which_impl_trait);
    //                     //     match last_item_type {
    //                     //         base_types::constraint_schema::ConstraintSchemaItemType::Template => {
    //                     //             let template = constraint_schema.template_library.get(&last_item_id).expect("constraint object must exist");
    //                     //             let trait_operative = template.trait_operatives.iter().find(|trait_op| &trait_op.tag.id == trait_operative_id).expect("trait operative must exist");
    //                     //             let trait_operative_id = trait_operative.tag.id;
    //                     //
    //                     //             method_impl_stream = quote!{ match graph_environment.get_element(&self.get_operative_by_id(&#trait_operative_id).expect("operative must exist.")).cloned().expect("element must exist") {
    //                     //                 #(Schema::#variants_which_impl_trait(variant) => {
    //                     //                 let val = variant.#inner_method_name(graph_environment);
    //                     //                 let val = val.into_owned();
    //                     //                 std::borrow::Cow::Owned(val)
    //                     //                 },)*
    //                     //                 _ => panic!(),
    //                     //                 } };
    //                     //             break;
    //                     //         }
    //                     //         base_types::constraint_schema::ConstraintSchemaItemType::Instance => {
    //                     //             break;
    //                     //             // TODO
    //                     //         }
    //                     //         base_types::constraint_schema::ConstraintSchemaItemType::Operative => {
    //                     //
    //                     //             method_impl_stream = quote!{ match graph_environment.get_element(
    //                     //                     &#prepend_call.get_operative_by_id(&#trait_operative_id).expect("operative must exist")
    //                     //                 ).cloned().expect("element must exist") {
    //                     //                 #(Schema::#variants_which_impl_trait(variant) => {
    //                     //                 let val = variant.#inner_method_name(graph_environment);
    //                     //                 let val = val.into_owned();
    //                     //                 std::borrow::Cow::Owned(val)
    //                     //                 },)*
    //                     //                     _ => {panic!()}
    //                     //                 }};
    //                     //             break;
    //                     //         }
    //                     //     };
    //                     // }
    //
    //                     base_types::constraint_schema::TraitMethodImplPath::InstanceConstituent(instance_constituent_id)=> {
    //                         last_item_id = *instance_constituent_id;
    //                         last_item_type =  base_types::constraint_schema::ConstraintSchemaItemType::Instance;
    //                     }
    //                     base_types::constraint_schema::TraitMethodImplPath::LibraryOperativeConstituent(library_operative_constituent_id) => {
    //                         if path_index == 0 {
    //                             prepend_call = quote! {graph_environment.get_element(&self.get_operative_by_id(&#library_operative_constituent_id).expect("constraint object must exist")).expect("element must exist")};
    //                         } else {
    //                             prepend_call = quote! {graph_environment.get_element(#prepend_call.get_operative_by_id(&#library_operative_constituent_id)).expect("element must exist")};
    //                         }
    //                         last_item_id = *library_operative_constituent_id;
    //                         last_item_type =  base_types::constraint_schema::ConstraintSchemaItemType::Operative;
    //                     }
    //                     _ => {}
    //                 };
    //
    //             };
    //
    //             quote! {
    //                 fn #method_name(&self, graph_environment: &dyn base_types::GraphEnvironment::<Schema = Schema>) -> std::borrow::Cow<#method_return_type> {
    //                     #method_impl_stream
    //                 }
    //             }
    //         });
    //         quote! {
    //             impl #trait_name for #instantiable_name {
    //                 #(#method_impl_streams)*
    //             }
    //         }
    //     }
    //     );
    // quote! {
    //     #(#trait_streams)*
    // }
}
