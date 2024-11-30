use leptos::either::{Either, EitherOf3, EitherOf4};
use schema_editor_generated_toolkit::prelude::*;

#[component]
pub fn TraitSpecializationTargetView(
    target: OperativeSlotTypeSpecializableTraitOperativeTraitObject,
) -> impl IntoView {
    match target {
        OperativeSlotTypeSpecializableTraitOperativeTraitObject::TemplateSlotTypeTraitOperative(trait_op) => {
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
        OperativeSlotTypeSpecializableTraitOperativeTraitObject::OperativeSlotTypeTraitObjectSpecialization(
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
                        <TraitSpecializationTargetView target=trait_op.get_upstreamtype_slot() />
                         {text}
                         <br/>
                         "↓"
                         <br/>

            }.into_any()
        }
    }
}
#[component]
pub fn GeneralSpecializationTargetView(
    target: OperativeSlotTypeSpecializableTraitObject,
) -> impl IntoView {
    match target {
        OperativeSlotTypeSpecializableTraitObject::TemplateSlotTypeTraitOperative(trait_op) => {
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
        OperativeSlotTypeSpecializableTraitObject::OperativeSlotTypeMultiSpecialization(multi) => {
            let text = format!(
                "Narrowed to this operative list (and their descendents): [{}]",
                multi
                    .get_allowedoperatives_slot()
                    .into_iter()
                    .map(|item| item.get_name())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            view! {
                <GeneralSpecializationTargetView target=multi.get_upstreamtype_slot() />
                         {text}
                         <br/>
                         "↓"
                         <br/>
            }
            .into_any()
        }
        OperativeSlotTypeSpecializableTraitObject::TemplateSlotTypeMultiOperative(multi) => {
            let text = format!(
                "Root Operative List (and their descendents): [{}]",
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
        OperativeSlotTypeSpecializableTraitObject::OperativeSlotTypeSingleSpecialization(
            single,
        ) => {
            let text = format!(
                "Narrowed to this operative (and its descendents): {}",
                single.get_allowedoperative_slot().get_name()
            );
            view! {
                <GeneralSpecializationTargetView target=single.get_upstreamtype_slot() />
                         {text}
                         <br/>
                         "↓"
                         <br/>
            }
            .into_any()
        }
        OperativeSlotTypeSpecializableTraitObject::TemplateSlotTypeSingleOperative(single) => {
            let text = format!(
                "Root Operative (and its descendents): {}",
                single.get_allowedoperative_slot().get_name()
            );
            view! {
                         {text}
                         <br/>
                         "↓"
                         <br/>
            }
            .into_any()
        }
        OperativeSlotTypeSpecializableTraitObject::OperativeSlotTypeTraitObjectSpecialization(
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
                        <TraitSpecializationTargetView target=trait_op.get_upstreamtype_slot() />
                         {text}
                         <br/>
                         "↓"
                         <br/>
            }
            .into_any()
        }
    }
}
#[component]
pub fn SlotTypeSpecializationLineage(
    specialization: OperativeSlotTypeSpecializationTraitObject,
    is_entry_point: bool,
) -> impl IntoView {
    // let operative_clone = operative.clone();
    let specialization_clone = specialization.clone();
    let trait_specialization_target_view =
        move |target: OperativeSlotTypeSpecializableTraitOperativeTraitObject| {
            match target {
            OperativeSlotTypeSpecializableTraitOperativeTraitObject::TemplateSlotTypeTraitOperative(_) => todo!(),
            OperativeSlotTypeSpecializableTraitOperativeTraitObject::OperativeSlotTypeTraitObjectSpecialization(
                _,
            ) => todo!(),
        }
        };
    let parent_view = move || match specialization_clone.clone() {
        OperativeSlotTypeSpecializationTraitObject::OperativeSlotTypeSingleSpecialization(item) => {
            view! {
                <GeneralSpecializationTargetView target=item.get_upstreamtype_slot() />
            }
            .into_any()
        }
        OperativeSlotTypeSpecializationTraitObject::OperativeSlotTypeMultiSpecialization(item) => {
            view! {

                <GeneralSpecializationTargetView target=item.get_upstreamtype_slot() />
            }
            .into_any()
        }
        OperativeSlotTypeSpecializationTraitObject::OperativeSlotTypeTraitObjectSpecialization(
            item,
        ) => view! {
            <TraitSpecializationTargetView target=item.get_upstreamtype_slot() />
        }
        .into_any(),
    };
    let specialization_clone = specialization.clone();
    let origin_view = move || {
        let specialization_clone = specialization_clone.clone();
        if is_entry_point {
            let inner_view = move || {
                match specialization_clone.clone() {
                OperativeSlotTypeSpecializationTraitObject::OperativeSlotTypeSingleSpecialization(
                    single,
                ) => EitherOf3::A(format!(
                    "Single Option Specialization: {}",
                    single.get_allowedoperative_slot().get_name()
                )),
                OperativeSlotTypeSpecializationTraitObject::OperativeSlotTypeMultiSpecialization(multi) => {
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
                OperativeSlotTypeSpecializationTraitObject::OperativeSlotTypeTraitObjectSpecialization(
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
            }
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
