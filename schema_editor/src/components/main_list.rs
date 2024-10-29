use std::sync::Arc;

use generated_crate::prelude::*;
use leptos::either::EitherOf4;

use super::{
    template_editor::TemplateEditor,
    workspace::{WorkspaceState, WorkspaceTab},
};
#[component]
pub fn MainList() -> impl IntoView {
    let ctx = use_context::<Arc<RBaseGraphEnvironment<Schema>>>().unwrap();
    let WorkspaceState {
        schema,
        selected_tab,
    } = use_context::<WorkspaceState>().unwrap();

    move || match selected_tab.get() {
        WorkspaceTab::Template(template) => EitherOf4::A(TemplateEditor),
        WorkspaceTab::Operative(operative) => EitherOf4::B(view! {}),
        WorkspaceTab::Instance(instance) => EitherOf4::C(view! {}),
        WorkspaceTab::Trait(trait_concrete) => EitherOf4::D(view! {}),
    }
}
