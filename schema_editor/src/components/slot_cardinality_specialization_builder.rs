use std::collections::BTreeSet;

use leptos::either::EitherOf3;
use schema_editor_generated_toolkit::prelude::*;
use web_sys::MouseEvent;

use super::{
    common::*, operative_slot_section::OperativeSlotContext,
    utils::get_all_descendent_operators_including_own,
};

#[derive(strum_macros::Display, strum_macros::EnumIter, strum_macros::EnumString, Clone)]
enum CardinalityInputTypeOptions {
    LowerBoundOnly,
    Both,
    None,
}

#[derive(Clone, PartialEq, Debug)]
pub struct CardinalityInfo {
    pub min: u32,
    pub max: Option<u32>,
    pub zero_allowed: bool,
}
impl CardinalityInfo {
    pub fn from_card_spec(spec: OperativeSlotCardinalitySpecializationTraitObject) -> Self {
        match spec {
            OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityLowerBoundOrZeroSpecialization(item) => {
                CardinalityInfo {min: item.get_lower_bound_field(), max: None, zero_allowed: true}
            },
            OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalitySingleSpecialization(_item) => {
                CardinalityInfo {min: 1, max: Some(1), zero_allowed: false}
            },
            OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityRangeSpecialization(item) => {
                CardinalityInfo {min: item.get_lower_bound_field(), max: Some(item.get_upper_bound_field()), zero_allowed: false}
            },
            OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityZeroSpecialization(_item) => {
                CardinalityInfo {min: 0, max: Some(0), zero_allowed: true}
            },
            OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityRangeOrZeroSpecialization(item) => {
                CardinalityInfo {min: item.get_lower_bound_field(), max: Some(item.get_upper_bound_field()), zero_allowed: true}
            },
            OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityLowerBoundSpecialization(item) => {
                CardinalityInfo {min: item.get_lower_bound_field(), max: None, zero_allowed: false}
            },
        }
    }
}

