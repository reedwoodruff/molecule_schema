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

#[derive(Clone)]
pub struct OperativeSlotContext {
    pub max_downstream_slotted_instances: Signal<u32>,
    pub operative: RGSOConcrete<OperativeConcrete, Schema>,
    pub slot_item: RGSOConcrete<TemplateSlot, Schema>,
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
    let maybe_childest_cardinality_spec = Memo::new(move |_| {
        let operative_clone = operative_clone.clone();
        let slot_clone = slot_clone.clone();
        // For some reason you have to call this in the closure to get the correct reactive tracking.
        operative_clone.get_slottypespecializations_slot();
        get_childest_cardinality_specialization_for_op_and_slot(operative_clone, slot_clone)
    });

    let slot_clone = slot_item.clone();
    let cardinality_info = Memo::new(move |_| {
        if let Some(spec) = maybe_childest_cardinality_spec.get() {
            CardinalityInfo::from_card_spec(spec)
        } else {
            match slot_clone.get_slotbound_slot() {
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
        operative_clone
            .get_slottedinstances_slot()
            .into_iter()
            .filter(|slotted_inst| {
                slotted_inst.get_slottedslot_slot().get_id() == slot_clone.get_id()
            })
            .collect::<Vec<_>>()
    });
    let operative_clone = operative.clone();
    let upstream_and_local_slotted_number = move || slotted_instances_for_slot.get().len() as u32;
    let upstream_and_local_slotted_number_clone = upstream_and_local_slotted_number.clone();
    let operative_clone = operative.clone();
    let slot_clone = slot_item.clone();
    let max_downstream_slotted_number = Signal::derive(move || {
        // track reactively
        operative_clone.get_childrenoperatives_slot();
        gather_all_downstream_slotted_instances(
            operative_clone.get_childrenoperatives_slot(),
            slot_clone.clone(),
            0,
        )
    });
    let downstream_slotted_number_clone = max_downstream_slotted_number.clone();
    let slot_clone = slot_item.clone();
    let is_fulfilled = Memo::new(move |_| {
        upstream_and_local_slotted_number_clone() > cardinality_info.get().min
            || (upstream_and_local_slotted_number_clone() == 0
                && cardinality_info.get().zero_allowed == true)
    });
    let is_maxed_independently = Memo::new(move |_| {
        cardinality_info
            .get()
            .max
            .is_some_and(|max| max == upstream_and_local_slotted_number())
    });
    let is_maxed_considering_children = Memo::new(move |_| {
        cardinality_info
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
        let maybe_childest_card_info =
            get_childest_cardinality_info_downstream(operative_clone.clone(), slot_clone.clone());
        if let Some(childest_card_info) = maybe_childest_card_info {
            return !childest_card_info
                .max
                .is_some_and(|max| max_downstream_slotted_number.get() == max);
        }
        true
    });

    let operative_clone = operative.clone();
    let slot_clone = slot_item.clone();
    let slot_bound_view = move || {
        let cur_slot_num = upstream_and_local_slotted_number_clone.clone()();
        let cur_downstream_slot_num = max_downstream_slotted_number.clone().get();
        match slot_clone.get_slotbound_slot() {
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
        operative_clone.get_slottypespecializations_slot();
        get_childest_type_specialization_for_op_and_slot(operative_clone, slot_clone)
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
                get_all_instances_which_satisfy_specialization(&schema_clone, childest_spec)
            } else {
                match slot_clone.get_templateslotvariant_slot() {
                    TemplateSlotTypeVariantTraitObject::TemplateSlotTypeTraitOperative(
                        trait_op,
                    ) => get_all_instances_which_impl_trait_set(
                        trait_op.get_allowedtraits_slot(),
                        &schema_clone,
                    ),
                    TemplateSlotTypeVariantTraitObject::TemplateSlotTypeSingleOperative(
                        single_op,
                    ) => get_all_descendent_instances_including_own(
                        single_op.get_allowedoperative_slot(),
                        &schema_clone,
                    ),
                    TemplateSlotTypeVariantTraitObject::TemplateSlotTypeMultiOperative(
                        multi_op,
                    ) => multi_op.get_allowedoperatives_slot().into_iter().fold(
                        BTreeSet::new(),
                        |mut agg, op| {
                            agg.extend(get_all_descendent_instances(op, &schema_clone));
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
            let mut this_item_and_descendents = BTreeSet::new();
            get_all_descendent_operators(operative_clone.clone(), &mut this_item_and_descendents);
            this_item_and_descendents.insert(operative_clone.clone());
            this_item_and_descendents
                .into_iter()
                .for_each(|descendent| {
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
        slot_item: slot_clone.clone(),
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
            // <Show when=move|| !is_maxed_considering_children.get() && !is_maxed_independently.get() && !is_adding_slotted_instance.get()>
            <LeafSection>
                <Button on:click=move |_| is_adding_slotted_instance.set(true) attr:disabled =move||!is_allowed_to_add_another_instance.get()>Slot an instance</Button>
            </LeafSection>
            // </Show>
            <Show when=move|| is_adding_slotted_instance.get()>
            {add_slotted_instance_view.clone()}
            </Show>
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
