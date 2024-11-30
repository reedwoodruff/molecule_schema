use std::collections::{BTreeSet, HashSet};

use crate::components::{
    common::*,
    slot_type_specialization_builder::SlotTypeSpecializationBuilder,
    slot_type_specialization_lineage::SlotTypeSpecializationLineage,
    utils::{
        get_deepest_downstream_specializations, restructure_slot_specialization_to_delete_input,
    },
    workspace::WorkspaceState,
};

use leptos::either::{Either, EitherOf3, EitherOf5, EitherOf7};
use schema_editor_generated_toolkit::prelude::*;

use super::{operative_slot_section::OperativeSlotContext, utils::get_all_descendent_operators};

#[component]
pub fn OperativeSlotTypeSpecializationSection() -> impl IntoView {
    let ctx = use_context::<SharedGraph<Schema>>().unwrap();
    let WorkspaceState {
        schema,
        selected_tab,
    }: WorkspaceState = use_context::<WorkspaceState>().unwrap();

    let OperativeSlotContext {
        max_downstream_slotted_instances,
        operative,
        template_slot,
        maybe_childest_type_spec,
        maybe_childest_cardinality_spec,
    } = use_context::<OperativeSlotContext>().unwrap();

    let ctx_clone = ctx.clone();
    let schema_clone = schema.clone();

    let operative_clone = operative.clone();
    let slot_clone = template_slot.clone();
    let operative_clone2 = operative_clone.clone();
    let operative_clone3 = operative_clone.clone();
    let operative_clone4 = operative_clone.clone();
    let slot = slot_clone.clone();
    let slot_clone = slot.clone();
    let operative_clone = operative_clone4.clone();

    let is_locally_owned_spec = Memo::new(move |_| {
        if let Some(type_specialization) = maybe_childest_type_spec.get() {
            match type_specialization {
                OperativeSlotTypeSpecializationTraitObject::OperativeSlotTypeSingleSpecialization(item) => item.get_specializedslot_slot().get_specializer_slot().get_id() == operative_clone.get_id(),
                OperativeSlotTypeSpecializationTraitObject::OperativeSlotTypeMultiSpecialization(item) => item.get_specializedslot_slot().get_specializer_slot().get_id() == operative_clone.get_id(),
                OperativeSlotTypeSpecializationTraitObject::OperativeSlotTypeTraitObjectSpecialization(item) => item.get_specializedslot_slot().get_specializer_slot().get_id() == operative_clone.get_id(),
            }
        } else {
            false
        }
    });
    let operative_clone = operative_clone4.clone();
    let exists_downstream_unique_type_spec = Memo::new(move |_| {
        let ds_specs = get_deepest_downstream_specializations(
            operative_clone.clone(),
            slot_clone.get_id(),
            false,
        );

        let maybe_this_op_and_slot_spec = operative_clone
            .get_slotspecializations_slot()
            .into_iter()
            .filter(|spec| spec.get_roottemplateslot_slot().get_id() == slot_clone.get_id())
            .next();
        if let Some(this_op_and_slot_spec) = maybe_this_op_and_slot_spec {
            let maybe_this_op_type_spec = this_op_and_slot_spec
                .get_typespecialization_slot()
                .into_iter()
                .next();
            ds_specs.into_iter().any(|ds_spec| {
                let is_unique_type = if let Some(ds_type) =
                    ds_spec.get_typespecialization_slot().into_iter().next()
                {
                    if let Some(this_op_type_spec) = maybe_this_op_type_spec.clone() {
                        this_op_type_spec.get_id() != ds_type.get_id()
                    } else {
                        true
                    }
                } else {
                    false
                };
                is_unique_type
                    && ds_spec.get_specializer_slot().get_id()
                        != this_op_and_slot_spec.get_specializer_slot().get_id()
            })
        } else {
            ds_specs
                .into_iter()
                .any(|ds_spec| !ds_spec.get_typespecialization_slot().is_empty())
        }
    });

    let slot_clone = template_slot.clone();
    let delete_type_view = move || {
        if !is_locally_owned_spec.get() {
            return Either::Right(view! {});
        }
        if let Some(type_specialization) = maybe_childest_type_spec.get() {
            let type_spec_clone = type_specialization.clone();
            let type_spec_clone_2 = type_spec_clone.clone();
            let operative_clone = operative_clone2.clone();
            let slot_clone = slot_clone.clone();
            let type_spec_clone_3 = type_spec_clone.clone();
            let ctx_clone = ctx_clone.clone();
            let operative = operative_clone.clone();
            let operative_clone = operative.clone();
            let slot_clone = slot_clone.clone();

            let on_delete_handler = move || {
                let mut editor = schema.get().edit(ctx_clone.clone());

                // Delete specialization node if obselete
                if let Some(spec_node) = operative_clone
                    .get_slotspecializations_slot()
                    .into_iter()
                    .filter(|slot_spec| {
                        slot_spec.get_roottemplateslot_slot().get_id() == slot_clone.get_id()
                    })
                    .next()
                {
                    let has_owned_slotted_instances = spec_node
                        .get_slottedinstances_slot()
                        .into_iter()
                        .any(|slotted_instance| {
                            slotted_instance.get_slottedslot_slot().get_id() == spec_node.get_id()
                        });
                    let has_owned_type_specs = spec_node
                        .get_typespecialization_slot()
                        .into_iter()
                        .any(|type_spec| {
                            let spec_node_matches = match &type_spec {
                                OperativeSlotTypeSpecializationTraitObject::OperativeSlotTypeSingleSpecialization(item) => {
                                    item.get_specializedslot_slot().get_id() == spec_node.get_id()
                                },
                                OperativeSlotTypeSpecializationTraitObject::OperativeSlotTypeMultiSpecialization(item) => {
                                    item.get_specializedslot_slot().get_id() == spec_node.get_id()
                                },
                                OperativeSlotTypeSpecializationTraitObject::OperativeSlotTypeTraitObjectSpecialization(item) => {
                                    item.get_specializedslot_slot().get_id() == spec_node.get_id()
                                },
                            };
                            spec_node_matches && type_spec.get_id() != type_spec_clone_3.get_id()
                        });
                    let has_owned_cardinality_specs = spec_node
                        .get_cardinalityspecialization_slot()
                        .into_iter()
                        .any(|card_spec| {
                            let spec_node_matches = match card_spec.clone() {
                                OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityLowerBoundOrZeroSpecialization(item) => {
                                    item.get_specializedslot_slot().get_id() == spec_node.get_id()
                                },
                                OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalitySingleSpecialization(item) => {
                                    item.get_specializedslot_slot().get_id() == spec_node.get_id()
                                },
                                OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityRangeSpecialization(item) => {
                                    item.get_specializedslot_slot().get_id() == spec_node.get_id()
                                },
                                OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityZeroSpecialization(item) => {
                                    item.get_specializedslot_slot().get_id() == spec_node.get_id()
                                },
                                OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityRangeOrZeroSpecialization(item) => {
                                    item.get_specializedslot_slot().get_id() == spec_node.get_id()
                                },
                                OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityLowerBoundSpecialization(item) => {
                                    item.get_specializedslot_slot().get_id() == spec_node.get_id()
                                },
                            };
                            spec_node_matches
                        });
                    let is_slot_specialization_node_obselete = !has_owned_slotted_instances
                        && !has_owned_type_specs
                        && !has_owned_cardinality_specs;
                    if is_slot_specialization_node_obselete {
                        restructure_slot_specialization_to_delete_input(
                            &mut editor,
                            ctx_clone.clone(),
                            spec_node,
                        );
                    }
                };

                match type_spec_clone_2.clone() {
                    OperativeSlotTypeSpecializationTraitObject::OperativeSlotTypeSingleSpecialization(item) => editor.incorporate(item.edit(ctx_clone.clone()).delete()),
                    OperativeSlotTypeSpecializationTraitObject::OperativeSlotTypeMultiSpecialization(item) => editor.incorporate(item.edit(ctx_clone.clone()).delete()),
                    OperativeSlotTypeSpecializationTraitObject::OperativeSlotTypeTraitObjectSpecialization(item) => editor.incorporate(item.edit(ctx_clone.clone()).delete()),
                };
                let upstream_item: (OperativeSlotTypeSpecializableTraitObjectDiscriminants, Uid) = match type_spec_clone_2.clone() {
                    OperativeSlotTypeSpecializationTraitObject::OperativeSlotTypeSingleSpecialization(item) => {
                        let upstream_item_id = item.get_upstreamtype_slot().get_id().clone();
                        (
                            match item.get_upstreamtype_slot() {
                                    OperativeSlotTypeSpecializableTraitObject::TemplateSlotTypeTraitOperative(_) => OperativeSlotTypeSpecializableTraitObjectDiscriminants::TemplateSlotTypeTraitOperative ,
                                    OperativeSlotTypeSpecializableTraitObject::OperativeSlotTypeMultiSpecialization(_) => OperativeSlotTypeSpecializableTraitObjectDiscriminants::OperativeSlotTypeMultiSpecialization ,
                                    OperativeSlotTypeSpecializableTraitObject::TemplateSlotTypeMultiOperative(_) => OperativeSlotTypeSpecializableTraitObjectDiscriminants::TemplateSlotTypeMultiOperative ,
                                    OperativeSlotTypeSpecializableTraitObject::OperativeSlotTypeSingleSpecialization(_) => OperativeSlotTypeSpecializableTraitObjectDiscriminants::OperativeSlotTypeSingleSpecialization ,
                                    OperativeSlotTypeSpecializableTraitObject::TemplateSlotTypeSingleOperative(_) => OperativeSlotTypeSpecializableTraitObjectDiscriminants::TemplateSlotTypeSingleOperative ,
                                    OperativeSlotTypeSpecializableTraitObject::OperativeSlotTypeTraitObjectSpecialization(_) => OperativeSlotTypeSpecializableTraitObjectDiscriminants::OperativeSlotTypeTraitObjectSpecialization ,
                            }
                            ,
                            upstream_item_id
                        )
                    },
                    OperativeSlotTypeSpecializationTraitObject::OperativeSlotTypeMultiSpecialization(item) => {
                        let upstream_item_id = item.get_upstreamtype_slot().get_id().clone();
                        (
                            match item.get_upstreamtype_slot() {
                                    OperativeSlotTypeSpecializableTraitObject::TemplateSlotTypeTraitOperative(_) => OperativeSlotTypeSpecializableTraitObjectDiscriminants::TemplateSlotTypeTraitOperative ,
                                    OperativeSlotTypeSpecializableTraitObject::OperativeSlotTypeMultiSpecialization(_) => OperativeSlotTypeSpecializableTraitObjectDiscriminants::OperativeSlotTypeMultiSpecialization ,
                                    OperativeSlotTypeSpecializableTraitObject::TemplateSlotTypeMultiOperative(_) => OperativeSlotTypeSpecializableTraitObjectDiscriminants::TemplateSlotTypeMultiOperative ,
                                    OperativeSlotTypeSpecializableTraitObject::OperativeSlotTypeSingleSpecialization(_) => OperativeSlotTypeSpecializableTraitObjectDiscriminants::OperativeSlotTypeSingleSpecialization ,
                                    OperativeSlotTypeSpecializableTraitObject::TemplateSlotTypeSingleOperative(_) => OperativeSlotTypeSpecializableTraitObjectDiscriminants::TemplateSlotTypeSingleOperative ,
                                    OperativeSlotTypeSpecializableTraitObject::OperativeSlotTypeTraitObjectSpecialization(_) => OperativeSlotTypeSpecializableTraitObjectDiscriminants::OperativeSlotTypeTraitObjectSpecialization ,
                            }
                            ,
                            upstream_item_id
                        )
                    },
                        OperativeSlotTypeSpecializationTraitObject::OperativeSlotTypeTraitObjectSpecialization(item) => {
                        let upstream_item_id = item.get_upstreamtype_slot().get_id().clone();
                        (
                            match item.get_upstreamtype_slot() {
                                OperativeSlotTypeSpecializableTraitOperativeTraitObject::TemplateSlotTypeTraitOperative(_) => OperativeSlotTypeSpecializableTraitObjectDiscriminants::TemplateSlotTypeTraitOperative ,
                                OperativeSlotTypeSpecializableTraitOperativeTraitObject::OperativeSlotTypeTraitObjectSpecialization(_) => OperativeSlotTypeSpecializableTraitObjectDiscriminants::OperativeSlotTypeTraitObjectSpecialization ,
                            }
                            ,
                            upstream_item_id
                        )
                    },
                };
                let mut all_descendent_operators_and_this_op = BTreeSet::new();
                all_descendent_operators_and_this_op.insert(operative_clone.clone());
                get_all_descendent_operators(
                    operative_clone.clone(),
                    &mut all_descendent_operators_and_this_op,
                );
                let settified_specs = all_descendent_operators_and_this_op
                    .into_iter()
                    .filter_map(|op| {
                        op.get_slotspecializations_slot()
                            .into_iter()
                            .filter(|spec| {
                                spec.get_roottemplateslot_slot().get_id() == slot_clone.get_id()
                            })
                            .next()
                    })
                    .into_iter()
                    .collect::<HashSet<_>>();
                settified_specs.into_iter().for_each(|spec| {
                    if let Some(existing_type_spec) =
                        spec.get_typespecialization_slot().into_iter().next()
                    {
                        editor.incorporate(
                            spec
                                .edit(ctx_clone.clone())
                                .remove_from_typespecialization(existing_type_spec.get_id()),
                        );
                    }
                    match upstream_item.0 {
                        OperativeSlotTypeSpecializableTraitObjectDiscriminants::TemplateSlotTypeTraitOperative => { },
                        OperativeSlotTypeSpecializableTraitObjectDiscriminants::OperativeSlotTypeMultiSpecialization => {
                            editor.incorporate(
                                &spec.edit(ctx_clone.clone())
                                    .add_existing_typespecialization::<OperativeSlotTypeMultiSpecialization>(
                                        &upstream_item.1,
                                        |na| na
                                    ),
                            );
                        },
                        OperativeSlotTypeSpecializableTraitObjectDiscriminants::TemplateSlotTypeMultiOperative => { },
                        OperativeSlotTypeSpecializableTraitObjectDiscriminants::OperativeSlotTypeSingleSpecialization => {
                            editor.incorporate(
                                &spec.edit(ctx_clone.clone())
                                    .add_existing_typespecialization::<OperativeSlotTypeSingleSpecialization>(
                                        &upstream_item.1,
                                        |na| na
                                    ),
                            );
                        },
                        OperativeSlotTypeSpecializableTraitObjectDiscriminants::TemplateSlotTypeSingleOperative => { },
                        OperativeSlotTypeSpecializableTraitObjectDiscriminants::OperativeSlotTypeTraitObjectSpecialization => {
                            editor.incorporate(
                                &spec.edit(ctx_clone.clone())
                                    .add_existing_typespecialization::<OperativeSlotTypeTraitObjectSpecialization>(
                                        &upstream_item.1,
                                        |na| na
                                    ),
                            );
                        },
                    }
                });
                editor.execute().unwrap();
            };
            Either::Left(view! {
                <LeafSection><Button on:click=move|_| {on_delete_handler()}>Delete Specialization</Button></LeafSection>
            })
        } else {
            Either::Right(view! {})
        }
    };
    let exists_downstream_view = move || {
        if exists_downstream_unique_type_spec.get() {
            Either::Left(view! {
                <LeafSection>
                <InfoNote>There exists a downstream specialization. Remove it to create a specialization here.</InfoNote>
                </LeafSection>
            })
        } else {
            Either::Right(view! {})
        }
    };

    let slot_clone = template_slot.clone();
    let operative_clone = operative.clone();
    let builder_view = move || {
        if is_locally_owned_spec.get() || exists_downstream_unique_type_spec.get() {
            return EitherOf3::C(view! {});
        }
        if let Some(specialization) = maybe_childest_type_spec.get() {
            let spec_clone = specialization.clone();
            EitherOf3::B(match spec_clone.clone() {
                OperativeSlotTypeSpecializationTraitObject::OperativeSlotTypeSingleSpecialization(single) => view! {
                    <SlotTypeSpecializationBuilder operative=operative_clone3.clone() spec_target=OperativeSlotTypeSpecializableTraitObject::OperativeSlotTypeSingleSpecialization(single) />
                }.into_any(),
                OperativeSlotTypeSpecializationTraitObject::OperativeSlotTypeMultiSpecialization(multi) => {
                    view!{<SlotTypeSpecializationBuilder operative=operative_clone3.clone() spec_target=OperativeSlotTypeSpecializableTraitObject::OperativeSlotTypeMultiSpecialization(multi) />}.into_any()
                },
                OperativeSlotTypeSpecializationTraitObject::OperativeSlotTypeTraitObjectSpecialization(trait_obj) =>
                view!{<SlotTypeSpecializationBuilder operative=operative_clone3.clone() spec_target=OperativeSlotTypeSpecializableTraitObject::OperativeSlotTypeTraitObjectSpecialization(trait_obj) />}.into_any() ,
            })
        } else {
            EitherOf3::A(match slot_clone.get_templateslotvariant_slot() {
                TemplateSlotTypeVariantTraitObject::TemplateSlotTypeTraitOperative(trait_op) => {
                    view! {
                        <SlotTypeSpecializationBuilder operative=operative_clone.clone() spec_target=OperativeSlotTypeSpecializableTraitObject::TemplateSlotTypeTraitOperative(trait_op) />
                    }.into_any()
                }
                TemplateSlotTypeVariantTraitObject::TemplateSlotTypeSingleOperative(single) => {
                    view!{
                        <SlotTypeSpecializationBuilder operative=operative_clone3.clone() spec_target=OperativeSlotTypeSpecializableTraitObject::TemplateSlotTypeSingleOperative(single) />
                    }.into_any()
                }
                TemplateSlotTypeVariantTraitObject::TemplateSlotTypeMultiOperative(multi) => {
                    view! {
                        <SlotTypeSpecializationBuilder operative=operative_clone.clone() spec_target=OperativeSlotTypeSpecializableTraitObject::TemplateSlotTypeMultiOperative(multi) />
                    }.into_any()
                }
            })
        }
    };

    let lineage_view = move || {
        if let Some(specialization) = maybe_childest_type_spec.get() {
            Either::Left(view! {<LeafSection attr:class="leafsection dependent">
                <SlotTypeSpecializationLineage specialization=specialization is_entry_point=true/>
                </LeafSection>
            })
        } else {
            Either::Right(view! {})
        }
    };
    let operative_clone = operative_clone4.clone();
    let slot_clone = slot.clone();

    view! {
        {lineage_view}
        {exists_downstream_view}
        {delete_type_view}
        {builder_view}
    }
}
