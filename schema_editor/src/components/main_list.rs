use std::sync::Arc;

use generated_crate::prelude::*;
use leptos::either::EitherOf4;

use super::{
    template_editor::TemplateEditor,
    workspace::{WorkspaceState, WorkspaceTab},
};
#[component]
pub fn MainList() -> impl IntoView {
    let ctx = use_context::<SharedGraph<Schema>>().unwrap();
    let WorkspaceState {
        schema,
        selected_tab,
    } = use_context::<WorkspaceState>().unwrap();

    let list_item_view = move || {};

    let list = move || {
        let schema = schema.clone();
        match selected_tab.get() {
            WorkspaceTab::Template(tab_state) => {
                let all_templates = schema.get_templates_slot();
                EitherOf4::A(view! {
                    <For
                    each=move || schema.get_templates_slot()
                    key=|item| item.get_id().clone()
                    children=move |template| {
                        let template = template.clone();
                        let template2 = template.clone();
                        view!{
                            <div>
                                <a class="clickable-list-item"
                                    on:click=move |_| {let template = template.clone(); tab_state.set(Some(template))}>
                                    {move || template2.get_name_field()}
                                </a>
                            </div>
                        }
                    }
                    >
                    </For>

                })
            }
            WorkspaceTab::Operative(operative) => EitherOf4::B(view! {}),
            WorkspaceTab::Instance(instance) => EitherOf4::C(view! {}),
            WorkspaceTab::Trait(trait_concrete) => EitherOf4::D(view! {}),
        }
    };

    view! {
        <div class="list-container">
        {list}
        </div>
    }
}
