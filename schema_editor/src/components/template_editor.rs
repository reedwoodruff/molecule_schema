use leptos::either::{Either, EitherOf3};
use schema_editor_generated_toolkit::prelude::*;

use crate::components::{
    common::{
        Button, LeafSection, LeafSectionHeader, ManagedEnumSelect, Section, SectionHeader,
        SignalEnumSelect, SignalTextInput, SubSection, SubSectionHeader, ToggleManagedTextInput,
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
            .add_new_fields::<StringTemplateField, _>(|field| {
                field.set_name("new_field".to_string())
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
        let details_view = move || match template_slot_clone.get_templateslotvariant_slot() {
            TemplateSlotVariantTraitObject::TemplateSlotMultiOperative(conc_ops) => {
                EitherOf3::A(view! {
                <div>
                "Operatives: ["
                    <For each=move || conc_ops.get_allowedoperatives_slot() key=|op| op.get_id().clone() let:op>
                        {
                        let op_clone = op.clone();
                        view!{
                        <a class="clickable-list-item"
                        on:click=move |_| {selected_tab_clone.set(WorkspaceTab::Operative(RwSignal::new(Some(op_clone.clone()))))}>
                            {move || op.get_name()}
                        </a>
                        ", "
                        }
                        }
                    </For>
                "]"
                </div>
                })
            }
            TemplateSlotVariantTraitObject::TemplateSlotTraitOperative(trait_op_variant) => {
                let trait_list = trait_op_variant
                    .get_allowedtraits_slot()
                    .iter()
                    .map(|item| item.get_name())
                    .collect::<Vec<_>>()
                    .join(", ");
                EitherOf3::B("Traits: ".to_string() + &trait_list)
            }
            TemplateSlotVariantTraitObject::TemplateSlotSingleOperative(conc_op) => {
                let op = conc_op.get_allowedoperative_slot();
                let op_clone = op.clone();
                EitherOf3::C(view! {
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

    let ctx_clone = ctx.clone();
    let template_clone = template.clone();
    // let template_field_view = move |field: RGSOConcrete<TemplateField, Schema>| {
    let template_field_view = move |field: GetNameFieldVariantTraitObject| {
        let ctx_clone = ctx_clone.clone();
        let ctx_clone_2 = ctx_clone.clone();
        let field_clone = field.clone();
        let field_clone_inner = field_clone.clone();
        let name_setter = move |new_val: String| {
            match field_clone_inner.clone() {
                GetNameFieldVariantTraitObject::StringTemplateField(field) => field
                    .edit(ctx_clone_2.clone())
                    .set_name(new_val)
                    .execute()
                    .unwrap(),
                GetNameFieldVariantTraitObject::BoolTemplateField(field) => field
                    .edit(ctx_clone_2.clone())
                    .set_name(new_val)
                    .execute()
                    .unwrap(),
                GetNameFieldVariantTraitObject::IntTemplateField(field) => field
                    .edit(ctx_clone_2.clone())
                    .set_name(new_val)
                    .execute()
                    .unwrap(),
            };
        };
        let field_clone_inner = field_clone.clone();
        let extraneous_value_type_signal =
            RwSignal::<GetNameFieldVariantTraitObjectDiscriminants>::new(field.clone().into());
        let ctx_clone_2 = ctx_clone.clone();
        let template_clone = template_clone.clone();
        let on_change_field = Callback::new(
            move |new_value: GetNameFieldVariantTraitObjectDiscriminants| {
                let field_clone_inner = field_clone_inner.clone();
                // let ctx_clone = ctx_clone.clone();
                // leptos::logging::log!(
                //     "running effect\nthe_signal: {:?}\n the_graph: {:?}",
                //     extraneous_value_type_signal.clone().get(),
                //     field_clone.get_fieldvariant_slot()
                // );
                match field_clone_inner {
                    GetNameFieldVariantTraitObject::StringTemplateField(item) => {
                        let mut edit = item.edit(ctx_clone_2.clone()).delete();
                        match new_value {
                            GetNameFieldVariantTraitObjectDiscriminants::StringTemplateField => {
                                edit.incorporate(
                                    template_clone
                                        .edit(ctx_clone_2.clone())
                                        .add_new_fields::<StringTemplateField, _>(|new_field| {
                                            new_field.set_name(item.get_name())
                                        }),
                                )
                            }
                            GetNameFieldVariantTraitObjectDiscriminants::BoolTemplateField => edit
                                .incorporate(
                                    template_clone
                                        .edit(ctx_clone_2.clone())
                                        .add_new_fields::<BoolTemplateField, _>(|new_field| {
                                            new_field.set_name(item.get_name())
                                        }),
                                ),
                            GetNameFieldVariantTraitObjectDiscriminants::IntTemplateField => edit
                                .incorporate(
                                    template_clone
                                        .edit(ctx_clone_2.clone())
                                        .add_new_fields::<IntTemplateField, _>(|new_field| {
                                            new_field.set_name(item.get_name())
                                        }),
                                ),
                        };
                        edit.execute().unwrap();
                    }
                    GetNameFieldVariantTraitObject::BoolTemplateField(item) => {
                        let mut edit = item.edit(ctx_clone_2.clone()).delete();
                        match new_value {
                            GetNameFieldVariantTraitObjectDiscriminants::StringTemplateField => {
                                edit.incorporate(
                                    template_clone
                                        .edit(ctx_clone_2.clone())
                                        .add_new_fields::<StringTemplateField, _>(|new_field| {
                                            new_field.set_name(item.get_name())
                                        }),
                                )
                            }
                            GetNameFieldVariantTraitObjectDiscriminants::BoolTemplateField => edit
                                .incorporate(
                                    template_clone
                                        .edit(ctx_clone_2.clone())
                                        .add_new_fields::<BoolTemplateField, _>(|new_field| {
                                            new_field.set_name(item.get_name())
                                        }),
                                ),
                            GetNameFieldVariantTraitObjectDiscriminants::IntTemplateField => edit
                                .incorporate(
                                    template_clone
                                        .edit(ctx_clone_2.clone())
                                        .add_new_fields::<IntTemplateField, _>(|new_field| {
                                            new_field.set_name(item.get_name())
                                        }),
                                ),
                        };
                        edit.execute().unwrap();
                    }
                    GetNameFieldVariantTraitObject::IntTemplateField(item) => {
                        let mut edit = item.edit(ctx_clone_2.clone()).delete();
                        match new_value {
                            GetNameFieldVariantTraitObjectDiscriminants::StringTemplateField => {
                                edit.incorporate(
                                    template_clone
                                        .edit(ctx_clone_2.clone())
                                        .add_new_fields::<StringTemplateField, _>(|new_field| {
                                            new_field.set_name(item.get_name())
                                        }),
                                )
                            }
                            GetNameFieldVariantTraitObjectDiscriminants::BoolTemplateField => edit
                                .incorporate(
                                    template_clone
                                        .edit(ctx_clone_2.clone())
                                        .add_new_fields::<BoolTemplateField, _>(|new_field| {
                                            new_field.set_name(item.get_name())
                                        }),
                                ),
                            GetNameFieldVariantTraitObjectDiscriminants::IntTemplateField => edit
                                .incorporate(
                                    template_clone
                                        .edit(ctx_clone_2.clone())
                                        .add_new_fields::<IntTemplateField, _>(|new_field| {
                                            new_field.set_name(item.get_name())
                                        }),
                                ),
                        };
                        edit.execute().unwrap();
                    }
                };
            },
        );

        let field_clone_inner = field.clone();
        let delete_field = move |_| match field_clone_inner.clone() {
            GetNameFieldVariantTraitObject::StringTemplateField(inner) => {
                inner.edit(ctx_clone.clone()).delete().execute().unwrap();
            }
            GetNameFieldVariantTraitObject::BoolTemplateField(inner) => {
                inner.edit(ctx_clone.clone()).delete().execute().unwrap();
            }
            GetNameFieldVariantTraitObject::IntTemplateField(inner) => {
                inner.edit(ctx_clone.clone()).delete().execute().unwrap();
            }
        };
        let field_clone_inner = field_clone.clone();
        view! {
            <LeafSection>
                <div class="flex">
                <div class="flex-grow">
                    <LeafSectionHeader>
                    <ToggleManagedTextInput getter=move || field.get_name() setter=name_setter/>
                    </LeafSectionHeader>
                    <LeafSection attr:class="leafsection dependent">
                    <ManagedEnumSelect getter=move || field_clone_inner.clone().into() setter=on_change_field/>
                    </LeafSection>
                </div>
                <div class="align-right">
                    <Button on:click=delete_field>Delete Field</Button>
                </div>
                </div>

            </LeafSection>
        }
    };

    let template_clone = template.clone();
    let template_clone_2 = template.clone();
    let template_clone_3 = template.clone();
    let ctx_clone = ctx.clone();
    view! {
        <div>
            <Section>
                <SectionHeader slot>Overview</SectionHeader>
                <SubSection>
                    <SubSectionHeader>
                        Name:
                    </SubSectionHeader>
                    <ToggleManagedTextInput getter=move || template.get_name_field() setter=update_name />
                </SubSection>
                <SubSection>
                    <Button on:click=delete_template_recursive>Delete Item</Button>
                </SubSection>
            </Section>

            <Section>
                <SectionHeader slot>Create Derivatives</SectionHeader>
                <SignalTextInput value=derivative_operative_name/><Button on:click=create_derivative_operative>Create derivative operative</Button>
            </Section>

            <Section>
                <SectionHeader slot>Fields</SectionHeader>
                <Button on:click=on_click_add_field>Add Field</Button>
                <SubSection>
                    <For each=move||template_clone_3.get_fields_slot() key=|item| item.get_id().clone() children=template_field_view />
                </SubSection>
            </Section>

            <Section>
                <SectionHeader slot>Slots</SectionHeader>
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
