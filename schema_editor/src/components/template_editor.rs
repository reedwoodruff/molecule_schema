use std::sync::Arc;

use generated_crate::prelude::*;

use super::workspace::WorkspaceState;
#[component]
pub fn TemplateEditor() -> impl IntoView {
    let ctx = use_context::<Arc<RBaseGraphEnvironment<Schema>>>().unwrap();
    let WorkspaceState {
        schema,
        selected_tab,
    } = use_context::<WorkspaceState>().unwrap();

    let ctx_clone = ctx.clone();
    let templates_view = move || {
        let ctx_clone = ctx_clone.clone();

        let templates = schema.get_templates_slot();
        let template_view = move |template: RGSOConcrete<TemplateConcrete, Schema>| {
            let ctx_clone = ctx_clone.clone();
            let template_clone = template.clone();
            let template_clone_2 = template.clone();
            let update_name = move |e: leptos::ev::Event| {
                let editor = template_clone.edit(ctx_clone.clone());
                let new_val = event_target_value(&e);
                editor.set_name(new_val).execute().unwrap();
            };
            view! {
                <div>
               {move || template_clone_2.get_name_field()}
               <input prop:value=move || template.get_name_field() on:input=update_name />
               something
               </div>
            }
        };
        view! {
            <For each=move || templates.clone() key=|item| item.get_id().clone() children=template_view>
            </For>
        }
    };

    templates_view
}
