use schema_editor_generated_toolkit::prelude::*;

use crate::components::{
    common::{Button, Section, SectionHeader, SignalTextInput, ToggleManagedTextInput},
    slot_builder::SlotBuilder,
    workspace::{WorkspaceState, WorkspaceTab},
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

    let ctx_clone = ctx.clone();
    let derivative_operative_name = RwSignal::new(template.get_name_field());
    let template_clone = template.clone();
    let create_derivative_operative = move |_| {
        let derivative_operative_name = derivative_operative_name.clone().get();
        schema
            .edit(ctx_clone.clone())
            .add_new_operatives(|op| {
                op.set_name(derivative_operative_name.clone())
                    .add_existing_roottemplate(template_clone.get_id(), |item| item)
            })
            .execute()
            .unwrap();
    };

    let template_clone = template.clone();
    let ctx_clone = ctx.clone();
    let delete_template = move |_| {
        let ctx_clone = ctx_clone.clone();
        template_clone.edit(ctx_clone).delete().execute().unwrap();
        selected_tab.set(WorkspaceTab::Template(RwSignal::new(None)))
    };
    let ctx_clone = ctx.clone();
    let template_clone = template.clone();
    let delete_template_recursive = move |_| {
        let ctx_clone = ctx_clone.clone();
        template_clone
            .edit(ctx_clone)
            .delete_recursive()
            .execute()
            .unwrap();
        selected_tab.set(WorkspaceTab::Template(RwSignal::new(None)))
    };

    let ctx_clone = ctx.clone();
    let template_clone = template.clone();
    let on_click_add_field = move |_| {
        template_clone
            .edit(ctx_clone.clone())
            .add_new_templatefields(|field| {
                field
                    .set_fieldname("new_field".to_string())
                    .add_new_fieldvariant::<StringFieldVariant, _>(|field_variant| field_variant)
            })
            .execute()
            .unwrap();
    };

    let is_building_slot = RwSignal::new(false);

    let template_clone = template.clone();
    let ctx_clone = ctx.clone();
    view! {
        <div>
            <Section>
                <SectionHeader>Overview</SectionHeader>
                <ToggleManagedTextInput getter=move || template.get_name_field() setter=update_name />
                <Button on:click=delete_template>Delete Item</Button>
                <Button on:click=delete_template_recursive>Delete Item Recursive</Button>
            </Section>
            <Section>
                <SectionHeader>Create Derivatives</SectionHeader>
                <SignalTextInput value=derivative_operative_name/><Button on:click=create_derivative_operative>Create derivative operative</Button>
            </Section>
            <Section>
                <SectionHeader>Fields</SectionHeader>
                <Button on:click=on_click_add_field>Add Field</Button>

            </Section>
            <Section>
                <SectionHeader>Slots</SectionHeader>
                <Show when=move || !is_building_slot.get()>
                    <Button on:click=move |_| is_building_slot.set(true)>Add Slot</Button>
                </Show>
                <Show when=move || is_building_slot.get()>
                // <SlotBuilder builder=TemplateSlot::new(ctx_clone.clone()) />
                <SlotBuilder template=template_clone.clone() />
                </Show>

            </Section>

       </div>
    }
}
