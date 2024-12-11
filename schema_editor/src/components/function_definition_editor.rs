use super::common::*;
use leptos::either::{either, EitherOf13, EitherOf3};
use schema_editor_generated_toolkit::prelude::*;
use web_sys::MouseEvent;

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
        RwSignal::new(GetNameFunctionIOTraitObjectDiscriminants::FunctionIOSingleOperative);
    let input_name = RwSignal::new("new_input".to_string());
    let input_selected_operative =
        RwSignal::<Option<RGSOConcrete<OperativeConcrete, Schema>>>::new(None);
    let input_selected_operative_list =
        RwSignal::<Vec<RGSOConcrete<OperativeConcrete, Schema>>>::new(vec![]);
    let input_selected_trait_list =
        RwSignal::<Vec<RGSOConcrete<TraitConcrete, Schema>>>::new(vec![]);
    let ctx_clone = ctx.clone();
    let fn_def_clone = fn_def.clone();

    let is_adding_output = RwSignal::new(false);
    let output_select_value =
        RwSignal::new(GetNameFunctionIOTraitObjectDiscriminants::FunctionIOSingleOperative);
    let output_name = RwSignal::new("new_output".to_string());
    let output_selected_operative =
        RwSignal::<Option<RGSOConcrete<OperativeConcrete, Schema>>>::new(None);
    let output_selected_operative_list =
        RwSignal::<Vec<RGSOConcrete<OperativeConcrete, Schema>>>::new(vec![]);
    let output_selected_trait_list =
        RwSignal::<Vec<RGSOConcrete<TraitConcrete, Schema>>>::new(vec![]);
    let ctx_clone = ctx.clone();
    let fn_def_clone = fn_def.clone();
    let get_on_click_add_input_or_output =
        move |select_value: RwSignal<GetNameFunctionIOTraitObjectDiscriminants>,
              is_adding_signal: RwSignal<bool>,
              name_signal: RwSignal<String>,
              selected_op: RwSignal<Option<RGSOConcrete<OperativeConcrete, Schema>>>,
              selected_op_list: RwSignal<Vec<RGSOConcrete<OperativeConcrete, Schema>>>,
              selected_trait_list: RwSignal<Vec<RGSOConcrete<TraitConcrete, Schema>>>,
              is_input: bool| {
            move |_| {
                let mut editor = fn_def_clone.edit(ctx_clone.clone());
                match select_value.get() {
                    GetNameFunctionIOTraitObjectDiscriminants::FunctionIOSingleOperative => {
                        if let Some(op) = selected_op.get() {
                            let input_or_output = FunctionIOSingleOperative::new(ctx_clone.clone()).set_temp_id("new_input_or_output")
                                .set_name(name_signal.get())
                                .add_existing_allowedoperative(op.get_id(), |na| na);
                            editor.incorporate(&input_or_output);
                            if is_input {
                                editor
                                    .add_temp_inputs::<FunctionIOSingleOperative>("new_input_or_output");
                            } else {
                                editor.add_temp_outputs::<FunctionIOSingleOperative>("new_input_or_output");
                            }
                        }
                    }
                    GetNameFunctionIOTraitObjectDiscriminants::FunctionIOPrimitiveBool => {
                        let input_or_output = FunctionIOPrimitiveBool::new(ctx_clone.clone()).set_temp_id("new_input_or_output")
                            .set_name(name_signal.get());
                        editor.incorporate(&input_or_output);
                        if is_input {
                            editor
                                .add_temp_inputs::<FunctionIOPrimitiveBool>("new_input_or_output");
                        } else {
                            editor.add_temp_outputs::<FunctionIOPrimitiveBool>("new_input_or_output");
                        }
                    }
                    GetNameFunctionIOTraitObjectDiscriminants::FunctionIOPrimitiveInt => {
                        let input_or_output = FunctionIOPrimitiveInt::new(ctx_clone.clone()).set_temp_id("new_input_or_output")
                            .set_name(name_signal.get());
                        editor.incorporate(&input_or_output);
                        if is_input {
                            editor
                                .add_temp_inputs::<FunctionIOPrimitiveInt>("new_input_or_output");
                        } else {
                            editor.add_temp_outputs::<FunctionIOPrimitiveInt>("new_input_or_output");
                        }
                    }
                    GetNameFunctionIOTraitObjectDiscriminants::FunctionIOPrimitiveString => {
                        let input_or_output = FunctionIOPrimitiveString::new(ctx_clone.clone()).set_temp_id("new_input_or_output")
                            .set_name(name_signal.get());
                        editor.incorporate(&input_or_output);
                        if is_input {
                            editor
                                .add_temp_inputs::<FunctionIOPrimitiveString>("new_input_or_output");
                        } else {
                            editor.add_temp_outputs::<FunctionIOPrimitiveString>("new_input_or_output");
                        }
                    }
                    GetNameFunctionIOTraitObjectDiscriminants::FunctionIOCollectionPrimitiveBool => {
                        let input_or_output = FunctionIOCollectionPrimitiveBool::new(ctx_clone.clone()).set_temp_id("new_input_or_output")
                            .set_name(name_signal.get());
                        editor.incorporate(&input_or_output);
                        if is_input {
                            editor
                                .add_temp_inputs::<FunctionIOCollectionPrimitiveBool>("new_input_or_output");
                        } else {
                            editor.add_temp_outputs::<FunctionIOCollectionPrimitiveBool>("new_input_or_output");
                        }
                    }
                    GetNameFunctionIOTraitObjectDiscriminants::FunctionIOCollectionMultiOperative => {
                        let input_editor = FunctionIOCollectionMultiOperative::new(ctx_clone.clone())
                            .set_temp_id("new_input_or_output")
                            .set_name(name_signal.get());
                        editor.incorporate(&input_editor);
                        selected_op_list
                            .get()
                            .into_iter()
                            .for_each(|op_item| {
                                editor.incorporate(
                                    &input_editor
                                        .clone()
                                        .add_existing_allowedoperatives(op_item.get_id(), |na| na),
                                )
                            });

                        if is_input {
                        editor
                            .add_temp_inputs::<FunctionIOCollectionMultiOperative>("new_input_or_output");
                        } else {
                            editor
                                .add_temp_outputs::<FunctionIOCollectionMultiOperative>("new_input_or_output");
                        }
                    }
                    GetNameFunctionIOTraitObjectDiscriminants::FunctionIOCollectionTraitOperative => {
                        let input_editor = FunctionIOCollectionTraitOperative::new(ctx_clone.clone())
                            .set_temp_id("new_input_or_output")
                            .set_name(name_signal.get());
                        editor.incorporate(&input_editor);
                        selected_trait_list
                            .get()
                            .into_iter()
                            .for_each(|trait_item| {
                                editor.incorporate(
                                    &input_editor
                                        .clone()
                                        .add_existing_requiredtraits(trait_item.get_id(), |na| na),
                                )
                            });

                        if is_input {
                        editor
                            .add_temp_inputs::<FunctionIOCollectionTraitOperative>("new_input_or_output");
                        } else {
                            editor
                                .add_temp_outputs::<FunctionIOCollectionTraitOperative>("new_input_or_output");

                        }
                    }
                    GetNameFunctionIOTraitObjectDiscriminants::FunctionIOCollectionPrimitiveString => {
                        let input_or_output = FunctionIOCollectionPrimitiveString::new(ctx_clone.clone()).set_temp_id("new_input_or_output")
                            .set_name(name_signal.get());
                        editor.incorporate(&input_or_output);
                        if is_input {
                            editor
                                .add_temp_inputs::<FunctionIOCollectionPrimitiveString>("new_input_or_output");
                        } else {
                            editor.add_temp_outputs::<FunctionIOCollectionPrimitiveString>("new_input_or_output");
                        }
                    }
                    GetNameFunctionIOTraitObjectDiscriminants::FunctionIOTraitOperative => {
                        let input_editor = FunctionIOTraitOperative::new(ctx_clone.clone())
                            .set_temp_id("new_input_or_output")
                            .set_name(name_signal.get());
                        editor.incorporate(&input_editor);
                        selected_trait_list
                            .get()
                            .into_iter()
                            .for_each(|trait_item| {
                                editor.incorporate(
                                    &input_editor
                                        .clone()
                                        .add_existing_requiredtraits(trait_item.get_id(), |na| na),
                                )
                            });

                        if is_input {
                        editor
                            .add_temp_inputs::<FunctionIOTraitOperative>("new_input_or_output");
                        } else {
                            editor
                                .add_temp_outputs::<FunctionIOTraitOperative>("new_input_or_output");
                        }
                    }
                    GetNameFunctionIOTraitObjectDiscriminants::FunctionIOCollectionSingleOperative => {
                        if let Some(op) = selected_op.get() {
                            let input_or_output = FunctionIOCollectionSingleOperative::new(ctx_clone.clone()).set_temp_id("new_input_or_output")
                                .add_existing_allowedoperative(op.get_id(), |na| na)
                                .set_name(name_signal.get());
                            editor.incorporate(&input_or_output);
                            if is_input {
                                editor
                                    .add_temp_inputs::<FunctionIOCollectionSingleOperative>("new_input_or_output");
                            } else {
                                editor.add_temp_outputs::<FunctionIOCollectionSingleOperative>("new_input_or_output");
                            }
                        }
                    }
                    GetNameFunctionIOTraitObjectDiscriminants::FunctionIOMultiOperative => {
                        let input_editor = FunctionIOMultiOperative::new(ctx_clone.clone())
                            .set_temp_id("new_input_or_output")
                            .set_name(name_signal.get());
                        editor.incorporate(&input_editor);
                        selected_op_list
                            .get()
                            .into_iter()
                            .for_each(|op_item| {
                                editor.incorporate(
                                    &input_editor
                                        .clone()
                                        .add_existing_allowedoperatives(op_item.get_id(), |na| na),
                                )
                            });

                        if is_input {
                            editor
                                .add_temp_inputs::<FunctionIOMultiOperative>("new_input_or_output");
                        } else {
                            editor
                                .add_temp_outputs::<FunctionIOMultiOperative>("new_input_or_output");

                        }
                    }
                    GetNameFunctionIOTraitObjectDiscriminants::FunctionIOCollectionPrimitiveInt => {
                        let input_or_output = FunctionIOCollectionPrimitiveInt::new(ctx_clone.clone()).set_temp_id("new_input_or_output")
                            .set_name(name_signal.get());
                        editor.incorporate(&input_or_output);
                        if is_input {
                            editor
                                .add_temp_inputs::<FunctionIOCollectionPrimitiveInt>("new_input_or_output");
                        } else {
                            editor.add_temp_outputs::<FunctionIOCollectionPrimitiveInt>("new_input_or_output");
                        }
                    }
                    GetNameFunctionIOTraitObjectDiscriminants::FunctionIOSelf => {
                        let input_or_output = FunctionIOSelf::new(ctx_clone.clone()).set_temp_id("new_input_or_output")
                            .set_name(name_signal.get());
                        editor.incorporate(&input_or_output);
                        if is_input {
                            editor
                                .add_temp_inputs::<FunctionIOSelf>("new_input_or_output");
                        } else {
                            editor.add_temp_outputs::<FunctionIOSelf>("new_input_or_output");
                        }
                    }
                };
                editor.execute().unwrap();
                is_adding_signal.set(false);
            }
        };

    let get_on_click_add_input_or_output_clone = get_on_click_add_input_or_output.clone();
    let ctx_clone = ctx.clone();
    let schema_clone = schema.clone();
    let schema_clone_2 = schema.clone();
    let fn_def_clone = fn_def.clone();
    let fn_def_clone_2 = fn_def.clone();
    let fn_def_clone_3 = fn_def.clone();
    let ctx_clone_2 = ctx.clone();
    let ctx_clone_3 = ctx.clone();

    let is_save_disabled =
        move |selected_variant: RwSignal<GetNameFunctionIOTraitObjectDiscriminants>,
              selected_op: RwSignal<Option<RGSOConcrete<OperativeConcrete, Schema>>>,
              selected_op_list: RwSignal<Vec<RGSOConcrete<OperativeConcrete, Schema>>>,
              selected_trait_list: RwSignal<Vec<RGSOConcrete<TraitConcrete, Schema>>>| {
            move || match selected_variant.get() {
                GetNameFunctionIOTraitObjectDiscriminants::FunctionIOCollectionPrimitiveBool => {
                    false
                }
                GetNameFunctionIOTraitObjectDiscriminants::FunctionIOSingleOperative => {
                    selected_op.get().is_none()
                }
                GetNameFunctionIOTraitObjectDiscriminants::FunctionIOPrimitiveBool => false,
                GetNameFunctionIOTraitObjectDiscriminants::FunctionIOCollectionMultiOperative => {
                    selected_op_list.get().len() < 2
                }
                GetNameFunctionIOTraitObjectDiscriminants::FunctionIOCollectionTraitOperative => {
                    selected_trait_list.get().len() < 1
                }
                GetNameFunctionIOTraitObjectDiscriminants::FunctionIOCollectionPrimitiveString => {
                    false
                }
                GetNameFunctionIOTraitObjectDiscriminants::FunctionIOTraitOperative => {
                    selected_trait_list.get().len() < 1
                }
                GetNameFunctionIOTraitObjectDiscriminants::FunctionIOSelf => false,
                GetNameFunctionIOTraitObjectDiscriminants::FunctionIOCollectionSingleOperative => {
                    selected_op.get().is_none()
                }
                GetNameFunctionIOTraitObjectDiscriminants::FunctionIOPrimitiveInt => false,
                GetNameFunctionIOTraitObjectDiscriminants::FunctionIOMultiOperative => {
                    selected_op_list.get().len() < 2
                }
                GetNameFunctionIOTraitObjectDiscriminants::FunctionIOPrimitiveString => false,
                GetNameFunctionIOTraitObjectDiscriminants::FunctionIOCollectionPrimitiveInt => {
                    false
                }
            }
        };

    let get_on_delete_input_or_output = move |input: GetNameFunctionIOTraitObject| {
        move |_e: MouseEvent| {
            match input.clone() {
                GetNameFunctionIOTraitObject::FunctionIOSingleOperative(item) => item
                    .edit(ctx_clone.clone())
                    .delete_recursive()
                    .execute()
                    .unwrap(),
                GetNameFunctionIOTraitObject::FunctionIOPrimitiveBool(item) => item
                    .edit(ctx_clone.clone())
                    .delete_recursive()
                    .execute()
                    .unwrap(),
                GetNameFunctionIOTraitObject::FunctionIOPrimitiveInt(item) => item
                    .edit(ctx_clone.clone())
                    .delete_recursive()
                    .execute()
                    .unwrap(),
                GetNameFunctionIOTraitObject::FunctionIOPrimitiveString(item) => item
                    .edit(ctx_clone.clone())
                    .delete_recursive()
                    .execute()
                    .unwrap(),
                GetNameFunctionIOTraitObject::FunctionIOCollectionPrimitiveBool(item) => item
                    .edit(ctx_clone.clone())
                    .delete_recursive()
                    .execute()
                    .unwrap(),
                GetNameFunctionIOTraitObject::FunctionIOCollectionMultiOperative(item) => item
                    .edit(ctx_clone.clone())
                    .delete_recursive()
                    .execute()
                    .unwrap(),
                GetNameFunctionIOTraitObject::FunctionIOCollectionTraitOperative(item) => item
                    .edit(ctx_clone.clone())
                    .delete_recursive()
                    .execute()
                    .unwrap(),
                GetNameFunctionIOTraitObject::FunctionIOCollectionPrimitiveString(item) => item
                    .edit(ctx_clone.clone())
                    .delete_recursive()
                    .execute()
                    .unwrap(),
                GetNameFunctionIOTraitObject::FunctionIOTraitOperative(item) => item
                    .edit(ctx_clone.clone())
                    .delete_recursive()
                    .execute()
                    .unwrap(),
                GetNameFunctionIOTraitObject::FunctionIOCollectionSingleOperative(item) => item
                    .edit(ctx_clone.clone())
                    .delete_recursive()
                    .execute()
                    .unwrap(),
                GetNameFunctionIOTraitObject::FunctionIOMultiOperative(item) => item
                    .edit(ctx_clone.clone())
                    .delete_recursive()
                    .execute()
                    .unwrap(),
                GetNameFunctionIOTraitObject::FunctionIOCollectionPrimitiveInt(item) => item
                    .edit(ctx_clone.clone())
                    .delete_recursive()
                    .execute()
                    .unwrap(),
                GetNameFunctionIOTraitObject::FunctionIOSelf(item) => item
                    .edit(ctx_clone.clone())
                    .delete_recursive()
                    .execute()
                    .unwrap(),
            };
        }
    };
    let get_on_delete_input_or_output_clone = get_on_delete_input_or_output.clone();

    view! {
        <div>
            <Section>
                <SectionHeader slot>Overview</SectionHeader>
                <SubSection>
                    <SubSectionHeader>Name:</SubSectionHeader>
                    <ToggleManagedTextInput
                        getter=move || fn_def_clone.get_name_field()
                        setter=update_name
                    />
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
                <Show when=move || {
                    is_adding_input.get()
                }>
                    {
                        let schema_clone = schema_clone.clone();
                        let on_click_add_input = get_on_click_add_input_or_output_clone
                            .clone()(
                            input_select_value,
                            is_adding_input,
                            input_name,
                            input_selected_operative,
                            input_selected_operative_list,
                            input_selected_trait_list,
                            true,
                        );
                        view! {
                            <SubSection>
                                <SubSectionHeader>Add New Input</SubSectionHeader>
                                <LeafSection>
                                    <LeafSectionHeader>Input Name</LeafSectionHeader>
                                    <LeafSection>
                                        <SignalTextInput value=input_name />
                                    </LeafSection>
                                </LeafSection>
                                <LeafSection>
                                    <LeafSectionHeader>Input to add:</LeafSectionHeader>
                                    <LeafSection>
                                        <SignalEnumSelect value=input_select_value />
                                    </LeafSection>
                                </LeafSection>

                                <DetailSelectionView
                                    selected_variant=input_select_value
                                    schema=schema_clone.get().clone()
                                    selected_op=input_selected_operative
                                    selected_op_list=input_selected_operative_list
                                    selected_trait_list=input_selected_trait_list
                                />

                                <LeafSection>
                                    <Button
                                        on:click=on_click_add_input
                                        prop:disabled=move || {
                                            is_save_disabled(
                                                input_select_value,
                                                input_selected_operative,
                                                input_selected_operative_list,
                                                input_selected_trait_list,
                                            )
                                        }
                                    >
                                        Add Input
                                    </Button>
                                    <Button on:click=move |_| {
                                        is_adding_input.set(false)
                                    }>Cancel</Button>
                                </LeafSection>
                            </SubSection>
                        }
                    }
                </Show>
                <SubSection>
                    <SubSectionHeader>Current Inputs</SubSectionHeader>
                    <For
                        each=move || fn_def_clone_2.get_inputs_slot()
                        key=|item| item.get_id().clone()
                        children=move |input| {
                            let ctx_clone = ctx_clone_2.clone();
                            let input_clone = input.clone();
                            let on_click_delete = get_on_delete_input_or_output
                                .clone()(input.clone());
                            let type_view: GetNameFunctionIOTraitObjectDiscriminants = input
                                .clone()
                                .into();
                            view! {
                                <LeafSection>
                                    <LeafSectionHeader>
                                        {move || input.get_name()}
                                    </LeafSectionHeader>
                                    <LeafSection>{move || type_view.to_string()}</LeafSection>
                                    <LeafSection>
                                        <Button on:click=on_click_delete>Delete</Button>
                                    </LeafSection>
                                </LeafSection>
                            }
                        }
                    />
                </SubSection>
            </Section>

            <Section>
                <SectionHeader slot>Outputs</SectionHeader>
                <Show when=move || !is_adding_output.get()>
                    <Button on:click=move |_| is_adding_output.set(true)>Add New Output</Button>
                </Show>
                <Show when=move || {
                    is_adding_output.get()
                }>
                    {
                        let schema_clone = schema_clone_2.clone();
                        let on_click_add_output = get_on_click_add_input_or_output
                            .clone()(
                            output_select_value,
                            is_adding_output,
                            output_name,
                            output_selected_operative,
                            output_selected_operative_list,
                            output_selected_trait_list,
                            false,
                        );
                        view! {
                            <SubSection>
                                <SubSectionHeader>Add New Output</SubSectionHeader>
                                <LeafSection>
                                    <LeafSectionHeader>Output Name</LeafSectionHeader>
                                    <LeafSection>
                                        <SignalTextInput value=output_name />
                                    </LeafSection>
                                </LeafSection>
                                <LeafSection>
                                    <LeafSectionHeader>Output to add:</LeafSectionHeader>
                                    <LeafSection>
                                        <SignalEnumSelect value=output_select_value />
                                    </LeafSection>
                                </LeafSection>

                                <DetailSelectionView
                                    selected_variant=output_select_value
                                    schema=schema_clone.get().clone()
                                    selected_op=output_selected_operative
                                    selected_op_list=output_selected_operative_list
                                    selected_trait_list=output_selected_trait_list
                                />

                                <LeafSection>
                                    <Button
                                        on:click=on_click_add_output
                                        prop:disabled=move || {
                                            is_save_disabled(
                                                output_select_value,
                                                output_selected_operative,
                                                output_selected_operative_list,
                                                output_selected_trait_list,
                                            )
                                        }
                                    >
                                        Add Output
                                    </Button>
                                    <Button on:click=move |_| {
                                        is_adding_output.set(false)
                                    }>Cancel</Button>
                                </LeafSection>
                            </SubSection>
                        }
                    }
                </Show>
                <SubSection>
                    <SubSectionHeader>Current Outputs</SubSectionHeader>
                    <For
                        each=move || fn_def_clone_3.get_outputs_slot()
                        key=|item| item.get_id().clone()
                        children=move |output| {
                            let ctx_clone = ctx_clone_3.clone();
                            let output_clone = output.clone();
                            let on_click_delete = get_on_delete_input_or_output_clone
                                .clone()(output.clone());
                            let type_view: GetNameFunctionIOTraitObjectDiscriminants = output
                                .clone()
                                .into();
                            view! {
                                <LeafSection>
                                    <LeafSectionHeader>
                                        {move || output.get_name()}
                                    </LeafSectionHeader>
                                    <LeafSection>{move || type_view.to_string()}</LeafSection>
                                    <LeafSection>
                                        <Button on:click=on_click_delete>Delete</Button>
                                    </LeafSection>
                                </LeafSection>
                            }
                        }
                    />
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
    selected_op_list: RwSignal<Vec<RGSOConcrete<OperativeConcrete, Schema>>>,
    selected_trait_list: RwSignal<Vec<RGSOConcrete<TraitConcrete, Schema>>>,
) -> impl IntoView {
    move || {
        let schema = schema.clone();
        match selected_variant.get() {
            GetNameFunctionIOTraitObjectDiscriminants::FunctionIOSingleOperative => {
                EitherOf13::A(view! {
                    <LeafSection>
                        <LeafSectionHeader>SelectedOperative:</LeafSectionHeader>
                        <LeafSection>
                            <SignalSelectRGSOWithOptions
                                empty_allowed=true
                                value=selected_op
                                options=Signal::derive(move || schema.get_operatives_slot())
                            />
                        </LeafSection>
                    </LeafSection>
                })
            }
            GetNameFunctionIOTraitObjectDiscriminants::FunctionIOPrimitiveBool => EitherOf13::B(()),
            GetNameFunctionIOTraitObjectDiscriminants::FunctionIOPrimitiveInt => EitherOf13::C(()),
            GetNameFunctionIOTraitObjectDiscriminants::FunctionIOPrimitiveString => {
                EitherOf13::D(())
            }
            GetNameFunctionIOTraitObjectDiscriminants::FunctionIOCollectionPrimitiveBool => {
                EitherOf13::E(())
            }
            GetNameFunctionIOTraitObjectDiscriminants::FunctionIOCollectionMultiOperative => {
                EitherOf13::F(view! {
                    <MultiSelectRGSO
                        list=selected_op_list
                        options=Signal::derive(move || schema.get_operatives_slot())
                    />
                })
            }
            GetNameFunctionIOTraitObjectDiscriminants::FunctionIOCollectionTraitOperative => {
                EitherOf13::G(view! {
                    <MultiSelectRGSO
                        list=selected_trait_list
                        options=Signal::derive(move || schema.get_traits_slot())
                    />
                })
            }
            GetNameFunctionIOTraitObjectDiscriminants::FunctionIOCollectionPrimitiveString => {
                EitherOf13::H(())
            }
            GetNameFunctionIOTraitObjectDiscriminants::FunctionIOTraitOperative => {
                EitherOf13::I(view! {
                    <MultiSelectRGSO
                        list=selected_trait_list
                        options=Signal::derive(move || schema.get_traits_slot())
                    />
                })
            }
            GetNameFunctionIOTraitObjectDiscriminants::FunctionIOSelf => EitherOf13::J(()),
            GetNameFunctionIOTraitObjectDiscriminants::FunctionIOCollectionSingleOperative => {
                EitherOf13::K(view! {
                    <LeafSection>
                        <LeafSectionHeader>SelectedOperative:</LeafSectionHeader>
                        <LeafSection>
                            <SignalSelectRGSOWithOptions
                                empty_allowed=true
                                value=selected_op
                                options=Signal::derive(move || schema.get_operatives_slot())
                            />
                        </LeafSection>
                    </LeafSection>
                })
            }
            GetNameFunctionIOTraitObjectDiscriminants::FunctionIOMultiOperative => {
                EitherOf13::L(view! {
                    <MultiSelectRGSO
                        list=selected_op_list
                        options=Signal::derive(move || schema.get_operatives_slot())
                    />
                })
            }
            GetNameFunctionIOTraitObjectDiscriminants::FunctionIOCollectionPrimitiveInt => {
                EitherOf13::M(())
            }
        }
    }
}
