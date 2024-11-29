use std::collections::HashMap;

use crate::components::{common::Button, control_panel::ControlPanel, workspace::Workspace};
use schema_editor_generated_toolkit::prelude::*;
#[component]
pub fn App() -> impl IntoView {
    let shared_graph = initialize_graph();
    // let schema_id = shared_graph
    //     .created_instances
    //     .get()
    //     .values()
    //     .find(|instance| instance.operative().tag.id == SchemaConcrete::get_operative_id())
    //     .unwrap()
    //     .get_id()
    //     .clone();
    let schema_id = RwSignal::new(Some(
        SchemaConcrete::new(shared_graph.clone())
            .set_temp_id("main_template")
            .execute()
            .unwrap()
            .get_final_id("main_template")
            .unwrap()
            .clone(),
    ));

    provide_context(shared_graph.clone());

    let ctx_for_undo = shared_graph.clone();
    let undo_graph_action = move |_| {
        ctx_for_undo.undo();
    };
    let ctx_for_redo = shared_graph.clone();
    let redo_graph_action = move |_| {
        ctx_for_redo.redo();
    };

    let ctx_clone = shared_graph.clone();
    let serialize_graph = move |_| {
        let rbase_graph: std::sync::Arc<RBaseGraphEnvironment<Schema>> = ctx_clone.clone().into();
        let json = serde_json::to_string_pretty(&rbase_graph).unwrap();
        leptos::logging::log!("{}", json);
    };

    let show_control_panel = RwSignal::new(false);
    let ctx_clone = shared_graph.clone();
    view! {
        <div>
            <div style="display:flex;">
                <div>
                <Button on:click=move |_| {leptos::logging::log!("{:#?}", ctx_clone.created_instances.get().values())}>debug print</Button>
                </div>
                <div>
                    <Button on:click=serialize_graph>export</Button>
                </div>
                <div>
                    <Button on:click=undo_graph_action>undo</Button>
                </div>
                <div>
                    <Button on:click=redo_graph_action>redo</Button>
                </div>
                <div>
                    <Button on:click=move |_|show_control_panel.update(|prev| *prev = !*prev)>toggle control panel</Button>
                </div>
            </div>
            <Show when=move || !show_control_panel.get()>
                <div style="display:flex">
                    <div style="flex-grow:1">
                        <Workspace schema_final_id=schema_id/>
                    </div>
                    <div style="flex-grow:1">
                        <Workspace schema_final_id=schema_id/>
                    </div>
                </div>
            </Show>
            <Show when=move||show_control_panel.get()>
                <ControlPanel schema_id=schema_id />
            </Show>
        </div>
    }
}
