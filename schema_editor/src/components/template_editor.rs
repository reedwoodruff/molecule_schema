use leptos::either::Either;
use schema_editor_generated_toolkit::prelude::*;

use crate::components::{
    common::{
        Button, LeafSection, LeafSectionHeader, Section, SectionHeader, SignalTextInput,
        SubSection, SubSectionHeader, ToggleManagedTextInput,
    },
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
    let selected_tab_clone = selected_tab.clone();
    let delete_template_recursive = move |_| {
        let ctx_clone = ctx_clone.clone();
        template_clone
            .edit(ctx_clone)
            .delete_recursive()
            .execute()
            .unwrap();
        selected_tab_clone.set(WorkspaceTab::Template(RwSignal::new(None)))
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
    let close_building_interface_callback = Callback::new(move |_| is_building_slot.set(false));

    let template_clone = template.clone();

    let selected_tab_clone = selected_tab.clone();
    let ctx_clone = ctx.clone();
    let template_slot_view = move |template_slot: RGSOConcrete<TemplateSlot, Schema>| {
        let template_slot_clone = template_slot.clone();
        let selected_tab_clone = selected_tab.clone();
        let details_view = move || match template_slot_clone.get_operativevariant_slot() {
            TemplateSlotVariantTraitObject::TraitOperativeVariant(trait_op_variant) => {
                let trait_list = trait_op_variant
                    .get_traits_slot()
                    .iter()
                    .map(|item| item.get_name())
                    .collect::<Vec<_>>()
                    .join(", ");
                Either::Left("Traits: ".to_string() + &trait_list)
            }
            TemplateSlotVariantTraitObject::ConcreteOperativeVariant(conc_op) => {
                let op = conc_op.get_operative_slot();
                let op_clone = op.clone();
                Either::Right(view! {
                    <div>
                    Operative: <a class="clickable-list-item"
                        on:click=move |_| {selected_tab_clone.set(WorkspaceTab::Operative(RwSignal::new(Some(op_clone.clone()))))}>
                        {move || op.get_name()}</a>
                    </div>
                })
            }
        };
        let ctx_clone = ctx_clone.clone();
        let template_slot_clone = template_slot.clone();
        let on_click_delete_slot = move |_| {
            template_slot_clone
                .edit(ctx_clone.clone())
                .delete_recursive()
                .execute()
                .unwrap();
        };
        view! {
            <LeafSection>
                <LeafSectionHeader>
                {move || template_slot.get_name()}
                </LeafSectionHeader>
                <div class="flex">
                    <LeafSection>
                    {details_view}
                    </LeafSection>
                </div>
                <div class="align-right">
                <Button on:click=on_click_delete_slot>
                    Delete Slot
                </Button>
                </div>
            </LeafSection>
        }
    };
    let template_clone = template.clone();
    let template_clone_2 = template.clone();
    let ctx_clone = ctx.clone();
    view! {
        <div>
            <Section>
                <SectionHeader>Overview</SectionHeader>
                <LeafSection>
                    <LeafSectionHeader>
                        Name:
                    </LeafSectionHeader>
                    <ToggleManagedTextInput getter=move || template.get_name_field() setter=update_name />
                </LeafSection>
                <LeafSection>
                    <Button on:click=delete_template>Delete Item</Button>
                    <Button on:click=delete_template_recursive>Delete Item Recursive</Button>
                </LeafSection>
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
                <SlotBuilder template=template_clone.clone() close_callback=close_building_interface_callback/>
                </Show>
                <SubSection>
                <SubSectionHeader>
                    Existing Slots
                </SubSectionHeader>
                <For each=move ||template_clone_2.get_templateslots_slot() key=|item| item.get_id().clone() children=template_slot_view />
                </SubSection>

            </Section>

       </div>
    }
}
