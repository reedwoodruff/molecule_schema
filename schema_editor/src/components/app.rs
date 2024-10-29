// use leptos::prelude::*;

use crate::components::workspace::Workspace;
use generated_crate::prelude::*;
#[component]
pub fn App() -> impl IntoView {
    let graph = RBaseGraphEnvironment::<Schema>::new(&CONSTRAINT_SCHEMA);
    let shared_graph = std::sync::Arc::new(graph);
    let execution = SchemaConcrete::new(shared_graph.clone())
        .set_temp_id("schema")
        .add_new_templates(|template| template.set_name("goofen".to_string()))
        .add_new_templates(|template| template.set_name("cloo".to_string()))
        .execute()
        .unwrap();
    let schema_id = execution.get_final_id("schema").unwrap().clone();

    provide_context(shared_graph);

    view! {
        <div>
        App
            <Workspace schema_final_id=schema_id/>
        </div>
    }
}
