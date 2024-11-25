use std::collections::BTreeSet;

use crate::components::{
    common::*, slot_type_specialization_builder::SlotTypeSpecializationBuilder,
    slot_type_specialization_lineage::SlotTypeSpecializationLineage, workspace::WorkspaceState,
};

use leptos::either::{EitherOf5, EitherOf7};
use schema_editor_generated_toolkit::prelude::*;

use super::{operative_slot_section::OperativeSlotContext, utils::get_all_descendent_operators};

#[component]
pub fn OperativeSlotTypeSpecializationSection() -> impl IntoView {
    let ctx = use_context::<SharedGraph<Schema>>().unwrap();
    let WorkspaceState {
        schema,
        selected_tab,
    }: WorkspaceState = use_context::<WorkspaceState>().unwrap();

    let OperativeSlotContext {
        max_downstream_slotted_instances,
        operative,
        slot_item,
        maybe_childest_type_spec,
        maybe_childest_cardinality_spec,
    } = use_context::<OperativeSlotContext>().unwrap();

    let ctx_clone = ctx.clone();
    let schema_clone = schema.clone();

    let operative_clone = operative.clone();
    let slot_clone = slot_item.clone();
    let operative_clone2 = operative_clone.clone();
    let operative_clone3 = operative_clone.clone();
    let operative_clone4 = operative_clone.clone();
    let slot = slot_clone.clone();
    let slot_clone = slot.clone();
    let operative_clone = operative_clone4.clone();

    let exists_downstream_spec = move || {
        let this_ops_specs = operative_clone.get_slottypespecializations_slot().into_iter().filter(|spec| {
            match spec {
                OperativeSlotTypeSpecializationTraitObject::OperativeSlotTypeSingleSpecialization(item) => item.get_roottemplateslot_slot().get_id() == slot_item.get_id(),
                OperativeSlotTypeSpecializationTraitObject::OperativeSlotTypeMultiSpecialization(item) => item.get_roottemplateslot_slot().get_id() == slot_item.get_id(),
                OperativeSlotTypeSpecializationTraitObject::OperativeSlotTypeTraitObjectSpecialization(item) => item.get_roottemplateslot_slot().get_id() == slot_item.get_id(),
            }
        }).map(|item| item.get_id().clone()).collect::<Vec<_>>();
        let mut downstream_ops = BTreeSet::new();
        get_all_descendent_operators(operative_clone.clone(), &mut downstream_ops);

        downstream_ops
            .into_iter()
            .flat_map(|op| op.get_slottypespecializations_slot().into_iter().filter(|spec| {
                match spec {
                    OperativeSlotTypeSpecializationTraitObject::OperativeSlotTypeSingleSpecialization(item) => item.get_roottemplateslot_slot().get_id() == slot_item.get_id(),
                    OperativeSlotTypeSpecializationTraitObject::OperativeSlotTypeMultiSpecialization(item) => item.get_roottemplateslot_slot().get_id() == slot_item.get_id(),
                    OperativeSlotTypeSpecializationTraitObject::OperativeSlotTypeTraitObjectSpecialization(item) => item.get_roottemplateslot_slot().get_id() == slot_item.get_id(),
                }
            }).map(|item| item.get_id().clone()))
            .any(|downstream_spec_id| !this_ops_specs.contains(&downstream_spec_id))
    };
    let exists_downstream_spec_clone = exists_downstream_spec.clone();

    let operative_clone = operative_clone4.clone();
    move || {
        let ctx_clone = ctx_clone.clone();
        if let Some(specialization) = maybe_childest_type_spec.get() {
            let spec_clone = specialization.clone();
            let is_locally_owned_spec = match spec_clone.clone() {
                    OperativeSlotTypeSpecializationTraitObject::OperativeSlotTypeSingleSpecialization(item) => item.get_specializer_slot().get_id() == operative_clone2.get_id(),
                    OperativeSlotTypeSpecializationTraitObject::OperativeSlotTypeMultiSpecialization(item) => item.get_specializer_slot().get_id() == operative_clone2.get_id(),
                    OperativeSlotTypeSpecializationTraitObject::OperativeSlotTypeTraitObjectSpecialization(item) => item.get_specializer_slot().get_id() == operative_clone2.get_id(),
                };
            let operative_clone3 = operative_clone3.clone();
            let exists_downstream_spec = exists_downstream_spec_clone.clone();
            let exists_downstream_spec_clone = exists_downstream_spec.clone();
            let modify_view = move || {
                let exists_downstream_spec_clone = exists_downstream_spec_clone.clone();
                let ctx_clone = ctx_clone.clone();
                if is_locally_owned_spec {
                    match spec_clone.clone() {
                        OperativeSlotTypeSpecializationTraitObject::OperativeSlotTypeSingleSpecialization( single, ) => {
                            let on_delete = move |_| {
                                single.edit(ctx_clone.clone()).delete().execute().unwrap();
                            };
                            EitherOf7::A(view! {
                                <LeafSection><Button on:click=on_delete attr:disabled=move||exists_downstream_spec_clone.clone()()>Delete Specialization</Button></LeafSection>
                            })
                        }
                        OperativeSlotTypeSpecializationTraitObject::OperativeSlotTypeMultiSpecialization(multi) => {
                            let on_delete = move |_| {
                                multi.edit(ctx_clone.clone()).delete().execute().unwrap();
                            };
                            EitherOf7::B(view! {
                                <LeafSection><Button on:click=on_delete attr:disabled=move||exists_downstream_spec_clone.clone()()>Delete Specialization</Button></LeafSection>
                            })
                        }
                        OperativeSlotTypeSpecializationTraitObject::OperativeSlotTypeTraitObjectSpecialization(
                            trait_object,
                        ) => {
                            let on_delete = move |_| {
                                trait_object.edit(ctx_clone.clone()).delete().execute().unwrap();
                            };
                            EitherOf7::C(view! {
                                <LeafSection><Button on:click=on_delete attr:disabled=move||exists_downstream_spec_clone.clone()()>Delete Specialization</Button></LeafSection>
                            })
                        },
                    }
                } else if exists_downstream_spec.clone()() {
                    EitherOf7::G(view! {
                        <LeafSection>
                        <InfoNote>There exists a downstream specialization. Remove it to create a specialization here.</InfoNote>
                        </LeafSection>
                    })
                } else {
                    match spec_clone.clone() {
                            OperativeSlotTypeSpecializationTraitObject::OperativeSlotTypeSingleSpecialization(_) => EitherOf7::D(view! {<LeafSection><InfoNote>Cannot be specialized further</InfoNote></LeafSection>}),
                            OperativeSlotTypeSpecializationTraitObject::OperativeSlotTypeMultiSpecialization(multi) => {
                                EitherOf7::E(view!{<SlotTypeSpecializationBuilder operative=operative_clone3.clone() spec_target=OperativeSlotTypeSpecializableTraitObject::OperativeSlotTypeMultiSpecialization(multi) />})
                            },
                            OperativeSlotTypeSpecializationTraitObject::OperativeSlotTypeTraitObjectSpecialization(trait_obj) =>
                            EitherOf7::F(view!{<SlotTypeSpecializationBuilder operative=operative_clone3.clone() spec_target=OperativeSlotTypeSpecializableTraitObject::OperativeSlotTypeTraitObjectSpecialization(trait_obj) />})
,
                        }
                }
            };
            EitherOf5::A(view! {
                <LeafSection attr:class="leafsection dependent">
                <SlotTypeSpecializationLineage specialization=specialization is_entry_point=true/>
                </LeafSection>
                {modify_view}
            })
        } else if exists_downstream_spec.clone()() {
            EitherOf5::E(view! {
                <LeafSection>
                <InfoNote>There exists a downstream specialization. Remove it to create a specialization here.</InfoNote>
                </LeafSection>
            })
        } else {
            match slot_clone.get_templateslotvariant_slot() {
                TemplateSlotTypeVariantTraitObject::TemplateSlotTypeTraitOperative(trait_op) => {
                    EitherOf5::B(view! {
                        <SlotTypeSpecializationBuilder operative=operative_clone.clone() spec_target=OperativeSlotTypeSpecializableTraitObject::TemplateSlotTypeTraitOperative(trait_op) />
                    })
                }
                TemplateSlotTypeVariantTraitObject::TemplateSlotTypeSingleOperative(single) => {
                    EitherOf5::C(
                        view! {<LeafSection><InfoNote>Cannot be specialized further</InfoNote></LeafSection>},
                    )
                }
                TemplateSlotTypeVariantTraitObject::TemplateSlotTypeMultiOperative(multi) => {
                    EitherOf5::D(view! {
                        <SlotTypeSpecializationBuilder operative=operative_clone.clone() spec_target=OperativeSlotTypeSpecializableTraitObject::TemplateSlotTypeMultiOperative(multi) />
                    })
                }
            }
        }
    }
}
