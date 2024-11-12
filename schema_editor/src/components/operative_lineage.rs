use leptos::either::Either;
use schema_editor_generated_toolkit::prelude::*;

#[component]
pub fn OperativeLineage(operative: RGSOConcrete<OperativeConcrete, Schema>) -> impl IntoView {
    let view = if let Some(parent) = operative.get_parentoperative_slot().into_iter().next() {
        view! {
            <OperativeLineage operative=parent.clone() />
            |<br/>
            V
            <div>
            Parent Operative: {move || parent.get_name()}
            </div>
        }
        .into_any()
    } else {
        let template_name = move || operative.get_roottemplate_slot().get_name();
        view! {
            <div>
            Root Template: {template_name}
            </div>
        }
        .into_any()
    };
    view
}
