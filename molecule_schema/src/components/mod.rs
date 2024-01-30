use crate::utils::reactive_types::{RConstraintObject, RConstraintSchema};
use leptos::{logging::log, *};
use serde_types::{
    constraint_schema::ConstraintSchema,
    primitives::{PrimitiveTypes, PrimitiveValues},
};

#[component]
pub fn App(schema: ConstraintSchema<PrimitiveTypes, PrimitiveValues>) -> impl IntoView {
    let reactive_schema: RConstraintSchema<PrimitiveTypes, PrimitiveValues> = schema.into();
    let constraint_objects = reactive_schema.constraint_objects;
    view! {
        <div class="flex">
            <div class="flex-grow">
            <For
                each=constraint_objects
                key=move |(id, _child)| id.clone()
                children=move |(_id, child)| view!{<ConstraintObject object={child} />}
            />
            </div>
        </div>
    }
}

#[component]
pub fn ConstraintObject(
    object: RConstraintObject<PrimitiveTypes, PrimitiveValues>,
) -> impl IntoView {
    view! {
    <div>
        {object.tag.name}
    </div>
    }
}
