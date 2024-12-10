use leptos::either::Either;
use schema_editor_generated_toolkit::prelude::*;

#[component]
pub fn OperativeLineage(
    operative: RGSOConcrete<OperativeConcrete, Schema>,
    is_entry_point: bool,
) -> impl IntoView {
    let operative_clone = operative.clone();
    let parent_view = if let Some(parent) = operative.get_parentoperative_slot().into_iter().next()
    {
        view! {
            <OperativeLineage operative=parent.clone() is_entry_point=false />
            "↓"
            <div>"Parent Operative: "{move || parent.get_name()}</div>
        }
        .into_any()
    } else {
        let template_name = move || operative_clone.get_roottemplate_slot().get_name();
        view! { <div>"Root Template: "{template_name}</div> }
        .into_any()
    };
    let origin_view = if is_entry_point {
        Either::Left(view! {
            "↓"
            <div>"This Element: "{move || operative.get_name()}</div>
        })
    } else {
        Either::Right(())
    };
    view! {
        {parent_view}
        {origin_view}
    }
}
