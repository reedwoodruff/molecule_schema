use crate::components::{common::*, method_impl_builder::MethodImplementationBuilder};
use schema_editor_generated_toolkit::prelude::*;

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
        move |incorporatable: ExistingBuilder<OperativeConcrete, Schema>| {
            // let mut editor = operative_clone.edit(ctx_clone.clone());
            // editor.add_temp_functionimpls("new_fn_impl");
            // editor.incorporate(&incorporatable);
            // editor.execute().unwrap();
            leptos::logging::log!("about to execute blueprint");
            match incorporatable.execute() {
                Ok(_) => (),
                Err(err) => leptos::logging::log!("Error executing blueprint: {:#?}", err),
            };
            leptos::logging::log!("executed blueprint");

            is_adding_impl.set(false);
        },
    );

    let operative_clone = operative.clone();
    let operative_clone_2 = operative.clone();
    let ctx_clone = ctx.clone();

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
            <Show when=move || { is_adding_impl.get() }>
                <MethodImplementationBuilder
                    fn_def=selected_fn_def.get().unwrap()
                    operative=operative_clone_2.clone()
                    on_save=on_save_new_fn_impl
                    on_cancel=Callback::new(move |_na: ()| { is_adding_impl.set(false) })
                />
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
                        let mut func_delete = func_impl_clone.edit(ctx_clone.clone());
                        func_delete.delete_recursive();
                        func_delete.execute().unwrap();
                    };
                    let func_impl_clone = func_impl.clone();
                    // let output_terminals = func_impl_clone.get_maptooutputs_slot();
                    // output_terminals
                    // .iter()
                    // .for_each(|terminal| {
                    // func_delete
                    // .incorporate(
                    // terminal.edit(ctx_clone.clone()).delete_recursive(),
                    // );
                    // });
                    // let input_terminals = func_impl_clone.get_mapfrominputs_slot();
                    // input_terminals
                    // .iter()
                    // .for_each(|terminal| {
                    // func_delete
                    // .incorporate(
                    // terminal.edit(ctx_clone.clone()).delete_recursive(),
                    // );
                    // });

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
