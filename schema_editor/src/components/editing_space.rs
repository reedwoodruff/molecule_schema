use std::sync::Arc;

use leptos::either::EitherOf5;
use schema_editor_generated_toolkit::prelude::*;

use crate::components::{
    instance_editor::InstanceEditor, operative_editor::OperativeEditor, trait_editor::TraitEditor,
};

use super::{
    template_editor::TemplateEditor,
    workspace::{WorkspaceState, WorkspaceTab},
};
#[component]
pub fn EditingSpace() -> impl IntoView {
    let WorkspaceState {
        schema,
        selected_tab,
    } = use_context::<WorkspaceState>().unwrap();

    let editor = move || {
        let selected_tab = selected_tab.clone();
        let list = match selected_tab.get() {
            WorkspaceTab::Template(inner) => match inner.get() {
                Some(template) => EitherOf5::A(view! {<TemplateEditor template />}),
                None => EitherOf5::E(()),
            },
            WorkspaceTab::Operative(operative) => match operative.get() {
                Some(operative) => EitherOf5::B(view! {<OperativeEditor operative />}),
                None => EitherOf5::E(()),
            },
            WorkspaceTab::Instance(instance) => match instance.get() {
                Some(instance) => EitherOf5::C(view! {<InstanceEditor instance />}),
                None => EitherOf5::E(()),
            },
            WorkspaceTab::Trait(trait_concrete) => match trait_concrete.get() {
                Some(trait_concrete) => EitherOf5::D(view! {<TraitEditor trait_concrete/>}),
                None => EitherOf5::E(()),
            },
        };
        list
    };

    view! {
        <div class="editing-space-container">
        {editor}
        </div>
    }
}
