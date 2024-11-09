use crate::components::{
    common::*,
    workspace::{WorkspaceState, WorkspaceTab},
};
use schema_editor_generated_toolkit::prelude::*;

#[component]
pub fn OperativeEditor(operative: RGSOConcrete<OperativeConcrete, Schema>) -> impl IntoView {
    let derivative_operative_name = RwSignal::new(operative.get_name_field());
    let ctx = use_context::<SharedGraph<Schema>>().unwrap();
    let WorkspaceState {
        schema,
        selected_tab,
    } = use_context::<WorkspaceState>().unwrap();
    let ctx_clone = ctx.clone();
    let schema_clone = schema.clone();
    let selected_tab = selected_tab.clone();
    let operative_clone = operative.clone();

    let create_derivative_operative = move |_| {
        let derivative_operative_name = derivative_operative_name.clone().get();
        schema
            .edit(ctx_clone.clone())
            .add_new_operatives(|op| {
                op.set_name(derivative_operative_name.clone())
                    .add_existing_roottemplate(operative_clone.get_id(), |item| item)
                    .add_existing_parentoperative(operative_clone.get_id(), |na| na)
            })
            .execute()
            .unwrap();
    };

    let operative_clone = operative.clone();
    let ctx_clone = ctx.clone();
    let update_name = move |new_val: String| {
        let editor = operative_clone.edit(ctx_clone.clone());
        editor.set_name(new_val).execute().unwrap();
    };

    let operative_clone = operative.clone();
    let ctx_clone = ctx.clone();
    let selected_tab_clone = selected_tab.clone();
    let delete_operative = move |_| {
        let ctx_clone = ctx_clone.clone();
        operative_clone
            .edit(ctx_clone)
            .delete_recursive()
            .execute()
            .unwrap();
        selected_tab_clone.set(WorkspaceTab::Operative(RwSignal::new(None)))
    };

    let operative_clone = operative.clone();
    let operative_clone_3 = operative.clone();
    view! {
        <Section>
            <SectionHeader>Overview</SectionHeader>
            <SubSection>
                <SubSectionHeader>
                    Name:
                </SubSectionHeader>
                <ToggleManagedTextInput getter=move || operative_clone.get_name_field() setter=update_name />
            </SubSection>
            <SubSection>
                <Button on:click=delete_operative>Delete Item</Button>
            </SubSection>
        </Section>

        <Section>
            <SectionHeader>Create Derivatives</SectionHeader>
            <SignalTextInput value=derivative_operative_name/><Button on:click=create_derivative_operative>Create derivative operative</Button>
        </Section>

        <Section>
            <SectionHeader>Fields</SectionHeader>
            // <SubSection>
            //     <For each=move||operative_clone_3.get_() key=|item| item.get_id().clone() children=template_field_view />
            // </SubSection>
        </Section>
    }
}
