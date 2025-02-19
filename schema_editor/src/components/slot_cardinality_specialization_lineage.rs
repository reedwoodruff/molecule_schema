use leptos::either::{Either, EitherOf8};
use schema_editor_generated_toolkit::prelude::*;

use crate::components::common::{LeafSection, LeafSectionHeader};

#[component]
pub fn SlotCardinalitySpecializationLineage(
    specialization: OperativeSlotCardinalitySpecializationTraitObject,
) -> impl IntoView {
    // let operative_clone = operative.clone();
    let specialization_clone = specialization.clone();
    move || {
        match specialization_clone.clone() {
            OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityLowerBoundOrZeroSpecialization(item) => {
                let target = OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundOrZeroSpecialization(item);
                view! { <GeneralSpecializationTargetView is_entry_point=true target=target /> }.into_any()
            },
            OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityRangeSpecialization(item) => {
                let target = OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityRangeSpecialization(item);
                view! { <GeneralSpecializationTargetView is_entry_point=true target=target /> }.into_any()
            },
            OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityRangeOrZeroSpecialization(item) => {
                let target = OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityRangeOrZeroSpecialization(item);
                view! { <GeneralSpecializationTargetView is_entry_point=true target=target /> }.into_any()
            },
            OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityLowerBoundSpecialization(item) => {
                let target = OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundSpecialization(item);
                view! { <GeneralSpecializationTargetView is_entry_point=true target=target /> }.into_any()
            },
            OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalityZeroSpecialization(item) => {
                let parent = match item.get_upstreamcardinality_slot() {
                    OperativeSlotCardinalitySpecializableByZeroTraitObject::OperativeSlotCardinalityLowerBoundOrZeroSpecialization(item) => {
                        OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundOrZeroSpecialization(item)
                    },
                    OperativeSlotCardinalitySpecializableByZeroTraitObject::TemplateSlotCardinalityRangeOrZero(item) => {
                        OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityRangeOrZero(item)
                    },
                    OperativeSlotCardinalitySpecializableByZeroTraitObject::OperativeSlotCardinalityRangeSpecialization(item) => {
                        OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityRangeSpecialization(item)
                    },
                    OperativeSlotCardinalitySpecializableByZeroTraitObject::TemplateSlotCardinalityLowerBoundOrZero(item) => {
                        OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityLowerBoundOrZero(item)
                    },
                    OperativeSlotCardinalitySpecializableByZeroTraitObject::TemplateSlotCardinalityRange(item) => {
                        OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityRange(item)
                    },
                    OperativeSlotCardinalitySpecializableByZeroTraitObject::TemplateSlotCardinalityLowerBound(item) => {
                        OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityLowerBound(item)
                    },
                    OperativeSlotCardinalitySpecializableByZeroTraitObject::OperativeSlotCardinalityRangeOrZeroSpecialization(item) => {
                        OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityRangeOrZeroSpecialization(item)
                    },
                    OperativeSlotCardinalitySpecializableByZeroTraitObject::OperativeSlotCardinalityLowerBoundSpecialization(item) => {
                        OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundSpecialization(item)
                    },
                };
                view! {
                    <GeneralSpecializationTargetView is_entry_point=false target=parent />
                    <LeafSectionHeader>"Specialized: Zero"</LeafSectionHeader>
                }.into_any()
            },
            OperativeSlotCardinalitySpecializationTraitObject::OperativeSlotCardinalitySingleSpecialization(item) => {
                let parent = match item.get_upstreamcardinality_slot() {
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundOrZeroSpecialization(item) => {
                        OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundOrZeroSpecialization(item)
                    },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityRangeOrZero(item) => {
                        OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityRangeOrZero(item)
                    },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityRangeSpecialization(item) => {
                        OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityRangeSpecialization(item)
                    },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityLowerBoundOrZero(item) => {
                        OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityLowerBoundOrZero(item)
                    },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityRange(item) => {
                        OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityRange(item)
                    },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityLowerBound(item) => {
                        OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityLowerBound(item)
                    },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityRangeOrZeroSpecialization(item) => {
                        OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityRangeOrZeroSpecialization(item)
                    },
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundSpecialization(item) => {
                        OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundSpecialization(item)
                    },
                };
                view! {
                    <GeneralSpecializationTargetView is_entry_point=false target=parent />
                    <LeafSectionHeader>"Specialized: Single"</LeafSectionHeader>
                }.into_any()
            }
        }
    }
}

