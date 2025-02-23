use graph_canvas::prelude::*;
use leptos::prelude::*;

#[component]
pub fn GraphEditor(config: GraphCanvasConfig) -> impl IntoView {
    let container_ref: NodeRef<leptos::html::Div> = NodeRef::new();

    Effect::new(move |_| {
        let config = config.clone();
        if let Some(container) = container_ref.get() {
            // Convert container to HtmlElement
            // let html_element = container;

            // Create your config

            // Initialize GraphCanvas
            let _graph_canvas =
                GraphCanvas::new_rust(&container, config).expect("Failed to create GraphCanvas");
            leptos::logging::log!("{:#?}", _graph_canvas)
        }
    });

    let is_popped_out = RwSignal::new(false);
    let popout_class = move || {
        if is_popped_out.get() {
            "graph_canvas_holder popout"
        } else {
            "graph_canvas_holder"
        }
    };

    view! {
        <div class=popout_class>
            <div
                node_ref=container_ref
                style="width: 100%; background-color: white; height: 100%; position: relative;"
            />
            <button on:click=move |_| {
                is_popped_out.update(|prev| *prev = !*prev)
            }>
                {move || match is_popped_out.get() {
                    false => "Popout Graph Visualizer",
                    true => "Popin Graph Visualizer",
                }}
            </button>
        </div>
    }
}
