use schema_editor_generated_toolkit::prelude::*;

use crate::components::{
    common::{Section, SectionHeader, SignalTextInput, ToggleManagedTextInput},
    workspace::WorkspaceState,
};

#[component]
pub fn TemplateEditor(template: RGSOConcrete<TemplateConcrete, Schema>) -> impl IntoView {
    let ctx = use_context::<SharedGraph<Schema>>().unwrap();
    let WorkspaceState {
        schema,
        selected_tab,
    } = use_context::<WorkspaceState>().unwrap();
    let ctx_clone = ctx.clone();

    let template_clone = template.clone();
    let update_name = move |new_val: String| {
        let editor = template_clone.edit(ctx_clone.clone());
        editor.set_name(new_val).execute().unwrap();
    };

    let derivative_operative_name = RwSignal::new(template.get_name_field());
    let template_clone = template.clone();
    let create_derivative_operative = move |_| {
        let derivative_operative_name = derivative_operative_name.clone().get();
        schema
            .edit(ctx.clone())
            .add_new_operatives(|op| {
                op.set_name(derivative_operative_name.clone())
                    .add_existing_roottemplate(template_clone.get_id(), |item| item)
            })
            .execute()
            .unwrap();
    };

    view! {
        <div>
            <Section>
                <SectionHeader>Overview</SectionHeader>
                <ToggleManagedTextInput getter=move || template.get_name_field() setter=update_name />
            </Section>
            <Section>
                <SectionHeader>Create Derivatives</SectionHeader>
                <SignalTextInput value=derivative_operative_name/><button on:click=create_derivative_operative>Create derivative operative</button>
            </Section>
            <Section>
                <SectionHeader>Fields</SectionHeader>

            </Section>

       </div>
    }
}
