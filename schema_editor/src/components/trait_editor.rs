use crate::components::{
    common::*,
    workspace::{WorkspaceState, WorkspaceTab},
};
use schema_editor_generated_toolkit::prelude::*;

#[component]
pub fn TraitEditor(trait_concrete: RGSOConcrete<TraitConcrete, Schema>) -> impl IntoView {
    let ctx = use_context::<SharedGraph<Schema>>().unwrap();
    let WorkspaceState {
        schema,
        selected_tab,
    } = use_context::<WorkspaceState>().unwrap();

    let ctx_clone = ctx.clone();
    let trait_concrete_clone = trait_concrete.clone();
    let delete_trait_concrete = move |_| {
        let ctx_clone = ctx_clone.clone();
        trait_concrete_clone
            .edit(ctx_clone)
            .delete()
            .execute()
            .unwrap();
        selected_tab.set(WorkspaceTab::Template(RwSignal::new(None)))
    };
    let trait_concrete_clone = trait_concrete.clone();
    let ctx_clone = ctx.clone();
    let update_name = move |new_val: String| {
        let editor = trait_concrete_clone.edit(ctx_clone.clone());
        editor.set_name(new_val).execute().unwrap();
    };

    view! {
        <div>
            <Section>
                <SectionHeader>Overview</SectionHeader>
                <ToggleManagedTextInput getter=move || trait_concrete.get_name_field() setter=update_name />
                <Button on:click=delete_trait_concrete>Delete Item</Button>
                // <Button on:click=delete_template_recursive>Delete Item Recursive</Button>
            </Section>
        </div>

    }
}
