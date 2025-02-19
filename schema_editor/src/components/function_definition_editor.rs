use super::common::*;
use leptos::either::EitherOf6;
use schema_editor_generated_toolkit::prelude::*;

use crate::components::workspace::{WorkspaceState, WorkspaceTab};

#[derive(
    Clone,
    Debug,
    Hash,
    PartialEq,
    Eq,
    strum_macros::Display,
    strum_macros::EnumString,
    strum_macros::EnumIter,
)]
enum InputOutputOptions {
    ImplDataMultiOperative,
    ImplDataBool,
    ImplDataInt,
    ImplDataString,
    ImplDataSingleOperative,
    ImplDataTraitOperative,
}
impl Into<ImplDataVariantTraitObjectDiscriminants> for InputOutputOptions {
    fn into(self) -> ImplDataVariantTraitObjectDiscriminants {
        match self {
            InputOutputOptions::ImplDataMultiOperative => {
                ImplDataVariantTraitObjectDiscriminants::ImplDataMultiOperative
            }
            InputOutputOptions::ImplDataBool => {
                ImplDataVariantTraitObjectDiscriminants::ImplDataBool
            }
            InputOutputOptions::ImplDataInt => ImplDataVariantTraitObjectDiscriminants::ImplDataInt,
            InputOutputOptions::ImplDataString => {
                ImplDataVariantTraitObjectDiscriminants::ImplDataString
            }
            InputOutputOptions::ImplDataSingleOperative => {
                ImplDataVariantTraitObjectDiscriminants::ImplDataSingleOperative
            }
            InputOutputOptions::ImplDataTraitOperative => {
                ImplDataVariantTraitObjectDiscriminants::ImplDataTraitOperative
            }
        }
    }
}

fn into_most_generic(item: ImplDataVariantTraitObject) -> FunctionInputVariantTraitObject {
    match item {
        ImplDataVariantTraitObject::ImplDataMultiOperative(item) => {
            FunctionInputVariantTraitObject::ImplDataMultiOperative(item)
        }
        ImplDataVariantTraitObject::ImplDataBool(item) => {
            FunctionInputVariantTraitObject::ImplDataBool(item)
        }
        ImplDataVariantTraitObject::ImplDataInt(item) => {
            FunctionInputVariantTraitObject::ImplDataInt(item)
        }
        ImplDataVariantTraitObject::ImplDataString(item) => {
            FunctionInputVariantTraitObject::ImplDataString(item)
        }
        ImplDataVariantTraitObject::ImplDataSingleOperative(item) => {
            FunctionInputVariantTraitObject::ImplDataSingleOperative(item)
        }
        ImplDataVariantTraitObject::ImplDataTraitOperative(item) => {
            FunctionInputVariantTraitObject::ImplDataTraitOperative(item)
        }
        ImplDataVariantTraitObject::ImplDataCollection(item) => {
            FunctionInputVariantTraitObject::ImplDataCollection(item)
        }
    }
}

