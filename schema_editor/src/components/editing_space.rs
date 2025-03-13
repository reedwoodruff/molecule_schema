use leptos::either::EitherOf6;
use schema_editor_generated_toolkit::prelude::*;

use crate::components::{
    function_definition_editor::FunctionDefinitionEditor, instance_editor::InstanceEditor,
    operative_editor::OperativeEditor, trait_editor::TraitEditor,
};

use super::{
    template_editor::TemplateEditor,
    workspace::{WorkspaceState, WorkspaceTab},
};
#[component]
pub fn EditingSpace() -> impl IntoView {
    let WorkspaceState { selected_tab, .. } = use_context::<WorkspaceState>().unwrap();

    let editor = move || {
        let selected_tab = selected_tab.clone();
        let list = match selected_tab.get() {
            WorkspaceTab::Template(inner) => match inner.get() {
                Some(template) => {
                    EitherOf6::A(view! { <TemplateEditor template=RwSignal::new(template) /> })
                }
                None => EitherOf6::E(()),
            },
            WorkspaceTab::Operative(operative) => match operative.get() {
                Some(operative) => {
                    EitherOf6::B(view! { <OperativeEditor operative=RwSignal::new(operative) /> })
                }
                None => EitherOf6::E(()),
            },
            WorkspaceTab::Instance(instance) => match instance.get() {
                Some(instance) => {
                    EitherOf6::C(view! { <InstanceEditor instance=RwSignal::new(instance) /> })
                }
                None => EitherOf6::E(()),
            },
            WorkspaceTab::Trait(trait_concrete) => match trait_concrete.get() {
                Some(trait_concrete) => EitherOf6::D(
                    view! { <TraitEditor trait_concrete=RwSignal::new(trait_concrete) /> },
                ),
                None => EitherOf6::E(()),
            },
            WorkspaceTab::Function(fn_def) => match fn_def.get() {
                Some(fn_def) => EitherOf6::F(
                    view! { <FunctionDefinitionEditor fn_def=RwSignal::new(fn_def) /> },
                ),
                None => EitherOf6::E(()),
            },
        };
        list
    };

    view! { <div class="editing-space-container">{editor}</div> }
}
