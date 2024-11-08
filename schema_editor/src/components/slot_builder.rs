use leptos::either::Either;
use schema_editor_generated_toolkit::prelude::*;

use crate::components::{
    common::{Button, SignalEnumSelect, SignalSelectWithOptions, SignalTextInput},
    workspace::WorkspaceState,
};

#[component]
pub fn SlotBuilder(
    template: RGSOConcrete<TemplateConcrete, Schema>, // builder: FreshBuilder<
    //     TemplateSlot,
    //     Schema,
    //     <TemplateSlot as StaticTypestate>::EmptyFieldTypestate,
    //     <TemplateSlot as StaticTypestate>::InitialSlotTypestate,
    // >,
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
    let slot_type =
        RwSignal::new(TemplateSlotVariantTraitObjectDiscriminants::ConreteOperativeVariant);
    let slot_bound = RwSignal::new(SlotBoundVariantTraitObjectDiscriminants::SlotBoundSingle);
    let slot_bound_max = RwSignal::new(0);
    let slot_bound_min = RwSignal::new(0);

    let dropdown_selected_trait = RwSignal::new(None); // TraitConcrete
    let selected_trait_list = RwSignal::new(Vec::new()); // TraitConcrete
    let final_selected_trait_list = Memo::new(move |_| {
        selected_trait_list
            .get()
            .into_iter()
            .filter(|item| schema_clone.get_traits_slot().contains(item))
            .collect::<Vec<_>>()
    });
    // let selected_operative_for_slot = RwSignal::new(None); //OperativeConcrete

    let schema_clone = schema.clone();

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
                .get_traits_slot()
                .into_iter()
                .filter(|item| {
                    selected_trait_list.with(|selected_list| !selected_list.contains(item))
                })
                .collect::<Vec<_>>()
        });
        let template_clone = template.clone();
        let on_click_save_trait_slot =
            move |_| {
                let mut trait_operative_variant_id = 0;
                final_selected_trait_list
                    .get()
                    .into_iter()
                    .enumerate()
                    .for_each(|(index, trait_concrete)| {
                        if index == 0 {
                            trait_operative_variant_id = match slot_bound.get() {
                            SlotBoundVariantTraitObjectDiscriminants::SlotBoundUpperBound => {
                                template_clone
                                    .edit(ctx_clone.clone())
                                    .add_new_templateslots(|new_template_slot| {
                                        new_template_slot
                                            .set_name(name.get())
                                            .add_new_operativevariant::<TraitOperativeVariant, _>(
                                                |new_op_var| {
                                                    new_op_var.add_existing_traits(
                                                        trait_concrete.get_id(),
                                                        |na| na,
                                                    )
                                                    .set_temp_id("this_is_ugly")
                                                },
                                            )
                                            .add_new_slotbound(|_| {
                                                SlotBoundUpperBound::new(ctx_clone.clone())
                                                    .set_upper_bound(slot_bound_max.get())
                                            })
                                    })
                                    .execute()
                            }
                            SlotBoundVariantTraitObjectDiscriminants::SlotBoundRangeOrZero => {
                                template_clone
                                    .edit(ctx_clone.clone())
                                    .add_new_templateslots(|new_template_slot| {
                                        new_template_slot
                                            .set_name(name.get())
                                            .add_new_operativevariant::<TraitOperativeVariant, _>(
                                                |new_op_var| {
                                                    new_op_var.add_existing_traits(
                                                        trait_concrete.get_id(),
                                                        |na| na,
                                                    )
                                                    .set_temp_id("this_is_ugly")
                                                },
                                            )
                                            .add_new_slotbound(|_| {
                                                SlotBoundRangeOrZero::new(ctx_clone.clone())
                                                    .set_upper_bound(slot_bound_max.get())
                                                    .set_lower_bound(slot_bound_min.get())
                                            })
                                    })
                                    .execute()
                            }
                            SlotBoundVariantTraitObjectDiscriminants::SlotBoundLowerBoundOrZero => {
                                template_clone
                                    .edit(ctx_clone.clone())
                                    .add_new_templateslots(|new_template_slot| {
                                        new_template_slot
                                            .set_name(name.get())
                                            .add_new_operativevariant::<TraitOperativeVariant, _>(
                                                |new_op_var| {
                                                    new_op_var.add_existing_traits(
                                                        trait_concrete.get_id(),
                                                        |na| na,
                                                    )
                                                    .set_temp_id("this_is_ugly")
                                                },
                                            )
                                            .add_new_slotbound(|_| {
                                                SlotBoundLowerBoundOrZero::new(ctx_clone.clone())
                                                    .set_lower_bound(slot_bound_min.get())
                                            })
                                    })
                                    .execute()
                            }
                            SlotBoundVariantTraitObjectDiscriminants::SlotBoundRange => {
                                template_clone
                                    .edit(ctx_clone.clone())
                                    .add_new_templateslots(|new_template_slot| {
                                        new_template_slot
                                            .set_name(name.get())
                                            .add_new_operativevariant::<TraitOperativeVariant, _>(
                                                |new_op_var| {
                                                    new_op_var.add_existing_traits(
                                                        trait_concrete.get_id(),
                                                        |na| na,
                                                    )
                                                    .set_temp_id("this_is_ugly")
                                                },
                                            )
                                            .add_new_slotbound(|_| {
                                                SlotBoundRange::new(ctx_clone.clone())
                                                    .set_upper_bound(slot_bound_max.get())
                                                    .set_lower_bound(slot_bound_min.get())
                                            })
                                    })
                                    .execute()
                            }
                            SlotBoundVariantTraitObjectDiscriminants::SlotBoundLowerBound => {
                                template_clone
                                    .edit(ctx_clone.clone())
                                    .add_new_templateslots(|new_template_slot| {
                                        new_template_slot
                                            .set_name(name.get())
                                            .add_new_operativevariant::<TraitOperativeVariant, _>(
                                                |new_op_var| {
                                                    new_op_var.add_existing_traits(
                                                        trait_concrete.get_id(),
                                                        |na| na,
                                                    )
                                                    .set_temp_id("this_is_ugly")
                                                },
                                            )
                                            .add_new_slotbound(|_| {
                                                SlotBoundLowerBound::new(ctx_clone.clone())
                                                    .set_lower_bound(slot_bound_min.get())
                                            })
                                    })
                                    .execute()
                            }
                            SlotBoundVariantTraitObjectDiscriminants::SlotBoundSingle => {
                                template_clone
                                    .edit(ctx_clone.clone())
                                    .add_new_templateslots(|new_template_slot| {
                                        new_template_slot
                                            .set_name(name.get())
                                            .add_new_operativevariant::<TraitOperativeVariant, _>(
                                                |new_op_var| {
                                                    new_op_var.add_existing_traits(
                                                        trait_concrete.get_id(),
                                                        |na| na,
                                                    )
                                                    .set_temp_id("this_is_ugly")
                                                },
                                            )
                                            .add_new_slotbound(|_| {
                                                SlotBoundSingle::new(ctx_clone.clone())
                                            })
                                    })
                                    .execute()
                            }
                        }
                        .unwrap()
                        .get_final_id("this_is_ugly")
                        .unwrap()
                        .clone() ;
                        } else {
                            let new_template_slot =
                                match ctx_clone.get(&trait_operative_variant_id).unwrap() {
                                    Schema::TraitOperativeVariant(item) => item,
                                    _ => panic!(),
                                };
                            new_template_slot
                                .edit(ctx_clone.clone())
                                .add_existing_traits(trait_concrete.get_id(), |na| na)
                                .execute()
                                .unwrap();
                        }
                    });
                close_callback.run(());
            };

        view! {
            <div>
                <SignalSelectWithOptions value=dropdown_selected_trait options=trait_options empty_allowed=true/>
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
            <div>
                <Button on:click=on_click_save_trait_slot>Save New Slot</Button>
            </div>
        }
    };
    let slot_bound_input_view = move || {
        match slot_bound.get() {
        SlotBoundVariantTraitObjectDiscriminants::SlotBoundUpperBound => view! {
            <div>
            Upper Bound: <SignalTextInput prop:min=0 prop:type="number" value=slot_bound_max />
            </div>
        }
        .into_any(),
        SlotBoundVariantTraitObjectDiscriminants::SlotBoundRangeOrZero => view! {
            <div>
            Lower Bound: <SignalTextInput prop:min=0 prop:max=move||slot_bound_max.get() prop:type="number" value=slot_bound_min />
            </div>
            <div>
            Upper Bound: <SignalTextInput prop:min=move||slot_bound_min.get().max(0) prop:type="number" value=slot_bound_max />
            </div>

        }
        .into_any(),
        SlotBoundVariantTraitObjectDiscriminants::SlotBoundLowerBoundOrZero => view! {
            <div>
            Lower Bound: <SignalTextInput prop:min=0 prop:type="number" value=slot_bound_min />
            </div>
        }
        .into_any(),
        SlotBoundVariantTraitObjectDiscriminants::SlotBoundRange => view! {
            <div>
            Lower Bound: <SignalTextInput prop:min=0 prop:max=move||slot_bound_max.get() prop:type="number" value=slot_bound_min />
            </div>
            <div>
            Upper Bound: <SignalTextInput prop:min=move||slot_bound_min.get().max(0) prop:type="number" value=slot_bound_max />
            </div>

        }
        .into_any(),
        SlotBoundVariantTraitObjectDiscriminants::SlotBoundLowerBound => view! {
            <div>
            Lower Bound: <SignalTextInput prop:min=0 prop:type="number" value=slot_bound_min />
            </div>
        }
        .into_any(),
        SlotBoundVariantTraitObjectDiscriminants::SlotBoundSingle => view! {}.into_any(),
    }
    };
    let concrete_op_slot_details_view = move || {
        view! {
            ConcreteOp stuff
        }
    };

    let schema_clone = schema.clone();
    view! {
    <div>
        <div>
            Slot name: <SignalTextInput value=name />
        </div>
        <div>
            Slot Bounds: <SignalEnumSelect value=slot_bound />
        </div>
        <div>
            Slot type: <SignalEnumSelect value=slot_type/>
        </div>
        <div>
        {move || match slot_type.get() {
                TemplateSlotVariantTraitObjectDiscriminants::TraitOperativeVariant => {
                    Either::Left(trait_slot_details_view.clone())
                }
                TemplateSlotVariantTraitObjectDiscriminants::ConreteOperativeVariant => {
                    Either::Right(concrete_op_slot_details_view)
                }
        }}
        </div>
    </div>
    }
}