#[component]
pub fn FunctionDefinitionEditor(fn_def: RGSOConcrete<FunctionDefinition, Schema>) -> impl IntoView {
    let ctx = use_context::<SharedGraph<Schema>>().unwrap();
    let WorkspaceState {
        schema,
        selected_tab,
    } = use_context::<WorkspaceState>().unwrap();

    let is_adding_input = RwSignal::new(false);
    let is_self_input = RwSignal::new(false);
    let input_select_value = RwSignal::new(InputOutputOptions::ImplDataSingleOperative);
    let input_name = RwSignal::new("new_input".to_string());
    let input_selected_operative =
        RwSignal::<Option<RGSOConcrete<OperativeConcrete, Schema>>>::new(None);
    let input_selected_operative_list =
        RwSignal::<Vec<RGSOConcrete<OperativeConcrete, Schema>>>::new(vec![]);
    let input_selected_trait_list =
        RwSignal::<Vec<RGSOConcrete<TraitConcrete, Schema>>>::new(vec![]);
    let input_collection_layers = RwSignal::<u32>::new(0);
    let ctx_clone = ctx.clone();

    let is_adding_output = RwSignal::new(false);
    let output_select_value = RwSignal::new(InputOutputOptions::ImplDataSingleOperative);
    let output_name = RwSignal::new("new_output".to_string());
    let output_selected_operative =
        RwSignal::<Option<RGSOConcrete<OperativeConcrete, Schema>>>::new(None);
    let output_selected_operative_list =
        RwSignal::<Vec<RGSOConcrete<OperativeConcrete, Schema>>>::new(vec![]);
    let output_selected_trait_list =
        RwSignal::<Vec<RGSOConcrete<TraitConcrete, Schema>>>::new(vec![]);
    let output_collection_layers = RwSignal::<u32>::new(0);

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

    let is_save_disabled =
        move |is_input: bool,
              selected_variant: RwSignal<InputOutputOptions>,
              selected_op: RwSignal<Option<RGSOConcrete<OperativeConcrete, Schema>>>,
              selected_op_list: RwSignal<Vec<RGSOConcrete<OperativeConcrete, Schema>>>,
              selected_trait_list: RwSignal<Vec<RGSOConcrete<TraitConcrete, Schema>>>| {
            move || {
                if is_input && is_self_input.get() {
                    return false;
                }
                match selected_variant.get() {
                    InputOutputOptions::ImplDataSingleOperative => selected_op.get().is_none(),
                    InputOutputOptions::ImplDataBool => false,
                    InputOutputOptions::ImplDataTraitOperative => {
                        selected_trait_list.get().len() < 1
                    }
                    // InputOutputOptions::FunctionIOSelf => false,
                    InputOutputOptions::ImplDataInt => false,
                    InputOutputOptions::ImplDataMultiOperative => selected_op_list.get().len() < 2,
                    InputOutputOptions::ImplDataString => false,
                }
            }
        };

    let ctx_clone = ctx.clone();
    let fn_def_clone = fn_def.clone();
    let on_click_add_input = move |_| {
        let mut editor = fn_def_clone.edit(ctx_clone.clone());
        let input_node = FunctionInput::new(ctx_clone.clone())
            .set_temp_id("new_input_node")
            .set_name(input_name.get());
        editor.incorporate(&input_node);
        editor.incorporate(
            fn_def_clone
                .edit(ctx_clone.clone())
                .add_temp_inputs("new_input_node"),
        );
        if is_self_input.get() {
            editor.incorporate(&input_node.add_new_type::<FunctionIOSelf, _>(|na| na));
            editor.execute().unwrap();
            input_collection_layers.set(0);
            is_adding_input.set(false);
            is_self_input.set(false);
            return;
        };
        match input_select_value.get() {
            InputOutputOptions::ImplDataMultiOperative => {
                let new_data_node =
                    ImplDataMultiOperative::new(ctx_clone.clone()).set_temp_id("new_data_node");
                editor.incorporate(&new_data_node);

                input_selected_operative_list
                    .get()
                    .into_iter()
                    .for_each(|op_item| {
                        editor.incorporate(
                            &new_data_node
                                .clone()
                                .add_existing_allowedoperatives(op_item.get_id(), |na| na),
                        );
                    });
            }
            InputOutputOptions::ImplDataBool => {
                editor.incorporate(
                    &ImplDataBool::new(ctx_clone.clone()).set_temp_id("new_data_node"),
                );
            }
            InputOutputOptions::ImplDataInt => {
                editor.incorporate(
                    &ImplDataBool::new(ctx_clone.clone()).set_temp_id("new_data_node"),
                );
            }
            InputOutputOptions::ImplDataString => {
                editor.incorporate(
                    &ImplDataBool::new(ctx_clone.clone()).set_temp_id("new_data_node"),
                );
            }
            InputOutputOptions::ImplDataSingleOperative => editor.incorporate(
                &ImplDataSingleOperative::new(ctx_clone.clone())
                    .set_temp_id("new_data_node")
                    .add_existing_allowedoperative(
                        &input_selected_operative.get().unwrap().get_id(),
                        |na| na,
                    ),
            ),
            InputOutputOptions::ImplDataTraitOperative => {
                let new_data_node =
                    ImplDataTraitOperative::new(ctx_clone.clone()).set_temp_id("new_data_node");
                editor.incorporate(&new_data_node);

                input_selected_trait_list
                    .get()
                    .into_iter()
                    .for_each(|trait_item| {
                        editor.incorporate(
                            &new_data_node
                                .clone()
                                .add_existing_requiredtraits(trait_item.get_id(), |na| na),
                        );
                    });
            }
        };

        let mut prev_collection_layer = ImplDataCollection::new(ctx_clone.clone()).set_temp_id("");
        for i in 0..input_collection_layers.get() {
            let layer_name = format!("layer_{}", i);
            let collection_layer =
                ImplDataCollection::new(ctx_clone.clone()).set_temp_id(&layer_name);
            editor.incorporate(&collection_layer);
            if i == 0 {
                editor.incorporate(
                    &input_node
                        .clone()
                        .add_temp_type::<ImplDataCollection>(layer_name),
                );
            } else {
                editor.incorporate(
                    &prev_collection_layer
                        .add_temp_collectiontype::<ImplDataCollection>(layer_name),
                )
            }
            if i == input_collection_layers.get() - 1 {
                match input_select_value.get() {
                    InputOutputOptions::ImplDataMultiOperative => {
                        editor.incorporate(
                            &collection_layer
                                .clone()
                                .add_temp_collectiontype::<ImplDataMultiOperative>("new_data_node"),
                        );
                    }
                    InputOutputOptions::ImplDataBool => {
                        editor.incorporate(
                            &collection_layer
                                .clone()
                                .add_temp_collectiontype::<ImplDataBool>("new_data_node"),
                        );
                    }
                    InputOutputOptions::ImplDataInt => {
                        editor.incorporate(
                            &collection_layer
                                .clone()
                                .add_temp_collectiontype::<ImplDataInt>("new_data_node"),
                        );
                    }
                    InputOutputOptions::ImplDataString => {
                        editor.incorporate(
                            &collection_layer
                                .clone()
                                .add_temp_collectiontype::<ImplDataString>("new_data_node"),
                        );
                    }
                    InputOutputOptions::ImplDataSingleOperative => {
                        editor.incorporate(
                            &collection_layer
                                .clone()
                                .add_temp_collectiontype::<ImplDataSingleOperative>(
                                    "new_data_node",
                                ),
                        );
                    }
                    InputOutputOptions::ImplDataTraitOperative => {
                        editor.incorporate(
                            &collection_layer
                                .clone()
                                .add_temp_collectiontype::<ImplDataTraitOperative>("new_data_node"),
                        );
                    }
                }
            }
            prev_collection_layer = collection_layer;
        }
        if input_collection_layers.get() == 0 {
            match input_select_value.get() {
                InputOutputOptions::ImplDataMultiOperative => {
                    editor.incorporate(
                        &input_node.add_temp_type::<ImplDataMultiOperative>("new_data_node"),
                    );
                }
                InputOutputOptions::ImplDataBool => {
                    editor.incorporate(&input_node.add_temp_type::<ImplDataBool>("new_data_node"));
                }
                InputOutputOptions::ImplDataInt => {
                    editor.incorporate(&input_node.add_temp_type::<ImplDataInt>("new_data_node"));
                }
                InputOutputOptions::ImplDataString => {
                    editor
                        .incorporate(&input_node.add_temp_type::<ImplDataString>("new_data_node"));
                }
                InputOutputOptions::ImplDataSingleOperative => {
                    editor.incorporate(
                        &input_node.add_temp_type::<ImplDataSingleOperative>("new_data_node"),
                    );
                }
                InputOutputOptions::ImplDataTraitOperative => {
                    editor.incorporate(
                        &input_node.add_temp_type::<ImplDataTraitOperative>("new_data_node"),
                    );
                }
            }
        }
        editor.execute().unwrap();
        input_collection_layers.set(0);
        is_adding_input.set(false);
        is_self_input.set(false);
    };

    let ctx_clone = ctx.clone();
    let fn_def_clone = fn_def.clone();
    let on_click_add_output = move |_| {
        let mut editor = fn_def_clone.edit(ctx_clone.clone());
        let output_node = FunctionOutput::new(ctx_clone.clone())
            .set_temp_id("new_output_node")
            .set_name(output_name.get());
        editor.incorporate(&output_node);
        editor.incorporate(
            fn_def_clone
                .edit(ctx_clone.clone())
                .add_temp_outputs("new_output_node"),
        );
        match output_select_value.get() {
            InputOutputOptions::ImplDataMultiOperative => {
                let new_data_node =
                    ImplDataMultiOperative::new(ctx_clone.clone()).set_temp_id("new_data_node");
                editor.incorporate(&new_data_node);

                output_selected_operative_list
                    .get()
                    .into_iter()
                    .for_each(|op_item| {
                        editor.incorporate(
                            &new_data_node
                                .clone()
                                .add_existing_allowedoperatives(op_item.get_id(), |na| na),
                        );
                    });
            }
            InputOutputOptions::ImplDataBool => {
                editor.incorporate(
                    &ImplDataBool::new(ctx_clone.clone()).set_temp_id("new_data_node"),
                );
            }
            InputOutputOptions::ImplDataInt => {
                editor.incorporate(
                    &ImplDataBool::new(ctx_clone.clone()).set_temp_id("new_data_node"),
                );
            }
            InputOutputOptions::ImplDataString => {
                editor.incorporate(
                    &ImplDataBool::new(ctx_clone.clone()).set_temp_id("new_data_node"),
                );
            }
            InputOutputOptions::ImplDataSingleOperative => editor.incorporate(
                &ImplDataSingleOperative::new(ctx_clone.clone())
                    .set_temp_id("new_data_node")
                    .add_existing_allowedoperative(
                        &output_selected_operative.get().unwrap().get_id(),
                        |na| na,
                    ),
            ),
            InputOutputOptions::ImplDataTraitOperative => {
                let new_data_node =
                    ImplDataTraitOperative::new(ctx_clone.clone()).set_temp_id("new_data_node");
                editor.incorporate(&new_data_node);

                output_selected_trait_list
                    .get()
                    .into_iter()
                    .for_each(|trait_item| {
                        editor.incorporate(
                            &new_data_node
                                .clone()
                                .add_existing_requiredtraits(trait_item.get_id(), |na| na),
                        );
                    });
            }
        };

        let mut prev_collection_layer = ImplDataCollection::new(ctx_clone.clone()).set_temp_id("");
        for i in 0..output_collection_layers.get() {
            let layer_name = format!("layer_{}", i);
            let collection_layer =
                ImplDataCollection::new(ctx_clone.clone()).set_temp_id(&layer_name);
            editor.incorporate(&collection_layer);
            if i == 0 {
                editor.incorporate(
                    &output_node
                        .clone()
                        .add_temp_type::<ImplDataCollection>(layer_name),
                );
            } else {
                editor.incorporate(
                    &prev_collection_layer
                        .add_temp_collectiontype::<ImplDataCollection>(layer_name),
                )
            }
            if i == output_collection_layers.get() - 1 {
                match output_select_value.get() {
                    InputOutputOptions::ImplDataMultiOperative => {
                        editor.incorporate(
                            &collection_layer
                                .clone()
                                .add_temp_collectiontype::<ImplDataMultiOperative>("new_data_node"),
                        );
                    }
                    InputOutputOptions::ImplDataBool => {
                        editor.incorporate(
                            &collection_layer
                                .clone()
                                .add_temp_collectiontype::<ImplDataBool>("new_data_node"),
                        );
                    }
                    InputOutputOptions::ImplDataInt => {
                        editor.incorporate(
                            &collection_layer
                                .clone()
                                .add_temp_collectiontype::<ImplDataInt>("new_data_node"),
                        );
                    }
                    InputOutputOptions::ImplDataString => {
                        editor.incorporate(
                            &collection_layer
                                .clone()
                                .add_temp_collectiontype::<ImplDataString>("new_data_node"),
                        );
                    }
                    InputOutputOptions::ImplDataSingleOperative => {
                        editor.incorporate(
                            &collection_layer
                                .clone()
                                .add_temp_collectiontype::<ImplDataSingleOperative>(
                                    "new_data_node",
                                ),
                        );
                    }
                    InputOutputOptions::ImplDataTraitOperative => {
                        editor.incorporate(
                            &collection_layer
                                .clone()
                                .add_temp_collectiontype::<ImplDataTraitOperative>("new_data_node"),
                        );
                    }
                }
            }
            prev_collection_layer = collection_layer;
        }
        if output_collection_layers.get() == 0 {
            match output_select_value.get() {
                InputOutputOptions::ImplDataMultiOperative => {
                    editor.incorporate(
                        &output_node.add_temp_type::<ImplDataMultiOperative>("new_data_node"),
                    );
                }
                InputOutputOptions::ImplDataBool => {
                    editor.incorporate(&output_node.add_temp_type::<ImplDataBool>("new_data_node"));
                }
                InputOutputOptions::ImplDataInt => {
                    editor.incorporate(&output_node.add_temp_type::<ImplDataInt>("new_data_node"));
                }
                InputOutputOptions::ImplDataString => {
                    editor
                        .incorporate(&output_node.add_temp_type::<ImplDataString>("new_data_node"));
                }
                InputOutputOptions::ImplDataSingleOperative => {
                    editor.incorporate(
                        &output_node.add_temp_type::<ImplDataSingleOperative>("new_data_node"),
                    );
                }
                InputOutputOptions::ImplDataTraitOperative => {
                    editor.incorporate(
                        &output_node.add_temp_type::<ImplDataTraitOperative>("new_data_node"),
                    );
                }
            }
        }
        editor.execute().unwrap();
        output_collection_layers.set(0);
        is_adding_output.set(false);
    };
    let fn_def_clone = fn_def.clone();
    let fn_def_clone_2 = fn_def.clone();
    let fn_def_clone_3 = fn_def.clone();
    let ctx_clone_2 = ctx.clone();
    let ctx_clone_3 = ctx.clone();
    let schema_clone = schema.clone();
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
                        let on_click_add_input_clone = on_click_add_input.clone();
                        // let on_click_add_output_clone = on_click_add_output.clone();
                        view! {
                            <SubSection>
                                <SubSectionHeader>Add New Input</SubSectionHeader>
                                <LeafSection>
                                    <LeafSectionHeader>Input Name</LeafSectionHeader>
                                    <LeafSection>
                                        <SignalTextInput value=input_name />
                                    </LeafSection>
                                    <LeafSectionHeader>
                                        Number of collection layers
                                    </LeafSectionHeader>
                                    <LeafSection>
                                        <SignalTextInput
                                            prop:type="number"
                                            value=input_collection_layers
                                        />
                                    </LeafSection>
                                </LeafSection>
                                <LeafSection>
                                    <LeafSectionHeader>Is Self Input</LeafSectionHeader>
                                    <input
                                        type="checkbox"
                                        value=move || is_self_input.get()
                                        on:change=move |_| {
                                            is_self_input.update(|prev| *prev = !*prev)
                                        }
                                    />
                                </LeafSection>
                                <Show when=move || {
                                    !is_self_input.get()
                                }>
                                    {
                                        let schema_clone = schema_clone.clone();
                                        view! {
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
                                        }
                                    }
                                </Show>
                                <LeafSection>
                                    <Button
                                        on:click=on_click_add_input_clone
                                        prop:disabled=move || {
                                            is_save_disabled(
                                                true,
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
                                        is_self_input.set(false);
                                        is_adding_input.set(false);
                                        input_collection_layers.set(0);
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
                            let on_click_delete = move |_| {
                                input_clone
                                    .edit(ctx_clone.clone())
                                    .delete_recursive()
                                    .execute()
                                    .unwrap();
                            };
                            let input_clone = input.clone();
                            let io = input_clone.clone().get_type_slot();

                            view! {
                                <LeafSection>
                                    <LeafSectionHeader>
                                        {move || input.get_name_field()}
                                    </LeafSectionHeader>
                                    <LeafSection>
                                        <InputOutputDisplay value=io />
                                    </LeafSection>
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
                        let on_click_add_output_clone = on_click_add_output.clone();
                        // let on_click_add_output_clone = on_click_add_output.clone();
                        view! {
                            <SubSection>
                                <SubSectionHeader>Add New Output</SubSectionHeader>
                                <LeafSection>
                                    <LeafSectionHeader>Output Name</LeafSectionHeader>
                                    <LeafSection>
                                        <SignalTextInput value=output_name />
                                    </LeafSection>
                                    <LeafSectionHeader>
                                        Number of collection layers
                                    </LeafSectionHeader>
                                    <LeafSection>
                                        <SignalTextInput
                                            prop:type="number"
                                            value=output_collection_layers
                                        />
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
                                        on:click=on_click_add_output_clone
                                        prop:disabled=move || {
                                            is_save_disabled(
                                                false,
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
                                        is_adding_output.set(false);
                                        output_collection_layers.set(0);
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
                            let on_click_delete = move |_| {
                                output_clone
                                    .edit(ctx_clone.clone())
                                    .delete_recursive()
                                    .execute()
                                    .unwrap();
                            };
                            let output_clone = output.clone();
                            let io = into_most_generic(output_clone.clone().get_type_slot());

                            view! {
                                <LeafSection>
                                    <LeafSectionHeader>
                                        {move || output.get_name_field()}
                                    </LeafSectionHeader>
                                    <LeafSection>
                                        <InputOutputDisplay value=io />
                                    </LeafSection>
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
pub fn InputOutputDisplay(value: FunctionInputVariantTraitObject) -> impl IntoView {
    let display_type: FunctionInputVariantTraitObjectDiscriminants = value.clone().into();
    match value {
        FunctionInputVariantTraitObject::ImplDataMultiOperative(rgsoconcrete) => {
            let operative_names = rgsoconcrete
                .get_allowedoperatives_slot()
                .into_iter()
                .map(|op| op.get_name())
                .collect::<Vec<_>>()
                .join(", ");
            view! { <div>{display_type.to_string()}": ["{operative_names}"]"</div> }.into_any()
        }
        FunctionInputVariantTraitObject::FunctionIOSelf(_) => {
            view! { <div>{display_type.to_string()}</div> }.into_any()
        }
        FunctionInputVariantTraitObject::ImplDataBool(_) => {
            view! { <div>{display_type.to_string()}</div> }.into_any()
        }
        FunctionInputVariantTraitObject::ImplDataInt(_) => {
            view! { <div>{display_type.to_string()}</div> }.into_any()
        }
        FunctionInputVariantTraitObject::ImplDataString(_) => {
            view! { <div>{display_type.to_string()}</div> }.into_any()
        }
        FunctionInputVariantTraitObject::ImplDataCollection(rgsoconcrete) => {
            let next_val = into_most_generic(rgsoconcrete.get_collectiontype_slot());
            view! { <div>{display_type.to_string()}" > "<InputOutputDisplay value=next_val /></div> }.into_any()
        }
        FunctionInputVariantTraitObject::ImplDataSingleOperative(rgsoconcrete) => {
            let operative_name = rgsoconcrete.get_allowedoperative_slot().get_name();
            view! { <div>{display_type.to_string()}": "{operative_name}</div> }.into_any()
        }
        FunctionInputVariantTraitObject::ImplDataTraitOperative(rgsoconcrete) => {
            let trait_names = rgsoconcrete
                .get_requiredtraits_slot()
                .into_iter()
                .map(|trait_object| trait_object.get_name())
                .collect::<Vec<_>>()
                .join(", ");
            view! { <div>{display_type.to_string()}": ["{trait_names}"]"</div> }.into_any()
        }
    }
}

#[component]
fn DetailSelectionView(
    selected_variant: RwSignal<InputOutputOptions>,
    schema: RGSOConcrete<SchemaConcrete, Schema>,
    selected_op: RwSignal<Option<RGSOConcrete<OperativeConcrete, Schema>>>,
    selected_op_list: RwSignal<Vec<RGSOConcrete<OperativeConcrete, Schema>>>,
    selected_trait_list: RwSignal<Vec<RGSOConcrete<TraitConcrete, Schema>>>,
) -> impl IntoView {
    move || {
        let schema = schema.clone();
        match selected_variant.get() {
            InputOutputOptions::ImplDataSingleOperative => EitherOf6::A(view! {
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
            }),
            InputOutputOptions::ImplDataBool => EitherOf6::B(()),
            InputOutputOptions::ImplDataInt => EitherOf6::C(()),
            InputOutputOptions::ImplDataString => EitherOf6::D(()),
            InputOutputOptions::ImplDataTraitOperative => EitherOf6::E(view! {
                <MultiSelectRGSO
                    list=selected_trait_list
                    options=Signal::derive(move || schema.get_traits_slot())
                />
            }),
            InputOutputOptions::ImplDataMultiOperative => EitherOf6::F(view! {
                <MultiSelectRGSO
                    list=selected_op_list
                    options=Signal::derive(move || schema.get_operatives_slot())
                />
            }),
        }
    }
}
