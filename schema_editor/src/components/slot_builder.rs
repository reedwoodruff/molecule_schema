use leptos::either::{Either, EitherOf3};
use schema_editor_generated_toolkit::prelude::*;

use crate::components::{
    common::{
        Button, LeafSection, LeafSectionHeader, SignalEnumSelect, SignalSelectRGSOWithOptions,
        SignalTextInput, SubSection, SubSectionHeader,
    },
    workspace::WorkspaceState,
};

#[component]
pub fn SlotBuilder(
    template: RGSOConcrete<TemplateConcrete, Schema>, // builder: FreshBuilder<
    close_callback: Callback<()>,
) -> impl IntoView {
    let ctx = use_context::<SharedGraph<Schema>>().unwrap();
    let ctx_clone = ctx.clone();
    let WorkspaceState {
        schema,
        selected_tab,
    } = use_context::<WorkspaceState>().unwrap();
    let schema_clone = schema.clone();

    let name = RwSignal::new("new_slot".to_string());
    let slot_type = RwSignal::new(
        TemplateSlotTypeVariantTraitObjectDiscriminants::TemplateSlotTypeSingleOperative,
    );
    let slot_bound = RwSignal::new(
        TemplateSlotCardinalityVariantTraitObjectDiscriminants::TemplateSlotCardinalitySingle,
    );
    let slot_bound_max = RwSignal::new(0);
    let slot_bound_min = RwSignal::new(0);

    let dropdown_selected_trait = RwSignal::new(None); // TraitConcrete
    let selected_trait_list = RwSignal::new(Vec::new()); // TraitConcrete
    let final_selected_trait_list = Memo::new(move |_| {
        selected_trait_list
            .get()
            .into_iter()
            .filter(|item| schema_clone.get().get_traits_slot().contains(item))
            .collect::<Vec<_>>()
    });
    // let selected_operative_for_slot = RwSignal::new(None); //OperativeConcrete

    let schema_clone = schema.clone();
    let template_clone = template.clone();
    let on_click_save_trait_slot = move || {
        let mut trait_operative_variant_id = 0;
        final_selected_trait_list
            .get()
            .into_iter()
            .enumerate()
            .for_each(|(index, trait_concrete)| {
                if index == 0 {
                    trait_operative_variant_id = match slot_bound.get() {
                        TemplateSlotCardinalityVariantTraitObjectDiscriminants::TemplateSlotCardinalityRangeOrZero => {
                            template_clone
                                .edit(ctx_clone.clone())
                                .add_new_templateslots(|new_template_slot| {
                                    new_template_slot
                                        .set_temp_id("template_slot")
                                        .set_name(name.get())
                                        .add_new_templateslotvariant::<TemplateSlotTypeTraitOperative, _>(
                                            |new_op_var| {
                                                new_op_var
                                                    .set_temp_id("this_is_ugly")
                                                    .add_temp_roottemplateslot("template_slot")
                                                    .add_existing_allowedtraits(
                                                        trait_concrete.get_id(),
                                                        |na| na,
                                                    )
                                            },
                                        )
                                        .add_new_slotcardinality::<TemplateSlotCardinalityRangeOrZero, _>(
                                            |slot_bound| {
                                                slot_bound
                                                    .set_upper_bound(slot_bound_max.get())
                                                    .set_lower_bound(slot_bound_min.get())
                                            },
                                        )
                                })
                                .execute()
                        }
                        TemplateSlotCardinalityVariantTraitObjectDiscriminants::TemplateSlotCardinalityLowerBoundOrZero => {
                            template_clone
                                .edit(ctx_clone.clone())
                                .add_new_templateslots(|new_template_slot| {
                                    new_template_slot
                                        .set_temp_id("template_slot")
                                        .set_name(name.get())
                                        .add_new_templateslotvariant::<TemplateSlotTypeTraitOperative, _>(
                                            |new_op_var| {
                                                new_op_var
                                                    .set_temp_id("this_is_ugly")
                                                    .add_temp_roottemplateslot("template_slot")
                                                    .add_existing_allowedtraits(
                                                        trait_concrete.get_id(),
                                                        |na| na,
                                                    )
                                            },
                                        )
                                        .add_new_slotcardinality::<TemplateSlotCardinalityLowerBoundOrZero, _>(
                                            |slot_bound| {
                                                slot_bound.set_lower_bound(slot_bound_min.get())
                                            },
                                        )
                                })
                                .execute()
                        }
                        TemplateSlotCardinalityVariantTraitObjectDiscriminants::TemplateSlotCardinalityRange => template_clone
                            .edit(ctx_clone.clone())
                            .add_new_templateslots(|new_template_slot| {
                                new_template_slot
                                    .set_temp_id("template_slot")
                                    .set_name(name.get())
                                    .add_new_templateslotvariant::<TemplateSlotTypeTraitOperative, _>(
                                        |new_op_var| {
                                            new_op_var
                                                .set_temp_id("this_is_ugly")
                                                .add_temp_roottemplateslot("template_slot")
                                                .add_existing_allowedtraits(
                                                    trait_concrete.get_id(),
                                                    |na| na,
                                                )
                                        },
                                    )
                                    .add_new_slotcardinality::<TemplateSlotCardinalityRange, _>(|slot_bound| {
                                        slot_bound
                                            .set_upper_bound(slot_bound_max.get())
                                            .set_lower_bound(slot_bound_min.get())
                                    })
                            })
                            .execute(),
                        TemplateSlotCardinalityVariantTraitObjectDiscriminants::TemplateSlotCardinalityLowerBound => {
                            template_clone
                                .edit(ctx_clone.clone())
                                .add_new_templateslots(|new_template_slot| {
                                    new_template_slot
                                        .set_temp_id("template_slot")
                                        .set_name(name.get())
                                        .add_new_templateslotvariant::<TemplateSlotTypeTraitOperative, _>(
                                            |new_op_var| {
                                                new_op_var
                                                    .set_temp_id("this_is_ugly")
                                                    .add_temp_roottemplateslot("template_slot")
                                                    .add_existing_allowedtraits(
                                                        trait_concrete.get_id(),
                                                        |na| na,
                                                    )
                                            },
                                        )
                                        .add_new_slotcardinality::<TemplateSlotCardinalityLowerBound, _>(|slot_bound| {
                                            slot_bound.set_lower_bound(slot_bound_min.get())
                                        })
                                })
                                .execute()
                        }
                        TemplateSlotCardinalityVariantTraitObjectDiscriminants::TemplateSlotCardinalitySingle => template_clone
                            .edit(ctx_clone.clone())
                            .add_new_templateslots(|new_template_slot| {
                                new_template_slot
                                    .set_temp_id("template_slot")
                                    .set_name(name.get())
                                    .add_new_templateslotvariant::<TemplateSlotTypeTraitOperative, _>(
                                        |new_op_var| {
                                            new_op_var
                                                .set_temp_id("this_is_ugly")
                                                .add_temp_roottemplateslot("template_slot")
                                                .add_existing_allowedtraits(
                                                    trait_concrete.get_id(),
                                                    |na| na,
                                                )
                                        },
                                    )
                                    .add_new_slotcardinality::<TemplateSlotCardinalitySingle, _>(|slot_bound| {
                                        slot_bound
                                    })
                            })
                            .execute(),
                    }
                    .unwrap()
                    .get_final_id("this_is_ugly")
                    .unwrap()
                    .clone();
                } else {
                    let new_template_slot =
                        match ctx_clone.get(&trait_operative_variant_id).unwrap() {
                            Schema::TemplateSlotTypeTraitOperative(item) => item,
                            _ => panic!(),
                        };
                    new_template_slot
                        .edit(ctx_clone.clone())
                        .add_existing_allowedtraits(trait_concrete.get_id(), |na| na)
                        .execute()
                        .unwrap();
                }
            });
    };

    let ctx_clone = ctx.clone();
    let trait_slot_details_view = move || {
        let ctx_clone = ctx_clone.clone();
        let schema_clone = schema_clone.clone();
        let on_click_add_trait = move |_| match dropdown_selected_trait.get() {
            Some(new_trait_item) => {
                dropdown_selected_trait.set(None);
                selected_trait_list.update(|prev| prev.push(new_trait_item));
            }
            None => (),
        };
        let trait_options = Signal::derive(move || {
            schema_clone
                .get()
                .get_traits_slot()
                .into_iter()
                .filter(|item| {
                    selected_trait_list.with(|selected_list| !selected_list.contains(item))
                })
                .collect::<Vec<_>>()
        });

        view! {
            <LeafSectionHeader>
                Required Traits For Slot
            </LeafSectionHeader>
            <div>
                <SignalSelectRGSOWithOptions value=dropdown_selected_trait options=trait_options empty_allowed=true/>
                <Button on:click=on_click_add_trait>Add</Button>
            </div>
            <div>
                <For each=move ||final_selected_trait_list.get() key=|selected_item| selected_item.get_id().clone() let:selected_item>
                {
                    let selected_item_clone = selected_item.clone();
                    let item_name = move || selected_item_clone.get_name();
                    let on_click = move |_|{
                        selected_trait_list.update(|prev| prev.retain(|prev_item| prev_item.get_id() != selected_item.get_id()))
                    };
                    view!{
                        <span>{item_name}</span><Button on:click=on_click>X</Button>,
                    }
                }
                </For>
            </div>
        }
    };
    let slot_bound_input_view = move || {
        match slot_bound.get() {
        TemplateSlotCardinalityVariantTraitObjectDiscriminants::TemplateSlotCardinalityRangeOrZero => view! {
            <div>
            Lower Bound: <SignalTextInput prop:min=0 prop:max=move||slot_bound_max.get() prop:type="number" value=slot_bound_min />
            </div>
            <div>
            Upper Bound: <SignalTextInput prop:min=move||slot_bound_min.get().max(0) prop:type="number" value=slot_bound_max />
            </div>

        }
        .into_any(),
        TemplateSlotCardinalityVariantTraitObjectDiscriminants::TemplateSlotCardinalityLowerBoundOrZero => view! {
            <div>
            Lower Bound: <SignalTextInput prop:min=0 prop:type="number" value=slot_bound_min />
            </div>
        }
        .into_any(),
        TemplateSlotCardinalityVariantTraitObjectDiscriminants::TemplateSlotCardinalityRange => view! {
            <div>
            Lower Bound: <SignalTextInput prop:min=0 prop:max=move||slot_bound_max.get() prop:type="number" value=slot_bound_min />
            </div>
            <div>
            Upper Bound: <SignalTextInput prop:min=move||slot_bound_min.get().max(0) prop:type="number" value=slot_bound_max />
            </div>

        }
        .into_any(),
        TemplateSlotCardinalityVariantTraitObjectDiscriminants::TemplateSlotCardinalityLowerBound => view! {
            <div>
            Lower Bound: <SignalTextInput prop:min=0 prop:type="number" value=slot_bound_min />
            </div>
        }
        .into_any(),
        TemplateSlotCardinalityVariantTraitObjectDiscriminants::TemplateSlotCardinalitySingle => view! {
            <div>
            Exactly: 1
            </div>
        }.into_any(),
    }
    };

    let template_clone = template.clone();
    let schema_clone = schema.clone();
    let selected_single_operative =
        RwSignal::<Option<RGSOConcrete<OperativeConcrete, Schema>>>::new(None);
    let multi_operative_list =
        RwSignal::<Vec<RGSOConcrete<OperativeConcrete, Schema>>>::new(vec![]);
    let ctx_clone = ctx.clone();
    let on_click_save_multi_op = move || {
        let mut editor = template_clone.edit(ctx_clone.clone());
        let mut tempslotvariant = TemplateSlotTypeMultiOperative::new(ctx_clone.clone())
            .set_temp_id("tempslotvariant")
            .add_temp_roottemplateslot("new_template_slot");
        // This is a bad hack and is subject to suddenly failing to work as intended.
        // I think it is relying on the merging of multiple "new" nodes into one since they share the same temp_id
        // The better solution would be to make a typestate-erased version of the FreshBuilder which can be opted into
        multi_operative_list.get().into_iter().for_each(|op| {
            editor.incorporate(
                &tempslotvariant
                    .clone()
                    .add_existing_allowedoperatives(op.get_id(), |na| na),
            )
        });
        match slot_bound.get() {
            TemplateSlotCardinalityVariantTraitObjectDiscriminants::TemplateSlotCardinalityRangeOrZero => editor
                .incorporate(
                    &TemplateSlot::new(ctx_clone.clone())
                        .add_new_slotcardinality::<TemplateSlotCardinalityRangeOrZero, _>(
                            |new_slot_bound| {
                                new_slot_bound
                                    .set_temp_id("slot_bound")
                                    .set_upper_bound(slot_bound_max.get())
                                    .set_lower_bound(slot_bound_min.get())
                            },
                        )
                        .set_name(name.get())
                        .set_temp_id("new_template_slot")
                        .add_temp_templateslotvariant::<TemplateSlotTypeMultiOperative>(
                            "tempslotvariant",
                        ),
                ),
            TemplateSlotCardinalityVariantTraitObjectDiscriminants::TemplateSlotCardinalityLowerBoundOrZero => editor
                .incorporate(
                    &TemplateSlot::new(ctx_clone.clone())
                        .add_new_slotcardinality::<TemplateSlotCardinalityLowerBoundOrZero, _>(|new_slot_bound| {
                            new_slot_bound
                                .set_temp_id("slot_bound")
                                .set_lower_bound(slot_bound_min.get())
                        })
                        .set_name(name.get())
                        .set_temp_id("new_template_slot")
                        .add_temp_templateslotvariant::<TemplateSlotTypeMultiOperative>(
                            "tempslotvariant",
                        ),
                ),
            TemplateSlotCardinalityVariantTraitObjectDiscriminants::TemplateSlotCardinalityRange => editor.incorporate(
                &TemplateSlot::new(ctx_clone.clone())
                    .add_new_slotcardinality::<TemplateSlotCardinalityRange, _>(|new_slot_bound| {
                        new_slot_bound
                            .set_temp_id("slot_bound")
                            .set_upper_bound(slot_bound_max.get())
                            .set_lower_bound(slot_bound_min.get())
                    })
                    .set_name(name.get())
                    .set_temp_id("new_template_slot")
                    .add_temp_templateslotvariant::<TemplateSlotTypeMultiOperative>("tempslotvariant"),
            ),
            TemplateSlotCardinalityVariantTraitObjectDiscriminants::TemplateSlotCardinalityLowerBound => editor.incorporate(
                &TemplateSlot::new(ctx_clone.clone())
                    .add_new_slotcardinality::<TemplateSlotCardinalityLowerBound, _>(|new_slot_bound| {
                        new_slot_bound
                            .set_temp_id("slot_bound")
                            .set_lower_bound(slot_bound_min.get())
                    })
                    .set_name(name.get())
                    .set_temp_id("new_template_slot")
                    .add_temp_templateslotvariant::<TemplateSlotTypeMultiOperative>("tempslotvariant"),
            ),
            TemplateSlotCardinalityVariantTraitObjectDiscriminants::TemplateSlotCardinalitySingle => editor.incorporate(
                &TemplateSlot::new(ctx_clone.clone())
                    .add_new_slotcardinality::<TemplateSlotCardinalitySingle, _>(|new_slot_bound| {
                        new_slot_bound.set_temp_id("slot_bound")
                    })
                    .set_name(name.get())
                    .set_temp_id("new_template_slot")
                    .add_temp_templateslotvariant::<TemplateSlotTypeMultiOperative>("tempslotvariant"),
            ),
        }
        editor
            .add_temp_templateslots("new_template_slot")
            .execute()
            .unwrap();
    };

    let ctx_clone = ctx.clone();
    let template_clone = template.clone();
    let on_click_save_single_op = move || {
        let maybe_op_id = selected_single_operative.get();
        if maybe_op_id.is_none() {
            return;
        }
        let op_id = &maybe_op_id.unwrap().get_id().clone();
        match slot_bound.get() {
            TemplateSlotCardinalityVariantTraitObjectDiscriminants::TemplateSlotCardinalityRangeOrZero => {
                template_clone
                    .edit(ctx_clone.clone())
                    .add_new_templateslots(|new_template_slot| {
                        new_template_slot
                            .set_name(name.get())
                            .set_temp_id("new_templateslot")
                            .add_new_templateslotvariant::<TemplateSlotTypeSingleOperative, _>(
                                |new_op_var| {
                                    new_op_var
                                        .add_existing_allowedoperative(op_id, |na| na)
                                        .add_temp_roottemplateslot(
                                            "new_templateslot"                                        )
                                },
                            )
                            .add_new_slotcardinality::<TemplateSlotCardinalityRangeOrZero, _>(
                                |slot_bound| {
                                    slot_bound
                                        .set_upper_bound(slot_bound_max.get())
                                        .set_lower_bound(slot_bound_min.get())
                                },
                            )
                    })
                    .execute()
            }
            TemplateSlotCardinalityVariantTraitObjectDiscriminants::TemplateSlotCardinalityLowerBoundOrZero => template_clone
                .edit(ctx_clone.clone())
                .add_new_templateslots(|new_template_slot| {
                    new_template_slot
                        .set_temp_id("new_templateslot")
                        .set_name(name.get())
                        .add_new_templateslotvariant::<TemplateSlotTypeSingleOperative, _>(
                            |new_op_var| {
                                new_op_var
                                    .add_existing_allowedoperative(op_id, |na| na)
                                    .add_temp_roottemplateslot("new_templateslot")
                            },
                        )
                        .add_new_slotcardinality::<TemplateSlotCardinalityLowerBoundOrZero, _>(|slot_bound| {
                            slot_bound.set_lower_bound(slot_bound_min.get())
                        })
                })
                .execute(),
            TemplateSlotCardinalityVariantTraitObjectDiscriminants::TemplateSlotCardinalityRange => template_clone
                .edit(ctx_clone.clone())
                .add_new_templateslots(|new_template_slot| {
                    new_template_slot
                        .set_temp_id("new_templateslot")
                        .set_name(name.get())
                        .add_new_templateslotvariant::<TemplateSlotTypeSingleOperative, _>(
                            |new_op_var| {
                                new_op_var
                                    .add_existing_allowedoperative(op_id, |na| na)
                                    .add_temp_roottemplateslot("new_templateslot")
                            },
                        )
                        .add_new_slotcardinality::<TemplateSlotCardinalityRange, _>(|slot_bound| {
                            slot_bound
                                .set_upper_bound(slot_bound_max.get())
                                .set_lower_bound(slot_bound_min.get())
                        })
                })
                .execute(),
            TemplateSlotCardinalityVariantTraitObjectDiscriminants::TemplateSlotCardinalityLowerBound => template_clone
                .edit(ctx_clone.clone())
                .add_new_templateslots(|new_template_slot| {
                    new_template_slot
                        .set_temp_id("new_templateslot")
                        .set_name(name.get())
                        .add_new_templateslotvariant::<TemplateSlotTypeSingleOperative, _>(
                            |new_op_var| {
                                new_op_var
                                    .add_existing_allowedoperative(op_id, |na| na)
                                    .add_temp_roottemplateslot("new_templateslot")
                            },
                        )
                        .add_new_slotcardinality::<TemplateSlotCardinalityLowerBound, _>(|slot_bound| {
                            slot_bound.set_lower_bound(slot_bound_min.get())
                        })
                })
                .execute(),
            TemplateSlotCardinalityVariantTraitObjectDiscriminants::TemplateSlotCardinalitySingle => template_clone
                .edit(ctx_clone.clone())
                .add_new_templateslots(|new_template_slot| {
                    new_template_slot
                        .set_temp_id("new_templateslot")
                        .set_name(name.get())
                        .add_new_templateslotvariant::<TemplateSlotTypeSingleOperative, _>(
                            |new_op_var| {
                                new_op_var
                                    .add_existing_allowedoperative(op_id, |na| na)
                                    .add_temp_roottemplateslot("new_templateslot")
                            },
                        )
                        .add_new_slotcardinality::<TemplateSlotCardinalitySingle, _>(|slot_bound| slot_bound)
                })
                .execute(),
        }
        .unwrap();
    };
    let schema_clone = schema.clone();
    let single_op_slot_details_view = move || {
        let operative_options = schema_clone.get().get_operatives_slot();
        view! {
            <LeafSectionHeader>
                Operative Chosen for Slot
            </LeafSectionHeader>
            <div>
               <SignalSelectRGSOWithOptions value=selected_single_operative options=operative_options empty_allowed=true/>
            </div>
        }
    };
    let schema_clone = schema.clone();
    let multi_op_slot_details_view = move || {
        let operative_options = schema_clone
            .get()
            .get_operatives_slot()
            .into_iter()
            .filter(|op| !multi_operative_list.get().contains(op))
            .collect::<Vec<_>>();
        let add_to_multi_select = move |_| {
            if let Some(single_op) = selected_single_operative.get() {
                multi_operative_list.update(|prev| {
                    prev.push(single_op);
                });
            }
        };
        view! {
            <LeafSectionHeader>
                Operatives Chosen for Slot
            </LeafSectionHeader>
            <LeafSection attr:class="leafsection dependent">
            <For each=move || multi_operative_list.get() key=|op| op.get_id().clone() children=move |op| {
                    let op_clone = op.clone();
                    let on_remove = move |_| {
                        multi_operative_list.update(|prev| {
                            prev.retain(|item| item.get_id() != op_clone.get_id());
                        });
                    };
                    view!{
                        <div>
                        {move || op.get_name()}
                        <Button on:click=on_remove>Remove</Button>
                        </div>
                    }

            } />
            </LeafSection>
            <div>
               <SignalSelectRGSOWithOptions value=selected_single_operative options=operative_options empty_allowed=true/>
               <Button on:click=add_to_multi_select attr:disabled=move || selected_single_operative.get().is_none()>+</Button>
            </div>
        }
    };

    let schema_clone = schema.clone();
    let close_callback_clone = close_callback.clone();
    view! {
    <SubSection>
        <div class="flex">
            <div class="flex-grow">
                <SubSectionHeader>
                    Adding New Slot
                </SubSectionHeader>
            </div>
            <div class="align-right">
                <Button on:click=move|_| close_callback_clone.run(())>Cancel</Button>
            </div>
        </div>
        <LeafSection>
            <LeafSectionHeader>
                Slot Name
            </LeafSectionHeader>
            <SignalTextInput value=name />
        </LeafSection>
        <LeafSection>
            <LeafSectionHeader>
                Slot Bounds
            </LeafSectionHeader>
            <SignalEnumSelect value=slot_bound />
            <LeafSection attr:class="leafsection dependent">
                {slot_bound_input_view}
            </LeafSection>
        </LeafSection>
        <LeafSection>
            <LeafSectionHeader>
                Slot Type
            </LeafSectionHeader>
            <SignalEnumSelect value=slot_type/>
            <LeafSection attr:class="leafsection dependent">
                {move || match slot_type.get() {
                        TemplateSlotTypeVariantTraitObjectDiscriminants::TemplateSlotTypeTraitOperative => {
                            EitherOf3::A(trait_slot_details_view.clone())
                        }
                        TemplateSlotTypeVariantTraitObjectDiscriminants::TemplateSlotTypeSingleOperative => {
                            EitherOf3::B(single_op_slot_details_view.clone())
                        }
                        TemplateSlotTypeVariantTraitObjectDiscriminants::TemplateSlotTypeMultiOperative => {
                            EitherOf3::C(multi_op_slot_details_view.clone())
                        }
                }}
            </LeafSection>
        </LeafSection>
        <div>
            <Button on:click=move |_| {
                match slot_type.get() {
                    TemplateSlotTypeVariantTraitObjectDiscriminants::TemplateSlotTypeTraitOperative => on_click_save_trait_slot() ,
                    TemplateSlotTypeVariantTraitObjectDiscriminants::TemplateSlotTypeSingleOperative => on_click_save_single_op(),
                    TemplateSlotTypeVariantTraitObjectDiscriminants::TemplateSlotTypeMultiOperative => on_click_save_multi_op(),
                };
                close_callback.run(());
            } attr:disabled=move || {
                match slot_type.get() {
                    TemplateSlotTypeVariantTraitObjectDiscriminants::TemplateSlotTypeTraitOperative => final_selected_trait_list.with(|list| list.is_empty()),
                    TemplateSlotTypeVariantTraitObjectDiscriminants::TemplateSlotTypeSingleOperative => selected_single_operative.with(|item| item.is_none()),
                    TemplateSlotTypeVariantTraitObjectDiscriminants::TemplateSlotTypeMultiOperative => multi_operative_list.with(|list| list.len() < 2),
                }
            }>Save New Slot</Button>
        </div>
    </SubSection>
    }
}
