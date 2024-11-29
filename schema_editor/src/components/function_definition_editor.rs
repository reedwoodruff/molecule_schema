use super::common::*;
use leptos::either::either;
use schema_editor_generated_toolkit::prelude::*;

use crate::components::workspace::{WorkspaceState, WorkspaceTab};

#[component]
pub fn FunctionDefinitionEditor(fn_def: RGSOConcrete<FunctionDefinition, Schema>) -> impl IntoView {
    let ctx = use_context::<SharedGraph<Schema>>().unwrap();
    let WorkspaceState {
        schema,
        selected_tab,
    } = use_context::<WorkspaceState>().unwrap();
    let ctx_clone = ctx.clone();

    let fn_def_clone = fn_def.clone();
    let update_name = move |new_val: String| {
        let mut editor = fn_def_clone.edit(ctx_clone.clone());
        editor.set_name(new_val).execute().unwrap();
    };

    let fn_def_clone = fn_def.clone();
    let selected_tab_clone = selected_tab.clone();
    let ctx_clone = ctx.clone();
    let delete_fn_def_recursive = move |_| {
        let ctx_clone = ctx_clone.clone();
        fn_def_clone
            .edit(ctx_clone)
            .delete_recursive()
            .execute()
            .unwrap();
        selected_tab_clone.set(WorkspaceTab::Function(RwSignal::new(None)))
    };
    let is_adding_input = RwSignal::new(false);
    let input_select_value =
        RwSignal::new(GetNameFunctionIOTraitObjectDiscriminants::FunctionIOOperative);
    let input_name = RwSignal::new("new_input".to_string());
    let input_selected_operative =
        RwSignal::<Option<RGSOConcrete<OperativeConcrete, Schema>>>::new(None);
    let ctx_clone = ctx.clone();
    let fn_def_clone = fn_def.clone();
    let on_click_add_input = move |_| {
        match input_select_value.get() {
            GetNameFunctionIOTraitObjectDiscriminants::FunctionIOOperative => {
                if let Some(op) = input_selected_operative.get() {
                    fn_def_clone
                        .edit(ctx_clone.clone())
                        .add_new_inputs::<FunctionIOOperative, _>(|new| {
                            new.set_name(input_name.get())
                                .add_existing_value(op.get_id(), |na| na)
                        })
                        .execute()
                        .unwrap();
                }
            }
            GetNameFunctionIOTraitObjectDiscriminants::FunctionIOPrimitiveBool => {
                fn_def_clone
                    .edit(ctx_clone.clone())
                    .add_new_inputs::<FunctionIOPrimitiveBool, _>(|new| {
                        new.set_name(input_name.get())
                    })
                    .execute()
                    .unwrap();
            }
            GetNameFunctionIOTraitObjectDiscriminants::FunctionIOPrimitiveInt => {
                fn_def_clone
                    .edit(ctx_clone.clone())
                    .add_new_inputs::<FunctionIOPrimitiveInt, _>(|new| {
                        new.set_name(input_name.get())
                    })
                    .execute()
                    .unwrap();
            }
            GetNameFunctionIOTraitObjectDiscriminants::FunctionIOPrimitiveString => {
                fn_def_clone
                    .edit(ctx_clone.clone())
                    .add_new_inputs::<FunctionIOPrimitiveString, _>(|new| {
                        new.set_name(input_name.get())
                    })
                    .execute()
                    .unwrap();
            }
        };
        is_adding_input.set(false);
    };

    let is_adding_output = RwSignal::new(false);
    let output_select_value =
        RwSignal::new(GetNameFunctionIOTraitObjectDiscriminants::FunctionIOOperative);
    let output_name = RwSignal::new("new_output".to_string());
    let output_selected_operative =
        RwSignal::<Option<RGSOConcrete<OperativeConcrete, Schema>>>::new(None);
    let ctx_clone = ctx.clone();
    let fn_def_clone = fn_def.clone();
    let on_click_add_output = move |_| {
        match output_select_value.get() {
            GetNameFunctionIOTraitObjectDiscriminants::FunctionIOOperative => {
                if let Some(op) = output_selected_operative.get() {
                    fn_def_clone
                        .edit(ctx_clone.clone())
                        .add_new_outputs::<FunctionIOOperative, _>(|new| {
                            new.set_name(output_name.get())
                                .add_existing_value(op.get_id(), |na| na)
                        })
                        .execute()
                        .unwrap();
                }
            }
            GetNameFunctionIOTraitObjectDiscriminants::FunctionIOPrimitiveBool => {
                fn_def_clone
                    .edit(ctx_clone.clone())
                    .add_new_outputs::<FunctionIOPrimitiveBool, _>(|new| {
                        new.set_name(output_name.get())
                    })
                    .execute()
                    .unwrap();
            }
            GetNameFunctionIOTraitObjectDiscriminants::FunctionIOPrimitiveInt => {
                fn_def_clone
                    .edit(ctx_clone.clone())
                    .add_new_outputs::<FunctionIOPrimitiveInt, _>(|new| {
                        new.set_name(output_name.get())
                    })
                    .execute()
                    .unwrap();
            }
            GetNameFunctionIOTraitObjectDiscriminants::FunctionIOPrimitiveString => {
                fn_def_clone
                    .edit(ctx_clone.clone())
                    .add_new_outputs::<FunctionIOPrimitiveString, _>(|new| {
                        new.set_name(output_name.get())
                    })
                    .execute()
                    .unwrap();
            }
        };
        is_adding_output.set(false);
    };

    let ctx_clone = ctx.clone();
    let schema_clone = schema.clone();
    let schema_clone_2 = schema.clone();
    let fn_def_clone = fn_def.clone();
    let fn_def_clone_2 = fn_def.clone();
    let fn_def_clone_3 = fn_def.clone();
    let ctx_clone_2 = ctx.clone();
    view! {
        <div>
            <Section>
                <SectionHeader slot>Overview</SectionHeader>
                <SubSection>
                    <SubSectionHeader>
                        Name:
                    </SubSectionHeader>
                    <ToggleManagedTextInput getter=move || fn_def_clone.get_name_field() setter=update_name />
                </SubSection>
                <SubSection>
                    <Button on:click=delete_fn_def_recursive>Delete Item</Button>
                </SubSection>
            </Section>

            <Section>
                <SectionHeader slot>Inputs</SectionHeader>
                <Show when=move || !is_adding_input.get()>
                    <Button on:click=move |_| is_adding_input.set(true)>Add New Input</Button>
                </Show>
                <Show when=move ||is_adding_input.get()>
                {
                    let schema_clone = schema_clone.clone();
                    let on_click_add_input = on_click_add_input.clone();
                    view!{
                    <SubSection>
                        <SubSectionHeader>
                        Add New Input
                        </SubSectionHeader>
                        <LeafSection>
                            <LeafSectionHeader>
                            Input Name
                            </LeafSectionHeader>
                            <LeafSection>
                                <SignalTextInput value=input_name />
                            </LeafSection>
                        </LeafSection>
                                <LeafSection>
                            <LeafSectionHeader>
                                Input to add:
                            </LeafSectionHeader>
                            <LeafSection>
                                <SignalEnumSelect value=input_select_value/>
                            </LeafSection>
                        </LeafSection>

                        <DetailSelectionView selected_variant=input_select_value schema=schema_clone.get().clone() selected_op=input_selected_operative/>

                        <LeafSection>
                            <Button on:click=on_click_add_input>Add Input</Button>
                            <Button on:click=move |_| is_adding_input.set(false)>Cancel</Button>
                        </LeafSection>
                    </SubSection>
                    }}
                </Show>
                <SubSection>
                    <SubSectionHeader>Current Inputs</SubSectionHeader>
                    <For each=move || fn_def_clone_2.get_inputs_slot() key=|item| item.get_id().clone() children=move |input| {
                        let ctx_clone = ctx_clone.clone();
                        let input_clone = input.clone();
                        let on_click_delete = move |_| {
                            let input = input_clone.clone();
                            match input {
                                GetNameFunctionIOTraitObject::FunctionIOOperative(item) => item.edit(ctx_clone.clone()).delete().execute().unwrap(),
                                GetNameFunctionIOTraitObject::FunctionIOPrimitiveBool(item) => item.edit(ctx_clone.clone()).delete().execute().unwrap(),
                                GetNameFunctionIOTraitObject::FunctionIOPrimitiveInt(item) => item.edit(ctx_clone.clone()).delete().execute().unwrap(),
                                GetNameFunctionIOTraitObject::FunctionIOPrimitiveString(item) => item.edit(ctx_clone.clone()).delete().execute().unwrap(),
                            };
                        };
                        let type_view: GetNameFunctionIOTraitObjectDiscriminants = input.clone().into();
                        view!{
                            <LeafSection>
                            <LeafSectionHeader>
                            {move || input.get_name()}
                            </LeafSectionHeader>
                            <LeafSection>
                                {move || type_view.to_string()}
                            </LeafSection>
                            <LeafSection>
                            <Button on:click=on_click_delete>Delete</Button>
                            </LeafSection>
                            </LeafSection>
                        }
                    }/>
                </SubSection>
            </Section>

            <Section>
                <SectionHeader slot>Outputs</SectionHeader>
                <Show when=move || !is_adding_output.get()>
                    <Button on:click=move |_| is_adding_output.set(true)>Add New Output</Button>
                </Show>
                <Show when=move ||is_adding_output.get()>
                {
                    let schema_clone = schema_clone_2.clone();
                    let on_click_add_output = on_click_add_output.clone();
                    view!{
                    <SubSection>
                        <SubSectionHeader>
                        Add New Output
                        </SubSectionHeader>
                        <LeafSection>
                            <LeafSectionHeader>
                            Output Name
                            </LeafSectionHeader>
                            <LeafSection>
                                <SignalTextInput value=output_name />
                            </LeafSection>
                        </LeafSection>
                                <LeafSection>
                            <LeafSectionHeader>
                                Output to add:
                            </LeafSectionHeader>
                            <LeafSection>
                                <SignalEnumSelect value=output_select_value/>
                            </LeafSection>
                        </LeafSection>

                        <DetailSelectionView selected_variant=output_select_value schema=schema_clone.get().clone() selected_op=output_selected_operative/>

                        <LeafSection>
                            <Button on:click=on_click_add_output>Add Output</Button>
                            <Button on:click=move |_| is_adding_output.set(false)>Cancel</Button>
                        </LeafSection>
                    </SubSection>
                    }}
                </Show>
                <SubSection>
                    <SubSectionHeader>Current Outputs</SubSectionHeader>
                    <For each=move || fn_def_clone_3.get_outputs_slot() key=|item| item.get_id().clone() children=move |output| {
                        let ctx_clone = ctx_clone_2.clone();
                        let output_clone = output.clone();
                        let on_click_delete = move |_| {
                            let output = output_clone.clone();
                            match output {
                                GetNameFunctionIOTraitObject::FunctionIOOperative(item) => item.edit(ctx_clone.clone()).delete().execute().unwrap(),
                                GetNameFunctionIOTraitObject::FunctionIOPrimitiveBool(item) => item.edit(ctx_clone.clone()).delete().execute().unwrap(),
                                GetNameFunctionIOTraitObject::FunctionIOPrimitiveInt(item) => item.edit(ctx_clone.clone()).delete().execute().unwrap(),
                                GetNameFunctionIOTraitObject::FunctionIOPrimitiveString(item) => item.edit(ctx_clone.clone()).delete().execute().unwrap(),
                            };
                        };
                        let type_view: GetNameFunctionIOTraitObjectDiscriminants = output.clone().into();
                        view!{
                            <LeafSection>
                            <LeafSectionHeader>
                            {move || output.get_name()}
                            </LeafSectionHeader>
                            <LeafSection>
                                {move || type_view.to_string()}
                            </LeafSection>
                            <LeafSection>
                            <Button on:click=on_click_delete>Delete</Button>
                            </LeafSection>
                            </LeafSection>
                        }
                    }/>
                </SubSection>
            </Section>
        </div>

    }
}

#[component]
fn DetailSelectionView(
    selected_variant: RwSignal<GetNameFunctionIOTraitObjectDiscriminants>,
    schema: RGSOConcrete<SchemaConcrete, Schema>,
    selected_op: RwSignal<Option<RGSOConcrete<OperativeConcrete, Schema>>>,
) -> impl IntoView {
    move || {
        let schema = schema.clone();
        either!(selected_variant.get(),
            GetNameFunctionIOTraitObjectDiscriminants::FunctionIOOperative => view!{
                <LeafSection>
                <LeafSectionHeader>
                    Selected Operative:
                </LeafSectionHeader>
                <LeafSection>
                <SignalSelectRGSOWithOptions value=selected_op options=Signal::derive(move || schema.get_operatives_slot()) />
                </LeafSection>
                </LeafSection>
            },
            GetNameFunctionIOTraitObjectDiscriminants::FunctionIOPrimitiveBool => (),
            GetNameFunctionIOTraitObjectDiscriminants::FunctionIOPrimitiveInt => (),
            GetNameFunctionIOTraitObjectDiscriminants::FunctionIOPrimitiveString => (),
        )
    }
}
