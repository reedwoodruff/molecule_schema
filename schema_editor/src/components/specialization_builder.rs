use std::collections::BTreeSet;

use leptos::either::{self, Either, EitherOf3};
use schema_editor_generated_toolkit::prelude::*;
use web_sys::MouseEvent;

use super::{
    common::*,
    utils::{
        get_all_descendent_operators, get_all_operatives_which_impl_trait_set,
        get_all_operatives_which_satisfy_specializable, get_all_traits_in_specialization,
    },
    workspace::WorkspaceState,
};

#[derive(strum_macros::Display, strum_macros::EnumIter, strum_macros::EnumString, Clone)]
enum LimitedSpecOptions {
    Single,
    Multiple,
}

#[component]
pub fn SpecializationBuilder(
    operative: RGSOConcrete<OperativeConcrete, Schema>,
    spec_target: SlotTypeSpecializableTraitObject,
) -> impl IntoView {
    let ctx = use_context::<SharedGraph<Schema>>().unwrap();
    let WorkspaceState {
        schema,
        selected_tab,
    } = use_context::<WorkspaceState>().unwrap();
    match spec_target {
        SlotTypeSpecializableTraitObject::TemplateSlotTraitOperative(trait_item) => {
            return view! {<TraitSpecializationBuilder operative spec_target=SlotTypeSpecializableTraitOperativeTraitObject::TemplateSlotTraitOperative(trait_item) />}.into_any()
        }
        SlotTypeSpecializableTraitObject::OperativeSlotTypeTraitObjectSpecialization(trait_item) => {
            return view! {<TraitSpecializationBuilder operative spec_target=SlotTypeSpecializableTraitOperativeTraitObject::OperativeSlotTypeTraitObjectSpecialization(trait_item) />}.into_any()
        }
        _ => (),
    }
    let is_adding = RwSignal::new(false);
    let selected_spec = RwSignal::new(LimitedSpecOptions::Single);

    let selected_single_op = RwSignal::<Option<RGSOConcrete<OperativeConcrete, Schema>>>::new(None);
    let selected_list_of_ops = RwSignal::new(vec![]);

    let spec_target_clone = spec_target.clone();
    let selectable_options = Memo::new(move |_| {
        let schema_clone = schema.clone();
        let spec_target_clone = spec_target_clone.clone();
        let mut ops =
            get_all_operatives_which_satisfy_specializable(&schema_clone, spec_target_clone);
        ops.retain(|item| !selected_list_of_ops.get().contains(item));
        ops.into_iter().collect::<Vec<_>>()
    });

    let choose_ops_view = move || match selected_spec.get() {
        LimitedSpecOptions::Single => Either::Right(view! {
                <SignalSelectWithOptions value=selected_single_op options=selectable_options empty_allowed=true />
        }),
        LimitedSpecOptions::Multiple => Either::Left(view! {
            <LeafSectionHeader>
            Selected:
            </LeafSectionHeader>
            <For each=move || selected_list_of_ops.get() key=|item| item.get_id().clone() children=move |op| {
                let op_clone = op.clone();

                view!{
                <LeafSection>
                    {move || op.get_name()} <Button on:click=move |_| selected_list_of_ops.update(|prev| prev.retain(|item| item.get_id() != op_clone.get_id()))>X</Button>
                </LeafSection>}
            }/>
            <div>
            <SignalSelectWithOptions value=selected_single_op options=selectable_options empty_allowed=true />
            <Button attr:disabled=move || selected_single_op.get().is_none() on:click=move |_| {
                if let Some(selected_single_op) = selected_single_op.get() {
                    selected_list_of_ops.update(|prev| prev.push(selected_single_op));
                }
            }>Add</Button>
            </div>
        }),
    };

    let ctx_clone = ctx.clone();

    let operative_clone = operative.clone();
    let spec_target_clone = spec_target.clone();
    let on_save = move |_e: MouseEvent| {
        leptos::logging::log!("running");
        let operative = operative_clone.clone();
        let operative_clone = operative.clone();
        let mut all_descendent_ops = BTreeSet::new();
        get_all_descendent_operators(operative_clone, &mut all_descendent_ops);
        let is_error = match selected_spec.get() {
            LimitedSpecOptions::Single => all_descendent_ops.clone().into_iter().any(|op| {
                op.get_slottedinstances_slot()
                    .into_iter()
                    .filter(|slint| *slint.get_slottedslot_slot().get_id() == match spec_target_clone.clone(){
    SlotTypeSpecializableTraitObject::TemplateSlotTraitOperative(item) => item.get_roottemplateslot_slot().get_id().clone(),
    SlotTypeSpecializableTraitObject::OperativeSlotTypeMultiSpecialization(item) => item.get_roottemplateslot_slot().get_id().clone(),
    SlotTypeSpecializableTraitObject::TemplateSlotMultiOperative(item) => item.get_roottemplateslot_slot().get_id().clone(),
    SlotTypeSpecializableTraitObject::OperativeSlotTypeTraitObjectSpecialization(item) => item.get_roottemplateslot_slot().get_id().clone(),
})
                    .map(|slint| slint.get_instance_slot().get_parentoperative_slot())
                    .any(|slotted_op| {
                        slotted_op.get_id() != selected_single_op.get().unwrap().get_id()
                    })
            }),
            LimitedSpecOptions::Multiple => all_descendent_ops.clone().into_iter().any(|op| {
                op.get_slottedinstances_slot()
                    .into_iter()
                    .filter(|slint| *slint.get_slottedslot_slot().get_id() == match spec_target_clone.clone(){
    SlotTypeSpecializableTraitObject::TemplateSlotTraitOperative(item) => item.get_roottemplateslot_slot().get_id().clone(),
    SlotTypeSpecializableTraitObject::OperativeSlotTypeMultiSpecialization(item) => item.get_roottemplateslot_slot().get_id().clone(),
    SlotTypeSpecializableTraitObject::TemplateSlotMultiOperative(item) => item.get_roottemplateslot_slot().get_id().clone(),
    SlotTypeSpecializableTraitObject::OperativeSlotTypeTraitObjectSpecialization(item) => item.get_roottemplateslot_slot().get_id().clone(),
})
                    .map(|slint| slint.get_instance_slot().get_parentoperative_slot())
                    .any(|slotted_op| !selected_list_of_ops.get().contains(&slotted_op))
            }),
        };
        if is_error {
            leptos::logging::warn!(
                "Some downstream operative has an incompatible instance slotted"
            );
            return ();
        }
        let operative_clone = operative.clone();
        match selected_spec.get() {
            LimitedSpecOptions::Single => {
                let mut editor = operative.edit(ctx_clone.clone());
                let mut new_slot_spec =
                    OperativeSlotTypeSingleSpecialization::new(ctx_clone.clone())
                        .set_temp_id("new_slot_spec")
                        .add_existing_allowedoperative(
                            selected_single_op.get().unwrap().get_id(),
                            |na| na,
                        )
                        .add_existing_specializer(operative_clone.get_id(), |na| na);
                match spec_target_clone.clone() {
                    SlotTypeSpecializableTraitObject::TemplateSlotTraitOperative(item) => {
                        editor.incorporate(
                            new_slot_spec
                                .clone()
                                .add_existing_roottemplateslot(
                                    item.get_roottemplateslot_slot().get_id(),
                                    |na| na,
                                )
                                .add_existing_specializationtarget::<TemplateSlotTraitOperative>(
                                    spec_target_clone.get_id(),
                                    |na| na,
                                ),
                        );
                    }
                    SlotTypeSpecializableTraitObject::OperativeSlotTypeMultiSpecialization(
                        item,
                    ) => {
                        editor.incorporate(
                        new_slot_spec.clone()
                            .add_existing_roottemplateslot(
                                item.get_roottemplateslot_slot().get_id(),
                                |na| na,
                            )
                            .add_existing_specializationtarget::<OperativeSlotTypeMultiSpecialization>(
                                spec_target_clone.get_id(),
                                |na| na,
                            ));
                    }
                    SlotTypeSpecializableTraitObject::TemplateSlotMultiOperative(item) => {
                        editor.incorporate(
                            new_slot_spec
                                .clone()
                                .add_existing_roottemplateslot(
                                    item.get_roottemplateslot_slot().get_id(),
                                    |na| na,
                                )
                                .add_existing_specializationtarget::<TemplateSlotMultiOperative>(
                                    spec_target_clone.get_id(),
                                    |na| na,
                                ),
                        );
                    }
                    SlotTypeSpecializableTraitObject::OperativeSlotTypeTraitObjectSpecialization(
                        item,
                    ) => {
                        editor.incorporate(
                        new_slot_spec.clone()
                            .add_existing_roottemplateslot(
                                item.get_roottemplateslot_slot().get_id(),
                                |na| na,
                            )
                            .add_existing_specializationtarget::<OperativeSlotTypeTraitObjectSpecialization>(spec_target_clone.get_id(), |na| na)
                        );
                    }
                };
                all_descendent_ops.into_iter().for_each(|desc_op| {
                    editor.incorporate(
                        desc_op
                            .edit(ctx_clone.clone())
                            .add_temp_slottypespecializations::<OperativeSlotTypeSingleSpecialization>(
                                "new_slot_spec",
                            ),
                    );
                });
                editor.execute().unwrap();
            }
            LimitedSpecOptions::Multiple => {
                let mut editor = operative.edit(ctx_clone.clone());
                let mut new_slot_spec =
                    OperativeSlotTypeMultiSpecialization::new(ctx_clone.clone())
                        .set_temp_id("new_slot_spec")
                        .add_existing_specializer(operative_clone.get_id(), |na| na);
                match spec_target_clone.clone() {
                    SlotTypeSpecializableTraitObject::TemplateSlotTraitOperative(item) => {
                        editor.incorporate(
                            new_slot_spec
                                .clone()
                                .add_existing_roottemplateslot(
                                    item.get_roottemplateslot_slot().get_id(),
                                    |na| na,
                                )
                                .add_existing_specializationtarget::<TemplateSlotTraitOperative>(
                                    spec_target_clone.get_id(),
                                    |na| na,
                                ),
                        );
                    }
                    SlotTypeSpecializableTraitObject::OperativeSlotTypeMultiSpecialization(
                        item,
                    ) => {
                        editor.incorporate(
                        new_slot_spec.clone()
                            .add_existing_roottemplateslot(
                                item.get_roottemplateslot_slot().get_id(),
                                |na| na,
                            )
                            .add_existing_specializationtarget::<OperativeSlotTypeMultiSpecialization>(
                                spec_target_clone.get_id(),
                                |na| na,
                            ));
                    }
                    SlotTypeSpecializableTraitObject::TemplateSlotMultiOperative(item) => {
                        editor.incorporate(
                            new_slot_spec
                                .clone()
                                .add_existing_roottemplateslot(
                                    item.get_roottemplateslot_slot().get_id(),
                                    |na| na,
                                )
                                .add_existing_specializationtarget::<TemplateSlotMultiOperative>(
                                    spec_target_clone.get_id(),
                                    |na| na,
                                ),
                        );
                    }
                    SlotTypeSpecializableTraitObject::OperativeSlotTypeTraitObjectSpecialization(
                        item,
                    ) => {
                        editor.incorporate(
                        new_slot_spec.clone()
                            .add_existing_roottemplateslot(
                                item.get_roottemplateslot_slot().get_id(),
                                |na| na,
                            )
                            .add_existing_specializationtarget::<OperativeSlotTypeTraitObjectSpecialization>(spec_target_clone.get_id(), |na| na)
                        );
                    }
                };
                selected_list_of_ops.get().into_iter().for_each(|op| {
                    editor.incorporate(
                        new_slot_spec
                            .clone()
                            .add_existing_allowedoperatives(op.get_id(), |na| na),
                    )
                });
                all_descendent_ops.into_iter().for_each(|desc_op| {
                    editor.incorporate(
                        desc_op
                            .edit(ctx_clone.clone())
                            .add_temp_slottypespecializations::<OperativeSlotTypeMultiSpecialization>(
                                "new_slot_spec",
                            ),
                    );
                });
                editor.execute().unwrap();
            }
        };
        is_adding.set(false);
    };

    view! {
        <LeafSection>
            <Show when=move || !is_adding.get()>
                <Button on:click=move|_|is_adding.set(true)>Add Specialization</Button>
            </Show>
            <Show when=move || is_adding.get()>
                <div>
                <SignalEnumSelect value=selected_spec />
                </div>

                {choose_ops_view}

                <div>
                <Button on:click=on_save.clone() attr:disabled=move||{
                    match selected_spec.get() {
                        LimitedSpecOptions::Single => {
                            selected_single_op.get().is_none()
                        },
                        LimitedSpecOptions::Multiple => {
                            selected_list_of_ops.get().len() < 2
                        },
                    }
                }>Save</Button>
                <Button on:click=move|_|is_adding.set(false)>Cancel</Button>
                </div>
            </Show>
        </LeafSection>
    }
    .into_any()
}

