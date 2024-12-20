use base_types::common::u128_to_string;
use base_types::operative_digest::OperativeSlotDigest;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};

use base_types::constraint_schema::*;
use base_types::constraint_schema_item::ConstraintSchemaItem;
use base_types::primitives::*;

use crate::utils::{
    get_all_operatives_which_implement_trait_set, get_all_slot_enum_name, get_all_subclasses,
    get_operative_subclass_enum_name, get_operative_variant_name, get_operative_wrapped_name,
    get_primitive_type, get_primitive_value,
};
use crate::{
    generate_trait_impl_streams, IntermediateFieldTraitInfo, IntermediateSlotTraitInfo, MetaData,
    SlotFnDetails,
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
    let slot_enum_name = get_all_slot_enum_name(&struct_name.clone().to_string());

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

    let slot_enum_variant_names = reference_template
        .operative_slots
        .values()
        .map(|slot| {
            let enum_variant_name = Ident::new(&slot.tag.name, proc_macro2::Span::call_site());
            let str_uid = u128_to_string(slot.tag.id.clone());
            quote! {
                #[strum(serialize=#str_uid)]
                #enum_variant_name
            }
        })
        .collect::<Vec<_>>();

    let field_digest = instantiable
        .get_locked_fields_digest(constraint_schema)
        .unwrap();
    // let locked_fields = field_digest.locked_fields;
    let unfulfilled_fields = field_digest.get_unfulfilled_fields();
    let unfulfilled_field_ids = unfulfilled_fields
        .iter()
        .map(|field| field.tag.id)
        .collect::<Vec<_>>();

    let locked_fields = field_digest.locked_fields;

    let _operative_tag_handle = syn::Ident::new(
        &(struct_name.to_string().clone() + "operative_tag"),
        Span::call_site(),
    );

    let op_digest = instantiable.get_operative_digest(constraint_schema);
    let all_slot_digests = op_digest.operative_slots.values().collect::<Vec<_>>();
    let active_slots = all_slot_digests
        .iter()
        .map(|unf_slot| {
            let slot_id = unf_slot.slot.tag.id;
            let slotted_instances = unf_slot
                    .related_instances
                    .iter()
                    .map(|ri| ri.instance_id)
                    .collect::<Vec<_>>();
            let slot_enum_variant = Ident::new(&unf_slot.slot.tag.name, proc_macro2::Span::call_site());
            quote! {
                SpecializedRActiveSlot {
                    base: RActiveSlot {
                        slot: &CONSTRAINT_SCHEMA.template_library.get(&#reference_template_id).unwrap().operative_slots.get(&#slot_id).unwrap(),
                        slotted_instances: leptos::prelude::RwSignal::new(vec![#(#slotted_instances,)*]),
                    },
                    slot_enum: #slot_enum_name::#slot_enum_variant
                }
            }
        })
        .collect::<Vec<_>>();
    let active_slot_ids = all_slot_digests
        .iter()
        .map(|active_slot| active_slot.slot.tag.id);
    let active_slot_tokens = if active_slots.is_empty() {
        quote! {None}
    } else {
        quote! {
            Some(std::collections::BTreeMap::from([#((#active_slot_ids, #active_slots),)*]))
        }
    };

    let field_generics_stream = (0..unfulfilled_fields.len()).map(|i| {
        let string = "TField".to_string() + &i.to_string();
        Ident::new(&string, Span::call_site())
    });
    let field_generics_stream = quote! { #(#field_generics_stream,)* };

    let get_locked_fields_stream = locked_fields.iter().map(|(_field_id, locked_field_digest)| {
        let field_getter_fn_name = Ident::new(
            &format!(
                "get_{}_field",
                locked_field_digest
                    .fulfilled_field
                    .field_constraint_name
                    .to_lowercase()
            ),
            Span::call_site(),
        );
        let field_getting_manipulate_field_trait_name = Ident::new(
            &format!(
                "{}{}FieldGetter",
                struct_name, locked_field_digest.fulfilled_field.field_constraint_name
            ),
            Span::call_site(),
        );
        let field_value_type = get_primitive_type(
            &locked_field_digest
                .fulfilled_field
                .value
                .get_primitive_type(),
        );
        let locked_return_val = get_primitive_value(&locked_field_digest.fulfilled_field.value);
        quote! {
            pub trait field_getting_manipulate_field_trait_name {
                fn #field_getter_fn_name(&self) -> #field_value_type;
            }
            impl #field_getting_manipulate_field_trait_name for RGSOConcrete<#struct_name, Schema> {
                fn #field_getter_fn_name(&self) -> #field_value_type {
                    #locked_return_val
                }
            }
        }
    });

    let get_fields_and_slots_stream = {
        let IntermediateFieldTraitInfo {
            trait_name: field_trait_name,
            trait_fns: field_trait_fns,
        } = &meta
            .template_field_trait_info
            .get(reference_template_id)
            .unwrap();
        let field_trait_fns_streams = field_trait_fns
            .values()
            .map(|item| item.fn_signature.clone())
            .collect::<Vec<_>>();
        let field_ids = field_trait_fns.keys().collect::<Vec<_>>();
        let field_value_types = field_trait_fns
            .values()
            .map(|item| item.field_return_type.clone())
            .collect::<Vec<_>>();
        let field_value_enum_variant_names = field_trait_fns
            .values()
            .map(|item| item.field_return_type_enum_name.clone())
            .collect::<Vec<_>>();

        let IntermediateSlotTraitInfo {
            trait_name: slot_trait_name,
            trait_fns: slot_trait_fns,
        } = meta
            .template_slots_trait_info
            .get(reference_template_id)
            .unwrap();
        let wrapped_name = get_operative_wrapped_name(&instantiable.get_tag().name);
        let slot_stream = slot_trait_fns.iter().map(|(id, SlotFnDetails { fn_name, fn_signature, return_enum_type, is_trait_slot, id_only_signature, id_only_name, is_single_slot_bound })|
            {
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
                                    match self.get_graph().get(slotted_instance_id).unwrap(){
                                        #slot_variants_match
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

                    OperativeVariants::TraitOperative(trait_op) => {
                        let trait_fulfillers = get_all_operatives_which_implement_trait_set(constraint_schema, &trait_op.trait_ids);
                        let trait_fulfiller_names = trait_fulfillers.iter().map(|op| {get_operative_variant_name(&op.tag.name)}).collect::<Vec<_>>();
                        quote!{
                            #fn_signature {
                                self.outgoing_slots().get(&#id).unwrap().slotted_instances.with(|slotted_instances| slotted_instances.iter().map(|slotted_instance_id| {
                                    match self.get_graph().get(slotted_instance_id).unwrap(){
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
            }
        ).collect::<Vec<_>>();
        let slot_stream = quote! {
            impl #slot_trait_name for #wrapped_name {
                #(#slot_stream)*
            }
        };

        quote! {
            impl #field_trait_name for #wrapped_name {
                #(#field_trait_fns_streams {
                     match self.fields.get(&#field_ids).unwrap().get() {
                         base_types::primitives::PrimitiveValues::#field_value_enum_variant_names(val) => val,
                         _ => panic!()
                     }
                })*
            }
            #slot_stream
        }
    };

    let manipulate_fields_stream = unfulfilled_fields.iter().enumerate().map(|(i, field)| {
        let field_id = field.tag.id;
        let field_value_type = get_primitive_type(&field.value_type);
        // let field_name = syn::Ident::new(&field.tag.name, Span::call_site());
        let building_manipulate_field_trait_name = Ident::new(
            &format!("{}{}Field", struct_name, field.tag.name),
            Span::call_site(),
        );

        let field_fulfilled_generic_stream = (0..unfulfilled_fields.len()).map(|j| {
            if j == i {
                quote!{ typenum::B1 }
            } else {
                let string = format!("TField{}", j);
                let generic_ident = Ident::new(&string, Span::call_site()).into_token_stream();
                generic_ident
            }
        });
        let field_fulfilled_generic_stream = {
            quote!{#(#field_fulfilled_generic_stream,)*}
        };

        let field_setter_fn_name = Ident::new(
            &format!("set_{}", field.tag.name.to_lowercase()),
            Span::call_site(),
        );


        quote! {
            pub trait #building_manipulate_field_trait_name<SlotsTS, #field_generics_stream> {
                fn #field_setter_fn_name(self, new_val: #field_value_type) -> FreshBuilder<#struct_name, Schema, (#field_fulfilled_generic_stream), SlotsTS>;
            }

            impl<#field_generics_stream SlotsTS> #building_manipulate_field_trait_name<SlotsTS, #field_generics_stream> for FreshBuilder<#struct_name, Schema, (#field_generics_stream), SlotsTS>
                // where #field_generic_in_question: typenum::B0,
            {
                fn #field_setter_fn_name(mut self, new_val: #field_value_type) -> FreshBuilder<#struct_name, Schema, (#field_fulfilled_generic_stream), SlotsTS>  {
                    let value = new_val.into_primitive_value();
                    self.inner_builder.edit_field(#field_id, value);
                    FreshBuilder::<#struct_name, Schema, (#field_fulfilled_generic_stream), SlotsTS> {
                        inner_builder: self.inner_builder,
                        _fields_typestate: std::marker::PhantomData,
                        _slots_typestate: std::marker::PhantomData,
                    }
                }
            }
            impl ExistingBuilder<#struct_name, Schema >
            {
                pub fn #field_setter_fn_name(mut self, new_val: #field_value_type) -> Self {
                    let value = new_val.into_primitive_value();
                    self.inner_builder.edit_field(#field_id, value);
                    Self {
                        inner_builder: self.inner_builder,
                    }
                }
            }
        }
    });

    let manipulate_slots_stream = all_slot_digests.iter().enumerate().map(|(slot_index, slot)| {
        let slot_name = &slot.slot.tag.name;
        let slot_id = slot.slot.tag.id;
        let add_existing_fn_name = Ident::new(
            &format!(
                "add_existing_{}",
                slot.slot.tag.name.to_lowercase()
            ),
            Span::call_site(),
        );
        let local_count_generic = Ident::new(&format!("TCount{}", slot_name), Span::call_site());
        let (
            local_min,
            local_min_nonexistent,
            local_max,
            local_max_nonexistent,
            local_zero_allowed,
        ) = get_static_slotdigest_typestate_signature_stream(slot);
        let slot_ts_consts_stream = quote!{#local_min, #local_min_nonexistent, #local_max, #local_max_nonexistent, #local_zero_allowed};
        let main_builder_slot_generics_stream =
            all_slot_digests
                .iter()
                .enumerate()
                .map(|(i, slot_digest)| {
                    if slot_digest.slot.tag.id == slot_id {
                        return quote!{base_types::post_generation::type_level::SlotTS<to_composite_id_macro::to_comp_id!(#i), #local_count_generic ,#slot_ts_consts_stream>}
                    }
                    let string1 = format!("T{}Slot{}", slot.slot.tag.name, &i.to_string());
                    Ident::new(&string1, Span::call_site()).to_token_stream()
                });
        let main_builder_slot_generics_stream = quote! { #(#main_builder_slot_generics_stream,)*};
        let generic_slot_generics_stream_with_trait_bound =
            all_slot_digests
                .iter()
                .enumerate()
                .map(|(i, slot_digest)| {
                    if slot_digest.slot.tag.id == slot_id {
                        return quote!{#local_count_generic:
                            typenum::Integer
                            + std::ops::Add<typenum::P1>
                            + std::ops::Sub<typenum::P1>
                            + IsGreaterOrEqual<#local_min>
                            + IsLessOrEqual<#local_max>
                            + IsGreater<typenum::Z0>
                            + Cmp<Z0>
                        }
                    }
                    let string1 = format!("T{}Slot{}", slot.slot.tag.name, &i.to_string());
                    let ident = Ident::new(&string1, Span::call_site());
                    quote! {#ident: base_types::post_generation::type_level::SlotTSMarker}
                });
        let generic_slot_generics_stream_with_trait_bound =
            quote! { #(#generic_slot_generics_stream_with_trait_bound,)*};
        let return_slot_generics_after_adding = all_slot_digests
            .iter()
            .enumerate()
            .map(|(i, slot_digest)| {
                if slot_digest.slot.tag.id == slot_id {
                    return quote!{base_types::post_generation::type_level::SlotTS<to_composite_id_macro::to_comp_id!(#i), typenum::Sum<#local_count_generic, typenum::P1>,#slot_ts_consts_stream >}
                }
                let string = format!(
                    "T{}Slot{}",
                    slot.slot.tag.name,
                    &i.to_string()
                );
                Ident::new(&string, Span::call_site()).to_token_stream()
            }).collect::<Vec<_>>();
        let return_type_after_adding = quote!{FreshBuilder<#struct_name, Schema, FieldsTS, (#(#return_slot_generics_after_adding,)*) >};
        let return_slot_generics_after_subtracting = all_slot_digests
            .iter()
            .enumerate()
            .map(|(i, slot_digest)| {
                if slot_digest.slot.tag.id == slot_id {
                    return quote!{base_types::post_generation::type_level::SlotTS<to_composite_id_macro::to_comp_id!(#i), typenum::Diff<#local_count_generic, typenum::P1>,#slot_ts_consts_stream >}
                }
                let string = format!(
                    "T{}Slot{}",
                    slot.slot.tag.name,
                    &i.to_string()
                );
                Ident::new(&string, Span::call_site()).to_token_stream()
            }).collect::<Vec<_>>();
        let return_type_after_subtracting = quote!{FreshBuilder<#struct_name, Schema, FieldsTS, (#(#return_slot_generics_after_subtracting,)*) >};


        // this closure takes a list of operatives which can fit into a given slot.
        // if there is only one element that can fit, it makes the method signatures to add and remove the items from the slot simpler
        // if there are multiple items, the signature must allow the user to specify which item it is that they are adding
        // - A marker trait is created which represents every type which can be slotted
        // - The marker trait is implemented on all operatives which can be slotted
        let get_slot_item_implementation = |items: &[LibraryOperative<PrimitiveTypes, PrimitiveValues>]| -> TokenStream {
            let single_item_id = if items.len() == 1 {
                Some(items.first().unwrap())
            } else {
                None
            };
            // let manipulate_slot_trait_name = Ident::new(&format!("ManipulateSlot{}{}", slot_name, struct_name), Span::call_site());
            let marker_trait_name = Ident::new(&format!("{}{}AcceptableTargetMarker", struct_name, slot_name), Span::call_site());
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

            let single_item_variant_name = if let Some(item) = single_item_id {
                let item_name_string = item.tag.name.clone();
                get_operative_variant_name(&
                    item_name_string
                )
            } else {
                syn::Ident::new("T", Span::call_site())
            };
            let fresh_single_item_generate_add_fn_signature = |method_name: Ident, item: &LibraryOperative<PrimitiveTypes, PrimitiveValues>| {
                let single_item_variant_name = {
                    let item_name_string = item.tag.name.clone();
                    get_operative_variant_name(&
                        item_name_string
                    )};
                quote!{
                    pub fn #method_name
                        <SlotsTSInnerSecondary,>
                    (mut self,
                        builder_closure: impl Fn( FreshBuilder<#single_item_variant_name, Schema, <#single_item_variant_name as StaticTypestate>::EmptyFieldTypestate, <#single_item_variant_name as StaticTypestate>::InitialSlotTypestate>)
                            -> FreshBuilder<#single_item_variant_name, Schema, <#single_item_variant_name as StaticTypestate>::FulfilledFieldTypestate, SlotsTSInnerSecondary>
                    ) -> #return_type_after_adding
                    where SlotsTSInnerSecondary: base_types::post_generation::type_level::FulfilledSlotTupleTS
                    {
                        let mut new_builder = FreshBuilder {
                            inner_builder: #single_item_variant_name::initiate_build(self.inner_builder.get_graph().clone()),
                            _fields_typestate: std::marker::PhantomData,
                            _slots_typestate: std::marker::PhantomData
                        } ;
                        let edge_to_this_element = base_types::post_generation::SlotRef {
                            host_instance_id: self.inner_builder.get_id().clone(),
                            target_instance_id: new_builder.inner_builder.get_id().clone(),
                            slot_id: #slot_id,
                        };
                        new_builder.inner_builder.add_incoming::<#struct_name>(edge_to_this_element.clone(), None);
                        let new_builder = builder_closure(new_builder);
                        self.inner_builder.add_outgoing(&#slot_id, BlueprintId::Existing(edge_to_this_element.target_instance_id.clone()), Some(new_builder.inner_builder));

                        let return_builder_plus_one_slot_typestate = FreshBuilder::<#struct_name, Schema, FieldsTS, (#(#return_slot_generics_after_adding,)*)>  {
                            inner_builder: self.inner_builder,
                            _fields_typestate: std::marker::PhantomData,
                            _slots_typestate: std::marker::PhantomData,
                        };
                        return_builder_plus_one_slot_typestate
                    }
                }
            };
            let fresh_multi_item_generate_add_fn_signature = |method_name: Ident| {
                let method_name_inner = Ident::new(&format!("{}_inner", method_name), Span::call_site());
                quote!{
                    fn #method_name_inner(mut self) {

                    }
                    pub fn #method_name
                        <T, SlotsTSInnerSecondary>
                    (mut self,
                        builder_closure: impl Fn( FreshBuilder<T, Schema, <T as StaticTypestate>::EmptyFieldTypestate, <T as StaticTypestate>::InitialSlotTypestate>)
                            -> FreshBuilder<T, Schema, <T as StaticTypestate>::FulfilledFieldTypestate, SlotsTSInnerSecondary>
                    ) -> #return_type_after_adding
                    where SlotsTSInnerSecondary: base_types::post_generation::type_level::FulfilledSlotTupleTS,
                        T: Send + Sync + StaticTypestate + std::fmt::Debug + std::clone::Clone +
                        RBuildable<Schema = Schema> + RIntoSchema<Schema = Schema> + #marker_trait_name + HasSlotEnum,
                        <T as HasSlotEnum>::SlotEnum: Send + Sync + Clone + std::fmt::Debug,
                    {
                        let mut new_builder = FreshBuilder {
                            inner_builder: #single_item_variant_name::initiate_build(self.inner_builder.get_graph().clone()),
                            _fields_typestate: std::marker::PhantomData,
                            _slots_typestate: std::marker::PhantomData
                        } ;
                        let edge_to_this_element = base_types::post_generation::SlotRef {
                            host_instance_id: self.inner_builder.get_id().clone(),
                            target_instance_id: new_builder.inner_builder.get_id().clone(),
                            slot_id: #slot_id,
                        };
                        new_builder.inner_builder.add_incoming::<#struct_name>(edge_to_this_element.clone(), None);
                        let new_builder = builder_closure(new_builder);
                        self.inner_builder.add_outgoing(&#slot_id, BlueprintId::Existing(edge_to_this_element.target_instance_id.clone()), Some(new_builder.inner_builder));

                        let return_builder_plus_one_slot_typestate = FreshBuilder::<#struct_name, Schema, FieldsTS, (#(#return_slot_generics_after_adding,)*)>  {
                            inner_builder: self.inner_builder,
                            _fields_typestate: std::marker::PhantomData,
                            _slots_typestate: std::marker::PhantomData,
                        };
                        return_builder_plus_one_slot_typestate
                    }
                }
            };
            let existing_single_item_generate_add_fn_signature = |method_name: Ident, item: &LibraryOperative<PrimitiveTypes, PrimitiveValues>| {
                let single_item_variant_name = {
                    let item_name_string = item.tag.name.clone();
                    get_operative_variant_name(&
                        item_name_string
                    )};
                quote!{
                    pub fn #method_name
                        <SlotsTSInnerSecondary>
                    (mut self,
                        builder_closure: impl Fn( FreshBuilder<#single_item_variant_name, Schema, <#single_item_variant_name as StaticTypestate>::EmptyFieldTypestate, <#single_item_variant_name as StaticTypestate>::InitialSlotTypestate>)
                            -> FreshBuilder<#single_item_variant_name, Schema, <#single_item_variant_name as StaticTypestate>::FulfilledFieldTypestate, SlotsTSInnerSecondary>
                    ) -> Self
                    where SlotsTSInnerSecondary: base_types::post_generation::type_level::FulfilledSlotTupleTS
                    {
                        let mut new_builder = FreshBuilder {
                            inner_builder: #single_item_variant_name::initiate_build(self.inner_builder.get_graph().clone()),
                            _fields_typestate: std::marker::PhantomData,
                            _slots_typestate: std::marker::PhantomData
                        } ;
                        let edge_to_this_element = base_types::post_generation::SlotRef {
                            host_instance_id: self.inner_builder.get_id().clone(),
                            target_instance_id: new_builder.inner_builder.get_id().clone(),
                            slot_id: #slot_id,
                        };
                        new_builder.inner_builder.add_incoming::<#struct_name>(edge_to_this_element.clone(), None);
                        let new_builder = builder_closure(new_builder);
                        self.inner_builder.add_outgoing(&#slot_id, BlueprintId::Existing(edge_to_this_element.target_instance_id.clone()), Some(new_builder.inner_builder));

                        self
                    }
                }
            };
            let existing_multi_item_generate_add_fn_signature = |method_name: Ident| {
                quote!{
                    pub fn #method_name
                        <T, SlotsTSInnerSecondary>
                    (mut self,
                        builder_closure: impl Fn( FreshBuilder<T, Schema, <T as StaticTypestate>::EmptyFieldTypestate, <T as StaticTypestate>::InitialSlotTypestate>)
                            -> FreshBuilder<T, Schema, <T as StaticTypestate>::FulfilledFieldTypestate, SlotsTSInnerSecondary>
                    ) -> Self
                    where SlotsTSInnerSecondary: base_types::post_generation::type_level::FulfilledSlotTupleTS,
                        T: Send + Sync + StaticTypestate + std::fmt::Debug + std::clone::Clone + RBuildable<Schema = Schema> + RIntoSchema<Schema = Schema> + #marker_trait_name + HasSlotEnum,
                        <T as HasSlotEnum>::SlotEnum: Send + Sync + Clone + std::fmt::Debug,
                    {
                        let mut new_builder = FreshBuilder {
                            inner_builder: #single_item_variant_name::initiate_build(self.inner_builder.get_graph().clone()),
                            _fields_typestate: std::marker::PhantomData,
                            _slots_typestate: std::marker::PhantomData
                        } ;
                        let edge_to_this_element = base_types::post_generation::SlotRef {
                            host_instance_id: self.inner_builder.get_id().clone(),
                            target_instance_id: new_builder.inner_builder.get_id().clone(),
                            slot_id: #slot_id,
                        };
                        new_builder.inner_builder.add_incoming::<#struct_name>(edge_to_this_element.clone(), None);
                        let new_builder = builder_closure(new_builder);
                        self.inner_builder.add_outgoing(&#slot_id, BlueprintId::Existing(edge_to_this_element.target_instance_id.clone()), Some(new_builder.inner_builder));

                        self
                    }
                }
            };

            let fresh_add_new_fn_definitions = if let Some(item) = single_item_id {
                let add_new_fn_name = Ident::new(
                    &format!("add_new_{}", slot.slot.tag.name.to_lowercase()),
                    Span::call_site(),
                );
                fresh_single_item_generate_add_fn_signature(add_new_fn_name, item)
            } else {
                let add_new_fn_name = Ident::new(
                    &format!("add_new_{}", slot.slot.tag.name.to_lowercase()),
                    Span::call_site(),
                );
                fresh_multi_item_generate_add_fn_signature(add_new_fn_name)
            };
            let existing_add_new_fn_definitions = if let Some(item) = single_item_id {
                let add_new_fn_name = Ident::new(
                    &format!("add_new_{}", slot.slot.tag.name.to_lowercase()),
                    Span::call_site(),
                );
                existing_single_item_generate_add_fn_signature(add_new_fn_name, item)
            } else {
                let add_new_fn_name = Ident::new(
                    &format!("add_new_{}", slot.slot.tag.name.to_lowercase()),
                    Span::call_site(),
                );
                existing_multi_item_generate_add_fn_signature(add_new_fn_name)
            };

            let fresh_add_existing_fn_signature = if let Some(item) = single_item_id {
                quote!{
                    fn #add_existing_fn_name
                    (mut self,
                        existing_item_id: &Uid,
                        builder_closure: impl Fn(ExistingBuilder<#single_item_variant_name, Schema>)
                            -> ExistingBuilder<#single_item_variant_name, Schema>
                    ) -> #return_type_after_adding
                }
            } else {
                quote!{
                    fn #add_existing_fn_name
                        <T: RBuildable<Schema=Schema> + RIntoSchema<Schema=Schema> + #marker_trait_name + Send + Sync + HasSlotEnum>
                    (mut self,
                        existing_item_id: &Uid,
                        builder_closure: impl Fn(ExistingBuilder<T, Schema>)
                            -> ExistingBuilder<T, Schema>
                    ) -> #return_type_after_adding
                    where <T as HasSlotEnum>::SlotEnum: Send + Sync + Clone + std::fmt::Debug,
                }
            };
            let existing_add_existing_fn_signature = if let Some(item) = single_item_id {
                quote!{
                    fn #add_existing_fn_name
                    (mut self,
                        existing_item_id: &Uid,
                        builder_closure: impl Fn(ExistingBuilder<#single_item_variant_name, Schema>)
                            -> ExistingBuilder<#single_item_variant_name, Schema>
                    ) -> Self
                }
            } else {
                quote!{
                    fn #add_existing_fn_name
                        <T: RBuildable<Schema=Schema> + RIntoSchema<Schema=Schema> + #marker_trait_name + Send + Sync + HasSlotEnum>
                    (mut self,
                        existing_item_id: &Uid,
                        builder_closure: impl Fn(ExistingBuilder<T, Schema>)
                            -> ExistingBuilder<T, Schema>
                    ) -> Self
                    where <T as HasSlotEnum>::SlotEnum: Send + Sync + Clone + std::fmt::Debug,
                }
            };

            let fresh_single_item_generate_add_temp_fn_definition = |method_name: Ident, item: &LibraryOperative<PrimitiveTypes, PrimitiveValues>| {
                let single_item_variant_name = {
                    let item_name_string = item.tag.name.clone();
                    get_operative_variant_name(&
                        item_name_string
                    )};
                quote!{
                    pub fn #method_name(mut self,
                        str_id: impl AsRef<str>,
                    ) -> #return_type_after_adding
                    {

                                let host_id = match &self.inner_builder.wip_instance {
                                    Some(instance) => BlueprintId::Temporary(instance.get_temp_id().clone()),
                                    None => BlueprintId::Existing(self.inner_builder.get_id().clone()),
                                };
                                self.inner_builder.temp_add_incoming(str_id.as_ref(), TempAddIncomingSlotRef {host_instance_id: host_id, slot_id:#slot_id });
                                self.inner_builder.add_outgoing::<#single_item_variant_name>(&#slot_id, BlueprintId::Temporary(str_id.as_ref().to_string()), None);
                                let return_builder_plus_one_slot_typestate = FreshBuilder::<#struct_name, Schema, FieldsTS, (#(#return_slot_generics_after_adding,)*)>  {
                                    inner_builder: self.inner_builder,
                                    _fields_typestate: std::marker::PhantomData,
                                    _slots_typestate: std::marker::PhantomData,
                                };
                                return_builder_plus_one_slot_typestate
                    }
                }
            };
            let fresh_multi_item_generate_add_temp_fn_definition = |method_name: Ident | {
                quote!{
                    pub fn #method_name<T>(mut self,
                        str_id: impl AsRef<str>,
                    ) -> #return_type_after_adding
                    where
                        T: Send + Sync + StaticTypestate + std::fmt::Debug + std::clone::Clone + RBuildable<Schema = Schema> + RIntoSchema<Schema = Schema> + #marker_trait_name + HasSlotEnum,
                        <T as HasSlotEnum>::SlotEnum: Send + Sync + Clone + std::fmt::Debug,
                    {
                                let host_id = match &self.inner_builder.wip_instance {
                                    Some(instance) => BlueprintId::Temporary(instance.get_temp_id().clone()),
                                    None => BlueprintId::Existing(self.inner_builder.get_id().clone()),
                                };
                                self.inner_builder.temp_add_incoming(str_id.as_ref(), TempAddIncomingSlotRef {host_instance_id: host_id, slot_id:#slot_id });
                                self.inner_builder.add_outgoing::<T>(&#slot_id, BlueprintId::Temporary(str_id.as_ref().to_string()), None);
                                let return_builder_plus_one_slot_typestate = FreshBuilder::<#struct_name, Schema, FieldsTS, (#(#return_slot_generics_after_adding,)*)>  {
                                    inner_builder: self.inner_builder,
                                    _fields_typestate: std::marker::PhantomData,
                                    _slots_typestate: std::marker::PhantomData,
                                };
                                return_builder_plus_one_slot_typestate
                    }
                }
            };
            let existing_single_item_generate_add_temp_fn_definition = |method_name: Ident, item: &LibraryOperative<PrimitiveTypes, PrimitiveValues>| {
                let single_item_variant_name = {
                    let item_name_string = item.tag.name.clone();
                    get_operative_variant_name(&
                        item_name_string
                    )};
                quote!{
                    pub fn #method_name(mut self,
                        str_id: impl AsRef<str>,
                    ) -> Self
                    {
                                let host_id = match &self.inner_builder.wip_instance {
                                    Some(instance) => BlueprintId::Temporary(instance.get_temp_id().clone()),
                                    None => BlueprintId::Existing(self.inner_builder.get_id().clone()),
                                };
                                self.inner_builder.temp_add_incoming(str_id.as_ref(), TempAddIncomingSlotRef {host_instance_id: host_id, slot_id:#slot_id });
                                self.inner_builder.add_outgoing::<#single_item_variant_name>(&#slot_id, BlueprintId::Temporary(str_id.as_ref().to_string()), None);
                                self
                    }
                }
            };
            let existing_multi_item_generate_add_temp_fn_definition = |method_name: Ident | {
                quote!{
                    pub fn #method_name<T>(mut self,
                        str_id: impl AsRef<str>,
                    ) -> Self
                    where
                    T: Send + Sync + StaticTypestate + std::fmt::Debug + std::clone::Clone + RBuildable<Schema = Schema> + RIntoSchema<Schema = Schema> + #marker_trait_name + HasSlotEnum,
                    <T as HasSlotEnum>::SlotEnum: Send + Sync + Clone + std::fmt::Debug,
                    {
                                let host_id = match &self.inner_builder.wip_instance {
                                    Some(instance) => BlueprintId::Temporary(instance.get_temp_id().clone()),
                                    None => BlueprintId::Existing(self.inner_builder.get_id().clone()),
                                };
                                self.inner_builder.temp_add_incoming(str_id.as_ref(), TempAddIncomingSlotRef {host_instance_id: host_id, slot_id:#slot_id });
                                self.inner_builder.add_outgoing::<T>(&#slot_id, BlueprintId::Temporary(str_id.as_ref().to_string()), None);
                                self
                    }
                }
            };
            let fresh_add_temp_fn_definition = if let Some(item) = single_item_id {
                let add_new_fn_name = Ident::new(
                        &format!("add_temp_{}", slot.slot.tag.name.to_lowercase()),
                        Span::call_site(),
                );
                fresh_single_item_generate_add_temp_fn_definition(add_new_fn_name, item)
            } else {
                let add_new_fn_name = Ident::new(
                        &format!("add_temp_{}", slot.slot.tag.name.to_lowercase()),
                        Span::call_site(),
                );
                fresh_multi_item_generate_add_temp_fn_definition(add_new_fn_name)
            };
            let existing_add_temp_fn_definition = if let Some(item) = single_item_id {
                let add_new_fn_name = Ident::new(
                        &format!("add_temp_{}", slot.slot.tag.name.to_lowercase()),
                        Span::call_site(),
                );
                existing_single_item_generate_add_temp_fn_definition(add_new_fn_name, item)
            } else {
                let add_new_fn_name = Ident::new(
                        &format!("add_temp_{}", slot.slot.tag.name.to_lowercase()),
                        Span::call_site(),
                );
                existing_multi_item_generate_add_temp_fn_definition(add_new_fn_name)
            };
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
                let error = if let Some(existing_item) = self.inner_builder.get_graph().get(&existing_item_id) {
                    match existing_item {
                        #(Schema::#acceptable_type_names(_) => None,)*
                        #(Schema::#unacceptable_type_names(_) => {
                            Some(base_types::post_generation::ElementCreationError::OutgoingElementIsWrongType{expected: #expected_type_names_string.to_string(), recieved: #unacceptable_string_names.to_string() })
                        },)*
                    }
                } else {
                    Some(base_types::post_generation::ElementCreationError::OutgoingElementDoesntExist{id: existing_item_id.clone()})
                };
                if error.is_some() {
                    self.inner_builder.add_error(error.unwrap());
                }
            };

            quote!{
                #(#marker_trait_stream)*
                impl< FieldsTS, #generic_slot_generics_stream_with_trait_bound> FreshBuilder<#struct_name, Schema, FieldsTS, (#main_builder_slot_generics_stream)>
                    where
                    <#local_count_generic as std::ops::Add<PInt<UInt<UTerm, B1>>>>::Output:
                    typenum::Integer
                    + std::ops::Add<typenum::P1>
                    + std::ops::Sub<typenum::P1>
                    + IsGreaterOrEqual<#local_min>
                    + IsLessOrEqual<#local_max>
                    + IsGreater<typenum::Z0>
                    + Cmp<Z0>
                    + Cmp<#local_min>
                    + Cmp<#local_max>
                    + Cmp<PInt<UInt<UTerm, B1>>>,
                    base_types::post_generation::type_level::SlotTS<to_composite_id_macro::to_comp_id!(#slot_index), #local_count_generic,#slot_ts_consts_stream >: base_types::post_generation::type_level::SlotCanAddOne
                {
                    #fresh_add_new_fn_definitions
                    pub #fresh_add_existing_fn_signature
                    {
                        let existing_item_id = existing_item_id.clone();
                        #mismatch_error_handling

                        let mut new_builder = ExistingBuilder {
                            inner_builder: #single_item_variant_name::initiate_edit(existing_item_id.clone(), self.inner_builder.get_graph().clone()) ,
                        };
                        let edge_to_this_element = base_types::post_generation::SlotRef {
                            host_instance_id: self.inner_builder.get_id().clone(),
                            target_instance_id: new_builder.inner_builder.get_id().clone(),
                            slot_id: #slot_id,
                        };
                        new_builder.inner_builder.add_incoming::<#struct_name>(edge_to_this_element.clone(), None);
                        let new_builder = builder_closure(new_builder);
                        self.inner_builder.add_outgoing(&#slot_id, BlueprintId::Existing(existing_item_id.clone()), Some(new_builder.inner_builder));
                        let return_builder_plus_one_slot_typestate = FreshBuilder::<#struct_name, Schema, FieldsTS, (#(#return_slot_generics_after_adding,)*)>  {
                            inner_builder: self.inner_builder,
                            _fields_typestate: std::marker::PhantomData,
                            _slots_typestate: std::marker::PhantomData,
                        };
                        return_builder_plus_one_slot_typestate
                    }
                    #fresh_add_temp_fn_definition
                }

                impl ExistingBuilder<#struct_name, Schema>
                {
                    #existing_add_new_fn_definitions
                    pub #existing_add_existing_fn_signature
                    {
                        let existing_item_id = existing_item_id.clone();
                        #mismatch_error_handling

                        let mut new_builder = ExistingBuilder {
                            inner_builder: #single_item_variant_name::initiate_edit(existing_item_id.clone(), self.inner_builder.get_graph().clone()) ,
                        };
                        let edge_to_this_element = base_types::post_generation::SlotRef {
                            host_instance_id: self.inner_builder.get_id().clone(),
                            target_instance_id: new_builder.inner_builder.get_id().clone(),
                            slot_id: #slot_id,
                        };
                        new_builder.inner_builder.add_incoming::<#struct_name>(edge_to_this_element.clone(), None);
                        let new_builder = builder_closure(new_builder);
                        self.inner_builder.add_outgoing(&#slot_id, BlueprintId::Existing(existing_item_id.clone()), Some(new_builder.inner_builder));
                        self
                    }
                    #existing_add_temp_fn_definition
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
        let remove_from_slot_fn_name = Ident::new(
            &format!("remove_from_{}", slot.slot.tag.name.to_lowercase()),
            Span::call_site(),
        );
        let remove_slot_trait_name = Ident::new(&format!("RemoveFromSlot{}{}", slot_name, struct_name), Span::call_site());
        quote! {
            impl<FieldsTS, #generic_slot_generics_stream_with_trait_bound> FreshBuilder<#struct_name, Schema, FieldsTS, (#main_builder_slot_generics_stream)>
            where <#local_count_generic as std::ops::Sub<PInt<UInt<UTerm, typenum::B1>>>>::Output:
            typenum::Integer
            + std::ops::Add<typenum::P1>
            + std::ops::Sub<typenum::P1>
            + IsGreaterOrEqual<#local_min>
            + IsLessOrEqual<#local_max>
            + IsGreater<typenum::Z0>
            + Cmp<Z0>
            + Cmp<#local_min>
            + Cmp<#local_max>
            + Cmp<PInt<UInt<UTerm, B1>>>,
            base_types::post_generation::type_level::SlotTS<to_composite_id_macro::to_comp_id!(#slot_index), #local_count_generic,#slot_ts_consts_stream >: base_types::post_generation::type_level::SlotCanSubtractOne
            {
                pub fn #remove_from_slot_fn_name(mut self, target_id: &Uid) -> #return_type_after_subtracting {
                    self.inner_builder.remove_outgoing(base_types::post_generation::SlotRef{
                        host_instance_id: self.inner_builder.get_id().clone(),
                        target_instance_id: target_id.clone(),
                        slot_id: #slot_id,
                    });
                    let return_builder_minus_one_slot_typestate = FreshBuilder::<#struct_name, Schema, FieldsTS, (#(#return_slot_generics_after_subtracting,)*)>  {
                        inner_builder: self.inner_builder,
                        _fields_typestate: std::marker::PhantomData,
                        _slots_typestate: std::marker::PhantomData,
                    };
                    return_builder_minus_one_slot_typestate
                }
            }

            #add_to_slot_stream
        }
    });

    // Refers to traits defined and implemented by the user in the schema
    let trait_impl_streams =
        generate_trait_impl_streams::generate_trait_impl_streams(&instantiable, constraint_schema);
    let edit_rgso_trait_name = Ident::new(
        &format!("EditRGSOConcrete{}", struct_name),
        Span::call_site(),
    );

    // Static typestate calculations
    let adding_item_field_digest = instantiable
        .get_locked_fields_digest(constraint_schema)
        .unwrap();
    let num_reqd_fields = adding_item_field_digest.field_constraints.len()
        - adding_item_field_digest.locked_fields.len();
    let empty_field_typestate_stream = (0..num_reqd_fields)
        .map(|_| {
            quote! {typenum::B0}
        })
        .collect::<Vec<_>>();
    let empty_field_typestate_stream = quote! {#(#empty_field_typestate_stream,)*};
    let fulfilled_field_typestate_stream = (0..num_reqd_fields).map(|_| {
        quote! {typenum::B1}
    });
    let fulfilled_field_typestate_stream = quote! {#(#fulfilled_field_typestate_stream,)*};
    let adding_item_operative_digest = instantiable.get_operative_digest(constraint_schema);
    let item_default_slot_typestate_stream = adding_item_operative_digest.operative_slots.iter().enumerate().map(|(index, (_slot_id, slot_digest))| {
        let count = slot_digest.related_instances.len();
        let count = if count == 0 {
            quote! {typenum::Z0}
        } else {
            let string = format! {"typenum::P{}", count};
            Ident::new(&string, Span::call_site()).to_token_stream()
        };

        let (
            local_min,
            local_min_nonexistent,
            local_max,
            local_max_nonexistent,
            local_zero_allowed,
        ) = get_static_slotdigest_typestate_signature_stream(slot_digest);
        quote! {base_types::post_generation::type_level::SlotTS<to_composite_id_macro::to_comp_id!(#index), #count, #local_min, #local_min_nonexistent, #local_max, #local_max_nonexistent, #local_zero_allowed>}
    });
    let item_default_slot_typestate_stream = quote! { #(#item_default_slot_typestate_stream,)*};

    quote! {
        #[derive(Clone, Debug, Default)]
        pub struct #struct_name {}
        #[derive(Clone, Debug, strum_macros::EnumString, PartialEq, serde::Serialize)]
        pub enum #slot_enum_name {
            #(#slot_enum_variant_names,)*
        }

        impl StaticTypestate for #struct_name {
            type InitialSlotTypestate = (#item_default_slot_typestate_stream);
            type EmptyFieldTypestate = (#empty_field_typestate_stream);
            type FulfilledFieldTypestate = (#fulfilled_field_typestate_stream);
        }

        impl HasSlotEnum for #struct_name {
            type SlotEnum = #slot_enum_name;
        }

        impl RIntoSchema for #struct_name {
            type Schema = Schema;
            fn into_schema(instantiable: RGSOConcrete<Self, Schema>) -> Schema {
                Schema::#struct_name(instantiable.to_owned())
            }
        }

        impl #struct_name {
            pub fn new(graph:impl Into<std::sync::Arc<RBaseGraphEnvironment<Schema>>>) -> FreshBuilder<#struct_name, Schema, <#struct_name as StaticTypestate>::EmptyFieldTypestate, <#struct_name as StaticTypestate>::InitialSlotTypestate> {
                FreshBuilder {
                    inner_builder: #struct_name::initiate_build(graph.into()),
                    _fields_typestate: std::marker::PhantomData,
                    _slots_typestate: std::marker::PhantomData,
                }
            }
        }


        pub trait #edit_rgso_trait_name {
            fn edit(&self, graph: impl Into<std::sync::Arc<RBaseGraphEnvironment<Schema>>>) -> ExistingBuilder<#struct_name, Schema> ;
        }
        impl #edit_rgso_trait_name for RGSOConcrete<#struct_name, Schema> {
            fn edit(&self, graph: impl Into<std::sync::Arc<RBaseGraphEnvironment<Schema>>>) -> ExistingBuilder<#struct_name, Schema> {
                ExistingBuilder {
                    inner_builder: #struct_name::initiate_edit(*self.get_id(), graph.into()),
                }
            }
        }
        impl RBuildable for #struct_name {
            type Schema = Schema;

            fn initiate_build(graph: impl Into<std::sync::Arc<RBaseGraphEnvironment<Self::Schema>>>) -> SubgraphBuilder<#struct_name, Schema> {
                let template_ref = CONSTRAINT_SCHEMA.template_library.get(&#reference_template_id).unwrap();
                let operative_ref = CONSTRAINT_SCHEMA.operative_library.get(&#operative_id).unwrap();
                #[allow(unused_mut)]
                let mut field_hashmap = std::collections::HashMap::new();
                #(field_hashmap.insert(#unfulfilled_field_ids, RwSignal::new(None));)*
                let graph: std::sync::Arc<RBaseGraphEnvironment<Self::Schema>> = graph.into();
                let wrapper_builder = RGSOConcreteBuilder::new(
                            field_hashmap,
                            #active_slot_tokens,
                            &operative_ref,
                            &template_ref,
                            graph.clone(),
                            );
                let id = wrapper_builder.get_id().clone();
                SubgraphBuilder::<#struct_name, Schema>::new(
                        Some(wrapper_builder),
                        id,
                        graph,
                    )
            }
            fn initiate_edit(id: base_types::common::Uid, graph: impl Into<std::sync::Arc<RBaseGraphEnvironment<Self::Schema>>>) -> SubgraphBuilder<#struct_name, Schema> {
                let graph: std::sync::Arc<RBaseGraphEnvironment<Self::Schema>> = graph.into();
                SubgraphBuilder::<#struct_name, Schema>::new(
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

enum Operation {
    Add,
    Subtract,
}
fn get_static_slotdigest_typestate_signature_stream(
    slot_digest: &OperativeSlotDigest,
) -> (
    TokenStream,
    TokenStream,
    TokenStream,
    TokenStream,
    TokenStream,
) {
    let (min, min_nonexistent, max, max_nonexistent, zero_allowed) = match slot_digest.slot.bounds {
        SlotBounds::Single => (
            quote! { typenum::P1 },
            quote! {typenum::B0},
            quote! { typenum::P1 },
            quote! {typenum::B0},
            quote! { typenum::B0 },
        ),
        SlotBounds::LowerBound(lower_bound) => {
            let lower_bound = if lower_bound == 0 {
                quote! {typenum::Z0}
            } else {
                let ident = Ident::new(&format!("P{}", lower_bound), Span::call_site());
                quote! {typenum::#ident}
            };
            (
                quote! {#lower_bound},
                quote! {typenum::B0},
                quote! {typenum::Z0},
                quote! {typenum::B1},
                quote! {typenum::B0},
            )
        }
        SlotBounds::UpperBound(upper_bound) => {
            let upper_bound = {
                let ident = Ident::new(&format!("P{}", upper_bound), Span::call_site());
                quote! {typenum::#ident}
            };
            (
                quote! {typenum::Z0},
                quote! {typenum::B1},
                quote! {#upper_bound},
                quote! {typenum::B0},
                quote! {typenum::B0},
            )
        }
        SlotBounds::Range(lower_bound, upper_bound) => {
            let lower_bound = if lower_bound == 0 {
                quote! {typenum::Z0}
            } else {
                let ident = Ident::new(&format!("P{}", lower_bound), Span::call_site());
                quote! {typenum::#ident}
            };
            let upper_bound = {
                let ident = Ident::new(&format!("P{}", upper_bound), Span::call_site());
                quote! {typenum::#ident}
            };

            (
                quote! {#lower_bound},
                quote! {typenum::B0},
                quote! {#upper_bound},
                quote! {typenum::B0},
                quote! {typenum::B0},
            )
        }

        SlotBounds::LowerBoundOrZero(lower_bound) => {
            let lower_bound = {
                let ident = Ident::new(&format!("P{}", lower_bound), Span::call_site());
                quote! {typenum::#ident}
            };
            (
                quote! {#lower_bound},
                quote! {typenum::B0},
                quote! {typenum::Z0},
                quote! {typenum::B1},
                quote! {typenum::B1},
            )
        }
        SlotBounds::RangeOrZero(lower_bound, upper_bound) => {
            let lower_bound = if lower_bound == 0 {
                quote! {typenum::Z0}
            } else {
                let ident = Ident::new(&format!("P{}", lower_bound), Span::call_site());
                quote! {typenum::#ident}
            };
            let upper_bound = {
                let ident = Ident::new(&format!("P{}", upper_bound), Span::call_site());
                quote! {typenum::#ident}
            };
            (
                quote! {#lower_bound},
                quote! {typenum::B0},
                quote! {#upper_bound},
                quote! {typenum::B0},
                quote! {typenum::B1},
            )
        }
    };
    (min, min_nonexistent, max, max_nonexistent, zero_allowed)
}
