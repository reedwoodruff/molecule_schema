use std::str::FromStr;

use crate::components::{
    common::*, graph_editor::GraphEditor, method_impl_builder::MethodImplementationBuilder,
    utils::constraint_to_canvas_template,
};
use graph_canvas::GraphCanvasConfig;
use schema_editor_generated_toolkit::prelude::*;
use strum::EnumProperty;
use strum::IntoEnumIterator;

use super::workspace::WorkspaceState;
#[component]
pub fn OperativeMethodImplementations(
    operative: RGSOConcrete<OperativeConcrete, Schema>,
) -> impl IntoView {
    let ctx = use_context::<SharedGraph<Schema>>().unwrap();
    let ctx_clone = ctx.clone();
    let WorkspaceState { schema, .. } = use_context::<WorkspaceState>().unwrap();
    let schema_clone = schema.clone();

    let is_adding_impl = RwSignal::new(false);
    let selected_fn_def = RwSignal::new(None);
    let fn_def_options = Memo::new(move |_| schema_clone.get().get_functions_slot());

    let operative_clone = operative.clone();
    let on_save_new_fn_impl = Callback::new(
        move |incorporatable: Box<dyn Incorporatable<MethodImplementation, Schema>>| {
            let mut editor = operative_clone.edit(ctx_clone.clone());
            editor.add_temp_functionimpls("new_fn_impl");
            editor.incorporate(&incorporatable);
            editor.execute().unwrap();

            is_adding_impl.set(false);
        },
    );

    let operative_clone = operative.clone();
    let operative_clone_2 = operative.clone();
    let ctx_clone = ctx.clone();

    let canvas_config = {
        // CONSTRAINT_SCHEMA.;

        let step_templates = ImplStepVariantTraitObjectDiscriminants::iter()
            .map(|step| {
                // let int_uid: u128 = uuid::Uuid::from_str(step.get_str("template_id").unwrap())
                //     .unwrap()
                //     .as_u128();
                let int_uid: u128 = u128::from_str(step.get_str("template_id").unwrap()).unwrap();
                let template = CONSTRAINT_SCHEMA.template_library.get(&int_uid).unwrap();
                constraint_to_canvas_template(template)
            })
            .collect();
        GraphCanvasConfig {
            node_templates: step_templates,
            ..GraphCanvasConfig::new()
        }
    };
    view! {
        <SubSection>
            <SubSectionHeader>Add New Implementation</SubSectionHeader>
            <Show when=move || !is_adding_impl.get()>
                <Button
                    attr:disabled=move || selected_fn_def.get().is_none()
                    on:click=move |_| {
                        if selected_fn_def.get().is_some() {
                            is_adding_impl.set(true)
                        }
                    }
                >
                    Add New Implementation
                </Button>
                " "
                for
                <SignalSelectRGSOWithOptions
                    value=selected_fn_def
                    options=Signal::derive(move || fn_def_options.get())
                    empty_allowed=true
                />
            </Show>
            <Show when=move || {
                is_adding_impl.get()
            }>
                {
                    let canvas_config = canvas_config.clone();
                    view! {
                        <MethodImplementationBuilder
                            fn_def=selected_fn_def.get().unwrap()
                            operative=operative_clone_2.clone()
                            on_save=on_save_new_fn_impl
                            on_cancel=Callback::new(move |_na: ()| { is_adding_impl.set(false) })
                        />
                        <GraphEditor config=canvas_config />
                    }
                }
            </Show>
        </SubSection>

        <SubSection>
            <SubSectionHeader>Current Implementations</SubSectionHeader>
            <For

                each=move || operative_clone.get_functionimpls_slot()
                key=|item| item.get_id().clone()
                children=move |func_impl| {
                    let ctx = ctx_clone.clone();
                    let ctx_clone = ctx.clone();
                    let func_impl_clone = func_impl.clone();
                    let on_delete = move |_| {
                        func_impl_clone
                            .edit(ctx_clone.clone())
                            .delete_recursive()
                            .execute()
                            .unwrap();
                    };
                    let func_impl_clone = func_impl.clone();
                    view! {
                        <SubSection>
                            <div class="flex">
                                <div class="flex-grow">
                                    <SubSectionHeader>
                                        {move || func_impl_clone.get_name()}
                                    </SubSectionHeader>
                                    <LeafSection attr:class="leaf-section dependent">
                                        "Implementation of "
                                        <strong>
                                            {move || func_impl.get_definition_slot().get_name()}
                                        </strong>
                                    </LeafSection>
                                </div>
                                <div>
                                    <Button on:click=on_delete>Delete Implementation</Button>
                                </div>
                            </div>
                            <LeafSection>Other stuff</LeafSection>
                        </SubSection>
                    }
                }
            />
        </SubSection>
    }
}
