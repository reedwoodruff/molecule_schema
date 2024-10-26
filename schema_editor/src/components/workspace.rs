use std::sync::Arc;

use crate::prelude::*;
#[component]
pub fn Workspace(schema_final_id: u128) -> impl IntoView {
    let ctx = use_context::<Arc<RBaseGraphEnvironment<Schema>>>().unwrap();

    let schema = match ctx.get(&schema_final_id).unwrap() {
        Schema::SchemaConcrete(inner) => inner,
        _ => panic!(),
    };

    let ctx_for_undo = ctx.clone();
    let undo_graph_action = move |_| {

        ctx_for_undo.undo();
        
    };
    let ctx_for_redo = ctx.clone();
    let redo_graph_action = move |_| {
        
        ctx_for_redo.redo();
    };

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
                // log!("{}", new_val);
                editor.set_name(new_val).execute().unwrap();
                // log!("{:#?}", ctx_clone.created_instances.get())
            };
            view! {
                <div>
               {move || template_clone_2.get_name_field()}
               <input prop:value=template.get_name_field() on:input=update_name />
               something
               </div>
            }
        };
        view! {
            <For each=move || templates.clone() key=|item| item.get_id().clone() children=template_view>
            </For>
        }
    };
    view! {
        <div>
            <div style="display:flex;">
            <div>
                <button on:click=undo_graph_action>undo</button>
            </div>
            <div>
                <button on:click=redo_graph_action>redo</button>
            </div>
            </div>
            Workspace
            {templates_view}
        </div>
    }
}
