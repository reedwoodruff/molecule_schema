use std::collections::BTreeSet;

use crate::components::{common::*, workspace::WorkspaceState};

use leptos::either::{EitherOf6, EitherOf7, EitherOf8};
use schema_editor_generated_toolkit::prelude::*;

use super::operative_slot_section::OperativeSlotContext;
use super::slot_cardinality_specialization_builder::SlotCardinalitySpecializationBuilder;

use super::slot_cardinality_specialization_lineage::SlotCardinalitySpecializationLineage;
use super::utils::get_all_descendent_operators;

#[component]
pub fn OperativeSlotCardinalitySpecializationSection() -> impl IntoView {
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
    let slot_clone = slot_item.clone();
    let operative_clone = operative_clone4.clone();

    let exists_downstream_spec = move || {
        let this_ops_specs = operative_clone.get_slotcardinalityspecializations_slot().into_iter().filter(|spec| {
            match spec {
                OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityLowerBoundOrZeroSpecialization(item) => {
                    item.get_roottemplateslot_slot().get_id() == slot_item.get_id()
                },
                OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalitySingleSpecialization(item) => {
                    item.get_roottemplateslot_slot().get_id() == slot_item.get_id()
                },
                OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityRangeSpecialization(item) => {
                    item.get_roottemplateslot_slot().get_id() == slot_item.get_id()
                },
                OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityZeroSpecialization(item) => {
                    item.get_roottemplateslot_slot().get_id() == slot_item.get_id()
                },
                OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityRangeOrZeroSpecialization(item) => {
                    item.get_roottemplateslot_slot().get_id() == slot_item.get_id()
                },
                OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityLowerBoundSpecialization(item) => {
                    item.get_roottemplateslot_slot().get_id() == slot_item.get_id()
                },
            }
        }).map(|item| item.get_id().clone()).collect::<Vec<_>>();
        let mut downstream_ops = BTreeSet::new();
        get_all_descendent_operators(operative_clone, &mut downstream_ops);

        downstream_ops
            .into_iter()
            .flat_map(|op| op.get_slotcardinalityspecializations_slot().into_iter().filter(|spec| {
                match spec {
                    OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityLowerBoundOrZeroSpecialization(item) => item.get_roottemplateslot_slot().get_id() == slot_item.get_id(),
                    OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalitySingleSpecialization(item) => item.get_roottemplateslot_slot().get_id() == slot_item.get_id(),
                    OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityRangeSpecialization(item) => item.get_roottemplateslot_slot().get_id() == slot_item.get_id(),
                    OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityZeroSpecialization(item) => item.get_roottemplateslot_slot().get_id() == slot_item.get_id(),
                    OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityRangeOrZeroSpecialization(item) => item.get_roottemplateslot_slot().get_id() == slot_item.get_id(),
                    OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityLowerBoundSpecialization(item) => item.get_roottemplateslot_slot().get_id() == slot_item.get_id(),
                }
            }).map(|item| item.get_id().clone()))
            .any(|downstream_spec_id| !this_ops_specs.contains(&downstream_spec_id))
    };
    let exists_downstream_spec_clone = exists_downstream_spec.clone();

    move || {
        let ctx_clone = ctx_clone.clone();
        if let Some(specialization) = maybe_childest_cardinality_spec.get() {
            let spec_clone = specialization.clone();
            let is_locally_owned_spec = match spec_clone.clone() {
                OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityLowerBoundOrZeroSpecialization(item) => item.get_specializer_slot().get_id() == operative_clone2.get_id(),
                OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalitySingleSpecialization(item) => item.get_specializer_slot().get_id() == operative_clone2.get_id(),
                OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityRangeSpecialization(item) => item.get_specializer_slot().get_id() == operative_clone2.get_id(),
                OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityZeroSpecialization(item) => item.get_specializer_slot().get_id() == operative_clone2.get_id(),
                OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityRangeOrZeroSpecialization(item) => item.get_specializer_slot().get_id() == operative_clone2.get_id(),
                OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityLowerBoundSpecialization(item) => item.get_specializer_slot().get_id() == operative_clone2.get_id(),
            };
            let operative_clone3 = operative_clone3.clone();
            let spec_clone = specialization.clone();
            let exists_downstream_spec_clone = exists_downstream_spec_clone.clone();

            let builder_view = move || {
                match spec_clone.clone() {
                    OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityLowerBoundOrZeroSpecialization(item) => {
                        EitherOf6::A(
                            view!{<SlotCardinalitySpecializationBuilder operative=operative_clone3.clone() spec_target=OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundOrZeroSpecialization(item) />}
                        )
                    },
                    OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalitySingleSpecialization(item) => {
                        EitherOf6::B(
                            view! {<LeafSection><InfoNote>Cannot be specialized further</InfoNote></LeafSection>}
                        )
                    },
                    OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityRangeSpecialization(item) => {
                        EitherOf6::C(
                            view!{<SlotCardinalitySpecializationBuilder operative=operative_clone3.clone() spec_target=OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityRangeSpecialization(item) />}
                        )
                    },
                    OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityZeroSpecialization(item) => {
                        EitherOf6::D(
                            view! {<LeafSection><InfoNote>Cannot be specialized further</InfoNote></LeafSection>}
                        )
                    },
                    OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityRangeOrZeroSpecialization(item) => {
                        EitherOf6::E(
                            view!{<SlotCardinalitySpecializationBuilder operative=operative_clone3.clone() spec_target=OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityRangeOrZeroSpecialization(item) />}
                        )
                    },
                    OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityLowerBoundSpecialization(item) => {
                        EitherOf6::F(
                            view!{<SlotCardinalitySpecializationBuilder operative=operative_clone3.clone() spec_target=OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundSpecialization(item) />}
                        )
                    },
                }
            };

            let exists_downstream_spec_clone = exists_downstream_spec_clone.clone();
            let spec_clone = specialization.clone();
            let modify_view = move || {
                let exists_downstream_spec_clone = exists_downstream_spec_clone.clone();
                let ctx_clone = ctx_clone.clone();
                if is_locally_owned_spec {
                    match spec_clone.clone() {
                    OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityLowerBoundOrZeroSpecialization(item) => {

                        let on_delete = move |_| {
                            item.edit(ctx_clone.clone()).delete().execute().unwrap();
                        };
                        EitherOf8::A(view! {
                            <LeafSection><Button on:click=on_delete attr:disabled=move||exists_downstream_spec_clone.clone()()>Delete Specialization</Button></LeafSection>
                        })
                    },
                    OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalitySingleSpecialization(item) => {

                        let on_delete = move |_| {
                            item.edit(ctx_clone.clone()).delete().execute().unwrap();
                        };
                        EitherOf8::B(view! {
                            <LeafSection><Button on:click=on_delete attr:disabled=move||exists_downstream_spec_clone.clone()()>Delete Specialization</Button></LeafSection>
                        })
                    },
                    OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityRangeSpecialization(item) => {

                        let on_delete = move |_| {
                            item.edit(ctx_clone.clone()).delete().execute().unwrap();
                        };
                        EitherOf8::C(view! {
                            <LeafSection><Button on:click=on_delete attr:disabled=move||exists_downstream_spec_clone.clone()()>Delete Specialization</Button></LeafSection>
                        })
                    },
                    OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityZeroSpecialization(item) => {

                        let on_delete = move |_| {
                            item.edit(ctx_clone.clone()).delete().execute().unwrap();
                        };
                        EitherOf8::D(view! {
                            <LeafSection><Button on:click=on_delete attr:disabled=move||exists_downstream_spec_clone.clone()()>Delete Specialization</Button></LeafSection>
                        })
                    },
                    OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityRangeOrZeroSpecialization(item) => {

                        let on_delete = move |_| {
                            item.edit(ctx_clone.clone()).delete().execute().unwrap();
                        };
                        EitherOf8::E(view! {
                            <LeafSection><Button on:click=on_delete attr:disabled=move||exists_downstream_spec_clone.clone()()>Delete Specialization</Button></LeafSection>
                        })
                    },
                    OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityLowerBoundSpecialization(item) => {

                        let on_delete = move |_| {
                            item.edit(ctx_clone.clone()).delete().execute().unwrap();
                        };
                        EitherOf8::F(view! {
                            <LeafSection><Button on:click=on_delete attr:disabled=move||exists_downstream_spec_clone.clone()()>Delete Specialization</Button></LeafSection>
                        })
                    },
                }
                } else if exists_downstream_spec_clone.clone()() {
                    EitherOf8::H(view! {
                        <LeafSection>
                        <InfoNote>There exists a downstream specialization. Remove it to create a specialization here.</InfoNote>
                        </LeafSection>
                    })
                } else {
                    EitherOf8::G(builder_view.clone())
                }
            };
            EitherOf7::D(view! {
                <LeafSection attr:class="leafsection dependent">
                <SlotCardinalitySpecializationLineage specialization=specialization is_entry_point=true/>
                </LeafSection>
                {modify_view}
            })
        } else if exists_downstream_spec.clone()() {
            EitherOf7::G(view! {
                <LeafSection>
                <InfoNote>There exists a downstream specialization. Remove it to create a specialization here.</InfoNote>
                </LeafSection>
            })
        } else {
            match slot_clone.get_slotbound_slot() {
                TemplateSlotCardinalityVariantTraitObject::TemplateSlotCardinalityLowerBoundOrZero(item) => {
                    EitherOf7::A(
                        view!{<SlotCardinalitySpecializationBuilder operative=operative_clone3.clone() spec_target=OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityLowerBoundOrZero(item) />}
                    )
                },
                TemplateSlotCardinalityVariantTraitObject::TemplateSlotCardinalitySingle(item) => {
                    EitherOf7::B(
                        view! {<LeafSection><InfoNote>Cannot be specialized further</InfoNote></LeafSection>}
                    )
                },
                TemplateSlotCardinalityVariantTraitObject::TemplateSlotCardinalityRange(item) => {
                    EitherOf7::C(
                        view!{<SlotCardinalitySpecializationBuilder operative=operative_clone3.clone() spec_target=OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityRange(item) />}
                    )
                },
                TemplateSlotCardinalityVariantTraitObject::TemplateSlotCardinalityRangeOrZero(item) => {
                    EitherOf7::E(
                        view!{<SlotCardinalitySpecializationBuilder operative=operative_clone3.clone() spec_target=OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityRangeOrZero(item) />}
                    )
                },
                TemplateSlotCardinalityVariantTraitObject::TemplateSlotCardinalityLowerBound(item) => {
                    EitherOf7::F(
                        view!{<SlotCardinalitySpecializationBuilder operative=operative_clone3.clone() spec_target=OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityLowerBound(item) />}
                    )
                },
            }
        }
    }
}
