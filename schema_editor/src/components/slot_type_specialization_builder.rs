use std::collections::BTreeSet;

use leptos::either::{self, Either, EitherOf3};
use schema_editor_generated_toolkit::prelude::*;
use web_sys::MouseEvent;

use crate::components::utils::{
    get_all_descendent_operators_including_own, get_deepest_downstream_specializations,
};

use super::{
    common::*,
    operative_slot_section::OperativeSlotContext,
    utils::{
        get_all_descendent_operators, get_all_operatives_which_impl_trait_set,
        get_all_operatives_which_satisfy_specializable,
    },
    workspace::WorkspaceState,
};

#[derive(strum_macros::Display, strum_macros::EnumIter, strum_macros::EnumString, Clone)]
enum LimitedSpecOptions {
    Single,
    Multiple,
}

#[component]
pub fn SlotTypeSpecializationBuilder(
    operative: RGSOConcrete<OperativeConcrete, Schema>,
    spec_target: OperativeSlotTypeSpecializableTraitObject,
) -> impl IntoView {
    let ctx = use_context::<SharedGraph<Schema>>().unwrap();
    let WorkspaceState {
        schema,
        selected_tab,
    } = use_context::<WorkspaceState>().unwrap();
    let OperativeSlotContext {
        max_downstream_slotted_instances,
        operative,
        template_slot,
        maybe_childest_type_spec,
        maybe_childest_cardinality_spec,
    } = use_context::<OperativeSlotContext>().unwrap();
    let spec_target_clone = spec_target.clone();
    let is_adding = RwSignal::new(false);
    let selected_spec = RwSignal::new(Some(OperativeSlotTypeSpecializationTraitObjectDiscriminants::OperativeSlotTypeSingleSpecialization));
    let spec_options = Signal::derive(move || {
        match spec_target_clone {
            OperativeSlotTypeSpecializableTraitObject::TemplateSlotTypeSingleOperative(_) => vec![
                OperativeSlotTypeSpecializationTraitObjectDiscriminants::OperativeSlotTypeMultiSpecialization,
                OperativeSlotTypeSpecializationTraitObjectDiscriminants::OperativeSlotTypeSingleSpecialization,
            ],
            OperativeSlotTypeSpecializableTraitObject::OperativeSlotTypeSingleSpecialization(_) => vec![
                OperativeSlotTypeSpecializationTraitObjectDiscriminants::OperativeSlotTypeMultiSpecialization,
                OperativeSlotTypeSpecializationTraitObjectDiscriminants::OperativeSlotTypeSingleSpecialization,
            ],
            OperativeSlotTypeSpecializableTraitObject::TemplateSlotTypeTraitOperative(_) => vec![
                OperativeSlotTypeSpecializationTraitObjectDiscriminants::OperativeSlotTypeTraitObjectSpecialization,
                OperativeSlotTypeSpecializationTraitObjectDiscriminants::OperativeSlotTypeMultiSpecialization,
                OperativeSlotTypeSpecializationTraitObjectDiscriminants::OperativeSlotTypeSingleSpecialization,
            ],
            OperativeSlotTypeSpecializableTraitObject::OperativeSlotTypeMultiSpecialization(_) => vec![
                OperativeSlotTypeSpecializationTraitObjectDiscriminants::OperativeSlotTypeMultiSpecialization,
                OperativeSlotTypeSpecializationTraitObjectDiscriminants::OperativeSlotTypeSingleSpecialization,
            ],
            OperativeSlotTypeSpecializableTraitObject::TemplateSlotTypeMultiOperative(_) => vec![
                OperativeSlotTypeSpecializationTraitObjectDiscriminants::OperativeSlotTypeMultiSpecialization,
                OperativeSlotTypeSpecializationTraitObjectDiscriminants::OperativeSlotTypeSingleSpecialization,
            ],
            OperativeSlotTypeSpecializableTraitObject::OperativeSlotTypeTraitObjectSpecialization(_) => vec![
                OperativeSlotTypeSpecializationTraitObjectDiscriminants::OperativeSlotTypeTraitObjectSpecialization,
                OperativeSlotTypeSpecializationTraitObjectDiscriminants::OperativeSlotTypeMultiSpecialization,
                OperativeSlotTypeSpecializationTraitObjectDiscriminants::OperativeSlotTypeSingleSpecialization,
            ],
        }
    });

    let selected_single_op = RwSignal::<Option<RGSOConcrete<OperativeConcrete, Schema>>>::new(None);
    let selected_single_out_of_list_of_ops =
        RwSignal::<Option<RGSOConcrete<OperativeConcrete, Schema>>>::new(None);
    let selected_list_of_ops = RwSignal::new(vec![]);
    let selected_single_trait = RwSignal::<Option<RGSOConcrete<TraitConcrete, Schema>>>::new(None);
    let selected_list_of_traits = RwSignal::new(vec![]);

    let spec_target_clone = spec_target.clone();
    let schema_clone = schema.clone();
    let selectable_options = Memo::new(move |_| {
        let schema_clone = schema_clone.clone();
        let spec_target_clone = spec_target_clone.clone();
        let mut ops =
            get_all_operatives_which_satisfy_specializable(&schema_clone.get(), spec_target_clone);
        ops.retain(|item| !selected_list_of_ops.get().contains(item));
        ops.into_iter().collect::<Vec<_>>()
    });

    let schema_clone = schema.clone();
    let spec_target_clone = spec_target.clone();
    let selectable_trait_options = Memo::new(move |_| {
        let schema_clone = schema_clone.clone();
        let mut spec_target_clone = match spec_target_clone.clone() {
            OperativeSlotTypeSpecializableTraitObject::TemplateSlotTypeTraitOperative(item) =>
                Some(OperativeSlotTypeSpecializableTraitOperativeTraitObject::TemplateSlotTypeTraitOperative(item)) ,
            OperativeSlotTypeSpecializableTraitObject::OperativeSlotTypeMultiSpecialization(_) => None,
            OperativeSlotTypeSpecializableTraitObject::TemplateSlotTypeMultiOperative(_) => None,
            OperativeSlotTypeSpecializableTraitObject::OperativeSlotTypeSingleSpecialization(_) => None,
            OperativeSlotTypeSpecializableTraitObject::TemplateSlotTypeSingleOperative(_) => None,
            OperativeSlotTypeSpecializableTraitObject::OperativeSlotTypeTraitObjectSpecialization(item) =>
                Some(OperativeSlotTypeSpecializableTraitOperativeTraitObject::OperativeSlotTypeTraitObjectSpecialization(item)) ,
        };
        if let Some(trait_spec_target) = spec_target_clone {
            let mut next_item = trait_spec_target;
            let mut upstream_traits = BTreeSet::new();
            let mut reached_end = false;
            while reached_end == false {
                match next_item {
                    OperativeSlotTypeSpecializableTraitOperativeTraitObject::TemplateSlotTypeTraitOperative(item) => { upstream_traits.extend(item
                        .get_allowedtraits_slot()
                        .into_iter()
                        .collect::<BTreeSet<_>>());
                        reached_end = true;
                        break;
                    },
                    OperativeSlotTypeSpecializableTraitOperativeTraitObject::OperativeSlotTypeTraitObjectSpecialization( item, ) => {
                        upstream_traits.extend(item.get_allowedtraits_slot().into_iter().collect::<BTreeSet<_>>());
                        next_item = item.get_upstreamtype_slot();
                    }
                    _ => {}
                };
            }
            let mut selectable_traits = schema_clone.get().get_traits_slot();
            selectable_traits.retain(|item| {
                !selected_list_of_traits.get().contains(item) && !upstream_traits.contains(item)
            });
            selectable_traits.into_iter().collect::<Vec<_>>()
        } else {
            vec![]
        }
    });
    let choose_ops_view = move || {
        match selected_spec.get().unwrap() {
        OperativeSlotTypeSpecializationTraitObjectDiscriminants::OperativeSlotTypeSingleSpecialization => EitherOf3::A(
            view! {
                <SignalSelectRGSOWithOptions
                    value=selected_single_op
                    options=selectable_options
                    empty_allowed=true
                />
            }
        ),
        OperativeSlotTypeSpecializationTraitObjectDiscriminants::OperativeSlotTypeMultiSpecialization => EitherOf3::B(
            view! {
                <LeafSectionHeader>Selected:</LeafSectionHeader>
                <For
                    each=move || selected_list_of_ops.get()
                    key=|item| item.get_id().clone()
                    children=move |op| {
                        let op_clone = op.clone();

                        view! {
                            <LeafSection>
                                {move || op.get_name()}
                                <Button on:click=move |_| {
                                    selected_list_of_ops
                                        .update(|prev| {
                                            prev.retain(|item| item.get_id() != op_clone.get_id())
                                        })
                                }>X</Button>
                            </LeafSection>
                        }
                    }
                />
                <div>
                    <SignalSelectRGSOWithOptions
                        value=selected_single_out_of_list_of_ops
                        options=selectable_options
                        empty_allowed=true
                    />
                    <Button
                        attr:disabled=move || selected_single_out_of_list_of_ops.get().is_none()
                        on:click=move |_| {
                            if let Some(selected_single_op) = selected_single_out_of_list_of_ops
                                .get()
                            {
                                selected_list_of_ops.update(|prev| prev.push(selected_single_op));
                            }
                        }
                    >
                        Add
                    </Button>
                </div>
            }

        ),
        OperativeSlotTypeSpecializationTraitObjectDiscriminants::OperativeSlotTypeTraitObjectSpecialization => EitherOf3::C(
            view! {
                <LeafSectionHeader>Selected:</LeafSectionHeader>
                <For
                    each=move || selected_list_of_traits.get()
                    key=|item| item.get_id().clone()
                    children=move |trait_item| {
                        let trait_item_clone = trait_item.clone();
                        view! {
                            <LeafSection>
                                {move || trait_item.get_name()}
                                <Button on:click=move |_| {
                                    selected_list_of_traits
                                        .update(|prev| {
                                            prev.retain(|item| {
                                                item.get_id() != trait_item_clone.get_id()
                                            })
                                        })
                                }>X</Button>
                            </LeafSection>
                        }
                    }
                />
                <div>
                    <SignalSelectRGSOWithOptions
                        value=selected_single_trait
                        options=selectable_trait_options
                        empty_allowed=true
                    />
                    <Button
                        attr:disabled=move || selected_single_trait.get().is_none()
                        on:click=move |_| {
                            if let Some(selected_single_trait) = selected_single_trait.get() {
                                selected_list_of_traits
                                    .update(|prev| prev.push(selected_single_trait));
                            }
                        }
                    >
                        Add
                    </Button>
                </div>
            }

        ),
    }
    };

    let ctx_clone = ctx.clone();

    let operative_clone = operative.clone();
    let spec_target_clone = spec_target.clone();
    let on_save = move |_e: MouseEvent| {
        let spec_target_root_templateslot_id = match &spec_target_clone{
            OperativeSlotTypeSpecializableTraitObject::TemplateSlotTypeTraitOperative(item) => item.get_roottemplateslot_slot().get_id().clone(),
            OperativeSlotTypeSpecializableTraitObject::OperativeSlotTypeMultiSpecialization(item) => item.get_specializedslot_slot().get_roottemplateslot_slot().get_id().clone(),
            OperativeSlotTypeSpecializableTraitObject::TemplateSlotTypeMultiOperative(item) => item.get_roottemplateslot_slot().get_id().clone(),
            OperativeSlotTypeSpecializableTraitObject::OperativeSlotTypeSingleSpecialization(item) => item.get_specializedslot_slot().get_roottemplateslot_slot().get_id().clone(),
            OperativeSlotTypeSpecializableTraitObject::TemplateSlotTypeSingleOperative(item) => item.get_roottemplateslot_slot().get_id().clone(),
            OperativeSlotTypeSpecializableTraitObject::OperativeSlotTypeTraitObjectSpecialization(item) => item.get_specializedslot_slot().get_roottemplateslot_slot().get_id().clone(),
        };
        let operative = operative_clone.clone();
        let operative_clone = operative.clone();
        let deepest_downstream_spec = get_deepest_downstream_specializations(
            operative_clone.clone(),
            &spec_target_root_templateslot_id,
            true,
        );
        let mut all_descendent_ops = BTreeSet::new();
        // all_descendent_ops_and_this_op.insert(operative_clone.clone());
        get_all_descendent_operators(operative_clone.clone(), &mut all_descendent_ops);
        let is_already_slotted_uncompliant_downstream_error = match selected_spec.get().unwrap() {
            OperativeSlotTypeSpecializationTraitObjectDiscriminants::OperativeSlotTypeSingleSpecialization => deepest_downstream_spec.iter().any(|op| {
                op.get_slottedinstances_slot()
                    .into_iter()
                    .filter(|slint| {
                        slint.get_slottedslot_slot().get_id() == &spec_target_root_templateslot_id
                    })
                    .map(|slint| slint.get_instance_slot().get_parentoperative_slot())
                    .any(|slotted_op| {
                        slotted_op.get_id() != selected_single_op.get().unwrap().get_id()
                    })
            }),
            OperativeSlotTypeSpecializationTraitObjectDiscriminants::OperativeSlotTypeMultiSpecialization => deepest_downstream_spec.iter().any(|op| {
                op.get_slottedinstances_slot()
                    .into_iter()
                    .filter(|slint| {
                        slint.get_slottedslot_slot().get_id() == &spec_target_root_templateslot_id
                    })
                    .map(|slint| slint.get_instance_slot().get_parentoperative_slot())
                    .any(|slotted_op| !selected_list_of_ops.get().contains(&slotted_op))
            }),
            OperativeSlotTypeSpecializationTraitObjectDiscriminants::OperativeSlotTypeTraitObjectSpecialization => deepest_downstream_spec.iter().any(|op| {
                op.get_slottedinstances_slot()
                    .into_iter()
                    .filter(|slint| {
                        slint.get_slottedslot_slot().get_id() == &spec_target_root_templateslot_id
                    })
                    .map(|slint| slint.get_instance_slot().get_parentoperative_slot())
                    .any(|slotted_op|
                        slotted_op.get_traitimpls_slot().into_iter().any(|trait_impl|
                            !selected_list_of_traits.get().contains(&trait_impl)
                        )
                    )
            })
        };
        if is_already_slotted_uncompliant_downstream_error {
            leptos::logging::warn!(
                "Some downstream operative has an incompatible instance slotted"
            );
            return ();
        }
        let operative_clone = operative.clone();
        let mut editor = schema_clone.get().edit(ctx_clone.clone());
        let mut maybe_new_unencumbered_self_spec = None;
        let mut maybe_new_dependent_self_spec = None;
        let mut maybe_existing_self_spec = None;
        let maybe_self_slot_spec = operative_clone
            .get_slotspecializations_slot()
            .into_iter()
            .filter(|spec| spec.get_roottemplateslot_slot().get_id() == template_slot.get_id())
            .next();
        let mut previous_slot_spec = None;
        if let Some(self_spec) = maybe_self_slot_spec {
            if self_spec.get_specializer_slot().get_id() == operative_clone.get_id() {
                let mut edit = self_spec.edit(ctx_clone.clone());
                if let Some(existing_type_spec) =
                    self_spec.get_typespecialization_slot().into_iter().next()
                {
                    edit.remove_from_typespecialization(existing_type_spec.get_id());
                }
                maybe_existing_self_spec = Some(edit);
            } else {
                previous_slot_spec = Some(self_spec.clone());
                editor.incorporate(
                    operative_clone
                        .edit(ctx_clone.clone())
                        .remove_from_slotspecializations(self_spec.get_id()),
                );
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
                    .get_cardinalityspecialization_slot()
                    .iter()
                    .for_each(|card_spec| {
                        match card_spec {
                            OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityLowerBoundOrZeroSpecialization(item) => {
                                editor.incorporate(&new_slot_spec.clone().add_existing_cardinalityspecialization::<OperativeSlotCardinalityLowerBoundOrZeroSpecialization>(card_spec.get_id(), |na|na))
                            },
                            OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalitySingleSpecialization(item) => {
                                editor.incorporate(&new_slot_spec.clone().add_existing_cardinalityspecialization::<OperativeSlotCardinalitySingleSpecialization>(card_spec.get_id(), |na|na))
                            },
                            OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityRangeSpecialization(item) => {
                                editor.incorporate(&new_slot_spec.clone().add_existing_cardinalityspecialization::<OperativeSlotCardinalityRangeSpecialization>(card_spec.get_id(), |na|na))
                            },
                            OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityZeroSpecialization(item) => {
                                editor.incorporate(&new_slot_spec.clone().add_existing_cardinalityspecialization::<OperativeSlotCardinalityZeroSpecialization>(card_spec.get_id(), |na|na))
                            },
                            OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityRangeOrZeroSpecialization(item) => {
                                editor.incorporate(&new_slot_spec.clone().add_existing_cardinalityspecialization::<OperativeSlotCardinalityRangeOrZeroSpecialization>(card_spec.get_id(), |na|na))
                            },
                            OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityLowerBoundSpecialization(item) => {
                                editor.incorporate(&new_slot_spec.clone().add_existing_cardinalityspecialization::<OperativeSlotCardinalityLowerBoundSpecialization>(card_spec.get_id(), |na|na))
                            },
                        };
                    });

                editor.incorporate(
                    operative_clone
                        .edit(ctx_clone.clone())
                        .add_temp_slotspecializations("new_slot_spec"),
                );
                maybe_new_dependent_self_spec = Some(new_slot_spec);
            }
        } else {
            editor.incorporate(
                operative_clone
                    .edit(ctx_clone.clone())
                    .add_temp_slotspecializations("new_slot_spec"),
            );
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
        }
        match selected_spec.get().unwrap() {
            OperativeSlotTypeSpecializationTraitObjectDiscriminants::OperativeSlotTypeTraitObjectSpecialization => {
                let mut new_slot_type_spec =
                    OperativeSlotTypeTraitObjectSpecialization::new(ctx_clone.clone())
                        .set_temp_id("new_type_spec");
                match spec_target_clone.clone() {
                    OperativeSlotTypeSpecializableTraitObject::TemplateSlotTypeTraitOperative(item) => {
                        editor.incorporate(&
                            new_slot_type_spec
                                .clone()
                                .add_existing_upstreamtype::<TemplateSlotTypeTraitOperative>(
                                    spec_target_clone.get_id(),
                                    |na| na,
                                ),
                        );
                    }
                    OperativeSlotTypeSpecializableTraitObject::OperativeSlotTypeTraitObjectSpecialization(
                        item,
                    ) => {
                        let edit = new_slot_type_spec.clone()
                                .add_existing_upstreamtype::<OperativeSlotTypeTraitObjectSpecialization>(spec_target_clone.get_id(), |na| na);
                        editor.incorporate(&edit                        );
                    }
                    _ => panic!("Can't add trait specialization to non-trait-bound slot")
                };
                selected_list_of_traits.get().into_iter().for_each(|selected_trait| {
                    let edit = new_slot_type_spec
                            .clone()
                            .add_existing_allowedtraits(selected_trait.get_id(), |na| na);
                    editor.incorporate(&edit );
                });
                if let Some(mut existing_self_spec) = maybe_existing_self_spec {
                    editor.incorporate(&new_slot_type_spec.clone().add_existing_specializedslot(existing_self_spec.get_id(), |na|na));
                    editor.incorporate(existing_self_spec.add_temp_typespecialization::<OperativeSlotTypeTraitObjectSpecialization>("new_type_spec"));
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
                            if let Some(existing_type_specialization) = existing_slot.get_typespecialization_slot().into_iter().next() {
                                editor.incorporate(existing_slot.edit(ctx_clone.clone())
                                    .remove_from_typespecialization(existing_type_specialization.get_id()));
                            }
                            editor.incorporate(
                                existing_slot
                                    .edit(ctx_clone.clone())
                                    .add_temp_typespecialization::<OperativeSlotTypeTraitObjectSpecialization>("new_type_spec"),
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
                if let Some(new_self_spec) = maybe_new_unencumbered_self_spec.clone() {
                    editor.incorporate(&new_slot_type_spec.clone().add_temp_specializedslot("new_slot_spec"));
                    editor.incorporate(&new_self_spec.add_temp_typespecialization::<OperativeSlotTypeTraitObjectSpecialization>("new_type_spec"));
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
                            if let Some(existing_type_specialization) = existing_slot.get_typespecialization_slot().into_iter().next() {
                                editor.incorporate(existing_slot.edit(ctx_clone.clone())
                                    .remove_from_typespecialization(existing_type_specialization.get_id()));
                            }
                            editor.incorporate(
                                existing_slot
                                    .edit(ctx_clone.clone())
                                    .add_temp_typespecialization::<OperativeSlotTypeTraitObjectSpecialization>("new_type_spec"),
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
                if let Some(new_self_spec) = maybe_new_dependent_self_spec.clone() {
                    editor.incorporate(&new_slot_type_spec.clone().add_temp_specializedslot("new_slot_spec"));
                    editor.incorporate(&new_self_spec.add_temp_typespecialization::<OperativeSlotTypeTraitObjectSpecialization>("new_type_spec"));
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
                            if existing_slot.get_id() == previous_slot_spec.clone().unwrap().get_id() {
                                editor.incorporate(desc_op.edit(ctx_clone.clone()).remove_from_slotspecializations(existing_slot.get_id())
                                    .add_temp_slotspecializations("new_slot_spec"));
                            } else {
                                if let Some(existing_type_specialization) = existing_slot.get_typespecialization_slot().into_iter().next() {
                                    editor.incorporate(existing_slot.edit(ctx_clone.clone())
                                        .remove_from_typespecialization(existing_type_specialization.get_id()));
                                }
                                editor.incorporate(
                                    existing_slot
                                        .edit(ctx_clone.clone())
                                        .remove_from_upstreamslotdescription(previous_slot_spec.clone().unwrap().get_id())
                                        .add_temp_upstreamslotdescription::<OperativeSlotSpecialized>("new_slot_spec")
                                        .add_temp_typespecialization::<OperativeSlotTypeTraitObjectSpecialization>("new_type_spec"),
                                )
                            }
                        } else {
                            editor.incorporate(
                                desc_op
                                    .edit(ctx_clone.clone())
                                    .add_temp_slotspecializations("new_slot_spec"),
                            );
                        }
                    });
                }
                editor.execute().unwrap();
            }
            OperativeSlotTypeSpecializationTraitObjectDiscriminants::OperativeSlotTypeSingleSpecialization => {
                let mut new_slot_type_spec =
                    OperativeSlotTypeSingleSpecialization::new(ctx_clone.clone())
                        .set_temp_id("new_type_spec")
                        .add_existing_allowedoperative(
                            selected_single_op.get().unwrap().get_id(),
                            |na| na,
                        );
                match spec_target_clone.clone() {
                    OperativeSlotTypeSpecializableTraitObject::TemplateSlotTypeTraitOperative(item) => {
                        let edit =
                                new_slot_type_spec
                                    .clone()
                                    .add_existing_upstreamtype::<TemplateSlotTypeTraitOperative>(
                                        spec_target_clone.get_id(),
                                        |na| na,
                                    );
                        editor.incorporate(&edit                        );
                    }
                    OperativeSlotTypeSpecializableTraitObject::OperativeSlotTypeMultiSpecialization(
                        item,
                    ) => {
                        let edit =
                        new_slot_type_spec.clone()
                            .add_existing_upstreamtype::<OperativeSlotTypeMultiSpecialization>(
                                spec_target_clone.get_id(),
                                |na| na,
                            );
                        editor.incorporate(&edit);
                    }
                    OperativeSlotTypeSpecializableTraitObject::TemplateSlotTypeMultiOperative(item) => {
                        let edit = new_slot_type_spec
                                    .clone()
                                    .add_existing_upstreamtype::<TemplateSlotTypeMultiOperative>(
                                        spec_target_clone.get_id(),
                                        |na| na,
                                    );
                        editor.incorporate(&edit                        );
                    }
                    OperativeSlotTypeSpecializableTraitObject::OperativeSlotTypeSingleSpecialization(
                        item,
                    ) => {
                        let edit =
                        new_slot_type_spec.clone()
                            .add_existing_upstreamtype::<OperativeSlotTypeSingleSpecialization>(
                                spec_target_clone.get_id(),
                                |na| na,
                            );
                        editor.incorporate(&edit);
                    }
                    OperativeSlotTypeSpecializableTraitObject::TemplateSlotTypeSingleOperative(item) => {
                        let edit = new_slot_type_spec
                                    .clone()
                                    .add_existing_upstreamtype::<TemplateSlotTypeSingleOperative>(
                                        spec_target_clone.get_id(),
                                        |na| na,
                                    );
                        editor.incorporate(&edit                        );
                    }
                    OperativeSlotTypeSpecializableTraitObject::OperativeSlotTypeTraitObjectSpecialization(
                        item,
                    ) => {
                        let edit =

                            new_slot_type_spec.clone()
                                .add_existing_upstreamtype::<OperativeSlotTypeTraitObjectSpecialization>(spec_target_clone.get_id(), |na| na);
                        editor.incorporate(&edit                        );
                    }
                };

                if let Some(mut existing_self_spec) = maybe_existing_self_spec {
                    editor.incorporate(&new_slot_type_spec.clone().add_existing_specializedslot(existing_self_spec.get_id(), |na|na));
                    editor.incorporate(existing_self_spec.add_temp_typespecialization::<OperativeSlotTypeSingleSpecialization>("new_type_spec"));
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
                            if let Some(existing_type_specialization) = existing_slot.get_typespecialization_slot().into_iter().next() {
                                editor.incorporate(existing_slot.edit(ctx_clone.clone())
                                    .remove_from_typespecialization(existing_type_specialization.get_id()));
                            }
                            editor.incorporate(
                                existing_slot
                                    .edit(ctx_clone.clone())
                                    .add_temp_typespecialization::<OperativeSlotTypeSingleSpecialization>("new_type_spec"),
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
                if let Some(new_self_spec) = maybe_new_unencumbered_self_spec.clone() {
                    editor.incorporate(&new_slot_type_spec.clone().add_temp_specializedslot("new_slot_spec"));
                    editor.incorporate(&new_self_spec.add_temp_typespecialization::<OperativeSlotTypeSingleSpecialization>("new_type_spec"));
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
                            if let Some(existing_type_specialization) = existing_slot.get_typespecialization_slot().into_iter().next() {
                                editor.incorporate(existing_slot.edit(ctx_clone.clone())
                                    .remove_from_typespecialization(existing_type_specialization.get_id()));
                            }
                            editor.incorporate(
                                existing_slot
                                    .edit(ctx_clone.clone())
                                    .add_temp_typespecialization::<OperativeSlotTypeSingleSpecialization>("new_type_spec"),
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
                if let Some(new_self_spec) = maybe_new_dependent_self_spec.clone() {
                    editor.incorporate(&new_slot_type_spec.clone().add_temp_specializedslot("new_slot_spec"));
                    editor.incorporate(&new_self_spec.add_temp_typespecialization::<OperativeSlotTypeSingleSpecialization>("new_type_spec"));
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
                            if existing_slot.get_id() == previous_slot_spec.clone().unwrap().get_id() {
                                editor.incorporate(desc_op.edit(ctx_clone.clone()).remove_from_slotspecializations(existing_slot.get_id())
                                    .add_temp_slotspecializations("new_slot_spec"));
                            } else {
                                if let Some(existing_type_specialization) = existing_slot.get_typespecialization_slot().into_iter().next() {
                                    editor.incorporate(existing_slot.edit(ctx_clone.clone())
                                        .remove_from_typespecialization(existing_type_specialization.get_id()));
                                }
                                editor.incorporate(
                                    existing_slot
                                        .edit(ctx_clone.clone())
                                        .remove_from_upstreamslotdescription(previous_slot_spec.clone().unwrap().get_id())
                                        .add_temp_upstreamslotdescription::<OperativeSlotSpecialized>("new_slot_spec")
                                        .add_temp_typespecialization::<OperativeSlotTypeSingleSpecialization>("new_type_spec"),
                                )
                            }
                        } else {
                            editor.incorporate(
                                desc_op
                                    .edit(ctx_clone.clone())
                                    .add_temp_slotspecializations("new_slot_spec"),
                            );
                        }
                    });
                }
                editor.execute().unwrap();
            }
            OperativeSlotTypeSpecializationTraitObjectDiscriminants::OperativeSlotTypeMultiSpecialization => {
                let mut new_slot_type_spec =
                    OperativeSlotTypeMultiSpecialization::new(ctx_clone.clone())
                        .set_temp_id("new_type_spec");
                match spec_target_clone.clone() {
                    OperativeSlotTypeSpecializableTraitObject::TemplateSlotTypeTraitOperative(item) => {
                        let edit = new_slot_type_spec
                                    .clone()
                                    .add_existing_upstreamtype::<TemplateSlotTypeTraitOperative>(
                                        spec_target_clone.get_id(),
                                        |na| na,
                                    );
                        editor.incorporate(&edit                        );
                    }
                    OperativeSlotTypeSpecializableTraitObject::OperativeSlotTypeMultiSpecialization(
                        item,
                    ) => {
                        let edit =
                        new_slot_type_spec.clone()
                            .add_existing_upstreamtype::<OperativeSlotTypeMultiSpecialization>(
                                spec_target_clone.get_id(),
                                |na| na,
                            );
                        editor.incorporate(&edit);
                    }
                    OperativeSlotTypeSpecializableTraitObject::TemplateSlotTypeMultiOperative(item) => {
                        let edit = new_slot_type_spec
                                    .clone()
                                    .add_existing_upstreamtype::<TemplateSlotTypeMultiOperative>(
                                        spec_target_clone.get_id(),
                                        |na| na,
                                    );
                        editor.incorporate(&edit                        );
                    }
                    OperativeSlotTypeSpecializableTraitObject::OperativeSlotTypeSingleSpecialization(
                        item,
                    ) => {
                        let edit =
                        new_slot_type_spec.clone()
                            .add_existing_upstreamtype::<OperativeSlotTypeSingleSpecialization>(
                                spec_target_clone.get_id(),
                                |na| na,
                            );
                        editor.incorporate(&edit);
                    }
                    OperativeSlotTypeSpecializableTraitObject::TemplateSlotTypeSingleOperative(item) => {
                        let edit = new_slot_type_spec
                                    .clone()
                                    .add_existing_upstreamtype::<TemplateSlotTypeSingleOperative>(
                                        spec_target_clone.get_id(),
                                        |na| na,
                                    );
                        editor.incorporate(&edit                        );
                    }
                    OperativeSlotTypeSpecializableTraitObject::OperativeSlotTypeTraitObjectSpecialization(
                        item,
                    ) => {
                        let edit =

                            new_slot_type_spec.clone()
                                .add_existing_upstreamtype::<OperativeSlotTypeTraitObjectSpecialization>(spec_target_clone.get_id(), |na| na);
                        editor.incorporate(&edit                        );
                    }
                };
                selected_list_of_ops.get().into_iter().for_each(|op| {
                    let edit =
                        new_slot_type_spec
                            .clone()
                            .add_existing_allowedoperatives(op.get_id(), |na| na);
                    editor.incorporate(&edit );
                });
                if let Some(mut existing_self_spec) = maybe_existing_self_spec {
                    editor.incorporate(&new_slot_type_spec.clone().add_existing_specializedslot(existing_self_spec.get_id(), |na|na));
                    editor.incorporate(existing_self_spec.add_temp_typespecialization::<OperativeSlotTypeMultiSpecialization>("new_type_spec"));
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
                            if let Some(existing_type_specialization) = existing_slot.get_typespecialization_slot().into_iter().next() {
                                editor.incorporate(existing_slot.edit(ctx_clone.clone())
                                    .remove_from_typespecialization(existing_type_specialization.get_id()));
                            }
                            editor.incorporate(
                                existing_slot
                                    .edit(ctx_clone.clone())
                                    .add_temp_typespecialization::<OperativeSlotTypeMultiSpecialization>("new_type_spec"),
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
                if let Some(new_self_spec) = maybe_new_unencumbered_self_spec.clone() {
                    editor.incorporate(&new_slot_type_spec.clone().add_temp_specializedslot("new_slot_spec"));
                    editor.incorporate(&new_self_spec.add_temp_typespecialization::<OperativeSlotTypeMultiSpecialization>("new_type_spec"));
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
                            if let Some(existing_type_specialization) = existing_slot.get_typespecialization_slot().into_iter().next() {
                                editor.incorporate(existing_slot.edit(ctx_clone.clone())
                                    .remove_from_typespecialization(existing_type_specialization.get_id()));
                            }
                            editor.incorporate(
                                existing_slot
                                    .edit(ctx_clone.clone())
                                    .add_temp_typespecialization::<OperativeSlotTypeMultiSpecialization>("new_type_spec"),
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
                if let Some(new_self_spec) = maybe_new_dependent_self_spec.clone() {
                    editor.incorporate(&new_slot_type_spec.clone().add_temp_specializedslot("new_slot_spec"));
                    editor.incorporate(&new_self_spec.add_temp_typespecialization::<OperativeSlotTypeMultiSpecialization>("new_type_spec"));
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
                            if existing_slot.get_id() == previous_slot_spec.clone().unwrap().get_id() {
                                editor.incorporate(desc_op.edit(ctx_clone.clone()).remove_from_slotspecializations(existing_slot.get_id())
                                    .add_temp_slotspecializations("new_slot_spec"));
                            } else {
                                if let Some(existing_type_specialization) = existing_slot.get_typespecialization_slot().into_iter().next() {
                                    editor.incorporate(existing_slot.edit(ctx_clone.clone())
                                        .remove_from_typespecialization(existing_type_specialization.get_id()));
                                }
                            editor.incorporate(
                                existing_slot
                                    .edit(ctx_clone.clone())
                                    .remove_from_upstreamslotdescription(previous_slot_spec.clone().unwrap().get_id())
                                    .add_temp_upstreamslotdescription::<OperativeSlotSpecialized>("new_slot_spec")
                                    .add_temp_typespecialization::<OperativeSlotTypeMultiSpecialization>("new_type_spec"),
                            )
                            }
                        } else {
                            editor.incorporate(
                                desc_op
                                    .edit(ctx_clone.clone())
                                    .add_temp_slotspecializations("new_slot_spec"),
                            );
                        }
                    });
                }
                editor.execute().unwrap();
            }
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
                // <Show when=move || !is_adding.get()>
                <Button on:click=move |_| is_adding.set(true)>Add Specialization</Button>
            // </Show>
            </div>
            <div class=move || {
                match is_adding.get() {
                    true => "",
                    false => "hidden",
                }
            }>
                // <Show when=move || is_adding.get()>
                <div>
                    <SignalSelectWithOptions value=selected_spec options=spec_options />
                </div>

                {choose_ops_view}

                <div>
                    <Button
                        on:click=on_save.clone()
                        attr:disabled=move || {
                            match selected_spec.get().unwrap() {
                                OperativeSlotTypeSpecializationTraitObjectDiscriminants::OperativeSlotTypeSingleSpecialization => {
                                    selected_single_op.get().is_none()
                                }
                                OperativeSlotTypeSpecializationTraitObjectDiscriminants::OperativeSlotTypeMultiSpecialization => {
                                    selected_list_of_ops.get().len() < 2
                                }
                                OperativeSlotTypeSpecializationTraitObjectDiscriminants::OperativeSlotTypeTraitObjectSpecialization => {
                                    selected_list_of_traits.get().len() < 1
                                }
                            }
                        }
                    >
                        Save
                    </Button>
                    <Button on:click=move |_| is_adding.set(false)>Cancel</Button>
                </div>
            // </Show>
            </div>
        </LeafSection>
    }
    .into_any()
}
