use leptos::either::{Either, EitherOf3, EitherOf4};
use schema_editor_generated_toolkit::prelude::*;

#[component]
pub fn TraitSpecializationTargetView(
    target: SlotSpecializableTraitOperativeTraitObject,
) -> impl IntoView {
    match target {
        SlotSpecializableTraitOperativeTraitObject::TemplateSlotTraitOperative(trait_op) => {
            let text = format!(
                "Root Trait List: [{}]",
                trait_op
                    .get_allowedtraits_slot()
                    .into_iter()
                    .map(|item| item.get_name())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            view! {
                         {text}
                         <br/>
                         "↓"
                         <br/>

            }
            .into_any()
        }
        SlotSpecializableTraitOperativeTraitObject::OperativeSlotTraitObjectSpecialization(
            trait_op,
        ) => {
            let text = format!(
                "Added to Trait List: [{}]",
                trait_op
                    .get_allowedtraits_slot()
                    .into_iter()
                    .map(|item| item.get_name())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            view! {
                        <TraitSpecializationTargetView target=trait_op.get_specializationtarget_slot() />
                         {text}
                         <br/>
                         "↓"
                         <br/>

            }.into_any()
        }
    }
}
#[component]
pub fn GeneralSpecializationTargetView(target: SlotSpecializableTraitObject) -> impl IntoView {
    match target {
        SlotSpecializableTraitObject::TemplateSlotTraitOperative(trait_op) => {
            let text = format!(
                "Root Trait List: [{}]",
                trait_op
                    .get_allowedtraits_slot()
                    .into_iter()
                    .map(|item| item.get_name())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            view! {
                         {text}
                         <br/>
                         "↓"
                         <br/>

            }
            .into_any()
        }
        SlotSpecializableTraitObject::OperativeSlotMultiSpecialization(multi) => {
            let text = format!(
                "Narrowed to this operative list: [{}]",
                multi
                    .get_allowedoperatives_slot()
                    .into_iter()
                    .map(|item| item.get_name())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            view! {
                <GeneralSpecializationTargetView target=multi.get_specializationtarget_slot() />
                         {text}
                         <br/>
                         "↓"
                         <br/>

            }
            .into_any()
        }
        SlotSpecializableTraitObject::TemplateSlotMultiOperative(multi) => {
            let text = format!(
                "Root Operative List: [{}]",
                multi
                    .get_allowedoperatives_slot()
                    .into_iter()
                    .map(|item| item.get_name())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            view! {
                         {text}
                         <br/>
                         "↓"
                         <br/>

            }
            .into_any()
        }
        SlotSpecializableTraitObject::OperativeSlotTraitObjectSpecialization(trait_op) => {
            let text = format!(
                "Added to Trait List: [{}]",
                trait_op
                    .get_allowedtraits_slot()
                    .into_iter()
                    .map(|item| item.get_name())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            view! {
                        <TraitSpecializationTargetView target=trait_op.get_specializationtarget_slot() />
                         {text}
                         <br/>
                         "↓"
                         <br/>

            }.into_any()
        }
    }
}
#[component]
pub fn SpecializationLineage(
    specialization: SlotSpecializationTraitObject,
    is_entry_point: bool,
) -> impl IntoView {
    // let operative_clone = operative.clone();
    let specialization_clone = specialization.clone();
    let trait_specialization_target_view =
        move |target: SlotSpecializableTraitOperativeTraitObject| match target {
            SlotSpecializableTraitOperativeTraitObject::TemplateSlotTraitOperative(_) => todo!(),
            SlotSpecializableTraitOperativeTraitObject::OperativeSlotTraitObjectSpecialization(
                _,
            ) => todo!(),
        };
    let parent_view = move || match specialization_clone.clone() {
        SlotSpecializationTraitObject::OperativeSlotSingleSpecialization(item) => view! {
            <GeneralSpecializationTargetView target=item.get_specializationtarget_slot() />
        }
        .into_any(),
        SlotSpecializationTraitObject::OperativeSlotMultiSpecialization(item) => view! {

            <GeneralSpecializationTargetView target=item.get_specializationtarget_slot() />
        }
        .into_any(),
        SlotSpecializationTraitObject::OperativeSlotTraitObjectSpecialization(item) => view! {
            <TraitSpecializationTargetView target=item.get_specializationtarget_slot() />
        }
        .into_any(),
    };
    let specialization_clone = specialization.clone();
    let origin_view = move || {
        let specialization_clone = specialization_clone.clone();
        if is_entry_point {
            let inner_view = move || match specialization_clone.clone() {
                SlotSpecializationTraitObject::OperativeSlotSingleSpecialization(single) => {
                    EitherOf3::A(format!(
                        "Single Option Specialization: {}",
                        single.get_allowedoperative_slot().get_name()
                    ))
                }
                SlotSpecializationTraitObject::OperativeSlotMultiSpecialization(multi) => {
                    EitherOf3::B(format!(
                        "Multiple Options Specialization: [{}]",
                        multi
                            .get_allowedoperatives_slot()
                            .into_iter()
                            .map(|op| op.get_name())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ))
                }
                SlotSpecializationTraitObject::OperativeSlotTraitObjectSpecialization(
                    trait_object,
                ) => EitherOf3::C(format!(
                    "Trait Addition Specialization: [{}]",
                    trait_object
                        .get_allowedtraits_slot()
                        .into_iter()
                        .map(|op| op.get_name())
                        .collect::<Vec<_>>()
                        .join(", ")
                )),
            };
            Either::Left(view! {
                <div>
                "This Specialization: "{inner_view}
                </div>
            })
        } else {
            Either::Right(())
        }
    };
    view! {
        {parent_view}
        {origin_view}
    }
}
