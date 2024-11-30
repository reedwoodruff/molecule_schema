use std::collections::BTreeSet;

use crate::components::{
    common::*,
    operative_slot_cardinality_specialization_section::OperativeSlotCardinalitySpecializationSection,
    operative_slot_type_specialization_section::OperativeSlotTypeSpecializationSection,
    slot_cardinality_specialization_builder::CardinalityInfo,
    utils::{
        get_all_descendent_instances, get_all_descendent_instances_including_own,
        get_all_descendent_operators, get_all_instances_which_impl_trait_set,
        get_all_instances_which_satisfy_specialization, get_childest_cardinality_info_downstream,
        get_childest_cardinality_specialization_for_op_and_slot,
        get_childest_type_specialization_for_op_and_slot,
    },
    workspace::WorkspaceState,
};

use leptos::{
    context::Provider,
    either::{Either, EitherOf3, EitherOf4, EitherOf5, EitherOf6},
};
use schema_editor_generated_toolkit::prelude::*;

use super::utils::get_deepest_downstream_specializations;

#[derive(Clone)]
pub struct OperativeSlotContext {
    pub max_downstream_slotted_instances: Signal<u32>,
    pub operative: RGSOConcrete<OperativeConcrete, Schema>,
    pub template_slot: RGSOConcrete<TemplateSlot, Schema>,
    pub maybe_childest_type_spec: Memo<Option<OperativeSlotTypeSpecializationTraitObject>>,
    pub maybe_childest_cardinality_spec:
        Memo<Option<OperativeSlotCardinalitySpecializationTraitObject>>,
}

