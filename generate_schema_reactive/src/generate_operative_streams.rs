

use base_types::traits::{ActiveSlot};

use proc_macro2::{Ident, TokenStream};
use quote::{quote};

use base_types::constraint_schema_item::ConstraintSchemaItem;
use base_types::primitives::*;
use base_types::{constraint_schema::*, };


use crate::generate_trait_impl_streams;
use crate::utils::{
    get_primitive_type, get_primitive_value, get_variant_builder_name, get_variant_name,
};

pub(crate) fn generate_operative_streams(
    instantiable: Box<
        &dyn ConstraintSchemaItem<TTypes = PrimitiveTypes, TValues = PrimitiveValues>,
    >,
    constraint_schema: &ConstraintSchema<PrimitiveTypes, PrimitiveValues>,
    // graph_environment: syn::Expr,
) -> proc_macro2::TokenStream {
    let _field_names = Vec::<syn::Ident>::new();
    let _field_names_setters = Vec::<syn::Ident>::new();
    let _field_values = Vec::<proc_macro2::TokenStream>::new();
    let _initial_values = Vec::<proc_macro2::TokenStream>::new();
    let struct_name = get_variant_name(&instantiable);
    let struct_builder_name = get_variant_builder_name(&instantiable);
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


    let fulfilled_fields = field_digest.locked_fields;
    let fulfilled_field_names = fulfilled_fields
        .values()
        .map(|field| {
            syn::Ident::new(
                &field.fulfilled_field.field_constraint_name,
                proc_macro2::Span::call_site(),
            )
        })
        .collect::<Vec<_>>();
    let fulfilled_field_value_types = fulfilled_fields
        .values()
        .map(|field| {
            let value_type = &constraint_schema.template_library[&template_tag.id]
                .field_constraints[&field.fulfilled_field.field_constraint_id]
                .value_type;
            get_primitive_type(value_type)
        })
        .collect::<Vec<_>>();
    let fulfilled_field_values = fulfilled_fields
        .values()
        .map(|field| get_primitive_value(&field.fulfilled_field.value));

    let _operative_tag_handle = syn::Ident::new(
        &(struct_name.to_string().clone() + "operative_tag"),
        proc_macro2::Span::call_site(),
    );

    let op_digest = instantiable.get_operative_digest(constraint_schema);
    // let unfulfilled_slots = op_digest.get_unfulfilled_operative_slots();
    let all_slots = op_digest.operative_slots.values().collect::<Vec<_>>();
    let active_slots = all_slots
        .iter()
        .map(|unf_slot| {
            // let slot_name = unf_slot.slot.tag.name;
            let slot_id = unf_slot.slot.tag.id;
            // let active_slot = ActiveSlot {
            //     slot: unf_slot.slot.clone(),
            //     slotted_instances: unf_slot
            //         .related_instances
            //         .iter()
            //         .map(|ri| ri.instance_id)
            //         .collect(),
            // };
            let slotted_instances = unf_slot
                    .related_instances
                    .iter()
                    .map(|ri| ri.instance_id)
                    .collect::<Vec<_>>();
            quote! {rt::RActiveSlot {
                slot: &CONSTRAINT_SCHEMA.template_library.get(&#reference_template_id).unwrap().operative_slots.get(&#slot_id).unwrap(),
                slotted_instances: leptos::RwSignal::new(vec![#(#slotted_instances,)*]),
            }}
            // active_slot
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

    let manipulate_fields_stream = unfulfilled_fields.iter().map(|field| {
        let field_id = field.tag.id;
        let field_value_type = get_primitive_type(&field.value_type);
        let field_name = syn::Ident::new(&field.tag.name, proc_macro2::Span::call_site());
        let building_manipulate_field_trait_name = proc_macro2::Ident::new(
            &format!("{}{}Field", struct_name, field.tag.name),
            proc_macro2::Span::call_site(),
        );
        let editing_manipulate_field_trait_name = proc_macro2::Ident::new(
            &format!("{}{}FieldBuilder", struct_name, field.tag.name),
            proc_macro2::Span::call_site(),
        );

        let setter_fn_name = proc_macro2::Ident::new(
            &format!("set_{}", field.tag.name.to_lowercase()),
            proc_macro2::Span::call_site(),
        );

        quote! {
            pub trait #editing_manipulate_field_trait_name {
                fn #setter_fn_name(&self, new_val: #field_value_type) -> &Self;
            }
            pub trait #building_manipulate_field_trait_name {
                fn #setter_fn_name(&mut self, new_val: #field_value_type) -> &mut Self;
            }

            impl #building_manipulate_field_trait_name for rt::RGSOBuilder<rt::RGSOWrapperBuilder<#struct_builder_name>, rt::RGSOWrapper<#struct_name, Schema>, Schema> {
                fn #setter_fn_name(&mut self, new_val: #field_value_type) -> &mut Self {
                    self.wip_instance.data.#field_name.set(Some(new_val));
                    self
                }
            }
            impl #editing_manipulate_field_trait_name for rt::RGSOWrapper<#struct_name, Schema> {
                fn #setter_fn_name(&self, new_val: #field_value_type) -> &Self {
                    let instance_id = self.get_id().clone();
                    self.history
                        .as_ref()
                        .unwrap()
                        .borrow_mut().undo.push(vec![rt::RHistoryItem::EditField(bt::HistoryFieldEdit {
                            instance_id: instance_id,
                            field_id: #field_id,
                            prev_value: self.data.#field_name.get().into_primitive_value(),
                            new_value: new_val.clone().into_primitive_value(), 
                        })]);
                    self.history.as_ref().unwrap().borrow_mut().redo.clear();
                    self.data.#field_name.set(new_val);
                    self
                }
            }
        }
    });

    let manipulate_slots_stream = all_slots.iter().map(|slot| {
        let slot_id = slot.slot.tag.id;
        let building_manipulate_slot_trait_name = proc_macro2::Ident::new(
            &format!("{}{}Slot", struct_name, slot.slot.tag.name),
            proc_macro2::Span::call_site(),
        );
        let editing_manipulate_slot_trait_name = proc_macro2::Ident::new(
            &format!("{}{}SlotExisting", struct_name, slot.slot.tag.name),
            proc_macro2::Span::call_site(),
        );

        let add_new_fn_name = proc_macro2::Ident::new(
            &format!("add_new_{}", slot.slot.tag.name.to_lowercase()),
            proc_macro2::Span::call_site(),
        );
        let add_existing_fn_name = proc_macro2::Ident::new(
            &format!(
                "add_existing_{}",
                slot.slot.tag.name.to_lowercase()
            ),
            proc_macro2::Span::call_site(),
        );
        let slot_marker_trait = proc_macro2::Ident::new(&format!("{}{}", struct_name, slot.slot.tag.name), proc_macro2::Span::call_site());

        let building_add_new_stream =            match &slot.slot.operative_descriptor {
            OperativeVariants::LibraryOperative(lib_op_id) => {
                let item_name = get_variant_name(&Box::new(
                    constraint_schema.operative_library.get(lib_op_id).unwrap(),
                ));
                quote! {
                    fn #add_new_fn_name(&mut self, new_item: rt::RInstantiableWrapper<rt::RGSOWrapper<#item_name, Schema>, Schema> ) -> &mut Self
                }
            }
            OperativeVariants::TraitOperative(trait_op) => {
                let trait_names = trait_op
                    .trait_ids
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
                    .collect::<Vec<_>>();
                let trait_names = trait_names.join(" + ");
                let trait_names = Ident::new(&trait_names, proc_macro2::Span::call_site());
                quote! {
                    fn #add_new_fn_name<T>(&mut self, new_item: rt::RInstantiableWrapper<rt::RGSOWrapper<T, Schema>, Schema>) -> &mut Self
                        where rt::RGSOWrapper<T, Schema>: #trait_names,
                          T: Clone + rt::RFieldEditable + std::fmt::Debug + rt::RIntoSchema<Schema=Schema> + #slot_marker_trait + 'static,
                }
            }
        };
        let get_editing_add_new_stream = |is_mut: TokenStream| {
            
            match &slot.slot.operative_descriptor {
                OperativeVariants::LibraryOperative(lib_op_id) => {
                    let item_name = get_variant_name(&Box::new(
                        constraint_schema.operative_library.get(lib_op_id).unwrap(),
                    ));
                    quote! {
                        fn #add_new_fn_name(&self, #is_mut new_item: rt::RInstantiableWrapper<rt::RGSOWrapper<#item_name, Schema>, Schema> ) -> rt::RInstantiableWrapper<rt::RGSOWrapper<#item_name, Schema>, Schema>
                    }
                }
                OperativeVariants::TraitOperative(trait_op) => {
                    let trait_names = trait_op
                        .trait_ids
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
                        .collect::<Vec<_>>();
                    let trait_names = trait_names.join(" + ");
                    let trait_names = Ident::new(&trait_names, proc_macro2::Span::call_site());
                    quote! {
                        fn #add_new_fn_name<T>(&self, #is_mut new_item: rt::RInstantiableWrapper<rt::RGSOWrapper<T, Schema>, Schema>) -> rt::RInstantiableWrapper<rt::RGSOWrapper<T, Schema>, Schema>
                            where rt::RGSOWrapper<T, Schema>: #trait_names,
                              T: Clone + rt::RFieldEditable + std::fmt::Debug + rt::RIntoSchema<Schema=Schema> + #slot_marker_trait + 'static,
                    }
                }
            }
        };
        let editing_add_new_stream_declaration = get_editing_add_new_stream(quote!{});
        let editing_add_new_stream_impl = get_editing_add_new_stream(quote!{mut });


        let integrable_stream = {
            let to_integrate_operatives = match &slot.slot.operative_descriptor {
                OperativeVariants::LibraryOperative(lib_op_id) => {
                    vec![constraint_schema.operative_library.get(lib_op_id).unwrap().clone()]
                },
                OperativeVariants::TraitOperative(trait_op) => {
                    constraint_schema.operative_library.values().filter(|filtering_op| {
                        let filter_op_trait_impls = filtering_op.get_trait_impl_digest(constraint_schema).trait_impls;//.map(|related_impl| {related_impl.trait_impl}).collect::<Vec<_>>();
                        for trait_id in trait_op.trait_ids.iter() {
                            if !filter_op_trait_impls.contains_key(trait_id) {
                                return false
                            };
                        };
                        true
                    }).cloned().collect::<Vec<_>>()
                },
            };

            let streams = to_integrate_operatives.iter().map(|operative| {
                let operative_name = get_variant_name(&Box::new(operative));
                quote!{
                    impl #slot_marker_trait for #operative_name {}
                }
            }).collect::<Vec<_>>();
            streams
        };

        quote! {
            trait #slot_marker_trait {}
            #(#integrable_stream)*
            pub trait #building_manipulate_slot_trait_name {
                #building_add_new_stream;
                fn #add_existing_fn_name(&mut self, existing_id: &base_types::common::Uid) -> &mut Self;
            }
            pub trait #editing_manipulate_slot_trait_name {
                #editing_add_new_stream_declaration;
                fn #add_existing_fn_name(&self, existing_id: &base_types::common::Uid) -> bt::ConnectionAction;
            }

            impl #building_manipulate_slot_trait_name for rt::RGSOBuilder<rt::RGSOWrapperBuilder<#struct_builder_name>, rt::RGSOWrapper<#struct_name, Schema>, Schema> {
                #building_add_new_stream {
                    rt::r_integrate_child(self, new_item, #slot_id);
                    self
                }
                fn #add_existing_fn_name(&mut self, existing_id: &base_types::common::Uid) -> &mut Self {
                    rt::r_integrate_child_id(self, existing_id, #slot_id);
                    self
                }
            }
            impl #editing_manipulate_slot_trait_name for rt::RGSOWrapper<#struct_name, Schema> {
                #editing_add_new_stream_impl {
                    let slot_ref = bt::SlotRef{
                        host_instance_id: self.get_id().clone(),
                        child_instance_id: new_item.get_instantiable_instance().get_id().clone(),
                        slot_id: #slot_id.clone(),
                    };
                    new_item.add_parent_slot(slot_ref.clone());
                    new_item.parent_updates.push((self.get_id().clone(),slot_ref));
                    new_item
                }
                fn #add_existing_fn_name(&self, existing_id: &base_types::common::Uid) -> bt::ConnectionAction {
                    let slot_ref = bt::SlotRef{
                        host_instance_id: self.get_id().clone(),
                        child_instance_id: *existing_id,
                        slot_id: #slot_id.clone(),
                    };
                    bt::ConnectionAction {
                        slot_ref: slot_ref,
                    }
                    
                }
                
            }
        }
    });

    let trait_impl_streams =
        generate_trait_impl_streams::generate_trait_impl_streams(&instantiable, constraint_schema);

    quote! {
        // const #operative_tag_handle:  base_types::common::Tag = base_types::common::Tag {name: #operative_tag_name, id: #operative_tag_id };
        #[derive(Clone, Debug, Default)]
        pub struct #struct_name {
            #(#unfulfilled_field_names: leptos::RwSignal<#unfulfilled_field_value_types>,)*
            #(#fulfilled_field_names: leptos::RwSignal<#fulfilled_field_value_types>,)*
        }

        impl rt::RFieldEditable for #struct_name {
            fn apply_field_edit(&self, field_edit: bt::FieldEdit) {
                match field_edit.field_id {
                    #(#unfulfilled_field_ids => {
                        match field_edit.value {
                            base_types::primitives::PrimitiveValues::#unfulfilled_field_value_types_enum(val) => {
                                self.#unfulfilled_field_names.set(val);
                            }
                            _ => panic!()
                        }
                    },)*
                    _ => panic!()
                }
            }
        }

        impl rt::RIntoSchema for #struct_name {
            type Schema = Schema;
            fn into_schema(instantiable: rt::RGSOWrapper<Self, Schema>) -> Self::Schema {
                Schema::#struct_name(instantiable.to_owned())
            }
        }

        impl rt::RBuildable for #struct_name {
            type Schema = Schema;
            type Builder = rt::RGSOWrapperBuilder<#struct_builder_name>;

            fn initiate_build() -> rt::RGSOBuilder<Self::Builder, rt::RGSOWrapper<Self, Schema>, Schema> {
                let template_ref = CONSTRAINT_SCHEMA.template_library.get(&#reference_template_id).unwrap();
                let operative_ref = CONSTRAINT_SCHEMA.operative_library.get(&#operative_id).unwrap();
                rt::RGSOBuilder::<Self::Builder, rt::RGSOWrapper<Self, Schema>, Schema>::new(
                        rt::RGSOWrapperBuilder::new(
                            #struct_builder_name::default(),
                            #active_slot_tokens,
                            // std::rc::Rc::new(#operative_tag),
                            // std::rc::Rc::new(#template_tag),
                            &operative_ref,
                            &template_ref,
                            ),
                    )
            }
            fn get_operative_id() -> base_types::common::Uid {
               #operative_id
            }
        }

        #[derive(validator::Validate, Default, Clone, Debug)]
        pub struct #struct_builder_name {
            #(#[validate(custom(function = "validate_signal_is_some"))] #unfulfilled_field_names: leptos::RwSignal<Option<#unfulfilled_field_value_types>>,)*
        }
        impl bt::Finalizable<#struct_name> for #struct_builder_name {
            fn finalize(&self) -> Result<#struct_name, anyhow::Error> {
                <Self as validator::Validate>::validate(self)?;
                Ok(<Self as bt::Producable<#struct_name>>::produce(self))
                // Ok(#struct_name {
                //     #(#unfulfilled_field_names: self.#unfulfilled_field_names.as_ref().unwrap().clone(),)*
                //     #(#fulfilled_field_names: #fulfilled_field_values,)*
                // })
            }
        }

        impl bt::Producable<#struct_name> for #struct_builder_name {
            fn produce(&self) -> #struct_name {
                #struct_name {
                    #(#unfulfilled_field_names: leptos::RwSignal::new(self.#unfulfilled_field_names.get().as_ref().unwrap().clone()),)*
                    #(#fulfilled_field_names: leptos::RwSignal::new(#fulfilled_field_values),)*
                }
            }
        }
        impl bt::Verifiable for #struct_builder_name {
            fn verify(&self) -> Result<(), anyhow::Error> {
                self.validate()?;
                Ok(())
            }
        }

        #(#manipulate_fields_stream)*
        #(#manipulate_slots_stream)*

        #trait_impl_streams
    }
}