#[component]
pub fn SlotCardinalitySpecializationBuilder(
    spec_target: OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject,
) -> impl IntoView {
    let ctx = use_context::<SharedGraph<Schema>>().unwrap();
    let OperativeSlotContext {
        max_downstream_slotted_instances,
        operative,
        template_slot,
        maybe_childest_cardinality_spec,
        ..
    } = use_context::<OperativeSlotContext>().unwrap();

    let spec_target_clone = spec_target.clone();
    let prev_bounds = Memo::new(move |_| {
        if let Some(childest_cardinality_spec) = maybe_childest_cardinality_spec.get() {
            CardinalityInfo::from_card_spec(childest_cardinality_spec)
        } else {
            match spec_target_clone.clone() {
        OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundOrZeroSpecialization(item) => {
            CardinalityInfo { min: item.get_lower_bound_field(), max: None, zero_allowed: true }
        },
        OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityRangeOrZero(item) => {
            CardinalityInfo { min: item.get_lower_bound_field(), max: Some( item.get_upper_bound_field() ), zero_allowed: true }
        },
        OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityRangeSpecialization(item) => {
            CardinalityInfo { min: item.get_lower_bound_field(), max: Some( item.get_upper_bound_field() ), zero_allowed: false }
        },
        OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityLowerBoundOrZero(item) => {
            CardinalityInfo { min: item.get_lower_bound_field(), max: None, zero_allowed: true }
        },
        OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityRange(item) => {
            CardinalityInfo { min: item.get_lower_bound_field(), max: Some( item.get_upper_bound_field() ), zero_allowed: false }
        },
        OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityLowerBound(item) => {
            CardinalityInfo { min: item.get_lower_bound_field(), max: None, zero_allowed: false }
        },
        OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityRangeOrZeroSpecialization(item) => {
            CardinalityInfo { min: item.get_lower_bound_field(), max: Some( item.get_upper_bound_field() ), zero_allowed: true }
        },
        OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundSpecialization(item) => {
            CardinalityInfo { min: item.get_lower_bound_field(), max: None, zero_allowed: false }
        },
    }
        }
    });

    let spec_target_clone = spec_target.clone();
    let options = Signal::derive(move || {
        match spec_target_clone {
        OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundOrZeroSpecialization(_) =>
            vec![OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityLowerBoundOrZeroSpecialization,
                OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityLowerBoundSpecialization,
                OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalitySingleSpecialization,
                OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityRangeSpecialization,
                OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityRangeOrZeroSpecialization,
                OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityZeroSpecialization,
            ],
        OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityRangeOrZero(_) =>
            vec![
                OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalitySingleSpecialization,
                OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityRangeSpecialization,
                OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityRangeOrZeroSpecialization,
                OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityZeroSpecialization,
            ],
        OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityRangeSpecialization(_) =>
            vec![
                OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalitySingleSpecialization,
                OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityRangeSpecialization,
                OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityZeroSpecialization,
            ],
        OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityLowerBoundOrZero(_) =>
            vec![OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityLowerBoundOrZeroSpecialization,
                OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityLowerBoundSpecialization,
                OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalitySingleSpecialization,
                OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityRangeSpecialization,
                OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityRangeOrZeroSpecialization,
                OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityZeroSpecialization,
            ],
        OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityRange(_) =>
            vec![
                OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalitySingleSpecialization,
                OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityRangeSpecialization,
                OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityZeroSpecialization,
            ],
        OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityLowerBound(_) =>
            vec![
                OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityLowerBoundSpecialization,
                OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalitySingleSpecialization,
                OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityRangeSpecialization,
                OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityRangeOrZeroSpecialization,
                OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityZeroSpecialization,
            ],
        OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityRangeOrZeroSpecialization(_) =>
            vec![
                OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalitySingleSpecialization,
                OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityRangeSpecialization,
                OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityRangeOrZeroSpecialization,
                OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityZeroSpecialization,
            ],
        OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundSpecialization(_) =>
            vec![
                OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityLowerBoundSpecialization,
                OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalitySingleSpecialization,
                OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityRangeSpecialization,
                OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityRangeOrZeroSpecialization,
                OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityZeroSpecialization,
            ],
    }
    });
    let is_adding = RwSignal::new(false);
    let selected_spec = RwSignal::new(Some(OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalitySingleSpecialization));

    let selected_lower_bound = RwSignal::new(prev_bounds.get().min);
    let selected_upper_bound = RwSignal::new(prev_bounds.get().max.unwrap_or(0));

    let current_input_variant = move || {
        match selected_spec.get().unwrap() {
        OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityLowerBoundOrZeroSpecialization => CardinalityInputTypeOptions::LowerBoundOnly,
        OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalitySingleSpecialization => CardinalityInputTypeOptions::None,
        OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityRangeSpecialization => CardinalityInputTypeOptions::Both,
        OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityZeroSpecialization => CardinalityInputTypeOptions::None,
        OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityRangeOrZeroSpecialization => CardinalityInputTypeOptions::Both,
        OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityLowerBoundSpecialization => CardinalityInputTypeOptions::LowerBoundOnly,
    }
    };

    let is_downstream_slot_outside_of_attempted_bounds = Signal::derive(move || {
        let max_ds = max_downstream_slotted_instances.get();
        match selected_spec.get().unwrap() {
            OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityLowerBoundOrZeroSpecialization => {
                false
            },
            OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalitySingleSpecialization => {
                max_ds > 1
            },
            OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityRangeSpecialization => {
                max_ds > selected_upper_bound.get()
            },
            OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityZeroSpecialization => {
                max_ds > 0
            },
            OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityRangeOrZeroSpecialization => {
                max_ds > selected_upper_bound.get()
            },
            OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityLowerBoundSpecialization => {
                false
            },
        }
    });

    let are_selected_values_outside_of_previous_bounds = Signal::derive(move || {
        let prev = prev_bounds.get();
        match selected_spec.get().unwrap() {
            OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityLowerBoundOrZeroSpecialization => {
                prev.min > selected_lower_bound.get()
            },
            OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalitySingleSpecialization => {
                prev.min > 1
            },
            OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityRangeSpecialization => {
                prev.max.is_some_and(|prev_max| prev_max < selected_upper_bound.get())
            },
            OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityZeroSpecialization => {
                prev.zero_allowed == false && prev.min > 0
            },
            OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityRangeOrZeroSpecialization => {
                prev.max.is_some_and(|prev_max| prev_max < selected_upper_bound.get())
            },
            OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityLowerBoundSpecialization => {
                prev.min > selected_lower_bound.get()
            },
        }
    });

    let ctx_clone = ctx.clone();

    let operative_clone = operative.clone();

    let on_save = move |_e: MouseEvent| {
        let operative = operative_clone.clone();
        let operative_clone = operative.clone();
        let mut all_descendent_ops = BTreeSet::new();
        // all_descendent_ops_and_this_op.insert(operative_clone.clone());
        get_all_descendent_operators_including_own(operative_clone, &mut all_descendent_ops);

        if is_downstream_slot_outside_of_attempted_bounds.get() {
            leptos::logging::warn!(
                "Some downstream operative has an incompatible number of instances slotted"
            );
            return ();
        }
        if are_selected_values_outside_of_previous_bounds.get() {
            leptos::logging::warn!(
                "Selected values are incompatible with previous cardinality constraints"
            );
        }
        let operative_clone = operative.clone();
        let mut editor = operative_clone.edit(ctx_clone.clone());
        let mut maybe_new_unencumbered_self_spec = None;
        let mut maybe_new_dependent_self_spec = None;
        let mut maybe_existing_self_spec = None;
        let maybe_self_slot_spec = operative_clone
            .get_slotspecializations_slot()
            .into_iter()
            .filter(|spec| spec.get_roottemplateslot_slot().get_id() == template_slot.get_id())
            .next();
        let mut previous_slot_spec_id = None;
        if let Some(self_spec) = maybe_self_slot_spec {
            if self_spec.get_specializer_slot().get_id() == operative_clone.get_id() {
                let edit = self_spec.edit(ctx_clone.clone());
                maybe_existing_self_spec = Some(edit);
            } else {
                previous_slot_spec_id = Some(self_spec.get_id().clone());
                editor.remove_from_slotspecializations(self_spec.get_id());
                let new_slot_spec = OperativeSlotSpecialized::new(ctx_clone.clone());
                let new_slot_spec = new_slot_spec
                    .set_temp_id("new_slot_spec")
                    .add_existing_specializer(operative_clone.get_id(), |na| na)
                    .add_existing_roottemplateslot(template_slot.get_id(), |na| na)
                    // set upstream to the previous spec
                    .add_existing_upstreamslotdescription::<OperativeSlotSpecialized>(
                        self_spec.get_id(),
                        |na| na,
                    );
                self_spec
                    .get_slottedinstances_slot()
                    .iter()
                    .for_each(|slotted_inst| {
                        editor.incorporate(
                            &new_slot_spec
                                .clone()
                                .add_existing_slottedinstances(slotted_inst.get_id(), |na| na),
                        );
                    });
                self_spec
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

                editor.add_temp_slotspecializations("new_slot_spec");
                maybe_new_dependent_self_spec = Some(new_slot_spec);
            }
        } else {
            maybe_new_unencumbered_self_spec = Some(
                OperativeSlotSpecialized::new(ctx_clone.clone())
                    .set_temp_id("new_slot_spec")
                    .add_existing_specializer(operative_clone.get_id(), |na| na)
                    .add_existing_roottemplateslot(template_slot.get_id(), |na| na)
                    .add_existing_upstreamslotdescription::<TemplateSlot>(
                        template_slot.get_id(),
                        |na| na,
                    ),
            );
            editor.add_temp_slotspecializations("new_slot_spec");
        }

        match selected_spec.get().unwrap() {
            OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityLowerBoundOrZeroSpecialization => {
                match spec_target {
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundOrZeroSpecialization(_) => {
                        let edit = OperativeSlotCardinalityLowerBoundOrZeroSpecialization::new(ctx_clone.clone())
                                .set_temp_id("new_cardinality_spec")
                                .set_lower_bound(selected_lower_bound.get())
                                .add_existing_upstreamcardinality::<OperativeSlotCardinalityLowerBoundOrZeroSpecialization>(spec_target.get_id(), |na|na)
                        ;
                        if let Some(existing_self_spec) = &mut maybe_existing_self_spec {
                            editor.incorporate(&edit.clone().add_existing_specializedslot(existing_self_spec.get_id(), |na|na));
                            editor.incorporate(existing_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityLowerBoundOrZeroSpecialization>("new_cardinality_spec"));                        }
                        if let Some( new_self_spec) = maybe_new_unencumbered_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityLowerBoundOrZeroSpecialization>("new_cardinality_spec"));
                        }
                        if let Some( new_self_spec) = maybe_new_dependent_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityLowerBoundOrZeroSpecialization>("new_cardinality_spec"));
                        }
                    },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityRangeOrZero(_) => {
                        leptos::logging::warn!("Somehow an ill-formed specialization was attempted (e.g. trying to make a lower_bound specialization for a range");
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityRangeSpecialization(_) => {
                        leptos::logging::warn!("Somehow an ill-formed specialization was attempted (e.g. trying to make a lower_bound specialization for a range");
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityLowerBoundOrZero(_) => {
                        let  edit = OperativeSlotCardinalityLowerBoundOrZeroSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_cardinality_spec")
                                .set_lower_bound(selected_lower_bound.get())
                                .add_existing_upstreamcardinality::<TemplateSlotCardinalityLowerBoundOrZero>(spec_target.get_id(), |na|na)
                        ;
                        if let Some(existing_self_spec) = &mut maybe_existing_self_spec {
                            editor.incorporate(&edit.clone().add_existing_specializedslot(existing_self_spec.get_id(), |na|na));
                            editor.incorporate(existing_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityLowerBoundOrZeroSpecialization>("new_cardinality_spec"));                        }
                        if let Some( new_self_spec) = maybe_new_unencumbered_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityLowerBoundOrZeroSpecialization>("new_cardinality_spec"));
                        }
                        if let Some( new_self_spec) = maybe_new_dependent_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityLowerBoundOrZeroSpecialization>("new_cardinality_spec"));
                        }
                    },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityRange(_) => {
                        leptos::logging::warn!("Somehow an ill-formed specialization was attempted (e.g. trying to make a lower_bound specialization for a range");
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityLowerBound(_) => {
                        leptos::logging::warn!("Somehow an ill-formed specialization was attempted (e.g. trying to make a lower_bound specialization for a range");
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityRangeOrZeroSpecialization(_) => {
                        leptos::logging::warn!("Somehow an ill-formed specialization was attempted (e.g. trying to make a lower_bound specialization for a range");
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundSpecialization(_) => {
                        leptos::logging::warn!("Somehow an ill-formed specialization was attempted (e.g. trying to make a lower_bound specialization for a range");
 },
                }
                if let Some(_existing_self_spec) = maybe_existing_self_spec {
                    all_descendent_ops.iter().for_each(|desc_op| {
                        let maybe_existing_spec_slot = desc_op
                            .get_slotspecializations_slot()
                            .into_iter()
                            .filter(|slot_spec| {
                                slot_spec.get_roottemplateslot_slot().get_id()
                                    == template_slot.get_id()
                            })
                            .next();
                        if let Some(existing_slot) = maybe_existing_spec_slot {
                            if let Some(existing_cardinality_specialization) = existing_slot.get_cardinalityspecialization_slot().into_iter().next() {
                                editor.incorporate(existing_slot.edit(ctx_clone.clone())
                                    .remove_from_cardinalityspecialization(existing_cardinality_specialization.get_id()));
                            }
                            editor.incorporate(
                                existing_slot
                                    .edit(ctx_clone.clone())
                                    .add_temp_cardinalityspecialization::<OperativeSlotCardinalityLowerBoundOrZeroSpecialization>("new_cardinality_spec"),
                            )
                        } else {
                            editor.incorporate(
                                desc_op
                                    .edit(ctx_clone.clone())
                                    .add_temp_slotspecializations("new_slot_spec"),
                            );
                        }
                    });
                }
                if let Some(_unencumbered_new) = maybe_new_unencumbered_self_spec {
                    all_descendent_ops.iter().for_each(|desc_op| {
                        let maybe_existing_spec_slot = desc_op
                            .get_slotspecializations_slot()
                            .into_iter()
                            .filter(|slot_spec| {
                                slot_spec.get_roottemplateslot_slot().get_id()
                                    == template_slot.get_id()
                            })
                            .next();
                        if let Some(existing_slot) = maybe_existing_spec_slot {
                            if let Some(existing_cardinality_specialization) = existing_slot.get_cardinalityspecialization_slot().into_iter().next() {
                                editor.incorporate(existing_slot.edit(ctx_clone.clone())
                                    .remove_from_cardinalityspecialization(existing_cardinality_specialization.get_id()));
                            }
                            editor.incorporate(
                                existing_slot
                                    .edit(ctx_clone.clone())
                                    .add_temp_cardinalityspecialization::<OperativeSlotCardinalityLowerBoundOrZeroSpecialization>("new_cardinality_spec"),
                            )
                        } else {
                            editor.incorporate(
                                desc_op
                                    .edit(ctx_clone.clone())
                                    .add_temp_slotspecializations("new_slot_spec"),
                            );
                        }
                    });
                }
                if let Some(_dependent_new) = maybe_new_dependent_self_spec {
                    all_descendent_ops.iter().for_each(|desc_op| {
                        let slot_spec_in_question =
                        desc_op.get_slotspecializations_slot().into_iter().filter(|spec| {
                            spec.get_roottemplateslot_slot().get_id() == template_slot.get_id()
                        }).next();
                        if let Some(existing_slot) = slot_spec_in_question {
                        if existing_slot.get_id() == &previous_slot_spec_id.unwrap() {
                            editor.incorporate(desc_op.edit(ctx_clone.clone()).remove_from_slotspecializations(existing_slot.get_id())
                                .add_temp_slotspecializations("new_slot_spec"));
                        } else {
                            if let Some(existing_cardinality_specialization) = existing_slot.get_cardinalityspecialization_slot().into_iter().next() {
                                editor.incorporate(existing_slot.edit(ctx_clone.clone())
                                    .remove_from_cardinalityspecialization(existing_cardinality_specialization.get_id()));
                            }
                            editor.incorporate(existing_slot.edit(ctx_clone.clone())
                                .remove_from_upstreamslotdescription(&previous_slot_spec_id.unwrap())
                                .add_temp_upstreamslotdescription::<OperativeSlotSpecialized>("new_slot_spec")
                                .add_temp_cardinalityspecialization::<OperativeSlotCardinalityLowerBoundOrZeroSpecialization>("new_cardinality_spec"));
                        }
                        } else {
                            editor.incorporate(desc_op.edit(ctx_clone.clone()).add_temp_slotspecializations("new_slot_spec"))
                        }
                    });
                }
                editor.execute().unwrap();
            },
            OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalitySingleSpecialization => {
                match spec_target {
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundOrZeroSpecialization(_) => {
                        let  edit = OperativeSlotCardinalitySingleSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_cardinality_spec")
                            .add_existing_upstreamcardinality::<OperativeSlotCardinalityLowerBoundOrZeroSpecialization>(spec_target.get_id(), |na|na)
                        ;
                        if let Some(existing_self_spec) = &mut maybe_existing_self_spec {
                            editor.incorporate(&edit.clone().add_existing_specializedslot(existing_self_spec.get_id(), |na|na));
                            editor.incorporate(existing_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalitySingleSpecialization>("new_cardinality_spec"));                        }
                        if let Some( new_self_spec) = maybe_new_unencumbered_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalitySingleSpecialization>("new_cardinality_spec"));
                        }
                        if let Some( new_self_spec) = maybe_new_dependent_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalitySingleSpecialization>("new_cardinality_spec"));
                        }
                    },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityRangeOrZero(_) => {
                        let  edit = OperativeSlotCardinalitySingleSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_cardinality_spec")
                            .add_existing_upstreamcardinality::<TemplateSlotCardinalityRangeOrZero>(spec_target.get_id(), |na|na)
                        ;
                        if let Some(existing_self_spec) = &mut maybe_existing_self_spec {
                            editor.incorporate(&edit.clone().add_existing_specializedslot(existing_self_spec.get_id(), |na|na));
                            editor.incorporate(existing_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalitySingleSpecialization>("new_cardinality_spec"));                        }
                        if let Some( new_self_spec) = maybe_new_unencumbered_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalitySingleSpecialization>("new_cardinality_spec"));
                        }
                        if let Some( new_self_spec) = maybe_new_dependent_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalitySingleSpecialization>("new_cardinality_spec"));
                        }
                    },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityRangeSpecialization(_) => {
                        let  edit = OperativeSlotCardinalitySingleSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_cardinality_spec")
                            .add_existing_upstreamcardinality::<OperativeSlotCardinalityRangeSpecialization>(spec_target.get_id(), |na|na)
                        ;
                        if let Some(existing_self_spec) = &mut maybe_existing_self_spec {
                            editor.incorporate(&edit.clone().add_existing_specializedslot(existing_self_spec.get_id(), |na|na));
                            editor.incorporate(existing_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalitySingleSpecialization>("new_cardinality_spec"));                        }
                        if let Some( new_self_spec) = maybe_new_unencumbered_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalitySingleSpecialization>("new_cardinality_spec"));
                        }
                        if let Some( new_self_spec) = maybe_new_dependent_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalitySingleSpecialization>("new_cardinality_spec"));
                        }
                    },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityLowerBoundOrZero(_) => {
                        let  edit = OperativeSlotCardinalitySingleSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_cardinality_spec")
                            .add_existing_upstreamcardinality::<TemplateSlotCardinalityLowerBoundOrZero>(spec_target.get_id(), |na|na)
                        ;
                        if let Some(existing_self_spec) = &mut maybe_existing_self_spec {
                            editor.incorporate(&edit.clone().add_existing_specializedslot(existing_self_spec.get_id(), |na|na));
                            editor.incorporate(existing_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalitySingleSpecialization>("new_cardinality_spec"));                        }
                        if let Some( new_self_spec) = maybe_new_unencumbered_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalitySingleSpecialization>("new_cardinality_spec"));
                        }
                        if let Some( new_self_spec) = maybe_new_dependent_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalitySingleSpecialization>("new_cardinality_spec"));
                        }
                    },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityRange(_) => {
                        let  edit = OperativeSlotCardinalitySingleSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_cardinality_spec")
                            .add_existing_upstreamcardinality::<TemplateSlotCardinalityRange>(spec_target.get_id(), |na|na)
                        ;
                        if let Some(existing_self_spec) = &mut maybe_existing_self_spec {
                            editor.incorporate(&edit.clone().add_existing_specializedslot(existing_self_spec.get_id(), |na|na));
                            editor.incorporate(existing_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalitySingleSpecialization>("new_cardinality_spec"));                        }
                        if let Some( new_self_spec) = maybe_new_unencumbered_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalitySingleSpecialization>("new_cardinality_spec"));
                        }
                        if let Some( new_self_spec) = maybe_new_dependent_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalitySingleSpecialization>("new_cardinality_spec"));
                        }
                    },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityLowerBound(_) => {
                        let  edit = OperativeSlotCardinalitySingleSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_cardinality_spec")
                            .add_existing_upstreamcardinality::<TemplateSlotCardinalityLowerBound>(spec_target.get_id(), |na|na)
                        ;
                        if let Some(existing_self_spec) = &mut maybe_existing_self_spec {
                            editor.incorporate(&edit.clone().add_existing_specializedslot(existing_self_spec.get_id(), |na|na));
                            editor.incorporate(existing_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalitySingleSpecialization>("new_cardinality_spec"));                        }
                        if let Some( new_self_spec) = maybe_new_unencumbered_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalitySingleSpecialization>("new_cardinality_spec"));
                        }
                        if let Some( new_self_spec) = maybe_new_dependent_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalitySingleSpecialization>("new_cardinality_spec"));
                        }
                    },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityRangeOrZeroSpecialization(_) => {
                        let  edit = OperativeSlotCardinalitySingleSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_cardinality_spec")
                            .add_existing_upstreamcardinality::<OperativeSlotCardinalityRangeOrZeroSpecialization>(spec_target.get_id(), |na|na)
                        ;
                        if let Some(existing_self_spec) = &mut maybe_existing_self_spec {
                            editor.incorporate(&edit.clone().add_existing_specializedslot(existing_self_spec.get_id(), |na|na));
                            editor.incorporate(existing_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalitySingleSpecialization>("new_cardinality_spec"));                        }
                        if let Some( new_self_spec) = maybe_new_unencumbered_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalitySingleSpecialization>("new_cardinality_spec"));
                        }
                        if let Some( new_self_spec) = maybe_new_dependent_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalitySingleSpecialization>("new_cardinality_spec"));
                        }
                    },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundSpecialization(_) => {
                        let  edit = OperativeSlotCardinalitySingleSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_cardinality_spec")
                            .add_existing_upstreamcardinality::<OperativeSlotCardinalityLowerBoundSpecialization>(spec_target.get_id(), |na|na)
                        ;
                        if let Some(existing_self_spec) = &mut maybe_existing_self_spec {
                            editor.incorporate(&edit.clone().add_existing_specializedslot(existing_self_spec.get_id(), |na|na));
                            editor.incorporate(existing_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalitySingleSpecialization>("new_cardinality_spec"));                        }
                        if let Some( new_self_spec) = maybe_new_unencumbered_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalitySingleSpecialization>("new_cardinality_spec"));
                        }
                        if let Some( new_self_spec) = maybe_new_dependent_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalitySingleSpecialization>("new_cardinality_spec"));
                        }
                    },
                }
                if let Some(_existing_self_spec) = maybe_existing_self_spec {
                    all_descendent_ops.iter().for_each(|desc_op| {
                        let maybe_existing_spec_slot = desc_op
                            .get_slotspecializations_slot()
                            .into_iter()
                            .filter(|slot_spec| {
                                slot_spec.get_roottemplateslot_slot().get_id()
                                    == template_slot.get_id()
                            })
                            .next();
                        if let Some(existing_slot) = maybe_existing_spec_slot {
                            if let Some(existing_cardinality_specialization) = existing_slot.get_cardinalityspecialization_slot().into_iter().next() {
                                editor.incorporate(existing_slot.edit(ctx_clone.clone())
                                    .remove_from_cardinalityspecialization(existing_cardinality_specialization.get_id()));
                            }
                            editor.incorporate(
                                existing_slot
                                    .edit(ctx_clone.clone())
                                    .add_temp_cardinalityspecialization::<OperativeSlotCardinalitySingleSpecialization>("new_cardinality_spec"),
                            )
                        } else {
                            editor.incorporate(
                                desc_op
                                    .edit(ctx_clone.clone())
                                    .add_temp_slotspecializations("new_slot_spec"),
                            );
                        }
                    });
                }
                if let Some(_unencumbered_new) = maybe_new_unencumbered_self_spec {
                    all_descendent_ops.iter().for_each(|desc_op| {
                        let maybe_existing_spec_slot = desc_op
                            .get_slotspecializations_slot()
                            .into_iter()
                            .filter(|slot_spec| {
                                slot_spec.get_roottemplateslot_slot().get_id()
                                    == template_slot.get_id()
                            })
                            .next();
                        if let Some(existing_slot) = maybe_existing_spec_slot {
                            if let Some(existing_cardinality_specialization) = existing_slot.get_cardinalityspecialization_slot().into_iter().next() {
                                editor.incorporate(existing_slot.edit(ctx_clone.clone())
                                    .remove_from_cardinalityspecialization(existing_cardinality_specialization.get_id()));
                            }
                            editor.incorporate(
                                existing_slot
                                    .edit(ctx_clone.clone())
                                    .add_temp_cardinalityspecialization::<OperativeSlotCardinalitySingleSpecialization>("new_cardinality_spec"),
                            )
                        } else {
                            editor.incorporate(
                                desc_op
                                    .edit(ctx_clone.clone())
                                    .add_temp_slotspecializations("new_slot_spec"),
                            );
                        }
                    });
                }
                if let Some(_dependent_new) = maybe_new_dependent_self_spec {
                    all_descendent_ops.iter().for_each(|desc_op| {
                        let slot_spec_in_question =
                        desc_op.get_slotspecializations_slot().into_iter().filter(|spec| {
                            spec.get_roottemplateslot_slot().get_id() == template_slot.get_id()
                        }).next();
                        if let Some(existing_slot) = slot_spec_in_question {
                        if existing_slot.get_id() == &previous_slot_spec_id.unwrap() {
                            editor.incorporate(desc_op.edit(ctx_clone.clone()).remove_from_slotspecializations(existing_slot.get_id())
                                .add_temp_slotspecializations("new_slot_spec"));
                        } else {
                            if let Some(existing_cardinality_specialization) = existing_slot.get_cardinalityspecialization_slot().into_iter().next() {
                                editor.incorporate(existing_slot.edit(ctx_clone.clone())
                                    .remove_from_cardinalityspecialization(existing_cardinality_specialization.get_id()));
                            }
                            editor.incorporate(existing_slot.edit(ctx_clone.clone())
                                .remove_from_upstreamslotdescription(&previous_slot_spec_id.unwrap())
                                .add_temp_upstreamslotdescription::<OperativeSlotSpecialized>("new_slot_spec")
                                .add_temp_cardinalityspecialization::<OperativeSlotCardinalitySingleSpecialization>("new_cardinality_spec"));
                        }
                        } else {
                            editor.incorporate(desc_op.edit(ctx_clone.clone()).add_temp_slotspecializations("new_slot_spec"))
                        }
                    });
                }
                editor.execute().unwrap();
 },
            OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityRangeSpecialization => {
                match spec_target {
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundOrZeroSpecialization(_) => {
                        let  edit = OperativeSlotCardinalityRangeSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_cardinality_spec")
                                .set_upper_bound(selected_upper_bound.get())
                                .set_lower_bound(selected_lower_bound.get())
                                .add_existing_upstreamcardinality::<OperativeSlotCardinalityLowerBoundOrZeroSpecialization>(spec_target.get_id(), |na|na)
                        ;
                        if let Some(existing_self_spec) = &mut maybe_existing_self_spec {
                            editor.incorporate(&edit.clone().add_existing_specializedslot(existing_self_spec.get_id(), |na|na));
                            editor.incorporate(existing_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityRangeSpecialization>("new_cardinality_spec"));                        }
                        if let Some( new_self_spec) = maybe_new_unencumbered_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityRangeSpecialization>("new_cardinality_spec"));
                        }
                        if let Some( new_self_spec) = maybe_new_dependent_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityRangeSpecialization>("new_cardinality_spec"));
                        }
                    },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityRangeOrZero(_) => {
                        let  edit = OperativeSlotCardinalityRangeSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_cardinality_spec")
                                .set_upper_bound(selected_upper_bound.get())
                                .set_lower_bound(selected_lower_bound.get())
                                .add_existing_upstreamcardinality::<TemplateSlotCardinalityRangeOrZero>(spec_target.get_id(), |na|na)
                        ;
                        if let Some(existing_self_spec) = &mut maybe_existing_self_spec {
                            editor.incorporate(&edit.clone().add_existing_specializedslot(existing_self_spec.get_id(), |na|na));
                            editor.incorporate(existing_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityRangeSpecialization>("new_cardinality_spec"));                        }
                        if let Some( new_self_spec) = maybe_new_unencumbered_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityRangeSpecialization>("new_cardinality_spec"));
                        }
                        if let Some( new_self_spec) = maybe_new_dependent_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityRangeSpecialization>("new_cardinality_spec"));
                        }
                    },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityRangeSpecialization(_) => {
                        let  edit = OperativeSlotCardinalityRangeSpecialization::new(ctx_clone.clone())
                        .set_temp_id("new_cardinality_spec")
                                .set_upper_bound(selected_upper_bound.get())
                                .set_lower_bound(selected_lower_bound.get())
                                .add_existing_upstreamcardinality::<OperativeSlotCardinalityRangeSpecialization>(spec_target.get_id(), |na|na)
                        ;
                        if let Some(existing_self_spec) = &mut maybe_existing_self_spec {
                            editor.incorporate(&edit.clone().add_existing_specializedslot(existing_self_spec.get_id(), |na|na));
                            editor.incorporate(existing_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityRangeSpecialization>("new_cardinality_spec"));                        }
                        if let Some( new_self_spec) = maybe_new_unencumbered_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityRangeSpecialization>("new_cardinality_spec"));
                        }
                        if let Some( new_self_spec) = maybe_new_dependent_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityRangeSpecialization>("new_cardinality_spec"));
                        }
                    },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityLowerBoundOrZero(_) => {
                        let  edit = OperativeSlotCardinalityRangeSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_cardinality_spec")
                                .set_upper_bound(selected_upper_bound.get())
                                .set_lower_bound(selected_lower_bound.get())
                                .add_existing_upstreamcardinality::<TemplateSlotCardinalityLowerBoundOrZero>(spec_target.get_id(), |na|na)
                        ;
                        if let Some(existing_self_spec) = &mut maybe_existing_self_spec {
                            editor.incorporate(&edit.clone().add_existing_specializedslot(existing_self_spec.get_id(), |na|na));
                            editor.incorporate(existing_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityRangeSpecialization>("new_cardinality_spec"));                        }
                        if let Some( new_self_spec) = maybe_new_unencumbered_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityRangeSpecialization>("new_cardinality_spec"));
                        }
                        if let Some( new_self_spec) = maybe_new_dependent_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityRangeSpecialization>("new_cardinality_spec"));
                        }
                    },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityRange(_) => {
                        let  edit = OperativeSlotCardinalityRangeSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_cardinality_spec")
                                .set_upper_bound(selected_upper_bound.get())
                                .set_lower_bound(selected_lower_bound.get())
                                .add_existing_upstreamcardinality::<TemplateSlotCardinalityRange>(spec_target.get_id(), |na|na)
                        ;
                        if let Some(existing_self_spec) = &mut maybe_existing_self_spec {
                            editor.incorporate(&edit.clone().add_existing_specializedslot(existing_self_spec.get_id(), |na|na));
                            editor.incorporate(existing_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityRangeSpecialization>("new_cardinality_spec"));                        }
                        if let Some( new_self_spec) = maybe_new_unencumbered_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityRangeSpecialization>("new_cardinality_spec"));
                        }
                        if let Some( new_self_spec) = maybe_new_dependent_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityRangeSpecialization>("new_cardinality_spec"));
                        }
                    },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityLowerBound(_) => {
                        let  edit = OperativeSlotCardinalityRangeSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_cardinality_spec")
                                .set_upper_bound(selected_upper_bound.get())
                                .set_lower_bound(selected_lower_bound.get())
                                .add_existing_upstreamcardinality::<TemplateSlotCardinalityLowerBound>(spec_target.get_id(), |na|na)
                        ;
                        if let Some(existing_self_spec) = &mut maybe_existing_self_spec {
                            editor.incorporate(&edit.clone().add_existing_specializedslot(existing_self_spec.get_id(), |na|na));
                            editor.incorporate(existing_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityRangeSpecialization>("new_cardinality_spec"));                        }
                        if let Some( new_self_spec) = maybe_new_unencumbered_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityRangeSpecialization>("new_cardinality_spec"));
                        }
                        if let Some( new_self_spec) = maybe_new_dependent_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityRangeSpecialization>("new_cardinality_spec"));
                        }
                    },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityRangeOrZeroSpecialization(_) => {
                        let  edit = OperativeSlotCardinalityRangeSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_cardinality_spec")
                                .set_upper_bound(selected_upper_bound.get())
                                .set_lower_bound(selected_lower_bound.get())
                                .add_existing_upstreamcardinality::<OperativeSlotCardinalityRangeOrZeroSpecialization>(spec_target.get_id(), |na|na)
                        ;
                        if let Some(existing_self_spec) = &mut maybe_existing_self_spec {
                            editor.incorporate(&edit.clone().add_existing_specializedslot(existing_self_spec.get_id(), |na|na));
                            editor.incorporate(existing_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityRangeSpecialization>("new_cardinality_spec"));                        }
                        if let Some( new_self_spec) = maybe_new_unencumbered_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityRangeSpecialization>("new_cardinality_spec"));
                        }
                        if let Some( new_self_spec) = maybe_new_dependent_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityRangeSpecialization>("new_cardinality_spec"));
                        }
                    },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundSpecialization(_) => {
                        let  edit = OperativeSlotCardinalityRangeSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_cardinality_spec")
                                .set_upper_bound(selected_upper_bound.get())
                                .set_lower_bound(selected_lower_bound.get())
                                .add_existing_upstreamcardinality::<OperativeSlotCardinalityLowerBoundSpecialization>(spec_target.get_id(), |na|na)
                        ;
                        if let Some(existing_self_spec) = &mut maybe_existing_self_spec {
                            editor.incorporate(&edit.clone().add_existing_specializedslot(existing_self_spec.get_id(), |na|na));
                            editor.incorporate(existing_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityRangeSpecialization>("new_cardinality_spec"));                        }
                        if let Some( new_self_spec) = maybe_new_unencumbered_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityRangeSpecialization>("new_cardinality_spec"));
                        }
                        if let Some( new_self_spec) = maybe_new_dependent_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityRangeSpecialization>("new_cardinality_spec"));
                        }
                    },
                }
                if let Some(_existing_self_spec) = maybe_existing_self_spec {
                    all_descendent_ops.iter().for_each(|desc_op| {
                        let maybe_existing_spec_slot = desc_op
                            .get_slotspecializations_slot()
                            .into_iter()
                            .filter(|slot_spec| {
                                slot_spec.get_roottemplateslot_slot().get_id()
                                    == template_slot.get_id()
                            })
                            .next();
                        if let Some(existing_slot) = maybe_existing_spec_slot {
                            if let Some(existing_cardinality_specialization) = existing_slot.get_cardinalityspecialization_slot().into_iter().next() {
                                editor.incorporate(existing_slot.edit(ctx_clone.clone())
                                    .remove_from_cardinalityspecialization(existing_cardinality_specialization.get_id()));
                            }
                            editor.incorporate(
                                existing_slot
                                    .edit(ctx_clone.clone())
                                    .add_temp_cardinalityspecialization::<OperativeSlotCardinalityRangeSpecialization>("new_cardinality_spec"),
                            )
                        } else {
                            editor.incorporate(
                                desc_op
                                    .edit(ctx_clone.clone())
                                    .add_temp_slotspecializations("new_slot_spec"),
                            );
                        }
                    });
                }
                if let Some(_unencumbered_new) = maybe_new_unencumbered_self_spec {
                    all_descendent_ops.iter().for_each(|desc_op| {
                        let maybe_existing_spec_slot = desc_op
                            .get_slotspecializations_slot()
                            .into_iter()
                            .filter(|slot_spec| {
                                slot_spec.get_roottemplateslot_slot().get_id()
                                    == template_slot.get_id()
                            })
                            .next();
                        if let Some(existing_slot) = maybe_existing_spec_slot {
                            if let Some(existing_cardinality_specialization) = existing_slot.get_cardinalityspecialization_slot().into_iter().next() {
                                editor.incorporate(existing_slot.edit(ctx_clone.clone())
                                    .remove_from_cardinalityspecialization(existing_cardinality_specialization.get_id()));
                            }
                            editor.incorporate(
                                existing_slot
                                    .edit(ctx_clone.clone())
                                    .add_temp_cardinalityspecialization::<OperativeSlotCardinalityRangeSpecialization>("new_cardinality_spec"),
                            )
                        } else {
                            editor.incorporate(
                                desc_op
                                    .edit(ctx_clone.clone())
                                    .add_temp_slotspecializations("new_slot_spec"),
                            );
                        }
                    });
                }
                if let Some(_dependent_new) = maybe_new_dependent_self_spec {
                    all_descendent_ops.iter().for_each(|desc_op| {
                        let slot_spec_in_question =
                        desc_op.get_slotspecializations_slot().into_iter().filter(|spec| {
                            spec.get_roottemplateslot_slot().get_id() == template_slot.get_id()
                        }).next();
                        if let Some(existing_slot) = slot_spec_in_question {
                        if existing_slot.get_id() == &previous_slot_spec_id.unwrap() {
                            editor.incorporate(desc_op.edit(ctx_clone.clone()).remove_from_slotspecializations(existing_slot.get_id())
                                .add_temp_slotspecializations("new_slot_spec"));
                        } else {
                            if let Some(existing_cardinality_specialization) = existing_slot.get_cardinalityspecialization_slot().into_iter().next() {
                                editor.incorporate(existing_slot.edit(ctx_clone.clone())
                                    .remove_from_cardinalityspecialization(existing_cardinality_specialization.get_id()));
                            }
                            if let Some(existing_cardinality_specialization) = existing_slot.get_cardinalityspecialization_slot().into_iter().next() {
                                editor.incorporate(existing_slot.edit(ctx_clone.clone())
                                    .remove_from_cardinalityspecialization(existing_cardinality_specialization.get_id()));
                            }
                            editor.incorporate(existing_slot.edit(ctx_clone.clone())
                                .remove_from_upstreamslotdescription(&previous_slot_spec_id.unwrap())
                                .add_temp_upstreamslotdescription::<OperativeSlotSpecialized>("new_slot_spec")
                                .add_temp_cardinalityspecialization::<OperativeSlotCardinalityRangeSpecialization>("new_cardinality_spec"));
                        }
                        } else {
                            editor.incorporate(desc_op.edit(ctx_clone.clone()).add_temp_slotspecializations("new_slot_spec"))
                        }
                    });
                }
                editor.execute().unwrap();
 },
            OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityZeroSpecialization => {
                match spec_target {
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundOrZeroSpecialization(_) => {
                        let  edit = OperativeSlotCardinalityZeroSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_cardinality_spec")
                            .add_existing_upstreamcardinality::<OperativeSlotCardinalityLowerBoundOrZeroSpecialization>(spec_target.get_id(), |na|na)
                        ;
                        if let Some(existing_self_spec) = &mut maybe_existing_self_spec {
                            editor.incorporate(&edit.clone().add_existing_specializedslot(existing_self_spec.get_id(), |na|na));
                            editor.incorporate(existing_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityZeroSpecialization>("new_cardinality_spec"));                        }
                        if let Some( new_self_spec) = maybe_new_unencumbered_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityZeroSpecialization>("new_cardinality_spec"));
                        }
                        if let Some( new_self_spec) = maybe_new_dependent_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityZeroSpecialization>("new_cardinality_spec"));
                        }
                    },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityRangeOrZero(_) => {
                        let  edit = OperativeSlotCardinalityZeroSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_cardinality_spec")
                            .add_existing_upstreamcardinality::<TemplateSlotCardinalityRangeOrZero>(spec_target.get_id(), |na|na)
                        ;
                        if let Some(existing_self_spec) = &mut maybe_existing_self_spec {
                            editor.incorporate(&edit.clone().add_existing_specializedslot(existing_self_spec.get_id(), |na|na));
                            editor.incorporate(existing_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityZeroSpecialization>("new_cardinality_spec"));                        }
                        if let Some( new_self_spec) = maybe_new_unencumbered_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityZeroSpecialization>("new_cardinality_spec"));
                        }
                        if let Some( new_self_spec) = maybe_new_dependent_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityZeroSpecialization>("new_cardinality_spec"));
                        }
                    },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityRangeSpecialization(_) => {
                        let  edit = OperativeSlotCardinalityZeroSpecialization::new(ctx_clone.clone())
                        .set_temp_id("new_cardinality_spec")
                                .add_existing_upstreamcardinality::<OperativeSlotCardinalityRangeSpecialization>(spec_target.get_id(), |na|na)
                        ;
                        if let Some(existing_self_spec) = &mut maybe_existing_self_spec {
                            editor.incorporate(&edit.clone().add_existing_specializedslot(existing_self_spec.get_id(), |na|na));
                            editor.incorporate(existing_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityZeroSpecialization>("new_cardinality_spec"));                        }
                        if let Some( new_self_spec) = maybe_new_unencumbered_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityZeroSpecialization>("new_cardinality_spec"));
                        }
                        if let Some( new_self_spec) = maybe_new_dependent_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityZeroSpecialization>("new_cardinality_spec"));
                        }
                    },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityLowerBoundOrZero(_) => {
                        let  edit = OperativeSlotCardinalityZeroSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_cardinality_spec")
                            .add_existing_upstreamcardinality::<TemplateSlotCardinalityLowerBoundOrZero>(spec_target.get_id(), |na|na)
                        ;
                        if let Some(existing_self_spec) = &mut maybe_existing_self_spec {
                            editor.incorporate(&edit.clone().add_existing_specializedslot(existing_self_spec.get_id(), |na|na));
                            editor.incorporate(existing_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityZeroSpecialization>("new_cardinality_spec"));                        }
                        if let Some( new_self_spec) = maybe_new_unencumbered_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityZeroSpecialization>("new_cardinality_spec"));
                        }
                        if let Some( new_self_spec) = maybe_new_dependent_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityZeroSpecialization>("new_cardinality_spec"));
                        }
                    },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityRange(_) => {
                        let  edit = OperativeSlotCardinalityZeroSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_cardinality_spec")
                            .add_existing_upstreamcardinality::<TemplateSlotCardinalityRange>(spec_target.get_id(), |na|na)
                        ;
                        if let Some(existing_self_spec) = &mut maybe_existing_self_spec {
                            editor.incorporate(&edit.clone().add_existing_specializedslot(existing_self_spec.get_id(), |na|na));
                            editor.incorporate(existing_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityZeroSpecialization>("new_cardinality_spec"));                        }
                        if let Some( new_self_spec) = maybe_new_unencumbered_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityZeroSpecialization>("new_cardinality_spec"));
                        }
                        if let Some( new_self_spec) = maybe_new_dependent_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityZeroSpecialization>("new_cardinality_spec"));
                        }
                    },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityLowerBound(_) => {
                        let  edit = OperativeSlotCardinalityZeroSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_cardinality_spec")
                            .add_existing_upstreamcardinality::<TemplateSlotCardinalityLowerBound>(spec_target.get_id(), |na|na)
                        ;
                        if let Some(existing_self_spec) = &mut maybe_existing_self_spec {
                            editor.incorporate(&edit.clone().add_existing_specializedslot(existing_self_spec.get_id(), |na|na));
                            editor.incorporate(existing_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityZeroSpecialization>("new_cardinality_spec"));                        }
                        if let Some( new_self_spec) = maybe_new_unencumbered_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityZeroSpecialization>("new_cardinality_spec"));
                        }
                        if let Some( new_self_spec) = maybe_new_dependent_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityZeroSpecialization>("new_cardinality_spec"));
                        }
                    },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityRangeOrZeroSpecialization(_) => {
                        let  edit = OperativeSlotCardinalityZeroSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_cardinality_spec")
                            .add_existing_upstreamcardinality::<OperativeSlotCardinalityRangeOrZeroSpecialization>(spec_target.get_id(), |na|na)
                        ;
                        if let Some(existing_self_spec) = &mut maybe_existing_self_spec {
                            editor.incorporate(&edit.clone().add_existing_specializedslot(existing_self_spec.get_id(), |na|na));
                            editor.incorporate(existing_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityZeroSpecialization>("new_cardinality_spec"));                        }
                        if let Some( new_self_spec) = maybe_new_unencumbered_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityZeroSpecialization>("new_cardinality_spec"));
                        }
                        if let Some( new_self_spec) = maybe_new_dependent_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityZeroSpecialization>("new_cardinality_spec"));
                        }
                    },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundSpecialization(_) => {
                        let  edit = OperativeSlotCardinalityZeroSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_cardinality_spec")
                            .add_existing_upstreamcardinality::<OperativeSlotCardinalityLowerBoundSpecialization>(spec_target.get_id(), |na|na)
                        ;
                        if let Some(existing_self_spec) = &mut maybe_existing_self_spec {
                            editor.incorporate(&edit.clone().add_existing_specializedslot(existing_self_spec.get_id(), |na|na));
                            editor.incorporate(existing_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityZeroSpecialization>("new_cardinality_spec"));                        }
                        if let Some( new_self_spec) = maybe_new_unencumbered_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityZeroSpecialization>("new_cardinality_spec"));
                        }
                        if let Some( new_self_spec) = maybe_new_dependent_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityZeroSpecialization>("new_cardinality_spec"));
                        }
                    },
                }
                if let Some(_existing_self_spec) = maybe_existing_self_spec {
                    all_descendent_ops.iter().for_each(|desc_op| {
                        let maybe_existing_spec_slot = desc_op
                            .get_slotspecializations_slot()
                            .into_iter()
                            .filter(|slot_spec| {
                                slot_spec.get_roottemplateslot_slot().get_id()
                                    == template_slot.get_id()
                            })
                            .next();
                        if let Some(existing_slot) = maybe_existing_spec_slot {
                            if let Some(existing_cardinality_specialization) = existing_slot.get_cardinalityspecialization_slot().into_iter().next() {
                                editor.incorporate(existing_slot.edit(ctx_clone.clone())
                                    .remove_from_cardinalityspecialization(existing_cardinality_specialization.get_id()));
                            }
                            editor.incorporate(
                                existing_slot
                                    .edit(ctx_clone.clone())
                                    .add_temp_cardinalityspecialization::<OperativeSlotCardinalityZeroSpecialization>("new_cardinality_spec"),
                            )
                        } else {
                            editor.incorporate(
                                desc_op
                                    .edit(ctx_clone.clone())
                                    .add_temp_slotspecializations("new_slot_spec"),
                            );
                        }
                    });
                }
                if let Some(_unencumbered_new) = maybe_new_unencumbered_self_spec {
                    all_descendent_ops.iter().for_each(|desc_op| {
                        let maybe_existing_spec_slot = desc_op
                            .get_slotspecializations_slot()
                            .into_iter()
                            .filter(|slot_spec| {
                                slot_spec.get_roottemplateslot_slot().get_id()
                                    == template_slot.get_id()
                            })
                            .next();
                        if let Some(existing_slot) = maybe_existing_spec_slot {
                            if let Some(existing_cardinality_specialization) = existing_slot.get_cardinalityspecialization_slot().into_iter().next() {
                                editor.incorporate(existing_slot.edit(ctx_clone.clone())
                                    .remove_from_cardinalityspecialization(existing_cardinality_specialization.get_id()));
                            }
                            editor.incorporate(
                                existing_slot
                                    .edit(ctx_clone.clone())
                                    .add_temp_cardinalityspecialization::<OperativeSlotCardinalityZeroSpecialization>("new_cardinality_spec"),
                            )
                        } else {
                            editor.incorporate(
                                desc_op
                                    .edit(ctx_clone.clone())
                                    .add_temp_slotspecializations("new_slot_spec"),
                            );
                        }
                    });
                }
                if let Some(_dependent_new) = maybe_new_dependent_self_spec {
                    all_descendent_ops.iter().for_each(|desc_op| {
                        let slot_spec_in_question =
                        desc_op.get_slotspecializations_slot().into_iter().filter(|spec| {
                            spec.get_roottemplateslot_slot().get_id() == template_slot.get_id()
                        }).next();
                        if let Some(existing_slot) = slot_spec_in_question {
                        if existing_slot.get_id() == &previous_slot_spec_id.unwrap() {
                            editor.incorporate(desc_op.edit(ctx_clone.clone()).remove_from_slotspecializations(existing_slot.get_id())
                                .add_temp_slotspecializations("new_slot_spec"));
                        } else {
                            if let Some(existing_cardinality_specialization) = existing_slot.get_cardinalityspecialization_slot().into_iter().next() {
                                editor.incorporate(existing_slot.edit(ctx_clone.clone())
                                    .remove_from_cardinalityspecialization(existing_cardinality_specialization.get_id()));
                            }
                            editor.incorporate(existing_slot.edit(ctx_clone.clone())
                                .remove_from_upstreamslotdescription(&previous_slot_spec_id.unwrap())
                                .add_temp_upstreamslotdescription::<OperativeSlotSpecialized>("new_slot_spec")
                                .add_temp_cardinalityspecialization::<OperativeSlotCardinalityZeroSpecialization>("new_cardinality_spec"));
                        }
                        } else {
                            editor.incorporate(desc_op.edit(ctx_clone.clone()).add_temp_slotspecializations("new_slot_spec"))
                        }
                    });
                }
                editor.execute().unwrap();
 },
            OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityRangeOrZeroSpecialization => {
                match spec_target {
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundOrZeroSpecialization(_) => {
                        let  edit = OperativeSlotCardinalityRangeOrZeroSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_cardinality_spec")
                                .set_upper_bound(selected_upper_bound.get())
                                .set_lower_bound(selected_lower_bound.get())
                                .add_existing_upstreamcardinality::<OperativeSlotCardinalityLowerBoundOrZeroSpecialization>(spec_target.get_id(), |na|na)
                        ;
                        if let Some(existing_self_spec) = &mut maybe_existing_self_spec {
                            editor.incorporate(&edit.clone().add_existing_specializedslot(existing_self_spec.get_id(), |na|na));
                            editor.incorporate(existing_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityRangeOrZeroSpecialization>("new_cardinality_spec"));                        }
                        if let Some( new_self_spec) = maybe_new_unencumbered_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityRangeOrZeroSpecialization>("new_cardinality_spec"));
                        }
                        if let Some( new_self_spec) = maybe_new_dependent_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityRangeOrZeroSpecialization>("new_cardinality_spec"));
                        }
                    },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityRangeOrZero(_) => {
                        let  edit = OperativeSlotCardinalityRangeOrZeroSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_cardinality_spec")
                                .set_upper_bound(selected_upper_bound.get())
                                .set_lower_bound(selected_lower_bound.get())
                                .add_existing_upstreamcardinality::<TemplateSlotCardinalityRangeOrZero>(spec_target.get_id(), |na|na)
                        ;
                        if let Some(existing_self_spec) = &mut maybe_existing_self_spec {
                            editor.incorporate(&edit.clone().add_existing_specializedslot(existing_self_spec.get_id(), |na|na));
                            editor.incorporate(existing_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityRangeOrZeroSpecialization>("new_cardinality_spec"));                        }
                        if let Some( new_self_spec) = maybe_new_unencumbered_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityRangeOrZeroSpecialization>("new_cardinality_spec"));
                        }
                        if let Some( new_self_spec) = maybe_new_dependent_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityRangeOrZeroSpecialization>("new_cardinality_spec"));
                        }
                    },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityRangeSpecialization(_) => {
                        leptos::logging::warn!("Somehow an ill-formed specialization was attempted (e.g. trying to make a lower_bound specialization for a range");
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityLowerBoundOrZero(_) => {
                        let  edit = OperativeSlotCardinalityRangeOrZeroSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_cardinality_spec")
                                .set_upper_bound(selected_upper_bound.get())
                                .set_lower_bound(selected_lower_bound.get())
                                .add_existing_upstreamcardinality::<TemplateSlotCardinalityLowerBoundOrZero>(spec_target.get_id(), |na|na)
                        ;
                        if let Some(existing_self_spec) = &mut maybe_existing_self_spec {
                            editor.incorporate(&edit.clone().add_existing_specializedslot(existing_self_spec.get_id(), |na|na));
                            editor.incorporate(existing_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityRangeOrZeroSpecialization>("new_cardinality_spec"));                        }
                        if let Some( new_self_spec) = maybe_new_unencumbered_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityRangeOrZeroSpecialization>("new_cardinality_spec"));
                        }
                        if let Some( new_self_spec) = maybe_new_dependent_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityRangeOrZeroSpecialization>("new_cardinality_spec"));
                        }
                    },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityRange(_) => {
                        leptos::logging::warn!("Somehow an ill-formed specialization was attempted (e.g. trying to make a lower_bound specialization for a range");
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityLowerBound(_) => {
                        leptos::logging::warn!("Somehow an ill-formed specialization was attempted (e.g. trying to make a lower_bound specialization for a range");
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityRangeOrZeroSpecialization(_) => {
                        let  edit = OperativeSlotCardinalityRangeOrZeroSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_cardinality_spec")
                                .set_upper_bound(selected_upper_bound.get())
                                .set_lower_bound(selected_lower_bound.get())
                                .add_existing_upstreamcardinality::<OperativeSlotCardinalityRangeOrZeroSpecialization>(spec_target.get_id(), |na|na)
                        ;
                        if let Some(existing_self_spec) = &mut maybe_existing_self_spec {
                            editor.incorporate(&edit.clone().add_existing_specializedslot(existing_self_spec.get_id(), |na|na));
                            editor.incorporate(existing_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityRangeOrZeroSpecialization>("new_cardinality_spec"));                        }
                        if let Some( new_self_spec) = maybe_new_unencumbered_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityRangeOrZeroSpecialization>("new_cardinality_spec"));
                        }
                        if let Some( new_self_spec) = maybe_new_dependent_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityRangeOrZeroSpecialization>("new_cardinality_spec"));
                        }
                    },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundSpecialization(_) => {
                        leptos::logging::warn!("Somehow an ill-formed specialization was attempted (e.g. trying to make a lower_bound specialization for a range");
 },
                }
                if let Some(_existing_self_spec) = maybe_existing_self_spec {
                    all_descendent_ops.iter().for_each(|desc_op| {
                        let maybe_existing_spec_slot = desc_op
                            .get_slotspecializations_slot()
                            .into_iter()
                            .filter(|slot_spec| {
                                slot_spec.get_roottemplateslot_slot().get_id()
                                    == template_slot.get_id()
                            })
                            .next();
                        if let Some(existing_slot) = maybe_existing_spec_slot {
                            if let Some(existing_cardinality_specialization) = existing_slot.get_cardinalityspecialization_slot().into_iter().next() {
                                editor.incorporate(existing_slot.edit(ctx_clone.clone())
                                    .remove_from_cardinalityspecialization(existing_cardinality_specialization.get_id()));
                            }
                            editor.incorporate(
                                existing_slot
                                    .edit(ctx_clone.clone())
                                    .add_temp_cardinalityspecialization::<OperativeSlotCardinalityRangeOrZeroSpecialization>("new_cardinality_spec"),
                            )
                        } else {
                            editor.incorporate(
                                desc_op
                                    .edit(ctx_clone.clone())
                                    .add_temp_slotspecializations("new_slot_spec"),
                            );
                        }
                    });
                }
                if let Some(_unencumbered_new) = maybe_new_unencumbered_self_spec {
                    all_descendent_ops.iter().for_each(|desc_op| {
                        let maybe_existing_spec_slot = desc_op
                            .get_slotspecializations_slot()
                            .into_iter()
                            .filter(|slot_spec| {
                                slot_spec.get_roottemplateslot_slot().get_id()
                                    == template_slot.get_id()
                            })
                            .next();
                        if let Some(existing_slot) = maybe_existing_spec_slot {
                            if let Some(existing_cardinality_specialization) = existing_slot.get_cardinalityspecialization_slot().into_iter().next() {
                                editor.incorporate(existing_slot.edit(ctx_clone.clone())
                                    .remove_from_cardinalityspecialization(existing_cardinality_specialization.get_id()));
                            }
                            editor.incorporate(
                                existing_slot
                                    .edit(ctx_clone.clone())
                                    .add_temp_cardinalityspecialization::<OperativeSlotCardinalityRangeOrZeroSpecialization>("new_cardinality_spec"),
                            )
                        } else {
                            editor.incorporate(
                                desc_op
                                    .edit(ctx_clone.clone())
                                    .add_temp_slotspecializations("new_slot_spec"),
                            );
                        }
                    });
                }
                if let Some(_dependent_new) = maybe_new_dependent_self_spec {
                    all_descendent_ops.iter().for_each(|desc_op| {
                        let slot_spec_in_question =
                        desc_op.get_slotspecializations_slot().into_iter().filter(|spec| {
                            spec.get_roottemplateslot_slot().get_id() == template_slot.get_id()
                        }).next();
                        if let Some(existing_slot) = slot_spec_in_question {
                        if existing_slot.get_id() == &previous_slot_spec_id.unwrap() {
                            editor.incorporate(desc_op.edit(ctx_clone.clone()).remove_from_slotspecializations(existing_slot.get_id())
                                .add_temp_slotspecializations("new_slot_spec"));
                        } else {
                            if let Some(existing_cardinality_specialization) = existing_slot.get_cardinalityspecialization_slot().into_iter().next() {
                                editor.incorporate(existing_slot.edit(ctx_clone.clone())
                                    .remove_from_cardinalityspecialization(existing_cardinality_specialization.get_id()));
                            }
                            editor.incorporate(existing_slot.edit(ctx_clone.clone())
                                .remove_from_upstreamslotdescription(&previous_slot_spec_id.unwrap())
                                .add_temp_upstreamslotdescription::<OperativeSlotSpecialized>("new_slot_spec")
                                .add_temp_cardinalityspecialization::<OperativeSlotCardinalityRangeOrZeroSpecialization>("new_cardinality_spec"));
                        }
                        } else {
                            editor.incorporate(desc_op.edit(ctx_clone.clone()).add_temp_slotspecializations("new_slot_spec"))
                        }
                    });
                }
                editor.execute().unwrap();
 },
            OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityLowerBoundSpecialization => {
                match spec_target {
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundOrZeroSpecialization(_) => {
                        let  edit = OperativeSlotCardinalityLowerBoundSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_cardinality_spec")
                                .set_lower_bound(selected_lower_bound.get())
                                .add_existing_upstreamcardinality::<OperativeSlotCardinalityLowerBoundOrZeroSpecialization>(spec_target.get_id(), |na|na)
                        ;
                        if let Some(existing_self_spec) = &mut maybe_existing_self_spec {
                            editor.incorporate(&edit.clone().add_existing_specializedslot(existing_self_spec.get_id(), |na|na));
                            editor.incorporate(existing_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityLowerBoundSpecialization>("new_cardinality_spec"));                        }
                        if let Some( new_self_spec) = maybe_new_unencumbered_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityLowerBoundSpecialization>("new_cardinality_spec"));
                        }
                        if let Some( new_self_spec) = maybe_new_dependent_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityLowerBoundSpecialization>("new_cardinality_spec"));
                        }
                    },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityRangeOrZero(_) => {
                        leptos::logging::warn!("Somehow an ill-formed specialization was attempted (e.g. trying to make a lower_bound specialization for a range");
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityRangeSpecialization(_) => {
                        leptos::logging::warn!("Somehow an ill-formed specialization was attempted (e.g. trying to make a lower_bound specialization for a range");
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityLowerBoundOrZero(_) => {
                        let  edit = OperativeSlotCardinalityLowerBoundSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_cardinality_spec")
                                .set_lower_bound(selected_lower_bound.get())
                                .add_existing_upstreamcardinality::<TemplateSlotCardinalityLowerBoundOrZero>(spec_target.get_id(), |na|na)
                        ;
                        if let Some(existing_self_spec) = &mut maybe_existing_self_spec {
                            editor.incorporate(&edit.clone().add_existing_specializedslot(existing_self_spec.get_id(), |na|na));
                            editor.incorporate(existing_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityLowerBoundSpecialization>("new_cardinality_spec"));                        }
                        if let Some( new_self_spec) = maybe_new_unencumbered_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityLowerBoundSpecialization>("new_cardinality_spec"));
                        }
                        if let Some( new_self_spec) = maybe_new_dependent_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityLowerBoundSpecialization>("new_cardinality_spec"));
                        }
                    },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityRange(_) => {
                        leptos::logging::warn!("Somehow an ill-formed specialization was attempted (e.g. trying to make a lower_bound specialization for a range");
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityLowerBound(_) => {
                        let  edit = OperativeSlotCardinalityLowerBoundSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_cardinality_spec")
                                .set_lower_bound(selected_lower_bound.get())
                                .add_existing_upstreamcardinality::<TemplateSlotCardinalityLowerBound>(spec_target.get_id(), |na|na)
                        ;
                        if let Some(existing_self_spec) = &mut maybe_existing_self_spec {
                            editor.incorporate(&edit.clone().add_existing_specializedslot(existing_self_spec.get_id(), |na|na));
                            editor.incorporate(existing_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityLowerBoundSpecialization>("new_cardinality_spec"));                        }
                        if let Some( new_self_spec) = maybe_new_unencumbered_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityLowerBoundSpecialization>("new_cardinality_spec"));
                        }
                        if let Some( new_self_spec) = maybe_new_dependent_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityLowerBoundSpecialization>("new_cardinality_spec"));
                        }
                    },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityRangeOrZeroSpecialization(_) => {
                        leptos::logging::warn!("Somehow an ill-formed specialization was attempted (e.g. trying to make a lower_bound specialization for a range");
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundSpecialization(_) => {
                        let  edit = OperativeSlotCardinalityLowerBoundSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_cardinality_spec")
                                .set_lower_bound(selected_lower_bound.get())
                                .add_existing_upstreamcardinality::<OperativeSlotCardinalityLowerBoundSpecialization>(spec_target.get_id(), |na|na)
                        ;
                        if let Some(existing_self_spec) = &mut maybe_existing_self_spec {
                            editor.incorporate(&edit.clone().add_existing_specializedslot(existing_self_spec.get_id(), |na|na));
                            editor.incorporate(existing_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityLowerBoundSpecialization>("new_cardinality_spec"));                        }
                        if let Some( new_self_spec) = maybe_new_unencumbered_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityLowerBoundSpecialization>("new_cardinality_spec"));
                        }
                        if let Some( new_self_spec) = maybe_new_dependent_self_spec.clone() {
                            editor.incorporate(&edit.clone().add_temp_specializedslot("new_slot_spec"));
                            editor.incorporate(&new_self_spec.add_temp_cardinalityspecialization::<OperativeSlotCardinalityLowerBoundSpecialization>("new_cardinality_spec"));
                        }
                    },
                }
                if let Some(_existing_self_spec) = maybe_existing_self_spec {
                    all_descendent_ops.iter().for_each(|desc_op| {
                        let maybe_existing_spec_slot = desc_op
                            .get_slotspecializations_slot()
                            .into_iter()
                            .filter(|slot_spec| {
                                slot_spec.get_roottemplateslot_slot().get_id()
                                    == template_slot.get_id()
                            })
                            .next();
                        if let Some(existing_slot) = maybe_existing_spec_slot {
                            if let Some(existing_cardinality_specialization) = existing_slot.get_cardinalityspecialization_slot().into_iter().next() {
                                editor.incorporate(existing_slot.edit(ctx_clone.clone())
                                    .remove_from_cardinalityspecialization(existing_cardinality_specialization.get_id()));
                            }
                            editor.incorporate(
                                existing_slot
                                    .edit(ctx_clone.clone())
                                    .add_temp_cardinalityspecialization::<OperativeSlotCardinalityLowerBoundSpecialization>("new_cardinality_spec"),
                            )
                        } else {
                            editor.incorporate(
                                desc_op
                                    .edit(ctx_clone.clone())
                                    .add_temp_slotspecializations("new_slot_spec"),
                            );
                        }
                    });
                }
                if let Some(_unencumbered_new) = maybe_new_unencumbered_self_spec {
                    all_descendent_ops.iter().for_each(|desc_op| {
                        let maybe_existing_spec_slot = desc_op
                            .get_slotspecializations_slot()
                            .into_iter()
                            .filter(|slot_spec| {
                                slot_spec.get_roottemplateslot_slot().get_id()
                                    == template_slot.get_id()
                            })
                            .next();
                        if let Some(existing_slot) = maybe_existing_spec_slot {
                            if let Some(existing_cardinality_specialization) = existing_slot.get_cardinalityspecialization_slot().into_iter().next() {
                                editor.incorporate(existing_slot.edit(ctx_clone.clone())
                                    .remove_from_cardinalityspecialization(existing_cardinality_specialization.get_id()));
                            }
                            editor.incorporate(
                                existing_slot
                                    .edit(ctx_clone.clone())
                                    .add_temp_cardinalityspecialization::<OperativeSlotCardinalityLowerBoundSpecialization>("new_cardinality_spec"),
                            )
                        } else {
                            editor.incorporate(
                                desc_op
                                    .edit(ctx_clone.clone())
                                    .add_temp_slotspecializations("new_slot_spec"),
                            );
                        }
                    });
                }
                if let Some(_dependent_new) = maybe_new_dependent_self_spec {
                    all_descendent_ops.iter().for_each(|desc_op| {
                        let slot_spec_in_question =
                        desc_op.get_slotspecializations_slot().into_iter().filter(|spec| {
                            spec.get_roottemplateslot_slot().get_id() == template_slot.get_id()
                        }).next();
                        if let Some(existing_slot) = slot_spec_in_question {
                        if existing_slot.get_id() == &previous_slot_spec_id.unwrap() {
                            editor.incorporate(desc_op.edit(ctx_clone.clone()).remove_from_slotspecializations(existing_slot.get_id())
                                    .add_temp_slotspecializations("new_slot_spec"));
                        } else {
                            if let Some(existing_cardinality_specialization) = existing_slot.get_cardinalityspecialization_slot().into_iter().next() {
                                editor.incorporate(existing_slot.edit(ctx_clone.clone())
                                    .remove_from_cardinalityspecialization(existing_cardinality_specialization.get_id()));
                            }
                            editor.incorporate(existing_slot.edit(ctx_clone.clone())
                                .remove_from_upstreamslotdescription(&previous_slot_spec_id.unwrap())
                                .add_temp_upstreamslotdescription::<OperativeSlotSpecialized>("new_slot_spec")
                                .add_temp_cardinalityspecialization::<OperativeSlotCardinalityLowerBoundSpecialization>("new_cardinality_spec"));
                        }
                        } else {
                            editor.incorporate(desc_op.edit(ctx_clone.clone()).add_temp_slotspecializations("new_slot_spec"))
                        }
                    });
                }
                editor.execute().unwrap();
            },
        };
    };

    view! {
        <LeafSection>
            <div class=move || {
                match is_adding.get() {
                    true => "hidden",
                    false => "",
                }
            }>
                <Button on:click=move |_| is_adding.set(true)>Add Specialization</Button>
            </div>
            // <Show when=move || is_adding.get()>
            <div class=move || if is_adding.get() { "" } else { "hidden" }>
                <div>
                    <SignalSelectWithOptions value=selected_spec options=options />
                </div>

                {move || match current_input_variant() {
                    CardinalityInputTypeOptions::LowerBoundOnly => {
                        EitherOf3::A(
                            view! {
                                <div>
                                    <SignalTextInput
                                        prop:type="number"
                                        value=selected_lower_bound
                                        attr:min=move || prev_bounds.get().min
                                    />
                                </div>
                            },
                        )
                    }
                    CardinalityInputTypeOptions::Both => {
                        EitherOf3::B(
                            view! {
                                <div>
                                    <SignalTextInput
                                        prop:type="number"
                                        value=selected_lower_bound
                                        attr:min=move || prev_bounds.get().min
                                    />
                                </div>
                                <div>
                                    <SignalTextInput
                                        prop:type="number"
                                        value=selected_upper_bound
                                        attr:min=move || selected_lower_bound.get()
                                        attr:max=move || prev_bounds.get().max
                                    />
                                </div>
                            },
                        )
                    }
                    CardinalityInputTypeOptions::None => EitherOf3::C(view! { <div>Fixed</div> }),
                }}

                <div>
                    <Button
                        on:click=on_save.clone()
                        attr:disabled=move || {
                            is_downstream_slot_outside_of_attempted_bounds.get()
                                || are_selected_values_outside_of_previous_bounds.get()
                        }
                    >
                        Save
                    </Button>
                    <Button on:click=move |_| is_adding.set(false)>Cancel</Button>
                </div>
            </div>
        // </Show>
        </LeafSection>
    }
}