#[component]
pub fn TraitSpecializationBuilder(
    operative: RGSOConcrete<OperativeConcrete, Schema>,
    spec_target: SlotTypeSpecializableTraitOperativeTraitObject,
) -> impl IntoView {
    let ctx = use_context::<SharedGraph<Schema>>().unwrap();
    let WorkspaceState {
        schema,
        selected_tab,
    } = use_context::<WorkspaceState>().unwrap();
    let is_adding = RwSignal::new(false);
    let selected_spec = RwSignal::new(
        SlotTypeSpecializationTraitObjectDiscriminants::OperativeSlotTypeSingleSpecialization,
    );

    let selected_single_op = RwSignal::<Option<RGSOConcrete<OperativeConcrete, Schema>>>::new(None);
    let selected_single_trait = RwSignal::<Option<RGSOConcrete<TraitConcrete, Schema>>>::new(None);
    let selected_list_of_ops = RwSignal::new(vec![]);
    let selected_list_of_traits = RwSignal::new(vec![]);

    let spec_target_clone = spec_target.clone();
    let schema_clone = schema.clone();
    let selectable_op_options = Memo::new(move |_| {
        let schema_clone = schema_clone.clone();
        let spec_target_clone = spec_target_clone.clone();
        let mut ops =
            get_all_operatives_which_satisfy_specializable(&schema_clone, match spec_target_clone {
            SlotTypeSpecializableTraitOperativeTraitObject::TemplateSlotTraitOperative(item) => SlotTypeSpecializableTraitObject::TemplateSlotTraitOperative(item),
            SlotTypeSpecializableTraitOperativeTraitObject::OperativeSlotTypeTraitObjectSpecialization(item) => SlotTypeSpecializableTraitObject::OperativeSlotTypeTraitObjectSpecialization(item),
        });
        ops.retain(|item| !selected_list_of_ops.get().contains(item));
        ops.into_iter().collect::<Vec<_>>()
    });
    let schema_clone = schema.clone();
    let spec_target_clone = spec_target.clone();
    let selectable_trait_options = Memo::new(move |_| {
        let schema_clone = schema_clone.clone();
        let spec_target_clone = spec_target_clone.clone();
        let mut upstream_traits = match spec_target_clone {
            SlotTypeSpecializableTraitOperativeTraitObject::TemplateSlotTraitOperative(item) => item
                .get_allowedtraits_slot()
                .into_iter()
                .collect::<BTreeSet<_>>(),
            SlotTypeSpecializableTraitOperativeTraitObject::OperativeSlotTypeTraitObjectSpecialization(
                item,
            ) => get_all_traits_in_specialization(item),
        };
        let mut selectable_traits = schema_clone.get_traits_slot();
        selectable_traits.retain(|item| {
            !selected_list_of_traits.get().contains(item) && !upstream_traits.contains(item)
        });
        selectable_traits.into_iter().collect::<Vec<_>>()
    });

    let choose_ops_view = move || {
        match selected_spec.get() {
        SlotTypeSpecializationTraitObjectDiscriminants::OperativeSlotTypeSingleSpecialization => {
            EitherOf3::A(view! {
                    <SignalSelectWithOptions value=selected_single_op options=selectable_op_options empty_allowed=true />
            })
        }
        SlotTypeSpecializationTraitObjectDiscriminants::OperativeSlotTypeMultiSpecialization => {
            EitherOf3::B(view! {
                <LeafSectionHeader>
                Selected:
                </LeafSectionHeader>
                <For each=move || selected_list_of_ops.get() key=|item| item.get_id().clone() children=move |op| {
                    let op_clone = op.clone();

                    view!{
                    <LeafSection>
                        {move || op.get_name()} <Button on:click=move |_| selected_list_of_ops.update(|prev| prev.retain(|item| item.get_id() != op_clone.get_id()))>X</Button>
                    </LeafSection>}
                }/>
                <div>
                <SignalSelectWithOptions value=selected_single_op options=selectable_op_options empty_allowed=true />
                <Button attr:disabled=move || selected_single_op.get().is_none() on:click=move |_| {
                    if let Some(selected_single_op) = selected_single_op.get() {
                        selected_list_of_ops.update(|prev| prev.push(selected_single_op));
                    }
                }>Add</Button>
                </div>
            })
        }
        SlotTypeSpecializationTraitObjectDiscriminants::OperativeSlotTypeTraitObjectSpecialization => {
            EitherOf3::C(view! {
                <LeafSectionHeader>
                Selected:
                </LeafSectionHeader>
                <For each=move || selected_list_of_traits.get() key=|item| item.get_id().clone() children=move |trait_item| {
                    let trait_item_clone = trait_item.clone();

                    view!{
                    <LeafSection>
                        {move || trait_item.get_name()} <Button on:click=move |_| selected_list_of_traits.update(|prev| prev.retain(|item| item.get_id() != trait_item_clone.get_id()))>X</Button>
                    </LeafSection>}
                }/>
                <div>
                <SignalSelectWithOptions value=selected_single_trait options=selectable_trait_options empty_allowed=true />
                <Button attr:disabled=move || selected_single_trait.get().is_none() on:click=move |_| {
                    if let Some(selected_single_trait) = selected_single_trait.get() {
                        selected_list_of_traits.update(|prev| prev.push(selected_single_trait));
                    }
                }>Add</Button>
                </div>

            })
        }
    }
    };

    let ctx_clone = ctx.clone();

    let operative_clone = operative.clone();
    let spec_target_clone = spec_target.clone();
    let schema_clone = schema.clone();
    let on_save = move |_e: MouseEvent| {
        leptos::logging::log!("running");
        let operative = operative_clone.clone();
        let operative_clone = operative.clone();
        let mut all_descendent_ops = BTreeSet::new();
        get_all_descendent_operators(operative_clone, &mut all_descendent_ops);
        let is_error = match selected_spec.get() {
            SlotTypeSpecializationTraitObjectDiscriminants::OperativeSlotTypeSingleSpecialization => {
                all_descendent_ops.clone().into_iter().any(|op| {
                    op.get_slottedinstances_slot()
                        .into_iter()
                        .filter(|slint| *slint.get_slottedslot_slot().get_id() == match spec_target_clone.clone(){
                            SlotTypeSpecializableTraitOperativeTraitObject::TemplateSlotTraitOperative(item) => item.get_roottemplateslot_slot().get_id().clone(),
                            SlotTypeSpecializableTraitOperativeTraitObject::OperativeSlotTypeTraitObjectSpecialization(item) => item.get_roottemplateslot_slot().get_id().clone(),
                        })
                        .map(|slint| slint.get_instance_slot().get_parentoperative_slot())
                        .any(|slotted_op| {
                            slotted_op.get_id() != selected_single_op.get().unwrap().get_id()
                        })
                })
            }
            SlotTypeSpecializationTraitObjectDiscriminants::OperativeSlotTypeMultiSpecialization => {
                all_descendent_ops.clone().into_iter().any(|op| {
                    op.get_slottedinstances_slot()
                        .into_iter()
                        .filter(|slint| *slint.get_slottedslot_slot().get_id() == match spec_target_clone.clone(){
                            SlotTypeSpecializableTraitOperativeTraitObject::TemplateSlotTraitOperative(item) => item.get_roottemplateslot_slot().get_id().clone(),
                            SlotTypeSpecializableTraitOperativeTraitObject::OperativeSlotTypeTraitObjectSpecialization(item) => item.get_roottemplateslot_slot().get_id().clone(),
                        })
                        .map(|slint| slint.get_instance_slot().get_parentoperative_slot())
                        .any(|slotted_op| !selected_list_of_ops.get().contains(&slotted_op))
                })
            }
            SlotTypeSpecializationTraitObjectDiscriminants::OperativeSlotTypeTraitObjectSpecialization => {
                let mut total_trait_list =  match spec_target_clone.clone() {
                            SlotTypeSpecializableTraitOperativeTraitObject::TemplateSlotTraitOperative(item) => item
                                .get_allowedtraits_slot()
                                .into_iter()
                                .collect::<BTreeSet<_>>(),
                            SlotTypeSpecializableTraitOperativeTraitObject::OperativeSlotTypeTraitObjectSpecialization(
                                item,
                            ) => get_all_traits_in_specialization(item),
                        };
                total_trait_list.extend(selected_list_of_traits.get());
                let total_trait_list = total_trait_list.into_iter().collect::<Vec<_>>();
                let all_compliant_ops =
                    get_all_operatives_which_impl_trait_set(total_trait_list, &schema_clone);
                all_descendent_ops.clone().into_iter().any(|op| {
                    op.get_slottedinstances_slot()
                        .into_iter()
                        .filter(|slint| *slint.get_slottedslot_slot().get_id() == match spec_target_clone.clone(){
                            SlotTypeSpecializableTraitOperativeTraitObject::TemplateSlotTraitOperative(item) => item.get_roottemplateslot_slot().get_id().clone(),
                            SlotTypeSpecializableTraitOperativeTraitObject::OperativeSlotTypeTraitObjectSpecialization(item) => item.get_roottemplateslot_slot().get_id().clone(),
                        })
                        .map(|slint| slint.get_instance_slot().get_parentoperative_slot())
                        .any(|slotted_op| !all_compliant_ops.contains(&slotted_op))
                })
            }
        };
        if is_error {
            leptos::logging::warn!(
                "Some downstream operative has an incompatible instance slotted"
            );
            return ();
        }
        let operative_clone = operative.clone();
        match selected_spec.get() {
            SlotTypeSpecializationTraitObjectDiscriminants::OperativeSlotTypeSingleSpecialization => {
                let mut editor = operative.edit(ctx_clone.clone());
                let mut new_slot_spec = OperativeSlotTypeSingleSpecialization::new(ctx_clone.clone())
                    .set_temp_id("new_slot_spec")
                    .add_existing_allowedoperative(
                        selected_single_op.get().unwrap().get_id(),
                        |na| na,
                    )
                    .add_existing_specializer(operative_clone.get_id(), |na| na);
                match spec_target_clone.clone() {
    SlotTypeSpecializableTraitOperativeTraitObject::TemplateSlotTraitOperative(item) => {
                            editor.incorporate(
                                new_slot_spec
                                    .clone()
                                    .add_existing_roottemplateslot(
                                        item.get_roottemplateslot_slot().get_id(),
                                        |na| na,
                                    )
                                    .add_existing_specializationtarget::<TemplateSlotTraitOperative>(
                                        spec_target_clone.get_id(),
                                        |na| na,
                                    ),
                            );
                        },
    SlotTypeSpecializableTraitOperativeTraitObject::OperativeSlotTypeTraitObjectSpecialization(item) => {
                                editor.incorporate(
                                    new_slot_spec
                                        .clone()
                                        .add_existing_roottemplateslot(
                                            item.get_roottemplateslot_slot().get_id(),
                                            |na| na,
                                        )
                                        .add_existing_specializationtarget::<OperativeSlotTypeTraitObjectSpecialization>(
                                            spec_target_clone.get_id(),
                                            |na| na,
                                        ),
                                );
                            },
                };
                all_descendent_ops.into_iter().for_each(|desc_op| {
                    editor.incorporate(
                        desc_op
                            .edit(ctx_clone.clone())
                            .add_temp_slottypespecializations::<OperativeSlotTypeSingleSpecialization>(
                                "new_slot_spec",
                            ),
                    );
                });
                editor.execute().unwrap();
            }
            SlotTypeSpecializationTraitObjectDiscriminants::OperativeSlotTypeMultiSpecialization => {
                let mut editor = operative.edit(ctx_clone.clone());
                let mut new_slot_spec = OperativeSlotTypeMultiSpecialization::new(ctx_clone.clone())
                    .set_temp_id("new_slot_spec")
                    .add_existing_specializer(operative_clone.get_id(), |na| na);
                match spec_target_clone.clone() {
    SlotTypeSpecializableTraitOperativeTraitObject::TemplateSlotTraitOperative(item) => {
                            editor.incorporate(
                                new_slot_spec
                                    .clone()
                                    .add_existing_roottemplateslot(
                                        item.get_roottemplateslot_slot().get_id(),
                                        |na| na,
                                    )
                                    .add_existing_specializationtarget::<TemplateSlotTraitOperative>(
                                        spec_target_clone.get_id(),
                                        |na| na,
                                    ),
                            );
                        },
    SlotTypeSpecializableTraitOperativeTraitObject::OperativeSlotTypeTraitObjectSpecialization(item) => {
                            editor.incorporate(
                                new_slot_spec
                                    .clone()
                                    .add_existing_roottemplateslot(
                                        item.get_roottemplateslot_slot().get_id(),
                                        |na| na,
                                    )
                                    .add_existing_specializationtarget::<OperativeSlotTypeTraitObjectSpecialization>(
                                        spec_target_clone.get_id(),
                                        |na| na,
                                    ),
                            );
                        },
};
                selected_list_of_ops.get().into_iter().for_each(|op| {
                    editor.incorporate(
                        new_slot_spec
                            .clone()
                            .add_existing_allowedoperatives(op.get_id(), |na| na),
                    )
                });
                all_descendent_ops.into_iter().for_each(|desc_op| {
                    editor.incorporate(
                        desc_op
                            .edit(ctx_clone.clone())
                            .add_temp_slottypespecializations::<OperativeSlotTypeMultiSpecialization>(
                                "new_slot_spec",
                            ),
                    );
                });
                editor.execute().unwrap();
            }

            SlotTypeSpecializationTraitObjectDiscriminants::OperativeSlotTypeTraitObjectSpecialization => {
                let mut editor = operative.edit(ctx_clone.clone());
                let mut new_slot_spec =
                    OperativeSlotTypeTraitObjectSpecialization::new(ctx_clone.clone())
                        .set_temp_id("new_slot_spec")
                        .add_existing_specializer(operative_clone.get_id(), |na| na);
                match spec_target_clone.clone() {
                SlotTypeSpecializableTraitOperativeTraitObject::TemplateSlotTraitOperative(item) => {
                                        editor.incorporate(
                                            new_slot_spec
                                                .clone()
                                                .add_existing_roottemplateslot(
                                                    item.get_roottemplateslot_slot().get_id(),
                                                    |na| na,
                                                )
                                                .add_existing_specializationtarget::<TemplateSlotTraitOperative>(
                                                    spec_target_clone.get_id(),
                                                    |na| na,
                                                ),
                                        );
                                    },
                SlotTypeSpecializableTraitOperativeTraitObject::OperativeSlotTypeTraitObjectSpecialization(item) => {
                                        editor.incorporate(
                                            new_slot_spec
                                                .clone()
                                                .add_existing_roottemplateslot(
                                                    item.get_roottemplateslot_slot().get_id(),
                                                    |na| na,
                                                )
                                                .add_existing_specializationtarget::<OperativeSlotTypeTraitObjectSpecialization>(
                                                    spec_target_clone.get_id(),
                                                    |na| na,
                                                ),
                                        );
                                    },
            };
                selected_list_of_traits
                    .get()
                    .into_iter()
                    .for_each(|trait_item| {
                        editor.incorporate(
                            new_slot_spec
                                .clone()
                                .add_existing_allowedtraits(trait_item.get_id(), |na| na),
                        )
                    });
                all_descendent_ops.into_iter().for_each(|desc_op| {
                    editor.incorporate(
                        desc_op
                            .edit(ctx_clone.clone())
                            .add_temp_slottypespecializations::<OperativeSlotTypeMultiSpecialization>(
                                "new_slot_spec",
                            ),
                    );
                });
                editor.execute().unwrap();
            }
        };
        is_adding.set(false);
    };

    view! {
        <LeafSection>
            <Show when=move || !is_adding.get()>
                <Button on:click=move|_|is_adding.set(true)>Add Specialization</Button>
            </Show>
            <Show when=move || is_adding.get()>
                <div>
                <SignalEnumSelect value=selected_spec />
                </div>

                {choose_ops_view}

                <div>
                <Button on:click=on_save.clone() attr:disabled=move||{
                    match selected_spec.get() {
                        SlotTypeSpecializationTraitObjectDiscriminants::OperativeSlotTypeSingleSpecialization => {

                                selected_single_op.get().is_none()
                        },
                        SlotTypeSpecializationTraitObjectDiscriminants::OperativeSlotTypeMultiSpecialization => {
                                selected_list_of_ops.get().len() < 2

                        },
                        SlotTypeSpecializationTraitObjectDiscriminants::OperativeSlotTypeTraitObjectSpecialization => {
                                selected_list_of_traits.get().len() == 0
                        },

                    }
                }>Save</Button>
                <Button on:click=move|_|is_adding.set(false)>Cancel</Button>
                </div>
            </Show>
        </LeafSection>
    }
}