#[component]
pub fn GeneralSpecializationTargetView(
    target: OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject,
    is_entry_point: bool,
) -> impl IntoView {
    let target_clone = target.clone();
    let parent_view = move || {
        match target_clone.clone() {

        OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityLowerBoundOrZero(_item) => {
            view!{}.into_any()
        }
        OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityLowerBound(_item) => {
            view!{}.into_any()
        }
        OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityRangeOrZero(_item) => {
            view!{}.into_any()
        }
        OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityRange(_item) => {
            view!{}.into_any()
        }
        OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundOrZeroSpecialization(item) => {
            let target = match item.get_upstreamcardinality_slot() {
                OperativeSlotCardinalitySpecializableByLowerBoundOrZeroTraitObject::OperativeSlotCardinalityLowerBoundOrZeroSpecialization(item) =>
                OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundOrZeroSpecialization(item),
                OperativeSlotCardinalitySpecializableByLowerBoundOrZeroTraitObject::TemplateSlotCardinalityLowerBoundOrZero(item) =>
                OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityLowerBoundOrZero(item),
            };
            view! { <GeneralSpecializationTargetView is_entry_point=false target=target /> }.into_any()
        },
        OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityRangeSpecialization(item) => {
            view! {
                <GeneralSpecializationTargetView
                    is_entry_point=false
                    target=item.get_upstreamcardinality_slot()
                />
            }.into_any()

        },
        OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityRangeOrZeroSpecialization(item) => {
            let target = match item.get_upstreamcardinality_slot() {
                OperativeSlotCardinalitySpecializableByRangeOrZeroTraitObject::OperativeSlotCardinalityLowerBoundOrZeroSpecialization(item) => {
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundOrZeroSpecialization(item)
                },
                OperativeSlotCardinalitySpecializableByRangeOrZeroTraitObject::TemplateSlotCardinalityRangeOrZero(item) => {
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityRangeOrZero(item)
                },
                OperativeSlotCardinalitySpecializableByRangeOrZeroTraitObject::TemplateSlotCardinalityLowerBoundOrZero(item) => {
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityLowerBoundOrZero(item)
                },
                OperativeSlotCardinalitySpecializableByRangeOrZeroTraitObject::OperativeSlotCardinalityRangeOrZeroSpecialization(item) => {
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityRangeOrZeroSpecialization(item)
                },
            };
            view! { <GeneralSpecializationTargetView is_entry_point=false target=target /> }.into_any()

        },
        OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundSpecialization(item) => {
            let target = match item.get_upstreamcardinality_slot() {
                OperativeSlotCardinalitySpecializableByLowerBoundTraitObject::OperativeSlotCardinalityLowerBoundOrZeroSpecialization(item) => {
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundOrZeroSpecialization(item)
                },
                OperativeSlotCardinalitySpecializableByLowerBoundTraitObject::TemplateSlotCardinalityLowerBoundOrZero(item) => {
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityLowerBoundOrZero(item)
                },
                OperativeSlotCardinalitySpecializableByLowerBoundTraitObject::TemplateSlotCardinalityLowerBound(item) => {
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityLowerBound(item)
                },
                OperativeSlotCardinalitySpecializableByLowerBoundTraitObject::OperativeSlotCardinalityLowerBoundSpecialization(item) => {
                    OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundSpecialization(item)
                },
            };
            view! { <GeneralSpecializationTargetView is_entry_point=false target=target /> }.into_any()

        },
    }
    };
    move || {
        match target.clone() {
        OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundOrZeroSpecialization(item) => {
            EitherOf8::A(
            view! {
                {parent_view.clone()}
                <LeafSectionHeader>"Specialized: Lower Bound Or Zero"</LeafSectionHeader>
                <LeafSection attr:class="leafsection dependent">
                    "Lower Bound: "{move || item.get_lower_bound_field()}
                </LeafSection>
                <div>
                    {move || {
                        if is_entry_point {
                            Either::Left(())
                        } else {
                            Either::Right(view! { <div>"↓"</div> })
                        }
                    }}
                </div>
            })
        },
        OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityRangeOrZero(item) => {
            let item_clone = item.clone();
            EitherOf8::H(
            view! {
                {parent_view.clone()}
                <LeafSectionHeader>"Root Cardinality: Range Or Zero"</LeafSectionHeader>
                <LeafSection attr:class="leafsection dependent">
                    "Lower Bound: "{move || item.get_lower_bound_field()}", Upper Bound: "
                    {move || item_clone.get_upper_bound_field()}
                </LeafSection>
                <div>
                    {move || {
                        if is_entry_point {
                            Either::Left(())
                        } else {
                            Either::Right(view! { <div>"↓"</div> })
                        }
                    }}
                </div>
            })
        },
        OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityRangeSpecialization(item) => {
            let item_clone = item.clone();
            EitherOf8::B(
            view! {
                {parent_view.clone()}
                <LeafSectionHeader>"Specialized: Range"</LeafSectionHeader>
                <LeafSection attr:class="leafsection dependent">
                    "Lower Bound: "{move || item.get_lower_bound_field()}", Upper Bound: "
                    {move || item_clone.get_upper_bound_field()}
                </LeafSection>
                <div>
                    {move || {
                        if is_entry_point {
                            Either::Left(())
                        } else {
                            Either::Right(view! { <div>"↓"</div> })
                        }
                    }}
                </div>
            })
        },
        OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityLowerBoundOrZero(item) => {
            EitherOf8::C(
            view! {
                {parent_view.clone()}
                <LeafSectionHeader>"Root Cardinality: Lower Bound Or Zero"</LeafSectionHeader>
                <LeafSection attr:class="leafsection dependent">
                    "Lower Bound: "{move || item.get_lower_bound_field()}
                </LeafSection>
                <div>
                    {move || {
                        if is_entry_point {
                            Either::Left(())
                        } else {
                            Either::Right(view! { <div>"↓"</div> })
                        }
                    }}
                </div>
            })
        },
        OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityRange(item) => {
            let item_clone = item.clone();
            EitherOf8::D(
            view! {
                {parent_view.clone()}
                <LeafSectionHeader>"Root Cardinality: Range"</LeafSectionHeader>
                <LeafSection attr:class="leafsection dependent">
                    "Lower Bound: "{move || item.get_lower_bound_field()}", Upper Bound: "
                    {move || item_clone.get_upper_bound_field()}
                </LeafSection>
                <div>
                    {move || {
                        if is_entry_point {
                            Either::Left(())
                        } else {
                            Either::Right(view! { <div>"↓"</div> })
                        }
                    }}
                </div>
            })
        },
        OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::TemplateSlotCardinalityLowerBound(item) => {
            EitherOf8::E(
            view! {
                {parent_view.clone()}
                <LeafSectionHeader>"Root Cardinality: Lower Bound"</LeafSectionHeader>
                <LeafSection attr:class="leafsection dependent">
                    "Lower Bound: "{move || item.get_lower_bound_field()}
                </LeafSection>
                <div>
                    {move || {
                        if is_entry_point {
                            Either::Left(())
                        } else {
                            Either::Right(view! { <div>"↓"</div> })
                        }
                    }}
                </div>
            })
        },
        OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityRangeOrZeroSpecialization(item) => {
            let item_clone = item.clone();
            EitherOf8::F(
            view! {
                {parent_view.clone()}
                <LeafSectionHeader>"Specialized: Range Or Zero"</LeafSectionHeader>
                <LeafSection attr:class="leafsection dependent">
                    "Lower Bound: "{move || item.get_lower_bound_field()}", Upper Bound: "
                    {move || item_clone.get_upper_bound_field()}
                </LeafSection>
                <div>
                    {move || {
                        if is_entry_point {
                            Either::Left(())
                        } else {
                            Either::Right(view! { <div>"↓"</div> })
                        }
                    }}
                </div>
            })
        },
        OperativeSlotCardinalitySpecializableBySingleOrRangeTraitObject::OperativeSlotCardinalityLowerBoundSpecialization(item) => {
            EitherOf8::G(
            view! {
                {parent_view.clone()}
                <LeafSectionHeader>"Specialized: Lower Bound"</LeafSectionHeader>
                <LeafSection attr:class="leafsection dependent">
                    "Lower Bound: "{move || item.get_lower_bound_field()}
                </LeafSection>
                <div>
                    {move || {
                        if is_entry_point {
                            Either::Left(())
                        } else {
                            Either::Right(view! { <div>"↓"</div> })
                        }
                    }}
                </div>
            })
        },
    }
    }
}