#[component]
pub fn OperativeSlotSection(
    operative: RGSOConcrete<OperativeConcrete, Schema>,
    slot_item: RGSOConcrete<TemplateSlot, Schema>,
    master_collapser: RwSignal<bool>,
) -> impl IntoView {
    let ctx = use_context::<SharedGraph<Schema>>().unwrap();
    let WorkspaceState {
        schema,
        selected_tab,
    }: WorkspaceState = use_context::<WorkspaceState>().unwrap();

    let ctx_clone = ctx.clone();
    let schema_clone = schema.clone();
    let selected_tab = selected_tab.clone();
    let operative_clone = operative.clone();
    let schema = schema_clone.clone();
    let operative = operative_clone.clone();
    let slot_clone = slot_item.clone();
    let slot_variant = move || match slot_clone.get_templateslotvariant_slot() {
        TemplateSlotTypeVariantTraitObject::TemplateSlotTypeTraitOperative(trait_op) => {
            let traits_string = move || {
                trait_op
                    .get_allowedtraits_slot()
                    .into_iter()
                    .map(|trait_conc| trait_conc.get_name())
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            let view = move || format!("Trait-Bound Slot: [{}]", traits_string());
            EitherOf3::A(view)
        }
        TemplateSlotTypeVariantTraitObject::TemplateSlotTypeSingleOperative(single_op) => {
            let view = move || {
                format!(
                    "Single Operative Slot: {}",
                    single_op.get_allowedoperative_slot().get_name()
                )
            };
            EitherOf3::B(view)
        }
        TemplateSlotTypeVariantTraitObject::TemplateSlotTypeMultiOperative(multi_op) => {
            let view = move || {
                format!(
                    "Multiple Operative Slot: [{}]",
                    multi_op
                        .get_allowedoperatives_slot()
                        .into_iter()
                        .map(|op| op.get_name())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            };
            EitherOf3::C(view)
        }
    };
    let operative_clone = operative.clone();
    let slot_clone = slot_item.clone();
    let maybe_childest_specialization = Memo::new(move |_| {
        let operative_clone = operative_clone.clone();
        let slot_clone = slot_clone.clone();
        operative_clone
            .get_slotspecializations_slot()
            .into_iter()
            .find(|slot_spec| slot_spec.get_roottemplateslot_slot().get_id() == slot_clone.get_id())
    });
    let operative_clone = operative.clone();
    let slot_clone = slot_item.clone();
    let maybe_childest_cardinality_spec = Memo::new(move |_| {
        let operative_clone = operative_clone.clone();
        let slot_clone = slot_clone.clone();
        // For some reason you have to call this in the closure to get the correct reactive tracking.
        operative_clone.get_slotspecializations_slot();
        maybe_childest_specialization
            .get()
            .map(|spec| spec.get_cardinalityspecialization_slot().into_iter().next())
            .flatten()
        // // For some reason you have to call this in the closure to get the correct reactive tracking.
        // operative_clone.get_slottypespecializations_slot();
        // get_childest_cardinality_specialization_for_op_and_slot(operative_clone, slot_clone)
    });

    let slot_clone = slot_item.clone();
    let local_cardinality_info = Memo::new(move |_| {
        if let Some(spec) = maybe_childest_cardinality_spec.get() {
            CardinalityInfo::from_card_spec(spec)
        } else {
            match slot_clone.get_slotcardinality_slot() {
                TemplateSlotCardinalityVariantTraitObject::TemplateSlotCardinalityRangeOrZero(item) => {
                    CardinalityInfo {min: item.get_lower_bound_field(), max: Some(item.get_upper_bound_field()), zero_allowed: true}
                },
                TemplateSlotCardinalityVariantTraitObject::TemplateSlotCardinalityLowerBoundOrZero(item) => {
                    CardinalityInfo {min: item.get_lower_bound_field(), max: None, zero_allowed: true}
                },
                TemplateSlotCardinalityVariantTraitObject::TemplateSlotCardinalityRange(item) => {
                    CardinalityInfo {min: item.get_lower_bound_field(), max: Some(item.get_upper_bound_field()), zero_allowed: false}
                },
                TemplateSlotCardinalityVariantTraitObject::TemplateSlotCardinalityLowerBound(item) => {
                    CardinalityInfo {min: item.get_lower_bound_field(), max: None, zero_allowed: false}
                },
                TemplateSlotCardinalityVariantTraitObject::TemplateSlotCardinalitySingle(item) => {
                    CardinalityInfo {min: 1, max: Some(1), zero_allowed: false}
                },
            }
        }
    });

    let operative_clone = operative.clone();
    let slot_clone = slot_item.clone();
    let slotted_instances_for_slot = Memo::new(move |_| {
        maybe_childest_specialization
            .get()
            .map(|spec| spec.get_slottedinstances_slot())
            .unwrap_or(vec![])
        // operative_clone
        //     .get_slottedinstances_slot()
        //     .into_iter()
        //     .filter(|slotted_inst| {
        //         slotted_inst.get_slottedslot_slot().get_id() == slot_clone.get_id()
        //     })
        //     .collect::<Vec<_>>()
    });
    let operative_clone = operative.clone();
    let upstream_and_local_slotted_number = move || slotted_instances_for_slot.get().len() as u32;
    let upstream_and_local_slotted_number_clone = upstream_and_local_slotted_number.clone();
    let operative_clone = operative.clone();
    let slot_clone = slot_item.clone();
    let max_downstream_slotted_number = Signal::derive(move || {
        let mut max_slotted = 0;
        let deepest_specs = get_deepest_downstream_specializations(
            operative_clone.clone(),
            slot_clone.get_id(),
            false,
        );
        deepest_specs.into_iter().for_each(|spec| {
            max_slotted = max_slotted.max(spec.get_slottedinstances_slot().len());
        });
        max_slotted as u32
    });
    let downstream_slotted_number_clone = max_downstream_slotted_number.clone();
    let slot_clone = slot_item.clone();
    let is_fulfilled = Memo::new(move |_| {
        upstream_and_local_slotted_number_clone() > local_cardinality_info.get().min
            || (upstream_and_local_slotted_number_clone() == 0
                && local_cardinality_info.get().zero_allowed == true)
    });
    let is_maxed_independently = Memo::new(move |_| {
        local_cardinality_info
            .get()
            .max
            .is_some_and(|max| max == upstream_and_local_slotted_number())
    });
    let operative_clone = operative.clone();
    let is_maxed_considering_children = Memo::new(move |_| {
        local_cardinality_info
            .get()
            .max
            .is_some_and(|max| max == max_downstream_slotted_number.get())
    });
    let operative_clone = operative.clone();
    let slot_clone = slot_item.clone();
    let is_allowed_to_add_another_instance = Memo::<bool>::new(move |_| {
        if is_maxed_considering_children.get() {
            return false;
        }
        let deepest_specs = get_deepest_downstream_specializations(
            operative_clone.clone(),
            slot_clone.get_id(),
            false,
        );
        let deepest_card_spec = deepest_specs.into_iter().filter_map(|spec| {
            if let Some(card_spec) = spec.get_cardinalityspecialization_slot().into_iter().next() {
                Some(CardinalityInfo::from_card_spec(card_spec))
            } else {
                None
            }
        });
        !deepest_card_spec.into_iter().any(|deep_card_spec| {
            deep_card_spec
                .max
                .is_some_and(|max| max_downstream_slotted_number.get() == max)
        })
    });

    let operative_clone = operative.clone();
    let slot_clone = slot_item.clone();
    let slot_bound_view = move || {
        let cur_slot_num = upstream_and_local_slotted_number_clone.clone()();
        let cur_downstream_slot_num = max_downstream_slotted_number.clone().get();
        match slot_clone.get_slotcardinality_slot() {
            TemplateSlotCardinalityVariantTraitObject::TemplateSlotCardinalityRangeOrZero(
                inner,
            ) => EitherOf5::B(move || {
                format!(
                    "Lower Bound: {}, Upper Bound: {}",
                    inner.get_lower_bound_field(),
                    inner.get_upper_bound_field()
                )
            }),
            TemplateSlotCardinalityVariantTraitObject::TemplateSlotCardinalityLowerBoundOrZero(
                inner,
            ) => EitherOf5::C(move || format!("Lower Bound: {}", inner.get_lower_bound_field(),)),
            TemplateSlotCardinalityVariantTraitObject::TemplateSlotCardinalityRange(inner) => {
                EitherOf5::D(move || {
                    format!(
                        "Lower Bound: {}, Upper Bound: {}",
                        inner.get_lower_bound_field(),
                        inner.get_upper_bound_field()
                    )
                })
            }
            TemplateSlotCardinalityVariantTraitObject::TemplateSlotCardinalityLowerBound(inner) => {
                EitherOf5::E(move || format!("Lower Bound: {}", inner.get_lower_bound_field(),))
            }
            TemplateSlotCardinalityVariantTraitObject::TemplateSlotCardinalitySingle(inner) => {
                EitherOf5::A(move || "Exactly 1")
            }
        }
    };

    let operative_clone = operative.clone();
    let ctx_clone = ctx.clone();
    let currently_slotted_view = move |instance: RGSOConcrete<SlottedInstance, Schema>| {
        let ctx_clone = ctx_clone.clone();
        let instance_clone = instance.clone();
        let operative_clone = operative_clone.clone();
        let is_owned_by_this_op =
            move || instance_clone.get_fulfiller_slot().get_id() == operative_clone.get_id();
        let instance_clone = instance.clone();
        move || {
            let ctx_clone = ctx_clone.clone();
            let instance_clone = instance_clone.clone();
            if is_owned_by_this_op() {
                let instance_clone_2 = instance_clone.clone();
                let on_click_remove = move |_| {
                    instance_clone_2
                        .edit(ctx_clone.clone())
                        .delete()
                        .execute()
                        .unwrap();
                };
                let instance_clone_3 = instance_clone.clone();
                Either::Left(
                    view! {<LeafSection>{move || instance_clone_3.get_instance_slot().get_name()} <Button on:click=on_click_remove>Remove</Button></LeafSection>},
                )
            } else {
                Either::Right(
                    view! {<LeafSection>{move || instance_clone.get_instance_slot().get_name()} (Slotted Upstream)</LeafSection>},
                )
            }
        }
    };
    let operative_clone = operative.clone();
    let slot_clone = slot_item.clone();
    let maybe_childest_type_spec = Memo::new(move |_| {
        let operative_clone = operative_clone.clone();
        let slot_clone = slot_clone.clone();

        // For some reason you have to call this in the closure to get the correct reactive tracking.
        operative_clone.get_slotspecializations_slot();
        maybe_childest_specialization
            .get()
            .map(|spec| spec.get_typespecialization_slot().into_iter().next())
            .flatten()
    });
    let is_adding_slotted_instance = RwSignal::new(false);
    let operative_clone = operative.clone();
    let slot_clone = slot_item.clone();
    let ctx_clone = ctx.clone();
    let add_slotted_instance_view = move || {
        let operative_clone = operative_clone.clone();
        let ctx_clone = ctx_clone.clone();
        let slot = slot_clone.clone();
        let schema_clone = schema.clone();
        let slot_clone = slot.clone();
        let selected_value = RwSignal::<Option<RGSOConcrete<InstanceConcrete, Schema>>>::new(None);
        let allowed_instances = Memo::new(move |_| {
            let schema_clone = schema_clone.clone();

            if let Some(childest_spec) = maybe_childest_type_spec.get() {
                get_all_instances_which_satisfy_specialization(&schema_clone.get(), childest_spec)
            } else {
                match slot_clone.get_templateslotvariant_slot() {
                    TemplateSlotTypeVariantTraitObject::TemplateSlotTypeTraitOperative(
                        trait_op,
                    ) => get_all_instances_which_impl_trait_set(
                        trait_op.get_allowedtraits_slot(),
                        &schema_clone.get(),
                    ),
                    TemplateSlotTypeVariantTraitObject::TemplateSlotTypeSingleOperative(
                        single_op,
                    ) => get_all_descendent_instances_including_own(
                        single_op.get_allowedoperative_slot(),
                        &schema_clone.get(),
                    ),
                    TemplateSlotTypeVariantTraitObject::TemplateSlotTypeMultiOperative(
                        multi_op,
                    ) => multi_op.get_allowedoperatives_slot().into_iter().fold(
                        BTreeSet::new(),
                        |mut agg, op| {
                            agg.extend(get_all_descendent_instances(op, &schema_clone.get()));
                            agg
                        },
                    ),
                }
            }
            .into_iter()
            .collect::<Vec<_>>()
        });
        let slot_clone = slot.clone();
        let on_click_save_slotted_instance = move |_| {
            if !is_allowed_to_add_another_instance.get() {
                leptos::logging::warn!("The slot is already maxed locally or downstream");
                return;
            }
            let mut editor = operative_clone.edit(ctx_clone.clone());
            let slotted_instance_builder = SlottedInstance::new(ctx_clone.clone())
                .set_temp_id("new_slotted_instance")
                .add_existing_fulfiller(operative_clone.get_id(), |na| na)
                .add_existing_instance(selected_value.get().unwrap().get_id(), |na| na);

            let slot_clone = slot_clone.clone();
            if let Some(specialization) = maybe_childest_specialization.get() {
                // Specialization exists and this operative owns it
                if specialization.get_specializer_slot().get_id() == operative_clone.get_id() {
                    editor.incorporate(
                        &slotted_instance_builder
                            .clone()
                            .add_existing_slottedslot(specialization.get_id(), |na| na),
                    );
                    editor.incorporate(
                        specialization
                            .edit(ctx_clone.clone())
                            .add_temp_slottedinstances("new_slotted_instance"),
                    );
                    let mut descendents = BTreeSet::new();
                    get_all_descendent_operators(operative_clone.clone(), &mut descendents);
                    descendents.into_iter().for_each(|descendent| {
                        let maybe_existing_spec_slot = descendent
                            .get_slotspecializations_slot()
                            .into_iter()
                            .filter(|slot_spec| {
                                slot_spec.get_roottemplateslot_slot().get_id()
                                    == slot_clone.get_id()
                            })
                            .next();
                        if let Some(existing_slot) = maybe_existing_spec_slot {
                            editor.incorporate(
                                existing_slot
                                    .edit(ctx_clone.clone())
                                    .add_temp_slottedinstances("new_slotted_instance"),
                            )
                        } else {
                            editor.incorporate(
                                descendent
                                    .edit(ctx_clone.clone())
                                    .add_temp_slotspecializations("new_slot_spec"),
                            );
                        }
                    });
                    editor.execute().unwrap();
                }
                // Specialization exists but another operative owns it. Remove the edge to that specialization, clone it,
                // and add the new instance to the new local specialization
                else {
                    editor.incorporate(
                        &slotted_instance_builder
                            .clone()
                            .add_temp_slottedslot("new_slot_spec"),
                    );
                    let mut edit = operative_clone.edit(ctx_clone.clone());
                    let new_slot_spec = OperativeSlotSpecialized::new(ctx_clone.clone());
                    let new_slot_spec = new_slot_spec
                        .set_temp_id("new_slot_spec")
                        .add_existing_specializer(operative_clone.get_id(), |na| na)
                        .add_existing_upstreamslotdescription::<OperativeSlotSpecialized>(
                            specialization.get_id(),
                            |na| na,
                        )
                        .add_existing_roottemplateslot(slot_clone.get_id(), |na| na)
                        .add_temp_slottedinstances("new_slotted_instance");
                    edit.remove_from_slotspecializations(specialization.get_id())
                        .add_temp_slotspecializations("new_slot_spec");
                    specialization
                        .get_slottedinstances_slot()
                        .iter()
                        .for_each(|slotted_inst| {
                            editor.incorporate(
                                &new_slot_spec
                                    .clone()
                                    .add_existing_slottedinstances(slotted_inst.get_id(), |na| na),
                            );
                        });
                    specialization
                        .get_typespecialization_slot()
                        .iter()
                        .for_each(|type_spec| {
                            match type_spec {
                                OperativeSlotTypeSpecializationTraitObject::OperativeSlotTypeSingleSpecialization(_) => {
                                    editor.incorporate(
                                        &new_slot_spec
                                            .clone()
                                            .add_existing_typespecialization::<OperativeSlotTypeSingleSpecialization>(type_spec.get_id(), |na| na),
                                    );
                                },
                                OperativeSlotTypeSpecializationTraitObject::OperativeSlotTypeMultiSpecialization(_) => {
                                    editor.incorporate(
                                        &new_slot_spec
                                            .clone()
                                            .add_existing_typespecialization::<OperativeSlotTypeMultiSpecialization>(type_spec.get_id(), |na| na),
                                    );
                                },
                                OperativeSlotTypeSpecializationTraitObject::OperativeSlotTypeTraitObjectSpecialization(_) => {
                                    editor.incorporate(
                                        &new_slot_spec
                                            .clone()
                                            .add_existing_typespecialization::<OperativeSlotTypeTraitObjectSpecialization>(type_spec.get_id(), |na| na),
                                    );
                                },
                            };
                        });
                    specialization
                        .get_cardinalityspecialization_slot()
                        .iter()
                        .for_each(|card_spec| {
                            match card_spec {
                                OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityLowerBoundOrZeroSpecialization(_) => {
                                    editor.incorporate(&new_slot_spec.clone().add_existing_cardinalityspecialization::<OperativeSlotCardinalityLowerBoundOrZeroSpecialization>(card_spec.get_id(), |na|na))
                                },
                                OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalitySingleSpecialization(_) => {
                                    editor.incorporate(&new_slot_spec.clone().add_existing_cardinalityspecialization::<OperativeSlotCardinalitySingleSpecialization>(card_spec.get_id(), |na|na))
                                },
                                OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityRangeSpecialization(_) => {
                                    editor.incorporate(&new_slot_spec.clone().add_existing_cardinalityspecialization::<OperativeSlotCardinalityRangeSpecialization>(card_spec.get_id(), |na|na))
                                },
                                OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityZeroSpecialization(_) => {
                                    editor.incorporate(&new_slot_spec.clone().add_existing_cardinalityspecialization::<OperativeSlotCardinalityZeroSpecialization>(card_spec.get_id(), |na|na))
                                },
                                OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityRangeOrZeroSpecialization(_) => {
                                    editor.incorporate(&new_slot_spec.clone().add_existing_cardinalityspecialization::<OperativeSlotCardinalityRangeOrZeroSpecialization>(card_spec.get_id(), |na|na))
                                },
                                OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityLowerBoundSpecialization(_) => {
                                    editor.incorporate(&new_slot_spec.clone().add_existing_cardinalityspecialization::<OperativeSlotCardinalityLowerBoundSpecialization>(card_spec.get_id(), |na|na))
                                },
                            };
                        });
                    editor.incorporate(&edit);
                    editor.incorporate(&new_slot_spec);
                    let mut descendents = BTreeSet::new();
                    get_all_descendent_operators(operative_clone.clone(), &mut descendents);
                    descendents.into_iter().for_each(|descendent| {
                        let maybe_existing_spec_slot = descendent
                            .get_slotspecializations_slot()
                            .into_iter()
                            .filter(|slot_spec| {
                                slot_spec.get_roottemplateslot_slot().get_id()
                                    == slot_clone.get_id()
                            })
                            .next();
                        if let Some(existing_slot) = maybe_existing_spec_slot {
                            if existing_slot.get_upstreamslotdescription_slot().get_id()
                                == specialization.get_id()
                            {
                                editor.incorporate(
                                    existing_slot
                                        .edit(ctx_clone.clone())
                                        .remove_from_upstreamslotdescription(
                                            specialization.get_id(),
                                        )
                                        .add_temp_upstreamslotdescription::<OperativeSlotSpecialized>("new_slot_spec"),
                                );
                            }
                            editor.incorporate(
                                existing_slot
                                    .edit(ctx_clone.clone())
                                    .add_temp_slottedinstances("new_slotted_instance"),
                            );
                        } else {
                            editor.incorporate(
                                descendent
                                    .edit(ctx_clone.clone())
                                    .remove_from_slotspecializations(specialization.get_id())
                                    .add_temp_slotspecializations("new_slot_spec"),
                            );
                        }
                    });
                    editor.execute().unwrap();
                }
            }
            // No specialization exists, create a new one
            else {
                editor.incorporate(
                    &slotted_instance_builder
                        .clone()
                        .add_temp_slottedslot("new_slot_spec"),
                );
                editor.incorporate(
                    operative_clone
                        .edit(ctx_clone.clone())
                        .add_new_slotspecializations(|new_slot_spec| {
                            new_slot_spec
                                .set_temp_id("new_slot_spec")
                                .add_existing_specializer(operative_clone.get_id(), |na| na)
                                .add_existing_upstreamslotdescription::<TemplateSlot>(
                                    slot_clone.get_id(),
                                    |na| na,
                                )
                                .add_existing_roottemplateslot(slot_clone.get_id(), |na| na)
                                .add_temp_slottedinstances("new_slotted_instance")
                        }),
                );
                let mut descendents = BTreeSet::new();
                get_all_descendent_operators(operative_clone.clone(), &mut descendents);
                descendents.into_iter().for_each(|descendent| {
                    let maybe_existing_spec_slot = descendent
                        .get_slotspecializations_slot()
                        .into_iter()
                        .filter(|slot_spec| {
                            slot_spec.get_roottemplateslot_slot().get_id() == slot_clone.get_id()
                        })
                        .next();
                    if let Some(existing_slot) = maybe_existing_spec_slot {
                        editor.incorporate(
                            existing_slot
                                .edit(ctx_clone.clone())
                                .add_temp_slottedinstances("new_slotted_instance"),
                        )
                    } else {
                        editor.incorporate(
                            descendent
                                .edit(ctx_clone.clone())
                                .add_temp_slotspecializations("new_slot_spec"),
                        );
                    }
                });
                editor.execute().unwrap();
            }
            is_adding_slotted_instance.set(false);
        };
        view! {
            <LeafSection>
            <LeafSectionHeader>Adding A Slotted Instance</LeafSectionHeader>
            <LeafSection>
            <SignalSelectRGSOWithOptions value=selected_value options=Signal::derive(move || allowed_instances.get()) empty_allowed=true/>
            </LeafSection>
            <LeafSection>
            <Button on:click=on_click_save_slotted_instance attr:disabled=move || !is_allowed_to_add_another_instance.get() >Save</Button>
            <Button on:click=move |_| is_adding_slotted_instance.set(false)>Cancel</Button>
            </LeafSection>
            </LeafSection>
        }
    };
    let operative_clone = operative.clone();
    let operative_clone_2 = operative.clone();
    let operative_clone_3 = operative.clone();
    let slot_clone = slot_item.clone();
    let slot_clone_2 = slot_item.clone();
    let slot_clone_3 = slot_item.clone();

    let ctx_clone = ctx.clone();

    let slot_context = OperativeSlotContext {
        max_downstream_slotted_instances: max_downstream_slotted_number,
        operative: operative_clone.clone(),
        template_slot: slot_clone.clone(),
        maybe_childest_type_spec: maybe_childest_type_spec.clone(),
        maybe_childest_cardinality_spec: maybe_childest_cardinality_spec.clone(),
    };

    view! {
        <Provider value=slot_context>
        <Section master_collapser=master_collapser>
        <SectionHeader slot>
        {move || slot_item.get_name()}
        </SectionHeader>

        <SubSection>
        <SubSectionHeader>
            Slot Details
            </SubSectionHeader>
            <SubSection attr:class="leafsection dependent">
                {slot_variant}
                <br/>
                "Required:" {slot_bound_view}
                <br/>
                "Upstream (including this node) slotted instances:" {upstream_and_local_slotted_number}
                <br/>
                "Downstream slotted instances:"
                {move ||
                    if downstream_slotted_number_clone.get() >= upstream_and_local_slotted_number() {
                    Either::Left(downstream_slotted_number_clone.get() - upstream_and_local_slotted_number())
                    } else {
                    Either::Right("Something is funky")
                    }
                }
                <br/>
                "Is Fulfilled:" {is_fulfilled}
                <br/>
                "Is Maxed Independently:" {is_maxed_independently}
                <br/>
                "Is Maxed Considering Children:" {is_maxed_considering_children}
            </SubSection>

            </SubSection>
        <SubSection>
            <SubSectionHeader>
            Currently Slotted Instances
            </SubSectionHeader>
            <LeafSection>
                <Button on:click=move |_| is_adding_slotted_instance.set(true) attr:disabled =move||!is_allowed_to_add_another_instance.get()>Slot an instance</Button>
            </LeafSection>
            // <Show when=move|| is_adding_slotted_instance.get()>
            <div class=move||{match is_adding_slotted_instance.get() { true => "", false => "hidden", }}>
            {add_slotted_instance_view.clone()}
            </div>
            // </Show>
            <For each=move || slotted_instances_for_slot.get() key=|item| item.get_id().clone() children=currently_slotted_view />
        </SubSection>

        <SubSection>
            <SubSectionHeader>Type Specialization</SubSectionHeader>
            <OperativeSlotTypeSpecializationSection  />
        </SubSection>
        <SubSection>
        <SubSectionHeader>Cardinality Specialization</SubSectionHeader>
            <OperativeSlotCardinalitySpecializationSection  />
        </SubSection>
        </Section>
        </Provider>
    }
}
