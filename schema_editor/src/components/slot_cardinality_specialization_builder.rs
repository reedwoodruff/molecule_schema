use std::collections::BTreeSet;

use leptos::either::{self, Either, EitherOf3};
use schema_editor_generated_toolkit::prelude::*;
use web_sys::MouseEvent;

use super::{
    common::*,
    operative_slot_section::OperativeSlotContext,
    utils::{
        get_all_descendent_operators, get_all_operatives_which_impl_trait_set,
        get_all_operatives_which_satisfy_specializable, get_all_traits_in_specialization,
    },
    workspace::WorkspaceState,
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
            OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalitySingleSpecialization(item) => {
                CardinalityInfo {min: 1, max: Some(1), zero_allowed: false}
            },
            OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityRangeSpecialization(item) => {
                CardinalityInfo {min: item.get_lower_bound_field(), max: Some(item.get_upper_bound_field()), zero_allowed: false}
            },
            OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityZeroSpecialization(item) => {
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
    operative: RGSOConcrete<OperativeConcrete, Schema>,
    spec_target: OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject,
) -> impl IntoView {
    let ctx = use_context::<SharedGraph<Schema>>().unwrap();
    let OperativeSlotContext {
        max_downstream_slotted_instances,
        operative,
        slot_item,
        maybe_childest_type_spec,
        maybe_childest_cardinality_spec,
    } = use_context::<OperativeSlotContext>().unwrap();
    let WorkspaceState {
        schema,
        selected_tab,
    } = use_context::<WorkspaceState>().unwrap();

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

    let choose_bounds_view = move || match current_input_variant() {
        CardinalityInputTypeOptions::LowerBoundOnly => EitherOf3::A(view! {
            <div>
            <SignalTextInput prop:type="number" value=selected_lower_bound attr:min=move || prev_bounds.get().min />
            </div>
        }),
        CardinalityInputTypeOptions::Both => EitherOf3::B(view! {
            <div>
            <SignalTextInput prop:type="number" value=selected_lower_bound attr:min = move || prev_bounds.get().min />
            </div>
            <div>
            <SignalTextInput prop:type="number" value=selected_upper_bound attr:min = move || selected_lower_bound.get() attr:max=move || prev_bounds.get().max />
            </div>
        }),
        CardinalityInputTypeOptions::None => EitherOf3::C(view! {
            <div>
            Fixed
            </div>
        }),
    };

    let is_downstream_slot_outside_of_attempted_bounds = move || {
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
    };
    let is_downstream_slot_outside_of_attempted_bounds_clone =
        is_downstream_slot_outside_of_attempted_bounds.clone();

    let are_selected_values_outside_of_previous_bounds = move || {
        let prev = prev_bounds.get();
        match selected_spec.get().unwrap() {
            OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityLowerBoundOrZeroSpecialization => {
                false
            },
            OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalitySingleSpecialization => {
                prev.min > 1
            },
            OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityRangeSpecialization => {
                prev.max.is_some_and(|prev_max| prev_max < selected_upper_bound.get()) || prev.min > selected_lower_bound.get()
            },
            OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityZeroSpecialization => {
                prev.zero_allowed == false && prev.min > 0
            },
            OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityRangeOrZeroSpecialization => {
                prev.max.is_some_and(|prev_max| prev_max < selected_upper_bound.get()) || prev.min > selected_lower_bound.get()
            },
            OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityLowerBoundSpecialization => {
                prev.min < selected_lower_bound.get()
            },
        }
    };
    let are_selected_values_outside_of_previous_bounds_clone =
        are_selected_values_outside_of_previous_bounds.clone();

    let ctx_clone = ctx.clone();

    let operative_clone = operative.clone();
    let spec_target_clone = spec_target.clone();

    let on_save = move |_e: MouseEvent| {
        let operative = operative_clone.clone();
        let operative_clone = operative.clone();
        let mut all_descendent_ops_and_this_op = BTreeSet::new();
        all_descendent_ops_and_this_op.insert(operative_clone.clone());
        get_all_descendent_operators(operative_clone, &mut all_descendent_ops_and_this_op);

        if is_downstream_slot_outside_of_attempted_bounds() {
            leptos::logging::warn!(
                "Some downstream operative has an incompatible number of instances slotted"
            );
            return ();
        }
        if are_selected_values_outside_of_previous_bounds() {
            leptos::logging::warn!(
                "Selected values are incompatible with previous cardinality constraints"
            );
        }
        let operative_clone = operative.clone();
        match selected_spec.get().unwrap() {
            OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityLowerBoundOrZeroSpecialization => {
                let mut editor = operative_clone.edit(ctx_clone.clone());
                match spec_target {
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundOrZeroSpecialization(_) => {
                        editor.incorporate(OperativeSlotCardinalityLowerBoundOrZeroSpecialization::new(ctx_clone.clone())
                                .set_temp_id("new_slot_spec")
                                .set_lower_bound(selected_lower_bound.get())
                                .add_existing_specializer(operative_clone.get_id(), |na|na)
                                .add_existing_roottemplateslot(slot_item.get_id(), |na|na)
                                .add_existing_specializationtarget::<OperativeSlotCardinalityLowerBoundOrZeroSpecialization>(spec_target.get_id(), |na|na)
                        );
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityRangeOrZero(_) => {
                        leptos::logging::warn!("Somehow an ill-formed specialization was attempted (e.g. trying to make a lower_bound specialization for a range");
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityRangeSpecialization(_) => {
                        leptos::logging::warn!("Somehow an ill-formed specialization was attempted (e.g. trying to make a lower_bound specialization for a range");
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityLowerBoundOrZero(_) => {
                        editor.incorporate(OperativeSlotCardinalityLowerBoundOrZeroSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_slot_spec")
                                .set_lower_bound(selected_lower_bound.get())
                                .add_existing_specializer(operative_clone.get_id(), |na|na)
                                .add_existing_roottemplateslot(slot_item.get_id(), |na|na)
                                .add_existing_specializationtarget::<TemplateSlotCardinalityLowerBoundOrZero>(spec_target.get_id(), |na|na)
                        );
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
                all_descendent_ops_and_this_op.into_iter().for_each(|desc_op| {
                    editor.incorporate(
                        desc_op
                            .edit(ctx_clone.clone())
                            .add_temp_slotcardinalityspecializations::<OperativeSlotCardinalityLowerBoundOrZeroSpecialization>(
                                "new_slot_spec",
                            ),
                    );
                });
                editor.execute().unwrap();
            },
            OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalitySingleSpecialization => {
                let mut editor = operative_clone.edit(ctx_clone.clone());
                match spec_target {
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundOrZeroSpecialization(_) => {
                        editor.incorporate(OperativeSlotCardinalitySingleSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_slot_spec")
                                .add_existing_specializer(operative_clone.get_id(), |na|na)
                                .add_existing_roottemplateslot(slot_item.get_id(), |na|na)
                                .add_existing_specializationtarget::<OperativeSlotCardinalityLowerBoundOrZeroSpecialization>(spec_target.get_id(), |na|na)
                        );
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityRangeOrZero(_) => {
                        editor.incorporate(OperativeSlotCardinalitySingleSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_slot_spec")
                                .add_existing_specializer(operative_clone.get_id(), |na|na)
                                .add_existing_roottemplateslot(slot_item.get_id(), |na|na)
                                .add_existing_specializationtarget::<TemplateSlotCardinalityRangeOrZero>(spec_target.get_id(), |na|na)
                        );
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityRangeSpecialization(_) => {
                        editor.incorporate(OperativeSlotCardinalitySingleSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_slot_spec")
                                .add_existing_specializer(operative_clone.get_id(), |na|na)
                                .add_existing_roottemplateslot(slot_item.get_id(), |na|na)
                                .add_existing_specializationtarget::<OperativeSlotCardinalityRangeSpecialization>(spec_target.get_id(), |na|na)
                        );
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityLowerBoundOrZero(_) => {
                        editor.incorporate(OperativeSlotCardinalitySingleSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_slot_spec")
                                .add_existing_specializer(operative_clone.get_id(), |na|na)
                                .add_existing_roottemplateslot(slot_item.get_id(), |na|na)
                                .add_existing_specializationtarget::<TemplateSlotCardinalityLowerBoundOrZero>(spec_target.get_id(), |na|na)
                        );
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityRange(_) => {
                        editor.incorporate(OperativeSlotCardinalitySingleSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_slot_spec")
                                .add_existing_specializer(operative_clone.get_id(), |na|na)
                                .add_existing_roottemplateslot(slot_item.get_id(), |na|na)
                                .add_existing_specializationtarget::<TemplateSlotCardinalityRange>(spec_target.get_id(), |na|na)
                        );
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityLowerBound(_) => {
                        editor.incorporate(OperativeSlotCardinalitySingleSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_slot_spec")
                                .add_existing_specializer(operative_clone.get_id(), |na|na)
                                .add_existing_roottemplateslot(slot_item.get_id(), |na|na)
                                .add_existing_specializationtarget::<TemplateSlotCardinalityLowerBound>(spec_target.get_id(), |na|na)
                        );
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityRangeOrZeroSpecialization(_) => {
                        editor.incorporate(OperativeSlotCardinalitySingleSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_slot_spec")
                                .add_existing_specializer(operative_clone.get_id(), |na|na)
                                .add_existing_roottemplateslot(slot_item.get_id(), |na|na)
                                .add_existing_specializationtarget::<OperativeSlotCardinalityRangeOrZeroSpecialization>(spec_target.get_id(), |na|na)
                        );
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundSpecialization(_) => {
                        editor.incorporate(OperativeSlotCardinalitySingleSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_slot_spec")
                                .add_existing_specializer(operative_clone.get_id(), |na|na)
                                .add_existing_roottemplateslot(slot_item.get_id(), |na|na)
                                .add_existing_specializationtarget::<OperativeSlotCardinalityLowerBoundSpecialization>(spec_target.get_id(), |na|na)
                        );
 },
                }
                all_descendent_ops_and_this_op.into_iter().for_each(|desc_op| {
                    editor.incorporate(
                        desc_op
                            .edit(ctx_clone.clone())
                            .add_temp_slotcardinalityspecializations::<OperativeSlotCardinalitySingleSpecialization>(
                                "new_slot_spec",
                            ),
                    );
                });
                editor.execute().unwrap();
 },
            OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityRangeSpecialization => {
                let mut editor = operative_clone.edit(ctx_clone.clone());
                match spec_target {
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundOrZeroSpecialization(_) => {
                        editor.incorporate(OperativeSlotCardinalityRangeSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_slot_spec")
                                .set_upper_bound(selected_upper_bound.get())
                                .set_lower_bound(selected_lower_bound.get())
                                .add_existing_specializer(operative_clone.get_id(), |na|na)
                                .add_existing_roottemplateslot(slot_item.get_id(), |na|na)
                                .add_existing_specializationtarget::<OperativeSlotCardinalityLowerBoundOrZeroSpecialization>(spec_target.get_id(), |na|na)
                        );
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityRangeOrZero(_) => {
                        editor.incorporate(OperativeSlotCardinalityRangeSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_slot_spec")
                                .set_upper_bound(selected_upper_bound.get())
                                .set_lower_bound(selected_lower_bound.get())
                                .add_existing_specializer(operative_clone.get_id(), |na|na)
                                .add_existing_roottemplateslot(slot_item.get_id(), |na|na)
                                .add_existing_specializationtarget::<TemplateSlotCardinalityRangeOrZero>(spec_target.get_id(), |na|na)
                        );
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityRangeSpecialization(_) => {
                        editor.incorporate(OperativeSlotCardinalityRangeSpecialization::new(ctx_clone.clone())
                        .set_temp_id("new_slot_spec")
                                .set_upper_bound(selected_upper_bound.get())
                                .set_lower_bound(selected_lower_bound.get())
                                .add_existing_specializer(operative_clone.get_id(), |na|na)
                                .add_existing_roottemplateslot(slot_item.get_id(), |na|na)
                                .add_existing_specializationtarget::<OperativeSlotCardinalityRangeSpecialization>(spec_target.get_id(), |na|na)
                        );
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityLowerBoundOrZero(_) => {
                        editor.incorporate(OperativeSlotCardinalityRangeSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_slot_spec")
                                .set_upper_bound(selected_upper_bound.get())
                                .set_lower_bound(selected_lower_bound.get())
                                .add_existing_specializer(operative_clone.get_id(), |na|na)
                                .add_existing_roottemplateslot(slot_item.get_id(), |na|na)
                                .add_existing_specializationtarget::<TemplateSlotCardinalityLowerBoundOrZero>(spec_target.get_id(), |na|na)
                        );
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityRange(_) => {
                        editor.incorporate(OperativeSlotCardinalityRangeSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_slot_spec")
                                .set_upper_bound(selected_upper_bound.get())
                                .set_lower_bound(selected_lower_bound.get())
                                .add_existing_specializer(operative_clone.get_id(), |na|na)
                                .add_existing_roottemplateslot(slot_item.get_id(), |na|na)
                                .add_existing_specializationtarget::<TemplateSlotCardinalityRange>(spec_target.get_id(), |na|na)
                        );
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityLowerBound(_) => {
                        editor.incorporate(OperativeSlotCardinalityRangeSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_slot_spec")
                                .set_upper_bound(selected_upper_bound.get())
                                .set_lower_bound(selected_lower_bound.get())
                                .add_existing_specializer(operative_clone.get_id(), |na|na)
                                .add_existing_roottemplateslot(slot_item.get_id(), |na|na)
                                .add_existing_specializationtarget::<TemplateSlotCardinalityLowerBound>(spec_target.get_id(), |na|na)
                        );
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityRangeOrZeroSpecialization(_) => {
                        editor.incorporate(OperativeSlotCardinalityRangeSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_slot_spec")
                                .set_upper_bound(selected_upper_bound.get())
                                .set_lower_bound(selected_lower_bound.get())
                                .add_existing_specializer(operative_clone.get_id(), |na|na)
                                .add_existing_roottemplateslot(slot_item.get_id(), |na|na)
                                .add_existing_specializationtarget::<OperativeSlotCardinalityRangeOrZeroSpecialization>(spec_target.get_id(), |na|na)
                        );
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundSpecialization(_) => {
                        editor.incorporate(OperativeSlotCardinalityRangeSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_slot_spec")
                                .set_upper_bound(selected_upper_bound.get())
                                .set_lower_bound(selected_lower_bound.get())
                                .add_existing_specializer(operative_clone.get_id(), |na|na)
                                .add_existing_roottemplateslot(slot_item.get_id(), |na|na)
                                .add_existing_specializationtarget::<OperativeSlotCardinalityLowerBoundSpecialization>(spec_target.get_id(), |na|na)
                        );
 },
                }
                all_descendent_ops_and_this_op.into_iter().for_each(|desc_op| {
                    editor.incorporate(
                        desc_op
                            .edit(ctx_clone.clone())
                            .add_temp_slotcardinalityspecializations::<OperativeSlotCardinalityRangeSpecialization>(
                                "new_slot_spec",
                            ),
                    );
                });
                editor.execute().unwrap();
 },
            OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityZeroSpecialization => {
                let mut editor = operative_clone.edit(ctx_clone.clone());
                match spec_target {
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundOrZeroSpecialization(_) => {
                        editor.incorporate(OperativeSlotCardinalityZeroSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_slot_spec")
                                .add_existing_specializer(operative_clone.get_id(), |na|na)
                                .add_existing_roottemplateslot(slot_item.get_id(), |na|na)
                                .add_existing_specializationtarget::<OperativeSlotCardinalityLowerBoundOrZeroSpecialization>(spec_target.get_id(), |na|na)
                        );
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityRangeOrZero(_) => {
                        editor.incorporate(OperativeSlotCardinalityZeroSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_slot_spec")
                                .add_existing_specializer(operative_clone.get_id(), |na|na)
                                .add_existing_roottemplateslot(slot_item.get_id(), |na|na)
                                .add_existing_specializationtarget::<TemplateSlotCardinalityRangeOrZero>(spec_target.get_id(), |na|na)
                        );
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityRangeSpecialization(_) => {
                        editor.incorporate(OperativeSlotCardinalityZeroSpecialization::new(ctx_clone.clone())
                        .set_temp_id("new_slot_spec")
                                .add_existing_specializer(operative_clone.get_id(), |na|na)
                                .add_existing_roottemplateslot(slot_item.get_id(), |na|na)
                                .add_existing_specializationtarget::<OperativeSlotCardinalityRangeSpecialization>(spec_target.get_id(), |na|na)
                        );
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityLowerBoundOrZero(_) => {
                        editor.incorporate(OperativeSlotCardinalityZeroSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_slot_spec")
                                .add_existing_specializer(operative_clone.get_id(), |na|na)
                                .add_existing_roottemplateslot(slot_item.get_id(), |na|na)
                                .add_existing_specializationtarget::<TemplateSlotCardinalityLowerBoundOrZero>(spec_target.get_id(), |na|na)
                        );
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityRange(_) => {
                        editor.incorporate(OperativeSlotCardinalityZeroSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_slot_spec")
                                .add_existing_specializer(operative_clone.get_id(), |na|na)
                                .add_existing_roottemplateslot(slot_item.get_id(), |na|na)
                                .add_existing_specializationtarget::<TemplateSlotCardinalityRange>(spec_target.get_id(), |na|na)
                        );
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityLowerBound(_) => {
                        editor.incorporate(OperativeSlotCardinalityZeroSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_slot_spec")
                                .add_existing_specializer(operative_clone.get_id(), |na|na)
                                .add_existing_roottemplateslot(slot_item.get_id(), |na|na)
                                .add_existing_specializationtarget::<TemplateSlotCardinalityLowerBound>(spec_target.get_id(), |na|na)
                        );
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityRangeOrZeroSpecialization(_) => {
                        editor.incorporate(OperativeSlotCardinalityZeroSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_slot_spec")
                                .add_existing_specializer(operative_clone.get_id(), |na|na)
                                .add_existing_roottemplateslot(slot_item.get_id(), |na|na)
                                .add_existing_specializationtarget::<OperativeSlotCardinalityRangeOrZeroSpecialization>(spec_target.get_id(), |na|na)
                        );
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundSpecialization(_) => {
                        editor.incorporate(OperativeSlotCardinalityZeroSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_slot_spec")
                                .add_existing_specializer(operative_clone.get_id(), |na|na)
                                .add_existing_roottemplateslot(slot_item.get_id(), |na|na)
                                .add_existing_specializationtarget::<OperativeSlotCardinalityLowerBoundSpecialization>(spec_target.get_id(), |na|na)
                        );
 },
                }
                all_descendent_ops_and_this_op.into_iter().for_each(|desc_op| {
                    editor.incorporate(
                        desc_op
                            .edit(ctx_clone.clone())
                            .add_temp_slotcardinalityspecializations::<OperativeSlotCardinalityZeroSpecialization>(
                                "new_slot_spec",
                            ),
                    );
                });
                editor.execute().unwrap();
 },
            OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityRangeOrZeroSpecialization => {
                let mut editor = operative_clone.edit(ctx_clone.clone());
                match spec_target {
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundOrZeroSpecialization(_) => {
                        editor.incorporate(OperativeSlotCardinalityRangeOrZeroSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_slot_spec")
                                .set_upper_bound(selected_upper_bound.get())
                                .set_lower_bound(selected_lower_bound.get())
                                .add_existing_specializer(operative_clone.get_id(), |na|na)
                                .add_existing_roottemplateslot(slot_item.get_id(), |na|na)
                                .add_existing_specializationtarget::<OperativeSlotCardinalityLowerBoundOrZeroSpecialization>(spec_target.get_id(), |na|na)
                        );
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityRangeOrZero(_) => {
                        editor.incorporate(OperativeSlotCardinalityRangeOrZeroSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_slot_spec")
                                .set_upper_bound(selected_upper_bound.get())
                                .set_lower_bound(selected_lower_bound.get())
                                .add_existing_specializer(operative_clone.get_id(), |na|na)
                                .add_existing_roottemplateslot(slot_item.get_id(), |na|na)
                                .add_existing_specializationtarget::<TemplateSlotCardinalityRangeOrZero>(spec_target.get_id(), |na|na)
                        );
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityRangeSpecialization(_) => {
                        leptos::logging::warn!("Somehow an ill-formed specialization was attempted (e.g. trying to make a lower_bound specialization for a range");
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityLowerBoundOrZero(_) => {
                        editor.incorporate(OperativeSlotCardinalityRangeOrZeroSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_slot_spec")
                                .set_upper_bound(selected_upper_bound.get())
                                .set_lower_bound(selected_lower_bound.get())
                                .add_existing_specializer(operative_clone.get_id(), |na|na)
                                .add_existing_roottemplateslot(slot_item.get_id(), |na|na)
                                .add_existing_specializationtarget::<TemplateSlotCardinalityLowerBoundOrZero>(spec_target.get_id(), |na|na)
                        );
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityRange(_) => {
                        leptos::logging::warn!("Somehow an ill-formed specialization was attempted (e.g. trying to make a lower_bound specialization for a range");
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityLowerBound(_) => {
                        leptos::logging::warn!("Somehow an ill-formed specialization was attempted (e.g. trying to make a lower_bound specialization for a range");
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityRangeOrZeroSpecialization(_) => {
                        editor.incorporate(OperativeSlotCardinalityRangeOrZeroSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_slot_spec")
                                .set_upper_bound(selected_upper_bound.get())
                                .set_lower_bound(selected_lower_bound.get())
                                .add_existing_specializer(operative_clone.get_id(), |na|na)
                                .add_existing_roottemplateslot(slot_item.get_id(), |na|na)
                                .add_existing_specializationtarget::<OperativeSlotCardinalityRangeOrZeroSpecialization>(spec_target.get_id(), |na|na)
                        );
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundSpecialization(_) => {
                        leptos::logging::warn!("Somehow an ill-formed specialization was attempted (e.g. trying to make a lower_bound specialization for a range");
 },
                }
                all_descendent_ops_and_this_op.into_iter().for_each(|desc_op| {
                    editor.incorporate(
                        desc_op
                            .edit(ctx_clone.clone())
                            .add_temp_slotcardinalityspecializations::<OperativeSlotCardinalityRangeOrZeroSpecialization>(
                                "new_slot_spec",
                            ),
                    );
                });
                editor.execute().unwrap();
 },
            OperativeSlotCardinalitySpecializationTraitObjectDiscriminants::OperativeSlotCardinalityLowerBoundSpecialization => {
                let mut editor = operative_clone.edit(ctx_clone.clone());
                match spec_target {
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundOrZeroSpecialization(_) => {
                        editor.incorporate(OperativeSlotCardinalityLowerBoundSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_slot_spec")
                                .set_lower_bound(selected_lower_bound.get())
                                .add_existing_specializer(operative_clone.get_id(), |na|na)
                                .add_existing_roottemplateslot(slot_item.get_id(), |na|na)
                                .add_existing_specializationtarget::<OperativeSlotCardinalityLowerBoundOrZeroSpecialization>(spec_target.get_id(), |na|na)
                        );
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityRangeOrZero(_) => {
                        leptos::logging::warn!("Somehow an ill-formed specialization was attempted (e.g. trying to make a lower_bound specialization for a range");
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityRangeSpecialization(_) => {
                        leptos::logging::warn!("Somehow an ill-formed specialization was attempted (e.g. trying to make a lower_bound specialization for a range");
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityLowerBoundOrZero(_) => {
                        editor.incorporate(OperativeSlotCardinalityLowerBoundSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_slot_spec")
                                .set_lower_bound(selected_lower_bound.get())
                                .add_existing_specializer(operative_clone.get_id(), |na|na)
                                .add_existing_roottemplateslot(slot_item.get_id(), |na|na)
                                .add_existing_specializationtarget::<TemplateSlotCardinalityLowerBoundOrZero>(spec_target.get_id(), |na|na)
                        );
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityRange(_) => {
                        leptos::logging::warn!("Somehow an ill-formed specialization was attempted (e.g. trying to make a lower_bound specialization for a range");
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityLowerBound(_) => {
                        editor.incorporate(OperativeSlotCardinalityLowerBoundSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_slot_spec")
                                .set_lower_bound(selected_lower_bound.get())
                                .add_existing_specializer(operative_clone.get_id(), |na|na)
                                .add_existing_roottemplateslot(slot_item.get_id(), |na|na)
                                .add_existing_specializationtarget::<TemplateSlotCardinalityLowerBound>(spec_target.get_id(), |na|na)
                        );
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityRangeOrZeroSpecialization(_) => {
                        leptos::logging::warn!("Somehow an ill-formed specialization was attempted (e.g. trying to make a lower_bound specialization for a range");
 },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundSpecialization(_) => {
                        editor.incorporate(OperativeSlotCardinalityLowerBoundSpecialization::new(ctx_clone.clone())
                            .set_temp_id("new_slot_spec")
                                .set_lower_bound(selected_lower_bound.get())
                                .add_existing_specializer(operative_clone.get_id(), |na|na)
                                .add_existing_roottemplateslot(slot_item.get_id(), |na|na)
                                .add_existing_specializationtarget::<OperativeSlotCardinalityLowerBoundSpecialization>(spec_target.get_id(), |na|na)
                        );
 },
                }
                all_descendent_ops_and_this_op.into_iter().for_each(|desc_op| {
                    editor.incorporate(
                        desc_op
                            .edit(ctx_clone.clone())
                            .add_temp_slotcardinalityspecializations::<OperativeSlotCardinalityLowerBoundSpecialization>(
                                "new_slot_spec",
                            ),
                    );
                });
                editor.execute().unwrap();
 },
        };
    };

    view! {
        <LeafSection>
            <Show when=move || !is_adding.get()>
                <Button on:click=move|_|is_adding.set(true)>Add Specialization</Button>
            </Show>
            <Show when=move || is_adding.get()>
                <div>
                <SignalSelectWithOptions value=selected_spec  options=options/>
                </div>

                {choose_bounds_view}

                <div>
                <Button on:click=on_save.clone() attr:disabled=move||{
                    is_downstream_slot_outside_of_attempted_bounds_clone() || are_selected_values_outside_of_previous_bounds_clone()

                }>Save</Button>
                <Button on:click=move|_|is_adding.set(false)>Cancel</Button>
                </div>
            </Show>
        </LeafSection>
    }
    .into_any()
}
