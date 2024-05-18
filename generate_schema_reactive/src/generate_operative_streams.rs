
use proc_macro2::{Ident, TokenStream};
use quote::{quote};

use base_types::constraint_schema_item::ConstraintSchemaItem;
use base_types::primitives::*;
use base_types::{constraint_schema::*, };

use crate::{generate_trait_impl_streams, IntermediateFieldTraitInfo, IntermediateSlotTraitInfo, MetaData, SlotFnDetails};
use crate::utils::{
    get_all_operatives_which_implement_trait_set, get_all_subclasses, get_operative_subclass_enum_name, get_operative_variant_name, get_operative_wrapped_name, get_primitive_type, get_primitive_value
};

pub(crate) fn generate_operative_streams(
    instantiable: Box<
        &dyn ConstraintSchemaItem<TTypes = PrimitiveTypes, TValues = PrimitiveValues>,
    >,
    constraint_schema: &ConstraintSchema<PrimitiveTypes, PrimitiveValues>,
    meta: &MetaData,
) -> proc_macro2::TokenStream {
    let _field_names = Vec::<syn::Ident>::new();
    let _field_names_setters = Vec::<syn::Ident>::new();
    let _field_values = Vec::<proc_macro2::TokenStream>::new();
    let _initial_values = Vec::<proc_macro2::TokenStream>::new();
    let struct_name = get_operative_variant_name(&instantiable.get_tag().name);
    let _item_trait_stream = crate::generate_trait_impl_streams::generate_trait_impl_streams(
        &instantiable,
        constraint_schema,
    );

    let reference_template_id = instantiable.get_template_id();
    let (_constraint_schema_tag_name, _constraint_schema_tag_id) = (
        instantiable.get_tag().name.clone(),
        instantiable.get_tag().id,
    );
    let operative_tag = instantiable.get_tag();
    let operative_id = operative_tag.id;
    let reference_template = constraint_schema
        .clone()
        .template_library
        .get(reference_template_id)
        .cloned()
        .expect("instantiable must be based on a constraint object");
    let template_tag = reference_template.get_tag();

    let field_digest = instantiable
        .get_locked_fields_digest(constraint_schema)
        .unwrap();
    // let locked_fields = field_digest.locked_fields;
    let unfulfilled_fields = field_digest.get_unfulfilled_fields();
    let unfulfilled_field_ids = unfulfilled_fields.iter().map(|field| field.tag.id).collect::<Vec<_>>();
    let unfulfilled_field_names = unfulfilled_fields
        .iter()
        .map(|field| syn::Ident::new(&field.tag.name, proc_macro2::Span::call_site()))
        .collect::<Vec<_>>();
    let unfulfilled_field_value_types = unfulfilled_fields
        .iter()
        .map(|field| get_primitive_type(&field.value_type))
        .collect::<Vec<_>>();
    let unfulfilled_field_value_types_enum = unfulfilled_fields
        .iter()
        .map(|field| syn::Ident::new(&field.value_type.to_string(), proc_macro2::Span::call_site()))
        .collect::<Vec<_>>();


    let locked_fields = field_digest.locked_fields;
    let locked_field_names = locked_fields
        .values()
        .map(|field| {
            syn::Ident::new(
                &field.fulfilled_field.field_constraint_name,
                proc_macro2::Span::call_site(),
            )
        })
        .collect::<Vec<_>>();
    let locked_field_value_types = locked_fields
        .values()
        .map(|field| {
            let value_type = &constraint_schema.template_library[&template_tag.id]
                .field_constraints[&field.fulfilled_field.field_constraint_id]
                .value_type;
            get_primitive_type(value_type)
        })
        .collect::<Vec<_>>();
    let locked_field_values = locked_fields
        .values()
        .map(|field| get_primitive_value(&field.fulfilled_field.value));

    let _operative_tag_handle = syn::Ident::new(
        &(struct_name.to_string().clone() + "operative_tag"),
        proc_macro2::Span::call_site(),
    );

    let op_digest = instantiable.get_operative_digest(constraint_schema);
    let all_slots = op_digest.operative_slots.values().collect::<Vec<_>>();
    let active_slots = all_slots
        .iter()
        .map(|unf_slot| {
            let slot_id = unf_slot.slot.tag.id;
            let slotted_instances = unf_slot
                    .related_instances
                    .iter()
                    .map(|ri| ri.instance_id)
                    .collect::<Vec<_>>();
            quote! {RActiveSlot {
                slot: &CONSTRAINT_SCHEMA.template_library.get(&#reference_template_id).unwrap().operative_slots.get(&#slot_id).unwrap(),
                slotted_instances: leptos::RwSignal::new(vec![#(#slotted_instances,)*]),
            }}
        })
        .collect::<Vec<_>>();
    let active_slot_ids = all_slots
        .iter()
        .map(|active_slot| active_slot.slot.tag.id);
    let active_slot_tokens = if active_slots.is_empty() {
        quote! {None}
    } else {
        quote! {
            Some(std::collections::HashMap::from([#((#active_slot_ids, #active_slots),)*]))
        }
    };

    let get_locked_fields_stream = locked_fields.iter().map(|(field_id, locked_field_digest)| {
        let field_getter_fn_name = proc_macro2::Ident::new(
            &format!("get_{}_field", locked_field_digest.fulfilled_field.field_constraint_name.to_lowercase()),
            proc_macro2::Span::call_site(),
        );
        let field_getting_manipulate_field_trait_name = proc_macro2::Ident::new(
            &format!("{}{}FieldGetter", struct_name, locked_field_digest.fulfilled_field.field_constraint_name),
            proc_macro2::Span::call_site(),
        );
        let field_value_type = get_primitive_type(&locked_field_digest.fulfilled_field.value.get_primitive_type());
        let locked_return_val = get_primitive_value(&locked_field_digest.fulfilled_field.value);
        quote!{
            pub trait field_getting_manipulate_field_trait_name {
                fn #field_getter_fn_name(&self) -> #field_value_type;                
            }
            impl #field_getting_manipulate_field_trait_name for RGSOWrapper<#struct_name, Schema> {
                fn #field_getter_fn_name(&self) -> #field_value_type {
                    #locked_return_val
                }
            }
        }
    });

    let get_fields_and_slots_stream = {
       let IntermediateFieldTraitInfo{trait_name: field_trait_name, trait_fns: field_trait_fns} = &meta.template_field_trait_info.get(reference_template_id).unwrap();
       let field_trait_fns_streams = field_trait_fns.values().map(|item| item.fn_signature.clone()).collect::<Vec<_>>();
       let field_trait_fns_names = field_trait_fns.values().map(|item| item.fn_name.clone()).collect::<Vec<_>>();
       let field_ids = field_trait_fns.keys().collect::<Vec<_>>();
       let field_value_types = field_trait_fns.values().map(|item| item.field_return_type.clone()).collect::<Vec<_>>();

       let IntermediateSlotTraitInfo { trait_name: slot_trait_name, trait_fns: slot_trait_fns } = meta.template_slots_trait_info.get(reference_template_id).unwrap();
       let wrapped_name = get_operative_wrapped_name(&instantiable.get_tag().name);
       let slot_stream = slot_trait_fns.iter().map(|(id, SlotFnDetails { fn_name, fn_signature, return_enum_type, is_trait_slot, id_only_signature, id_only_name, is_single_slot_bound })| {
        let id_only_body = match is_single_slot_bound {
            true => quote!{self.outgoing_slots().get(&#id).unwrap().slotted_instances.get().first().unwrap().clone()},
            false => quote!{self.outgoing_slots().get(&#id).unwrap().slotted_instances.get()},
        };
        let collection_stream = match is_single_slot_bound {
            true => quote!{.next().unwrap()},
            false => quote!{.collect::<Vec<_>>()},
        };
           let fn_streams = match &constraint_schema.template_library.get(reference_template_id).unwrap().operative_slots.get(id).unwrap().operative_descriptor {
            OperativeVariants::LibraryOperative(slot_op_id) => {
               let operative_subclass_enum_name = get_operative_subclass_enum_name(constraint_schema, slot_op_id);
               let subclasses = get_all_subclasses(constraint_schema, slot_op_id );
               let slot_op_struct_name = get_operative_variant_name(&constraint_schema.operative_library.get(slot_op_id).unwrap().tag.name);
               let subclasses_names = subclasses.iter().map(|sub| get_operative_variant_name(&sub.get_tag().name)).collect::<Vec<_>>();
               let slot_variants_match = if subclasses_names.len() <= 1 {
                   quote!{
                       Schema::#slot_op_struct_name(wrapper) => wrapper,
                       _ => panic!()
                   }
               } else {
                 quote!{
                     #(Schema::#subclasses_names(wrapper) => #operative_subclass_enum_name::#subclasses_names(wrapper),)*
                     _ => panic!(),
                 }  
               };
                quote!{
                   #fn_signature {
                       self.outgoing_slots().get(&#id).unwrap().slotted_instances.with(|slotted_instances| slotted_instances.iter().map(|slotted_instance_id| {
                           match self.graph.get(slotted_instance_id).unwrap(){
                               #slot_variants_match
                           }
                       })
                        #collection_stream
                    )
                   }
                   #id_only_signature {
                       #id_only_body
                   }
               }},

            OperativeVariants::TraitOperative(trait_op) => {
               let trait_fulfillers = get_all_operatives_which_implement_trait_set(constraint_schema, &trait_op.trait_ids);
               let trait_fulfiller_names = trait_fulfillers.iter().map(|op| {get_operative_variant_name(&op.tag.name)}).collect::<Vec<_>>();
                quote!{
                    #fn_signature {
                       self.outgoing_slots().get(&#id).unwrap().slotted_instances.with(|slotted_instances| slotted_instances.iter().map(|slotted_instance_id| {
                           match self.graph.get(slotted_instance_id).unwrap(){
                               #(Schema::#trait_fulfiller_names(wrapper) => #return_enum_type::#trait_fulfiller_names(wrapper),)*
                               _ => panic!()
                           }
                       })
                        #collection_stream
                    )
                    }
                   #id_only_signature {
                       #id_only_body
                   }
                }
                
            },
        };
        fn_streams
       }).collect::<Vec<_>>();
       let slot_stream = quote!{
           impl #slot_trait_name for #wrapped_name {
               #(#slot_stream)*
           }
       };


       quote!{
           impl #field_trait_name for #wrapped_name {
               #(#field_trait_fns_streams {
                    match self.fields.get(&#field_ids).unwrap().get() {
                        base_types::primitives::PrimitiveValues::#field_value_types(val) => val,
                        _ => panic!()
                    }
               })*
           }
           #slot_stream
       }
    };

    let manipulate_fields_stream = unfulfilled_fields.iter().map(|field| {
        let field_id = field.tag.id;
        let field_value_type = get_primitive_type(&field.value_type);
        let field_name = syn::Ident::new(&field.tag.name, proc_macro2::Span::call_site());
        let building_manipulate_field_trait_name = proc_macro2::Ident::new(
            &format!("{}{}Field", struct_name, field.tag.name),
            proc_macro2::Span::call_site(),
        );
        let field_editing_manipulate_field_trait_name = proc_macro2::Ident::new(
            &format!("{}{}FieldBuilder", struct_name, field.tag.name),
            proc_macro2::Span::call_site(),
        );

        let field_setter_fn_name = proc_macro2::Ident::new(
            &format!("set_{}", field.tag.name.to_lowercase()),
            proc_macro2::Span::call_site(),
        );

        quote! {
            pub trait #building_manipulate_field_trait_name {
                fn #field_setter_fn_name(&mut self, new_val: #field_value_type) -> &mut Self;
            }

            impl #building_manipulate_field_trait_name for RGSOBuilder<#struct_name, Schema> {
                fn #field_setter_fn_name(&mut self, new_val: #field_value_type) -> &mut Self {
                    let value = new_val.into_primitive_value();
                    self.edit_field(#field_id, value);
                    self
                }
            }
        }
    });

    let manipulate_slots_stream = all_slots.iter().map(|slot| {
        let slot_name = &slot.slot.tag.name;
        let slot_id = slot.slot.tag.id;
        let add_new_fn_name = proc_macro2::Ident::new(
            &format!("add_new_{}", slot.slot.tag.name.to_lowercase()),
            proc_macro2::Span::call_site(),
        );
        let add_existing_and_edit_fn_name = proc_macro2::Ident::new(
            &format!(
                "add_existing_{}_and_edit",
                slot.slot.tag.name.to_lowercase()
            ),
            proc_macro2::Span::call_site(),
        );
        let add_existing_or_temp_fn_name = proc_macro2::Ident::new(
            &format!(
                "add_existing_or_temp_{}",
                slot.slot.tag.name.to_lowercase()
            ),
            proc_macro2::Span::call_site(),
        );
        let remove_from_slot_fn_name = proc_macro2::Ident::new(
            &format!("remove_from_{}", slot.slot.tag.name.to_lowercase()),
            proc_macro2::Span::call_site(),
        );


        let get_slot_item_implementation = |items: &[LibraryOperative<PrimitiveTypes, PrimitiveValues>]| -> TokenStream {
            let single_item_id = if items.len() == 1 {
                Some(items.first().unwrap())
            } else {
                None
            };
            let marker_trait_name = Ident::new(&format!("{}{}AcceptableTargetMarker", struct_name, slot_name), proc_macro2::Span::call_site());
            let marker_impls = items.iter().map(|item| {
                let item_name = get_operative_variant_name(&item.get_tag().name);
                quote!{
                    impl #marker_trait_name for #item_name {}
                }
            }).collect::<Vec<_>>();

            let marker_trait_stream = if single_item_id.is_none() {
                vec![quote! {
                    trait #marker_trait_name {}
                    #(#marker_impls)*
                }]
            } else {
                vec![]
            };
            
            let item_or_t = if let Some(item) = single_item_id {
                let item_name_string = item.tag.name.clone();
                get_operative_variant_name(&
                    item_name_string
                )
            } else {
                syn::Ident::new("T", proc_macro2::Span::call_site())
            };

            let add_new_fn_signature = if let Some(item) = single_item_id {
                quote!{
                    pub fn #add_new_fn_name(&mut self, 
                        builder_closure: impl Fn(&mut RGSOBuilder<#item_or_t, Schema>) -> &mut RGSOBuilder<#item_or_t, Schema>
                    ) -> &mut Self
                 }
            } else {
                quote!{
                    pub fn #add_new_fn_name<T: RBuildable<Schema=Schema> + RIntoSchema<Schema=Schema> + #marker_trait_name>(&mut self, 
                        builder_closure: impl Fn(&mut RGSOBuilder<T, Schema>) -> &mut RGSOBuilder<T, Schema>
                    ) -> &mut Self                 
                }
            };
            let add_existing_and_edit_fn_signature = if let Some(item) = single_item_id {
                quote!{
                    pub fn #add_existing_and_edit_fn_name(&mut self,
                        existing_item_id: &Uid,
                        builder_closure: impl Fn(&mut RGSOBuilder<#item_or_t, Schema>) -> &mut RGSOBuilder<#item_or_t, Schema>
                    ) -> &mut Self
                }
            } else {
                quote!{
                    pub fn #add_existing_and_edit_fn_name<T: RBuildable<Schema=Schema> + RIntoSchema<Schema=Schema> + #marker_trait_name>(&mut self,
                        existing_item_id: &Uid,
                        builder_closure: impl Fn(&mut RGSOBuilder<T, Schema>) -> &mut RGSOBuilder<T, Schema>
                    ) -> &mut Self    
                }
            };
            let add_existing_or_temp_fn_signature = if let Some(item) = single_item_id {
                quote!{
                    pub fn #add_existing_or_temp_fn_name(&mut self,
                        existing_item_id: impl Into<BlueprintId>,
                    ) -> &mut Self
                }
            } else {
                quote!{
                    pub fn #add_existing_or_temp_fn_name<T: RBuildable<Schema=Schema> + RIntoSchema<Schema=Schema> + #marker_trait_name>(&mut self,
                        existing_item_id: impl Into<BlueprintId>,
                    ) -> &mut Self                
                }
            };
            // let super_types = get_all_superclasses(constraint_schema, item_id);
            let all_unacceptable_types = constraint_schema.operative_library.values().filter_map(|op| 
                if !items.iter().any(|super_el| super_el.tag.id == op.tag.id) {
                    Some(op)
                } else {
                    None
                }
            );
            let unacceptable_type_names = all_unacceptable_types.clone().map(|op| get_operative_variant_name(&op.tag.name)).collect::<Vec<_>>();
            let unacceptable_string_names = all_unacceptable_types.map(|op| op.tag.name.clone()).collect::<Vec<_>>();
            let acceptable_type_names = items.iter().map(|op| get_operative_variant_name(&op.tag.name)).collect::<Vec<_>>();
            let expected_type_names_string = items.iter().map(|op| op.tag.name.clone()).collect::<Vec<_>>().join(", ");
            let mismatch_error_handling = quote!{
                let error = if let Some(existing_item) = self.graph.get(&existing_item_id) {
                    match existing_item {
                        #(Schema::#acceptable_type_names(_) => None,)*
                        #(Schema::#unacceptable_type_names(_) => {
                            Some(base_types::traits::ElementCreationError::OutgoingElementIsWrongType{expected: #expected_type_names_string.to_string(), recieved: #unacceptable_string_names.to_string() })
                        },)*
                    }
                } else {
                    Some(base_types::traits::ElementCreationError::OutgoingElementDoesntExist{id: existing_item_id.clone()})
                };
                if error.is_some() {
                    self.add_error(error.unwrap());
                }
            };

            quote!{
                #(#marker_trait_stream)*
                impl RGSOBuilder<#struct_name, Schema> {
                    #add_new_fn_signature
                    {
                        let mut new_builder = #item_or_t::initiate_build(self.get_graph().clone());
                        let edge_to_this_element = base_types::traits::SlotRef {
                            host_instance_id: self.get_id().clone(),
                            target_instance_id: new_builder.get_id().clone(),
                            slot_id: #slot_id,
                        };
                        new_builder.add_incoming::<#struct_name>(edge_to_this_element.clone(), None);
                        builder_closure(&mut new_builder);
                        self.add_outgoing(&#slot_id, BlueprintId::Existing(edge_to_this_element.target_instance_id.clone()), Some(new_builder));
                        self
                    }
                    #add_existing_and_edit_fn_signature
                     {
                        let existing_item_id = existing_item_id.clone();
                        #mismatch_error_handling

                        let mut new_builder = #item_or_t::initiate_edit(existing_item_id.clone(), self.get_graph().clone());
                        let edge_to_this_element = base_types::traits::SlotRef {
                            host_instance_id: self.get_id().clone(),
                            target_instance_id: new_builder.get_id().clone(),
                            slot_id: #slot_id,
                        };
                        new_builder.add_incoming::<#struct_name>(edge_to_this_element.clone(), None);
                        builder_closure(&mut new_builder);
                        self.add_outgoing(&#slot_id, BlueprintId::Existing(existing_item_id.clone()), Some(new_builder));
                        self
                    }

                    #add_existing_or_temp_fn_signature {
                        let existing_item_blueprint_id: BlueprintId = existing_item_id.into();
                        match &existing_item_blueprint_id {
                            BlueprintId::Existing(existing_item_id) => {
                                #mismatch_error_handling
                                let edge_to_this_element = base_types::traits::SlotRef {
                                    host_instance_id: self.get_id().clone(),
                                    target_instance_id: existing_item_id.clone(),
                                    slot_id: #slot_id,
                                };
                                self.raw_add_incoming_to_updates(edge_to_this_element);
                                self.add_outgoing::<#item_or_t>(&#slot_id, existing_item_blueprint_id.clone(), None);
                                self
                            }
                            BlueprintId::Temporary(str_id) => {
                                let host_id = match &self.wip_instance {
                                    Some(instance) => BlueprintId::Temporary(instance.get_temp_id().clone()),
                                    None => BlueprintId::Existing(self.get_id().clone()),
                                };
                                self.temp_add_incoming(BlueprintId::Temporary(str_id.clone()), TempAddIncomingSlotRef {host_instance_id: host_id, slot_id:#slot_id });
                                self.add_outgoing::<#item_or_t>(&#slot_id, existing_item_blueprint_id.clone(), None);
                                self
                            }
                        }
                    }
                        
                }
            }
        };

        let add_to_slot_stream = match &slot.slot.operative_descriptor {
            OperativeVariants::LibraryOperative(lib_op_id) => {
                let subclasses = get_all_subclasses(constraint_schema, lib_op_id);
                if subclasses.len() <= 1 {
                    let item = constraint_schema.operative_library.get(lib_op_id).unwrap().clone();
                    get_slot_item_implementation(&vec![item])
                } else {
                    get_slot_item_implementation(&subclasses)
                }
            }
            OperativeVariants::TraitOperative(trait_op) => {
                let ops_which_impl_traits = get_all_operatives_which_implement_trait_set(constraint_schema, &trait_op.trait_ids);
                if ops_which_impl_traits.is_empty() {
                    quote!{}
                } else if ops_which_impl_traits.len() == 1 {
                    get_slot_item_implementation(&vec![ops_which_impl_traits.first().unwrap().clone()])
                } else {
                    get_slot_item_implementation(&ops_which_impl_traits)
                }
            }
        };
        quote! {
            impl RGSOBuilder<#struct_name, Schema> {
                pub fn #remove_from_slot_fn_name(&mut self, target_id: &Uid) -> &mut Self {
                    self.remove_outgoing(base_types::traits::SlotRef{
                        host_instance_id: self.get_id().clone(),
                        target_instance_id: target_id.clone(),
                        slot_id: #slot_id,
                    });
                    self
                }
            }

            #add_to_slot_stream
        }
    });

    let trait_impl_streams =
        generate_trait_impl_streams::generate_trait_impl_streams(&instantiable, constraint_schema);

    quote! {
        #[derive(Clone, Debug, Default)]
        pub struct #struct_name {}

        impl RIntoSchema for #struct_name {
            type Schema = Schema;
            fn into_schema(instantiable: RGSOWrapper<Self, Schema>) -> Self::Schema {
                Schema::#struct_name(instantiable.to_owned())
            }
        }

        impl RBuildable for #struct_name {
            type Schema = Schema;

            fn initiate_build(graph: impl Into<std::rc::Rc<RBaseGraphEnvironment<Self::Schema>>>) -> RGSOBuilder<#struct_name, Schema> {
                let template_ref = CONSTRAINT_SCHEMA.template_library.get(&#reference_template_id).unwrap();
                let operative_ref = CONSTRAINT_SCHEMA.operative_library.get(&#operative_id).unwrap();
                #[allow(unused_mut)]
                let mut field_hashmap = std::collections::HashMap::new();
                #(field_hashmap.insert(#unfulfilled_field_ids, RwSignal::new(None));)*
                let graph: std::rc::Rc<RBaseGraphEnvironment<Self::Schema>> = graph.into();
                let wrapper_builder = RGSOWrapperBuilder::new(
                            field_hashmap,
                            #active_slot_tokens,
                            &operative_ref,
                            &template_ref,
                            graph.clone(),
                            );
                let id = wrapper_builder.get_id().clone();
                RGSOBuilder::<#struct_name, Schema>::new(
                        Some(wrapper_builder),
                        id,
                        graph,
                    )
            }
            fn initiate_edit(id: base_types::common::Uid, graph: impl Into<std::rc::Rc<RBaseGraphEnvironment<Self::Schema>>>) -> RGSOBuilder<#struct_name, Schema> {
                let graph: std::rc::Rc<RBaseGraphEnvironment<Self::Schema>> = graph.into();
                RGSOBuilder::<#struct_name, Schema>::new(
                        None,
                            id,
                        graph,
                    )
            }
            fn get_operative_id() -> base_types::common::Uid {
               #operative_id
            }
        }

        #(#manipulate_fields_stream)*
        #(#manipulate_slots_stream)*
        #(#get_locked_fields_stream)*

        #trait_impl_streams
        #get_fields_and_slots_stream
    }
}
