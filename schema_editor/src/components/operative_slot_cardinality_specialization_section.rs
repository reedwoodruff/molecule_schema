use std::collections::{BTreeSet, HashSet};

use crate::components::utils::restructure_slot_specialization_to_delete_input;
use crate::components::{common::*, workspace::WorkspaceState};

use leptos::either::Either;
use schema_editor_generated_toolkit::prelude::*;

use super::operative_slot_section::OperativeSlotContext;
use super::slot_cardinality_specialization_builder::SlotCardinalitySpecializationBuilder;

use super::slot_cardinality_specialization_lineage::SlotCardinalitySpecializationLineage;
use super::utils::{get_all_descendent_operators, get_deepest_downstream_specializations};

const DOWNSTREAM_NOTICE: &str =
    "There exists a downstream specialization. Remove it to create a specialization here.";
#[component]
pub fn OperativeSlotCardinalitySpecializationSection() -> impl IntoView {
    let ctx = use_context::<SharedGraph<Schema>>().unwrap();
    let WorkspaceState { schema, .. }: WorkspaceState = use_context::<WorkspaceState>().unwrap();
    let OperativeSlotContext {
        operative,
        template_slot,
        maybe_childest_cardinality_spec,
        ..
    } = use_context::<OperativeSlotContext>().unwrap();

    let ctx_clone = ctx.clone();

    let operative_clone = operative.clone();
    let operative_clone2 = operative_clone.clone();
    let operative_clone4 = operative_clone.clone();
    let slot_clone = template_slot.clone();
    let operative_clone = operative_clone4.clone();

    let is_locally_owned_spec = Memo::new(move |_| {
        if let Some(cardinality_specialization) = maybe_childest_cardinality_spec.get() {
            match cardinality_specialization {
                OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityLowerBoundOrZeroSpecialization(item) => item.get_specializedslot_slot().get_specializer_slot().get_id() == operative_clone2.get_id(),
                OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalitySingleSpecialization(item) => item.get_specializedslot_slot().get_specializer_slot().get_id() == operative_clone2.get_id(),
                OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityRangeSpecialization(item) => item.get_specializedslot_slot().get_specializer_slot().get_id() == operative_clone2.get_id(),
                OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityZeroSpecialization(item) => item.get_specializedslot_slot().get_specializer_slot().get_id() == operative_clone2.get_id(),
                OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityRangeOrZeroSpecialization(item) => item.get_specializedslot_slot().get_specializer_slot().get_id() == operative_clone2.get_id(),
                OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityLowerBoundSpecialization(item) => item.get_specializedslot_slot().get_specializer_slot().get_id() == operative_clone2.get_id(),
            }
        } else {
            false
        }
    });

    let exists_downstream_unique_cardinality_spec = Memo::new(move |_| {
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
            let maybe_this_op_card_spec = this_op_and_slot_spec
                .get_cardinalityspecialization_slot()
                .into_iter()
                .next();
            ds_specs.into_iter().any(|ds_spec| {
                let is_unique_cardinality = if let Some(ds_card) = ds_spec
                    .get_cardinalityspecialization_slot()
                    .into_iter()
                    .next()
                {
                    if let Some(this_op_card_spec) = maybe_this_op_card_spec.clone() {
                        this_op_card_spec.get_id() != ds_card.get_id()
                    } else {
                        true
                    }
                } else {
                    false
                };
                is_unique_cardinality
                    && ds_spec.get_specializer_slot().get_id()
                        != this_op_and_slot_spec.get_specializer_slot().get_id()
            })
        } else {
            ds_specs
                .into_iter()
                .any(|ds_spec| !ds_spec.get_cardinalityspecialization_slot().is_empty())
        }
    });

    let slot_clone = template_slot.clone();

    let builder_view = move || {
        if is_locally_owned_spec.get() || exists_downstream_unique_cardinality_spec.get() {
            return view! {}.into_any();
        }
        if let Some(cardinality_specialization) = maybe_childest_cardinality_spec.get() {
            match cardinality_specialization.clone() {
                OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityLowerBoundOrZeroSpecialization(item) => {
                        view! {
                            // operative=operative_clone3.clone()
                            <SlotCardinalitySpecializationBuilder spec_target=OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundOrZeroSpecialization(
                                item,
                            ) />
                        }.into_any()
                },
                OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalitySingleSpecialization(_item) => {
                        view! {
                            <LeafSection>
                                <InfoNote>Cannot be specialized further</InfoNote>
                            </LeafSection>
                        }.into_any()
                },
                OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityRangeSpecialization(item) => {
                        view! {
                            // operative=operative_clone3.clone()
                            <SlotCardinalitySpecializationBuilder spec_target=OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityRangeSpecialization(
                                item,
                            ) />
                        }.into_any()
                },
                OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityZeroSpecialization(_item) => {
                        view! {
                            <LeafSection>
                                <InfoNote>Cannot be specialized further</InfoNote>
                            </LeafSection>
                        }.into_any()
                },
                OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityRangeOrZeroSpecialization(item) => {
                        view! {
                            // operative=operative_clone3.clone()
                            <SlotCardinalitySpecializationBuilder spec_target=OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityRangeOrZeroSpecialization(
                                item,
                            ) />
                        }.into_any()
                },
                OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityLowerBoundSpecialization(item) => {
                        view! {
                            // operative=operative_clone3.clone()
                            <SlotCardinalitySpecializationBuilder spec_target=OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundSpecialization(
                                item,
                            ) />
                        }.into_any()
                },
            }
        } else {
            match slot_clone.get_slotcardinality_slot() {
                TemplateSlotCardinalityVariantTraitObject::TemplateSlotCardinalityRangeOrZero(item) =>  {
                    view! {
                        // operative=operative_clone3.clone()
                        <SlotCardinalitySpecializationBuilder spec_target=OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityRangeOrZero(
                            item,
                        ) />
                    }.into_any()
                }
                TemplateSlotCardinalityVariantTraitObject::TemplateSlotCardinalityLowerBoundOrZero(item) =>  {
                    view! {
                        // operative=operative_clone3.clone()
                        <SlotCardinalitySpecializationBuilder spec_target=OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityLowerBoundOrZero(
                            item,
                        ) />
                    }.into_any()
                }
                TemplateSlotCardinalityVariantTraitObject::TemplateSlotCardinalityRange(item) =>  {
                    view! {
                        // operative=operative_clone3.clone()
                        <SlotCardinalitySpecializationBuilder spec_target=OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityRange(
                            item,
                        ) />
                    }.into_any()
                }
                TemplateSlotCardinalityVariantTraitObject::TemplateSlotCardinalityLowerBound(item) =>  {
                    view! {
                        // operative=operative_clone3.clone()
                        <SlotCardinalitySpecializationBuilder spec_target=OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityLowerBound(
                            item,
                        ) />
                    }.into_any()
                }
                TemplateSlotCardinalityVariantTraitObject::TemplateSlotCardinalitySingle(_) => {
                    view! {
                        <LeafSection>
                            <InfoNote>Cannot be specialized further</InfoNote>
                        </LeafSection>
                    }.into_any()
                } ,
            }
        }
    };

    let delete_cardinality_view = move || {
        if let Some(cardinality_specialization) = maybe_childest_cardinality_spec.get() {
            if !is_locally_owned_spec.get() {
                return Either::Right(view! {});
            }

            let card_spec_clone = cardinality_specialization.clone();
            let operative_clone = operative.clone();
            let slot_clone = template_slot.clone();
            let card_spec_clone_2 = card_spec_clone.clone();
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
                            let spec_node_matches = match type_spec {
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
                            spec_node_matches
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
                            spec_node_matches && card_spec.get_id() != card_spec_clone_2.get_id()
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

                match card_spec_clone_2.clone() {
                    OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityLowerBoundOrZeroSpecialization(item) => {
                        editor.incorporate(item.edit(ctx_clone.clone()).delete());
                    },
                    OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalitySingleSpecialization(item) => {
                        editor.incorporate(item.edit(ctx_clone.clone()).delete());
                    },
                    OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityRangeSpecialization(item) => {
                        editor.incorporate(item.edit(ctx_clone.clone()).delete());
                    },
                    OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityZeroSpecialization(item) => {
                        editor.incorporate(item.edit(ctx_clone.clone()).delete());
                    },
                    OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityRangeOrZeroSpecialization(item) => {
                        editor.incorporate(item.edit(ctx_clone.clone()).delete());
                    },
                    OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityLowerBoundSpecialization(item) => {
                        editor.incorporate(item.edit(ctx_clone.clone()).delete());
                    },
                };
                let upstream_item: (OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObjectDiscriminants, Uid) = match card_spec_clone_2.clone() {
            OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityZeroSpecialization(item) => {
                let upstream_item = item.get_upstreamcardinality_slot().get_id().clone();
                (match item.get_upstreamcardinality_slot() {
                    OperativeSlotCardinalitySpecializableByZeroTraitObject::OperativeSlotCardinalityLowerBoundOrZeroSpecialization(_) => OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObjectDiscriminants::OperativeSlotCardinalityLowerBoundOrZeroSpecialization,
                    OperativeSlotCardinalitySpecializableByZeroTraitObject::TemplateSlotCardinalityRangeOrZero(_) => OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObjectDiscriminants::TemplateSlotCardinalityRangeOrZero,
                    OperativeSlotCardinalitySpecializableByZeroTraitObject::OperativeSlotCardinalityRangeSpecialization(_) => OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObjectDiscriminants::OperativeSlotCardinalityRangeSpecialization,
                    OperativeSlotCardinalitySpecializableByZeroTraitObject::TemplateSlotCardinalityLowerBoundOrZero(_) => OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObjectDiscriminants::TemplateSlotCardinalityLowerBoundOrZero,
                    OperativeSlotCardinalitySpecializableByZeroTraitObject::TemplateSlotCardinalityRange(_) => OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObjectDiscriminants::TemplateSlotCardinalityRange,
                    OperativeSlotCardinalitySpecializableByZeroTraitObject::TemplateSlotCardinalityLowerBound(_) => OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObjectDiscriminants::TemplateSlotCardinalityLowerBound,
                    OperativeSlotCardinalitySpecializableByZeroTraitObject::OperativeSlotCardinalityRangeOrZeroSpecialization(_) => OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObjectDiscriminants::OperativeSlotCardinalityRangeOrZeroSpecialization,
                    OperativeSlotCardinalitySpecializableByZeroTraitObject::OperativeSlotCardinalityLowerBoundSpecialization(_) => OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObjectDiscriminants::OperativeSlotCardinalityLowerBoundSpecialization,
                }, upstream_item)
            }
            OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalitySingleSpecialization(item) => {
                let upstream_item = item.get_upstreamcardinality_slot().get_id().clone();
                (match item.get_upstreamcardinality_slot() {
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundOrZeroSpecialization(_) => OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObjectDiscriminants::OperativeSlotCardinalityLowerBoundOrZeroSpecialization,
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityRangeOrZero(_) => OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObjectDiscriminants::TemplateSlotCardinalityRangeOrZero,
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityRangeSpecialization(_) => OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObjectDiscriminants::OperativeSlotCardinalityRangeSpecialization,
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityLowerBoundOrZero(_) => OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObjectDiscriminants::TemplateSlotCardinalityLowerBoundOrZero,
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityRange(_) => OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObjectDiscriminants::TemplateSlotCardinalityRange,
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityLowerBound(_) => OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObjectDiscriminants::TemplateSlotCardinalityLowerBound,
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityRangeOrZeroSpecialization(_) => OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObjectDiscriminants::OperativeSlotCardinalityRangeOrZeroSpecialization,
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundSpecialization(_) => OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObjectDiscriminants::OperativeSlotCardinalityLowerBoundSpecialization,
                }, upstream_item)
            }
            OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityRangeSpecialization(item) => {
                let upstream_item = item.get_upstreamcardinality_slot().get_id().clone();
                (match item.get_upstreamcardinality_slot(){
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundOrZeroSpecialization(_) => OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObjectDiscriminants::OperativeSlotCardinalityLowerBoundOrZeroSpecialization,
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityRangeOrZero(_) => OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObjectDiscriminants::TemplateSlotCardinalityRangeOrZero,
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityRangeSpecialization(_) => OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObjectDiscriminants::OperativeSlotCardinalityRangeSpecialization,
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityLowerBoundOrZero(_) => OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObjectDiscriminants::TemplateSlotCardinalityLowerBoundOrZero,
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityRange(_) => OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObjectDiscriminants::TemplateSlotCardinalityRange,
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityLowerBound(_) => OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObjectDiscriminants::TemplateSlotCardinalityLowerBound,
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityRangeOrZeroSpecialization(_) => OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObjectDiscriminants::OperativeSlotCardinalityRangeOrZeroSpecialization,
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundSpecialization(_) => OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObjectDiscriminants::OperativeSlotCardinalityLowerBoundSpecialization,
                }, upstream_item)
            }
            OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityRangeOrZeroSpecialization(item) => {
                let upstream_item = item.get_upstreamcardinality_slot().get_id().clone();
                (match item.get_upstreamcardinality_slot(){
                    OperativeSlotCardinalitySpecializableByRangeOrZeroTraitObject::OperativeSlotCardinalityLowerBoundOrZeroSpecialization(_) => OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObjectDiscriminants::OperativeSlotCardinalityLowerBoundOrZeroSpecialization,
                    OperativeSlotCardinalitySpecializableByRangeOrZeroTraitObject::TemplateSlotCardinalityRangeOrZero(_) => OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObjectDiscriminants::TemplateSlotCardinalityRangeOrZero,
                    OperativeSlotCardinalitySpecializableByRangeOrZeroTraitObject::TemplateSlotCardinalityLowerBoundOrZero(_) => OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObjectDiscriminants::TemplateSlotCardinalityLowerBoundOrZero,
                    OperativeSlotCardinalitySpecializableByRangeOrZeroTraitObject::OperativeSlotCardinalityRangeOrZeroSpecialization(_) => OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObjectDiscriminants::OperativeSlotCardinalityRangeOrZeroSpecialization,
                }, upstream_item)
            }
            OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityLowerBoundSpecialization(item) => {
                let upstream_item = item.get_upstreamcardinality_slot().get_id().clone();
                (match item.get_upstreamcardinality_slot(){
                    OperativeSlotCardinalitySpecializableByLowerBoundTraitObject::OperativeSlotCardinalityLowerBoundOrZeroSpecialization(_) => OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObjectDiscriminants::OperativeSlotCardinalityLowerBoundOrZeroSpecialization,
                    OperativeSlotCardinalitySpecializableByLowerBoundTraitObject::TemplateSlotCardinalityLowerBoundOrZero(_) => OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObjectDiscriminants::TemplateSlotCardinalityLowerBoundOrZero,
                    OperativeSlotCardinalitySpecializableByLowerBoundTraitObject::TemplateSlotCardinalityLowerBound(_) => OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObjectDiscriminants::TemplateSlotCardinalityLowerBound,
                    OperativeSlotCardinalitySpecializableByLowerBoundTraitObject::OperativeSlotCardinalityLowerBoundSpecialization(_) => OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObjectDiscriminants::OperativeSlotCardinalityLowerBoundSpecialization,
                }, upstream_item)
            }
            OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityLowerBoundOrZeroSpecialization(item) => {
                let upstream_item = item.get_upstreamcardinality_slot().get_id().clone();
                (match item.get_upstreamcardinality_slot() {
                    OperativeSlotCardinalitySpecializableByLowerBoundOrZeroTraitObject::OperativeSlotCardinalityLowerBoundOrZeroSpecialization(_) => OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObjectDiscriminants::OperativeSlotCardinalityLowerBoundOrZeroSpecialization,
                    OperativeSlotCardinalitySpecializableByLowerBoundOrZeroTraitObject::TemplateSlotCardinalityLowerBoundOrZero(_) => OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObjectDiscriminants::TemplateSlotCardinalityLowerBoundOrZero,
                }, upstream_item)
            }
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
                leptos::logging::log!("{:#?}", settified_specs);
                settified_specs.into_iter().for_each(|spec| {
                if let Some(existing_card_spec) =
                    spec.get_cardinalityspecialization_slot().into_iter().next()
                {
                    editor.incorporate(
                        spec
                            .edit(ctx_clone.clone())
                            .remove_from_cardinalityspecialization(existing_card_spec.get_id()),
                    );
                }
                match upstream_item.0 {
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObjectDiscriminants::OperativeSlotCardinalityLowerBoundOrZeroSpecialization =>
                    {
                        editor.incorporate(
                            &spec.edit(ctx_clone.clone())
                                .add_existing_cardinalityspecialization::<OperativeSlotCardinalityLowerBoundOrZeroSpecialization>(
                                    &upstream_item.1,
                                    |na| na
                                ),
                        );
                    } ,
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObjectDiscriminants::TemplateSlotCardinalityRangeOrZero => { } ,
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObjectDiscriminants::OperativeSlotCardinalityRangeSpecialization =>
                    { editor.incorporate(
                            &spec.edit(ctx_clone.clone())
                                .add_existing_cardinalityspecialization::<OperativeSlotCardinalityRangeSpecialization>(
                                    &upstream_item.1,
                                    |na| na
                                ),
                        ); } ,
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObjectDiscriminants::TemplateSlotCardinalityLowerBoundOrZero => { } ,
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObjectDiscriminants::TemplateSlotCardinalityRange => { } ,
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObjectDiscriminants::TemplateSlotCardinalityLowerBound => { } ,
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObjectDiscriminants::OperativeSlotCardinalityRangeOrZeroSpecialization =>
                    { editor.incorporate(
                            &spec.edit(ctx_clone.clone())
                                .add_existing_cardinalityspecialization::<OperativeSlotCardinalityRangeOrZeroSpecialization>(
                                    &upstream_item.1,
                                    |na| na
                                ),
                        ); } ,
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObjectDiscriminants::OperativeSlotCardinalityLowerBoundSpecialization =>
                    { editor.incorporate(
                            &spec.edit(ctx_clone.clone())
                                .add_existing_cardinalityspecialization::<OperativeSlotCardinalityLowerBoundSpecialization>(
                                    &upstream_item.1,
                                    |na| na
                                ),
                        ) } ,
                    }
                });
                editor.execute().unwrap();
            };
            Either::Left(view! {
                <LeafSection>
                    <Button on:click=move |_| { on_delete_handler() }>Delete Specialization</Button>
                </LeafSection>
            })
        } else {
            Either::Right(view! {})
        }
    };
    let exists_downstream_view = move || {
        if exists_downstream_unique_cardinality_spec.get() {
            Either::Left(view! {
                <LeafSection>
                    <InfoNote>{DOWNSTREAM_NOTICE}</InfoNote>
                </LeafSection>
            })
        } else {
            Either::Right(view! {})
        }
    };
    let lineage_view = move || {
        if let Some(cardinality_specialization) = maybe_childest_cardinality_spec.get() {
            Either::Left(view! {
                <LeafSection attr:class="leafsection dependent">
                    <SlotCardinalitySpecializationLineage specialization=cardinality_specialization />
                </LeafSection>
            })
        } else {
            Either::Right(view! {})
        }
    };

    view! {
        {lineage_view}
        {exists_downstream_view}
        {delete_cardinality_view}
        {builder_view}
    }
}
