use leptos::either::{Either, EitherOf3, EitherOf4};
use schema_editor_generated_toolkit::prelude::*;

#[component]
pub fn TraitSpecializationTargetView(
    target: SlotTypeSpecializableTraitOperativeTraitObject,
) -> impl IntoView {
    match target {
        SlotTypeSpecializableTraitOperativeTraitObject::TemplateSlotTraitOperative(trait_op) => {
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
        SlotTypeSpecializableTraitOperativeTraitObject::OperativeSlotTypeTraitObjectSpecialization(
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
pub fn GeneralSpecializationTargetView(target: SlotTypeSpecializableTraitObject) -> impl IntoView {
    match target {
        SlotTypeSpecializableTraitObject::TemplateSlotTraitOperative(trait_op) => {
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
        SlotTypeSpecializableTraitObject::OperativeSlotTypeMultiSpecialization(multi) => {
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
        SlotTypeSpecializableTraitObject::TemplateSlotMultiOperative(multi) => {
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
        SlotTypeSpecializableTraitObject::OperativeSlotTypeTraitObjectSpecialization(trait_op) => {
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
    specialization: SlotTypeSpecializationTraitObject,
    is_entry_point: bool,
) -> impl IntoView {
    // let operative_clone = operative.clone();
    let specialization_clone = specialization.clone();
    let trait_specialization_target_view =
        move |target: SlotTypeSpecializableTraitOperativeTraitObject| {
            match target {
            SlotTypeSpecializableTraitOperativeTraitObject::TemplateSlotTraitOperative(_) => todo!(),
            SlotTypeSpecializableTraitOperativeTraitObject::OperativeSlotTypeTraitObjectSpecialization(
                _,
            ) => todo!(),
        }
        };
    let parent_view = move || match specialization_clone.clone() {
        SlotTypeSpecializationTraitObject::OperativeSlotTypeSingleSpecialization(item) => view! {
            <GeneralSpecializationTargetView target=item.get_specializationtarget_slot() />
        }
        .into_any(),
        SlotTypeSpecializationTraitObject::OperativeSlotTypeMultiSpecialization(item) => view! {

            <GeneralSpecializationTargetView target=item.get_specializationtarget_slot() />
        }
        .into_any(),
        SlotTypeSpecializationTraitObject::OperativeSlotTypeTraitObjectSpecialization(item) => {
            view! {
                <TraitSpecializationTargetView target=item.get_specializationtarget_slot() />
            }
            .into_any()
        }
    };
    let specialization_clone = specialization.clone();
    let origin_view = move || {
        let specialization_clone = specialization_clone.clone();
        if is_entry_point {
            let inner_view = move || match specialization_clone.clone() {
                SlotTypeSpecializationTraitObject::OperativeSlotTypeSingleSpecialization(
                    single,
                ) => EitherOf3::A(format!(
                    "Single Option Specialization: {}",
                    single.get_allowedoperative_slot().get_name()
                )),
                SlotTypeSpecializationTraitObject::OperativeSlotTypeMultiSpecialization(multi) => {
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
                SlotTypeSpecializationTraitObject::OperativeSlotTypeTraitObjectSpecialization(
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
