use std::{collections::BTreeSet, str::FromStr};

use crate::components::{
    common::*,
    operative_lineage::OperativeLineage,
    specialization_builder::SpecializationBuilder,
    specialization_lineage::SpecializationLineage,
    utils::{
        get_all_descendent_instances, get_all_descendent_operators,
        get_all_instances_which_impl_trait_set, get_all_instances_which_satisfy_specialization,
        get_all_operatives_which_satisfy_specializable,
        get_childest_specialization_for_op_and_slot,
    },
    workspace::{WorkspaceState, WorkspaceTab},
};
use leptos::either::{Either, EitherOf3, EitherOf4, EitherOf6};
use schema_editor_generated_toolkit::{
    prelude::*, slot_markers::OperativeConcreteLockedFieldsAcceptableTargetMarker,
};
use strum::{Display, EnumIter, EnumString};
#[derive(EnumIter, Display, EnumString, PartialEq, Clone)]
enum BoolOptions {
    #[strum(serialize = "true")]
    True,
    #[strum(serialize = "false")]
    False,
}

#[component]
pub fn OperativeEditor(operative: RGSOConcrete<OperativeConcrete, Schema>) -> impl IntoView {
    let derivative_operative_name = RwSignal::new(operative.get_name_field());
    let derivative_instance_name = RwSignal::new(operative.get_name_field());
    let ctx = use_context::<SharedGraph<Schema>>().unwrap();
    let WorkspaceState {
        schema,
        selected_tab,
    } = use_context::<WorkspaceState>().unwrap();
    let ctx_clone = ctx.clone();
    let schema_clone = schema.clone();
    let selected_tab = selected_tab.clone();
    let operative_clone = operative.clone();

    let create_derivative_operative = move |_| {
        // Really not liking being forced to do two graph actions -- need to figure out how to fix the api.
        let derivative_operative_name = derivative_operative_name.clone().get();
        let mut editor = schema_clone
            .edit(ctx_clone.clone())
            .add_new_operatives(|op| {
                op.set_name(derivative_operative_name.clone())
                    .add_existing_roottemplate(
                        operative_clone.get_roottemplate_slot().get_id(),
                        |item| item,
                    )
                    .add_existing_parentoperative(operative_clone.get_id(), |na| na)
                    .set_temp_id("new_op")
            });
        editor.incorporate(
            operative_clone
                .edit(ctx_clone.clone())
                .add_temp_childrenoperatives("new_op"),
        );
        let new_op_id = editor
            .execute()
            .unwrap()
            .get_final_id("new_op")
            .unwrap()
            .clone();

        let locked_fields = operative_clone.get_lockedfields_slot();
        if locked_fields.len() > 0 {
            match ctx_clone.get(&new_op_id).unwrap() {
                Schema::OperativeConcrete(item) => {
                    let mut editor = item.edit(ctx_clone.clone());
                    for locked_field in operative_clone.get_lockedfields_slot() {
                        match locked_field {
                            FulfilledFieldVariantTraitObject::BoolFulfilledField(_) => {
                                editor = editor.add_existing_lockedfields::<BoolFulfilledField>(
                                    locked_field.get_id(),
                                    |na| na,
                                )
                            }
                            FulfilledFieldVariantTraitObject::IntFulfilledField(_) => {
                                editor = editor.add_existing_lockedfields::<IntFulfilledField>(
                                    locked_field.get_id(),
                                    |na| na,
                                )
                            }
                            FulfilledFieldVariantTraitObject::StringFulfilledField(_) => {
                                editor = editor.add_existing_lockedfields::<StringFulfilledField>(
                                    locked_field.get_id(),
                                    |na| na,
                                )
                            }
                        }
                    }
                    editor.execute().unwrap();
                }
                _ => panic!(),
            };
        }
    };
    let ctx_clone = ctx.clone();
    let operative_clone = operative.clone();
    let schema_clone = schema.clone();
    let create_derivative_instance = move |_| {
        let derivative_instance_name = derivative_instance_name.clone().get();
        let mut editor = schema_clone
            .edit(ctx_clone.clone())
            .add_new_instances(|new_inst| {
                new_inst
                    .set_name(derivative_instance_name.clone())
                    .add_existing_parentoperative(operative_clone.get_id(), |na| na)
                    .set_temp_id("new_inst")
            });
        editor.execute().unwrap();
    };

    let operative_clone = operative.clone();
    let ctx_clone = ctx.clone();
    let update_name = move |new_val: String| {
        let editor = operative_clone.edit(ctx_clone.clone());
        editor.set_name(new_val).execute().unwrap();
    };

    let operative_clone = operative.clone();
    let ctx_clone = ctx.clone();
    let selected_tab_clone = selected_tab.clone();
    let delete_operative = move |_| {
        let ctx_clone = ctx_clone.clone();
        operative_clone
            .edit(ctx_clone)
            .delete_recursive()
            .execute()
            .unwrap();
        selected_tab_clone.set(WorkspaceTab::Operative(RwSignal::new(None)))
    };

    let operative_clone = operative.clone();
    let non_locked_fields: Memo<(Vec<_>, Vec<_>)> = Memo::new(move |_| {
        let locked_fields = operative_clone
            .get_lockedfields_slot()
            .into_iter()
            .map(|item| match item {
                FulfilledFieldVariantTraitObject::BoolFulfilledField(item) => {
                    item.get_constraintreference_slot().get_id().clone()
                }
                FulfilledFieldVariantTraitObject::IntFulfilledField(item) => {
                    item.get_constraintreference_slot().get_id().clone()
                }
                FulfilledFieldVariantTraitObject::StringFulfilledField(item) => {
                    item.get_constraintreference_slot().get_id().clone()
                }
            })
            .collect::<Vec<_>>();
        let non_locked = operative_clone
            .get_roottemplate_slot()
            .get_fields_slot()
            .into_iter()
            .filter(|field| !locked_fields.contains(field.get_id()));
        // .collect::<Vec<_>>();
        non_locked.partition(|item| {
            recursive_search_for_locked_field(
                operative_clone.get_childrenoperatives_slot(),
                item.get_id(),
            )
        })
    });

    let operative_clone = operative.clone();
    let ctx_clone = ctx.clone();
    let non_locked_field_view = move |field: GetNameFieldVariantTraitObject| {
        let field_clone = field.clone();
        let string_of_thing: GetNameFieldVariantTraitObjectDiscriminants = field.clone().into();
        let operative_clone = operative_clone.clone();
        let ctx_clone = ctx_clone.clone();
        let on_click_lock = move |_| {
            match field_clone {
                GetNameFieldVariantTraitObject::StringTemplateField(_) => {
                    let mut editor = operative_clone
                        .edit(ctx_clone.clone())
                        .add_new_lockedfields::<StringFulfilledField, _>(|locked_field| {
                            locked_field
                                .set_temp_id("the_field")
                                .set_value("".to_string())
                                .add_existing_fulfiller(operative_clone.get_id(), |na| na)
                                .add_existing_constraintreference(field_clone.get_id(), |na| na)
                        });
                    recurse_add_locked_field::<StringFulfilledField>(
                        operative_clone.get_childrenoperatives_slot(),
                        &mut editor,
                        &ctx_clone,
                    );
                    editor.execute().unwrap();
                }
                GetNameFieldVariantTraitObject::BoolTemplateField(_) => {
                    let mut editor = operative_clone
                        .edit(ctx_clone.clone())
                        .add_new_lockedfields::<BoolFulfilledField, _>(|locked_field| {
                            locked_field
                                .set_temp_id("the_field")
                                .set_value(true)
                                .add_existing_fulfiller(operative_clone.get_id(), |na| na)
                                .add_existing_constraintreference(field_clone.get_id(), |na| na)
                        });
                    recurse_add_locked_field::<BoolFulfilledField>(
                        operative_clone.get_childrenoperatives_slot(),
                        &mut editor,
                        &ctx_clone,
                    );
                    editor.execute().unwrap();
                }
                GetNameFieldVariantTraitObject::IntTemplateField(_) => {
                    let mut editor = operative_clone
                        .edit(ctx_clone.clone())
                        .add_new_lockedfields::<IntFulfilledField, _>(|locked_field| {
                            locked_field
                                .set_temp_id("the_field")
                                .set_value(0)
                                .add_existing_fulfiller(operative_clone.get_id(), |na| na)
                                .add_existing_constraintreference(field_clone.get_id(), |na| na)
                        });
                    recurse_add_locked_field::<IntFulfilledField>(
                        operative_clone.get_childrenoperatives_slot(),
                        &mut editor,
                        &ctx_clone,
                    );
                    editor.execute().unwrap();
                }
            };
        };
        view! {
            <LeafSection>
            <LeafSectionHeader>
            {move || field.get_name()}
            </LeafSectionHeader>
            {string_of_thing.to_string()} <Button on:click=on_click_lock>Lock</Button>
            </LeafSection>
        }
    };
    let non_locked_but_dependent_field_view = move |field: GetNameFieldVariantTraitObject| {
        let string_of_thing: GetNameFieldVariantTraitObjectDiscriminants = field.clone().into();
        view! {
            <LeafSection>
            <LeafSectionHeader>
            {move || field.get_name()}
            </LeafSectionHeader>
            {string_of_thing.to_string()}
            </LeafSection>
        }
    };

    let operative_clone = operative.clone();
    let locked_fields: Memo<(Vec<_>, Vec<_>)> = Memo::new(move |_| {
        operative_clone
            .get_lockedfields_slot()
            .into_iter()
            .partition(|locked_field| match locked_field {
                FulfilledFieldVariantTraitObject::BoolFulfilledField(item) => {
                    item.get_fulfiller_slot().get_id() == operative_clone.get_id()
                }
                FulfilledFieldVariantTraitObject::IntFulfilledField(item) => {
                    item.get_fulfiller_slot().get_id() == operative_clone.get_id()
                }
                FulfilledFieldVariantTraitObject::StringFulfilledField(item) => {
                    item.get_fulfiller_slot().get_id() == operative_clone.get_id()
                }
            })
    });

    let unowned_locked_field_view = move |field: FulfilledFieldVariantTraitObject| {
        let field_view = move || match field.clone() {
            FulfilledFieldVariantTraitObject::BoolFulfilledField(value) => {
                let value_clone = value.clone();
                let name = move || value_clone.get_constraintreference_slot().get_name();
                let value_clone = value.clone();
                let value = move || value_clone.get_value_field();
                EitherOf3::A(view! {
                    <LeafSectionHeader>
                    {name}
                    </LeafSectionHeader>
                    {value}
                })
            }
            FulfilledFieldVariantTraitObject::IntFulfilledField(value) => {
                let value_clone = value.clone();
                let name = move || value_clone.get_constraintreference_slot().get_name();
                let value_clone = value.clone();
                let value = move || value_clone.get_value_field();
                EitherOf3::B(view! {
                    <LeafSectionHeader>
                    {name}
                    </LeafSectionHeader>
                    {value}
                })
            }
            FulfilledFieldVariantTraitObject::StringFulfilledField(value) => {
                let value_clone = value.clone();
                let name = move || value_clone.get_constraintreference_slot().get_name();
                let value_clone = value.clone();
                let value = move || value_clone.get_value_field();
                EitherOf3::C(view! {
                    <LeafSectionHeader>
                    {name}
                    </LeafSectionHeader>
                    {value}
                })
            }
        };
        view! {<LeafSection>
            {field_view}
        </LeafSection>}
    };
    let ctx_clone = ctx.clone();
    let owned_locked_field_view = move |field: FulfilledFieldVariantTraitObject| {
        let field_clone = field.clone();
        let ctx_clone = ctx_clone.clone();
        let ctx_clone_2 = ctx_clone.clone();
        let field_view = move || match field_clone.clone() {
            FulfilledFieldVariantTraitObject::BoolFulfilledField(value) => {
                let value_clone = value.clone();
                let name = move || value_clone.get_constraintreference_slot().get_name();
                let ctx_clone = ctx_clone.clone();
                let value_clone = value.clone();
                let field_value = move || {
                    BoolOptions::from_str(&value_clone.get_value_field().to_string()).unwrap()
                };
                let value_clone = value.clone();
                let setter = Callback::new(move |new_val: BoolOptions| {
                    let bool_val: bool = new_val.to_string().parse().unwrap();
                    value_clone
                        .edit(ctx_clone.clone())
                        .set_value(bool_val)
                        .execute()
                        .unwrap();
                });
                EitherOf3::A(view! {
                    <LeafSectionHeader>
                    {name}
                    </LeafSectionHeader>
                    <ManagedEnumSelect getter=field_value setter=setter/>
                })
            }
            FulfilledFieldVariantTraitObject::IntFulfilledField(value) => {
                let value_clone = value.clone();
                let name = move || value_clone.get_constraintreference_slot().get_name();
                let value_clone = value.clone();
                let field_value = move || value_clone.get_value_field().to_string();
                let ctx_clone = ctx_clone.clone();
                let value_clone = value.clone();
                let setter = move |item: String| {
                    let num_val: u32 = item.parse().expect("bad number input");
                    value_clone
                        .edit(ctx_clone.clone())
                        .set_value(num_val)
                        .execute()
                        .unwrap();
                };
                EitherOf3::B(view! {
                    <LeafSectionHeader>
                    {name}
                    </LeafSectionHeader>
                    <ToggleManagedTextInput prop:type="number" prop:min=0 getter=field_value setter=setter/>
                })
            }
            FulfilledFieldVariantTraitObject::StringFulfilledField(value) => {
                let value_clone = value.clone();
                let name = move || value_clone.get_constraintreference_slot().get_name();
                let value_clone = value.clone();
                let field_value = move || value_clone.get_value_field();
                let ctx_clone = ctx_clone.clone();
                let value_clone = value.clone();

                let setter = move |item: String| {
                    value_clone
                        .edit(ctx_clone.clone())
                        .set_value(item)
                        .execute()
                        .unwrap();
                };
                EitherOf3::C(view! {
                    <LeafSectionHeader>
                    {name}
                    </LeafSectionHeader>
                    <ToggleManagedTextInput getter=field_value setter=setter/>
                })
            }
        };

        let field_clone = field.clone();
        let ctx_clone = ctx_clone_2.clone();
        let on_click_unlock = move |_| {
            match field_clone.clone() {
                FulfilledFieldVariantTraitObject::BoolFulfilledField(inner_field) => inner_field
                    .edit(ctx_clone.clone())
                    .delete()
                    .execute()
                    .unwrap(),
                FulfilledFieldVariantTraitObject::IntFulfilledField(inner_field) => inner_field
                    .edit(ctx_clone.clone())
                    .delete()
                    .execute()
                    .unwrap(),
                FulfilledFieldVariantTraitObject::StringFulfilledField(inner_field) => inner_field
                    .edit(ctx_clone.clone())
                    .delete()
                    .execute()
                    .unwrap(),
            };
        };
        view! {<LeafSection>
            {field_view}
            <Button on:click=on_click_unlock>Unlock</Button>
        </LeafSection>}
    };

    let operative_clone = operative.clone();

    let schema_clone = schema.clone();
    let each_slot_view = move |slot: RGSOConcrete<TemplateSlot, Schema>| {
        let schema = schema_clone.clone();
        let operative = operative_clone.clone();
        let slot_clone = slot.clone();
        let slot_variant = move || match slot_clone.get_templateslotvariant_slot() {
            TemplateSlotVariantTraitObject::TemplateSlotTraitOperative(trait_op) => {
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
            TemplateSlotVariantTraitObject::TemplateSlotSingleOperative(single_op) => {
                let view = move || {
                    format!(
                        "Single Operative Slot: {}",
                        single_op.get_allowedoperative_slot().get_name()
                    )
                };
                EitherOf3::B(view)
            }
            TemplateSlotVariantTraitObject::TemplateSlotMultiOperative(multi_op) => {
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
        let slot_clone = slot.clone();
        let slotted_instances_for_slot = Memo::new(move |_| {
            operative_clone
                .get_slottedinstances_slot()
                .into_iter()
                .filter(|slotted_inst| {
                    slotted_inst.get_slottedslot_slot().get_id() == slot_clone.get_id()
                })
                .collect::<Vec<_>>()
        });
        let operative_clone = operative.clone();
        let upstream_and_local_slotted_number =
            move || slotted_instances_for_slot.get().len() as u32;
        let upstream_and_local_slotted_number_clone = upstream_and_local_slotted_number.clone();
        let operative_clone = operative.clone();
        let slot_clone = slot.clone();
        let recursive_downstream_search =
            move |children: Vec<RGSOConcrete<OperativeConcrete, Schema>>,
                  slot: RGSOConcrete<TemplateSlot, Schema>,
                  current_largest: u32|
                  -> u32 {
                {
                    children.into_iter().fold(current_largest, |agg, child| {
                        let node_num = child
                            .get_slottedinstances_slot()
                            .into_iter()
                            .filter(|slotted_instance| {
                                slotted_instance.get_slottedslot_slot().get_id() == slot.get_id()
                            })
                            .collect::<Vec<_>>()
                            .len();
                        let branch_num = gather_all_downstream_slotted_instances(
                            child.get_childrenoperatives_slot(),
                            slot.clone(),
                            node_num as u32,
                        );
                        agg.max(branch_num)
                    })
                }
            };
        let downstream_slotted_number = move || {
            recursive_downstream_search(
                operative_clone.get_childrenoperatives_slot(),
                slot_clone.clone(),
                0,
            )
        };
        let downstream_slotted_number_clone = downstream_slotted_number.clone();
        let slot_clone = slot.clone();
        let is_fulfilled = RwSignal::new(false);
        let is_maxed_independently = RwSignal::new(false);
        let is_maxed_considering_children = RwSignal::new(false);
        let slot_bound_view = move || {
            let cur_slot_num = upstream_and_local_slotted_number_clone.clone()();
            let cur_downstream_slot_num = downstream_slotted_number.clone()();
            match slot_clone.get_slotbound_slot() {
                SlotBoundVariantTraitObject::SlotBoundUpperBound(inner) => {
                    is_fulfilled.set(true);
                    is_maxed_independently.set(
                        inner.get_upper_bound_field() == upstream_and_local_slotted_number_clone(),
                    );
                    is_maxed_considering_children
                        .set(inner.get_upper_bound_field() == (cur_downstream_slot_num));

                    EitherOf6::A(move || inner.get_upper_bound_field())
                }
                SlotBoundVariantTraitObject::SlotBoundRangeOrZero(inner) => {
                    EitherOf6::B(move || {
                        is_fulfilled.set(
                            cur_slot_num == 0 || (cur_slot_num >= inner.get_lower_bound_field()),
                        );
                        is_maxed_independently.set(inner.get_upper_bound_field() == cur_slot_num);
                        is_maxed_considering_children
                            .set(inner.get_upper_bound_field() == (cur_downstream_slot_num));
                        format!(
                            "Lower Bound: {}, Upper Bound: {}",
                            inner.get_lower_bound_field(),
                            inner.get_upper_bound_field()
                        )
                    })
                }
                SlotBoundVariantTraitObject::SlotBoundLowerBoundOrZero(inner) => {
                    is_fulfilled
                        .set(cur_slot_num == 0 || (cur_slot_num >= inner.get_lower_bound_field()));
                    is_maxed_independently.set(false);
                    is_maxed_considering_children.set(false);
                    EitherOf6::C(move || format!("Lower Bound: {}", inner.get_lower_bound_field(),))
                }
                SlotBoundVariantTraitObject::SlotBoundRange(inner) => EitherOf6::D(move || {
                    is_fulfilled.set(cur_slot_num >= inner.get_lower_bound_field());
                    is_maxed_independently.set(inner.get_upper_bound_field() == cur_slot_num);
                    is_maxed_considering_children
                        .set(inner.get_upper_bound_field() == (cur_downstream_slot_num));
                    format!(
                        "Lower Bound: {}, Upper Bound: {}",
                        inner.get_lower_bound_field(),
                        inner.get_upper_bound_field()
                    )
                }),
                SlotBoundVariantTraitObject::SlotBoundLowerBound(inner) => {
                    is_fulfilled.set(cur_slot_num >= inner.get_lower_bound_field());
                    is_maxed_independently.set(false);
                    is_maxed_considering_children.set(false);
                    EitherOf6::E(move || format!("Lower Bound: {}", inner.get_lower_bound_field(),))
                }
                SlotBoundVariantTraitObject::SlotBoundSingle(inner) => {
                    is_fulfilled.set(cur_slot_num == 1);
                    is_maxed_independently.set(cur_slot_num == 1);
                    is_maxed_considering_children.set((cur_downstream_slot_num) == 1);
                    EitherOf6::F(move || "Exactly 1")
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
        let slot_clone = slot.clone();
        let maybe_childest_spec = Memo::new(move |_| {
            let operative_clone = operative_clone.clone();
            let slot_clone = slot_clone.clone();
            // For some reason you have to call this in the closure to get the correct reactive tracking.
            operative_clone.get_slotspecializations_slot();
            get_childest_specialization_for_op_and_slot(operative_clone, slot_clone)
        });
        let is_adding_slotted_instance = RwSignal::new(false);
        let operative_clone = operative.clone();
        let slot_clone = slot.clone();
        let ctx_clone = ctx.clone();
        let add_slotted_instance_view = move || {
            let operative_clone = operative_clone.clone();
            let ctx_clone = ctx_clone.clone();
            let slot = slot_clone.clone();
            let schema_clone = schema.clone();
            let slot_clone = slot.clone();
            let selected_value =
                RwSignal::<Option<RGSOConcrete<InstanceConcrete, Schema>>>::new(None);
            let allowed_instances = Memo::new(move |_| {
                let schema_clone = schema_clone.clone();

                if let Some(childest_spec) = maybe_childest_spec.get() {
                    get_all_instances_which_satisfy_specialization(&schema_clone, childest_spec)
                } else {
                    match slot_clone.get_templateslotvariant_slot() {
                        TemplateSlotVariantTraitObject::TemplateSlotTraitOperative(trait_op) => {
                            get_all_instances_which_impl_trait_set(
                                trait_op.get_allowedtraits_slot(),
                                &schema_clone,
                            )
                        }
                        TemplateSlotVariantTraitObject::TemplateSlotSingleOperative(single_op) => {
                            get_all_descendent_instances(
                                single_op.get_allowedoperative_slot(),
                                &schema_clone,
                            )
                        }
                        TemplateSlotVariantTraitObject::TemplateSlotMultiOperative(multi_op) => {
                            multi_op.get_allowedoperatives_slot().into_iter().fold(
                                BTreeSet::new(),
                                |mut agg, op| {
                                    agg.extend(get_all_descendent_instances(op, &schema_clone));
                                    agg
                                },
                            )
                        }
                    }
                }
                .into_iter()
                .collect::<Vec<_>>()
            });
            let slot_clone = slot.clone();
            let on_click_save_slotted_instance = move |_| {
                let slot_clone = slot_clone.clone();
                let mut editor = operative_clone
                    .edit(ctx_clone.clone())
                    .add_new_slottedinstances(|new_slotted_instance| {
                        new_slotted_instance
                            .set_temp_id("the_new_slotted_instance")
                            .add_existing_fulfiller(operative_clone.get_id(), |na| na)
                            .add_existing_instance(selected_value.get().unwrap().get_id(), |na| na)
                            .add_existing_slottedslot(slot_clone.get_id(), |na| na)
                    });
                let mut all_descendents = BTreeSet::new();
                get_all_descendent_operators(operative_clone.clone(), &mut all_descendents);
                all_descendents.into_iter().for_each(|descendent| {
                    editor.incorporate(
                        descendent
                            .edit(ctx_clone.clone())
                            .add_temp_slottedinstances("the_new_slotted_instance"),
                    );
                });
                editor.execute().unwrap();
                is_adding_slotted_instance.set(false);
            };
            view! {
                <LeafSection>
                <LeafSectionHeader>Adding A Slotted Instance</LeafSectionHeader>
                <LeafSection>
                <SignalSelectWithOptions value=selected_value options=Signal::derive(move || allowed_instances.get()) empty_allowed=true/>
                </LeafSection>
                <LeafSection>
                <Button on:click=on_click_save_slotted_instance attr:disabled=move || is_maxed_considering_children.get() || is_maxed_independently.get()>Save</Button>
                <Button on:click=move |_| is_adding_slotted_instance.set(false)>Cancel</Button>
                </LeafSection>
                </LeafSection>
            }
        };
        let operative_clone = operative.clone();
        let slot_clone = slot.clone();

        let ctx_clone = ctx.clone();
        let specialization_view = move || {
            let ctx_clone = ctx_clone.clone();
            let operative_clone = operative_clone.clone();
            let slot_clone = slot_clone.clone();
            let view = move || {
                let ctx_clone = ctx_clone.clone();
                let operative_clone = operative_clone.clone();
                let operative_clone2 = operative_clone.clone();
                let operative_clone3 = operative_clone.clone();
                let operative_clone4 = operative_clone.clone();
                let slot = slot_clone.clone();
                let slot_clone = slot.clone();
                let maybe_childest_spec = Memo::new(move |_| {
                    get_childest_specialization_for_op_and_slot(
                        operative_clone.clone(),
                        slot_clone.clone(),
                    )
                });
                let slot_clone = slot.clone();
                let operative_clone = operative_clone4.clone();

                move || {
                    let ctx_clone = ctx_clone.clone();
                    if let Some(specialization) = maybe_childest_spec.get() {
                        let spec_clone = specialization.clone();
                        let is_locally_owned_spec = match spec_clone.clone() {
                            SlotSpecializationTraitObject::OperativeSlotSingleSpecialization(item) => item.get_specializer_slot().get_id() == operative_clone2.get_id(),
                            SlotSpecializationTraitObject::OperativeSlotMultiSpecialization(item) => item.get_specializer_slot().get_id() == operative_clone2.get_id(),
                            SlotSpecializationTraitObject::OperativeSlotTraitObjectSpecialization(item) => item.get_specializer_slot().get_id() == operative_clone2.get_id(),
                        };
                        let operative_clone3 = operative_clone3.clone();
                        let modify_view = move || {
                            let ctx_clone = ctx_clone.clone();
                            if is_locally_owned_spec {
                                match spec_clone.clone() {
                                SlotSpecializationTraitObject::OperativeSlotSingleSpecialization( single, ) => {
                                    let on_delete = move |_| {
                                        single.edit(ctx_clone.clone()).delete().execute().unwrap();
                                    };
                                    EitherOf6::A(view! {
                                        <LeafSection><Button on:click=on_delete>Delete Specialization</Button></LeafSection>
                                    })
                                }
                                SlotSpecializationTraitObject::OperativeSlotMultiSpecialization(multi) => {
                                    let on_delete = move |_| {
                                        multi.edit(ctx_clone.clone()).delete().execute().unwrap();
                                    };
                                    EitherOf6::B(view! {
                                        <LeafSection><Button on:click=on_delete>Delete Specialization</Button></LeafSection>
                                    })
                                }
                                SlotSpecializationTraitObject::OperativeSlotTraitObjectSpecialization(
                                    trait_object,
                                ) => {
                                    let on_delete = move |_| {
                                        trait_object.edit(ctx_clone.clone()).delete().execute().unwrap();
                                    };
                                    EitherOf6::C(view! {
                                        <LeafSection><Button on:click=on_delete>Delete Specialization</Button></LeafSection>
                                    })
                                },
                            }
                            } else {
                                match spec_clone.clone() {
                                    SlotSpecializationTraitObject::OperativeSlotSingleSpecialization(_) => EitherOf6::D(()),
                                    SlotSpecializationTraitObject::OperativeSlotMultiSpecialization(multi) => {
                                        EitherOf6::E(view!{<SpecializationBuilder operative=operative_clone3.clone() spec_target=SlotSpecializableTraitObject::OperativeSlotMultiSpecialization(multi) />})
                                    },
                                    SlotSpecializationTraitObject::OperativeSlotTraitObjectSpecialization(trait_obj) =>
                                    EitherOf6::F(view!{<SpecializationBuilder operative=operative_clone3.clone() spec_target=SlotSpecializableTraitObject::OperativeSlotTraitObjectSpecialization(trait_obj) />})
,
                                }
                            }
                        };
                        EitherOf4::A(view! {
                            <LeafSectionHeader>Specialization</LeafSectionHeader>
                            <LeafSection attr:class="leafsection dependent">
                            <SpecializationLineage specialization=specialization is_entry_point=true/>
                            </LeafSection>
                            {modify_view}
                        })
                    } else {
                        match slot_clone.get_templateslotvariant_slot() {
                            TemplateSlotVariantTraitObject::TemplateSlotTraitOperative(
                                trait_op,
                            ) => EitherOf4::B(view! {
                                <LeafSectionHeader>Specialization</LeafSectionHeader>
                                <SpecializationBuilder operative=operative_clone.clone() spec_target=SlotSpecializableTraitObject::TemplateSlotTraitOperative(trait_op) />
                            }),
                            TemplateSlotVariantTraitObject::TemplateSlotSingleOperative(single) => {
                                EitherOf4::C(view! {})
                            }
                            TemplateSlotVariantTraitObject::TemplateSlotMultiOperative(multi) => {
                                EitherOf4::D(view! {
                                    <LeafSectionHeader>Specialization</LeafSectionHeader>
                                    <SpecializationBuilder operative=operative_clone.clone() spec_target=SlotSpecializableTraitObject::TemplateSlotMultiOperative(multi) />
                                })
                            }
                        }
                    }
                }
            };
            view
        };

        view! {
            <SubSection>
            <SubSectionHeader>
            {move || slot.get_name()}
            </SubSectionHeader>

            <LeafSection>
                <LeafSectionHeader>
                Slot Details
                </LeafSectionHeader>
                <LeafSection attr:class="leafsection dependent">
                    {slot_variant}
                    <br/>
                    "Required:" {slot_bound_view}
                    <br/>
                    "Upstream (including this node) slotted instances:" {upstream_and_local_slotted_number}
                    <br/>
                    "Downstream slotted instances:" {downstream_slotted_number_clone}
                    <br/>
                    "Is Fulfilled:" {is_fulfilled}
                    <br/>
                    "Is Maxed Independently:" {is_maxed_independently}
                    <br/>
                    "Is Maxed Considering Children:" {is_maxed_considering_children}
                </LeafSection>

            </LeafSection>
            <LeafSection>
                <LeafSectionHeader>
                Currently Slotted
                </LeafSectionHeader>
                <For each=move || slotted_instances_for_slot.get() key=|item| item.get_id().clone() children=currently_slotted_view />
            </LeafSection>

            <Show when=move|| !is_maxed_considering_children.get() && !is_maxed_independently.get() && !is_adding_slotted_instance.get()>
            <LeafSection>
                <Button on:click=move |_| is_adding_slotted_instance.set(true)>Slot an instance</Button>
            </LeafSection>
            </Show>
            <Show when=move|| is_adding_slotted_instance.get()>
            {add_slotted_instance_view.clone()}
            </Show>
            <LeafSection>
            {specialization_view}
            </LeafSection>
            </SubSection>
        }
    };

    let operative_clone = operative.clone();

    let operative_clone_3 = operative.clone();
    let operative_clone_4 = operative.clone();
    view! {
        <Section>
            <SectionHeader slot>Overview</SectionHeader>
            <SubSection>
                <SubSectionHeader>
                    Name:
                </SubSectionHeader>
                <ToggleManagedTextInput getter=move || operative_clone.get_name_field() setter=update_name />
            </SubSection>
            <SubSection>
                <Button on:click=delete_operative>Delete Item</Button>
            </SubSection>
            <SubSection>
                <OperativeLineage operative=operative_clone_3 is_entry_point=true/>
            </SubSection>
        </Section>

        <Section>
            <SectionHeader slot>Create Derivatives</SectionHeader>
            <LeafSection>
            <SignalTextInput value=derivative_operative_name/><Button on:click=create_derivative_operative>Create derivative operative</Button>
            </LeafSection>
            <LeafSection>
            <SignalTextInput value=derivative_instance_name/><Button on:click=create_derivative_instance>Create derivative instance</Button>
            </LeafSection>
        </Section>
        <Section>
            <SectionHeader slot>Fields</SectionHeader>
            <SubSection>
            <Show when=move|| {locked_fields.get().1.len() > 0}>
            {let unowned_locked_field_view = unowned_locked_field_view.clone();
                view!{
                <SubSectionHeader>Locked By Parent</SubSectionHeader>
                <For each=move||locked_fields.get().1 key=|item| item.get_id().clone() children=unowned_locked_field_view />
                }}
            </Show>
            <Show when=move|| {locked_fields.get().0.len() > 0}>
                {let owned_locked_field_view = owned_locked_field_view.clone();
                    view!{
                    <SubSectionHeader>Locked Here</SubSectionHeader>
                    <For each=move||locked_fields.get().0 key=|item| item.get_id().clone() children=owned_locked_field_view />
                    }
                }
            </Show>
            <Show when=move|| {non_locked_fields.get().1.len() > 0}>
            {let non_locked_field_view = non_locked_field_view.clone();
                view!{
                <SubSectionHeader>Unlocked and Independent</SubSectionHeader>
                <For each=move || non_locked_fields.get().1 key=|item| item.get_id().clone() children=non_locked_field_view />
                }}
            </Show>
            <Show when=move|| {non_locked_fields.get().0.len() > 0}>
            {let non_locked_but_dependent_field_view = non_locked_but_dependent_field_view.clone();
                view!{
                <SubSectionHeader>Unlocked but locked downstream and therefore dependent</SubSectionHeader>
                <For each=move || non_locked_fields.get().0 key=|item| item.get_id().clone() children=non_locked_but_dependent_field_view />
                }}
            </Show>
            </SubSection>
        </Section>
        <Section>
            <SectionHeader slot>Slots</SectionHeader>
            <For each=move || operative_clone_4.get_roottemplate_slot().get_templateslots_slot() key=|slot| slot.get_id().clone() children=each_slot_view>
            </For>
        </Section>
    }
}

fn recurse_add_locked_field<
    T: OperativeConcreteLockedFieldsAcceptableTargetMarker
        + RootConstraints<Schema>
        + schema_editor_generated_toolkit::prelude::StaticTypestate,
>(
    children: Vec<RGSOConcrete<OperativeConcrete, Schema>>,
    mut_editor: &mut ExistingBuilder<OperativeConcrete, Schema>,
    ctx_clone: &SharedGraph<Schema>,
) {
    children.into_iter().for_each(|child| {
        mut_editor.incorporate(
            child
                .edit(ctx_clone.clone())
                .add_temp_lockedfields::<T>("the_field"),
        );
        recurse_add_locked_field::<T>(child.get_childrenoperatives_slot(), mut_editor, ctx_clone)
    });
}

fn recursive_search_for_locked_field(
    children: Vec<RGSOConcrete<OperativeConcrete, Schema>>,
    field_id: &Uid,
) -> bool {
    children.into_iter().any(|child| {
        let is_match =
            child
                .get_lockedfields_slot()
                .iter()
                .any(|locked_field| match locked_field {
                    FulfilledFieldVariantTraitObject::BoolFulfilledField(locked_field) => {
                        locked_field.get_constraintreference_slot().get_id() == field_id
                    }
                    FulfilledFieldVariantTraitObject::IntFulfilledField(locked_field) => {
                        locked_field.get_constraintreference_slot().get_id() == field_id
                    }
                    FulfilledFieldVariantTraitObject::StringFulfilledField(locked_field) => {
                        locked_field.get_constraintreference_slot().get_id() == field_id
                    }
                });
        if is_match {
            true
        } else {
            recursive_search_for_locked_field(child.get_childrenoperatives_slot(), field_id)
        }
    })
}

// Should take into account all children-trees and return the branch with the largest number of children
fn gather_all_downstream_slotted_instances(
    children: Vec<RGSOConcrete<OperativeConcrete, Schema>>,
    slot: RGSOConcrete<TemplateSlot, Schema>,
    current_largest: u32,
) -> u32 {
    children.into_iter().fold(current_largest, |agg, child| {
        let node_num = child
            .get_slottedinstances_slot()
            .into_iter()
            .filter(|slotted_instance| {
                slotted_instance.get_slottedslot_slot().get_id() == slot.get_id()
            })
            .collect::<Vec<_>>()
            .len();
        let branch_num = gather_all_downstream_slotted_instances(
            child.get_childrenoperatives_slot(),
            slot.clone(),
            node_num as u32,
        );
        agg.max(branch_num)
    })
}
