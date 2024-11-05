// use leptos::prelude::*;

use crate::components::workspace::Workspace;
use generated_crate::prelude::*;
#[component]
pub fn App() -> impl IntoView {
    let graph = RBaseGraphEnvironment::<Schema>::new(&CONSTRAINT_SCHEMA);
    let shared_graph = std::sync::Arc::new(graph.clone());
    let execution = SchemaConcrete::new(shared_graph.clone())
        .set_temp_id("schema")
        .add_new_templates(|template| template.set_name("goofen".to_string()))
        .add_new_templates(|template| template.set_name("cloo".to_string()))
        .add_new_templates(|template| template.set_name("aaa".to_string()))
        .add_new_templates(|template| template.set_name("ccc".to_string()))
        .execute()
        .unwrap();
    let schema_id = execution.get_final_id("schema").unwrap().clone();

    provide_context(shared_graph);

    let ctx_for_undo = graph.clone();
    let undo_graph_action = move |_| {
        ctx_for_undo.undo();
    };
    let ctx_for_redo = graph.clone();
    let redo_graph_action = move |_| {
        ctx_for_redo.redo();
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
            <div style="display:flex">
                <div style="flex-grow:1">
                    <Workspace schema_final_id=schema_id/>
                </div>
                <div style="flex-grow:1">
                    <Workspace schema_final_id=schema_id/>
                </div>
            </div>
        </div>
    }
}
