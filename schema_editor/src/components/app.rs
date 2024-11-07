use std::collections::HashMap;

use crate::components::workspace::Workspace;
use schema_editor_generated_toolkit::prelude::*;
#[component]
pub fn App() -> impl IntoView {
    let shared_graph = initialize_graph();
    let schema_id = shared_graph
        .created_instances
        .get()
        .values()
        .find(|instance| instance.operative().tag.id == SchemaConcrete::get_operative_id())
        .unwrap()
        .get_id()
        .clone();

    provide_context(shared_graph.clone());

    print!("{:#?}", shared_graph);

    let ctx_for_undo = shared_graph.clone();
    let undo_graph_action = move |_| {
        ctx_for_undo.undo();
    };
    let ctx_for_redo = shared_graph.clone();
    let redo_graph_action = move |_| {
        ctx_for_redo.redo();
    };

    let serialize_graph = move |_| {
        let rbase_graph: std::sync::Arc<RBaseGraphEnvironment<Schema>> =
            shared_graph.clone().into();
        let json = serde_json::to_string_pretty(&rbase_graph).unwrap();
        leptos::logging::log!("{}", json);
    };
    view! {
        <div>
            <div style="display:flex;">
                <div>
                    <button on:click=serialize_graph>export</button>
                </div>
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
