use crate::components::{common::*, method_impl_builder::MethodImplementationBuilder};
use schema_editor_generated_toolkit::prelude::*;

use super::workspace::WorkspaceState;
#[component]
pub fn OperativeMethodImplementations(
    operative: RGSOConcrete<OperativeConcrete, Schema>,
) -> impl IntoView {
    let ctx = use_context::<SharedGraph<Schema>>().unwrap();
    let WorkspaceState { schema, .. } = use_context::<WorkspaceState>().unwrap();
    let schema_clone = schema.clone();

    let is_adding_impl = RwSignal::new(false);
    let selected_fn_def = RwSignal::<Option<RGSOConcrete<FunctionDefinition, Schema>>>::new(None);
    let operative_clone = operative.clone();
    let fn_def_options = Memo::new(move |_| {
        schema_clone
            .get()
            .get_functions_slot()
            .into_iter()
            .filter(|fn_def| {
                fn_def.get_inputs_slot().into_iter().any(|input| {
                    matches!(
                        input.get_type_slot(),
                        FunctionInputVariantTraitObject::FunctionIOSelf(_)
                    )
                }) && !operative_clone
                    .get_methodimpls_slot()
                    .into_iter()
                    .any(|func_impl| func_impl.get_definition_slot().get_id() == fn_def.get_id())
            })
            .collect::<Vec<_>>()
    });

    let operative_clone = operative.clone();
    let on_save_new_fn_impl = Callback::new(
        move |incorporatable: ExistingBuilder<OperativeConcrete, Schema>| {
            leptos::logging::log!("about to execute blueprint");
            match incorporatable.execute() {
                Ok(_) => (),
                Err(err) => leptos::logging::log!("Error executing blueprint: {:#?}", err),
            };
            leptos::logging::log!("executed blueprint");

            is_adding_impl.set(false);
        },
    );
    let ancestor_ops_with_method_impls = Memo::new(move |_| {
        let mut ancestor_ops_with_method_impls = vec![];
        let mut next_operative = operative_clone.get_parentoperative_slot().first().cloned();
        while let Some(confirmed_parent_op) = next_operative {
            if !confirmed_parent_op.get_methodimpls_slot().is_empty() {
                ancestor_ops_with_method_impls.push(confirmed_parent_op.clone());
            }
            next_operative = confirmed_parent_op
                .get_parentoperative_slot()
                .first()
                .cloned();
        }
        ancestor_ops_with_method_impls
    });

    let operative_clone = operative.clone();
    let operative_clone_2 = operative.clone();
    let operative_clone_3 = operative.clone();
    let operative_clone_4 = operative.clone();
    let ctx_clone = ctx.clone();

    view! {
        <SubSection>
            <SubSectionHeader>Add New Implementation</SubSectionHeader>
            <Show when=move || {
                !is_adding_impl.get()
            }>
                {
                    let operative_clone_4 = operative_clone_4.clone();
                    view! {
                        <Button
                            attr:disabled=move || {
                                selected_fn_def.get().is_none()
                                    || operative_clone_4
                                        .get_methodimpls_slot()
                                        .into_iter()
                                        .find(|func_impl| {
                                            match selected_fn_def.get() {
                                                Some(selected_fn_def) => {
                                                    func_impl.get_id() == selected_fn_def.get_id()
                                                }
                                                None => false,
                                            }
                                        })
                                        .is_some()
                            }
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
                    }
                }
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
            <SubSectionHeader>"Current Local Method Implementations"</SubSectionHeader>
            <For

                each=move || operative_clone.get_methodimpls_slot()
                key=|item| item.get_id().clone()
                children=move |func_impl| {
                    let operative = operative_clone_3.clone();
                    let ctx = ctx_clone.clone();
                    let ctx_clone = ctx.clone();
                    let func_impl_clone = func_impl.clone();
                    let is_editing_impl = RwSignal::new(false);
                    let on_delete = move || {
                        let mut func_delete = func_impl_clone.edit(ctx_clone.clone());
                        func_delete.delete();
                        let output_terminals = func_impl_clone.get_maptooutputs_slot();
                        output_terminals
                            .iter()
                            .for_each(|terminal| {
                                func_delete
                                    .incorporate(
                                        terminal.edit(ctx_clone.clone()).delete_recursive(),
                                    );
                            });
                        let input_terminals = func_impl_clone.get_mapfrominputs_slot();
                        input_terminals
                            .iter()
                            .for_each(|terminal| {
                                let steps = terminal.get_output_slot().get_downstreamsteps_slot();
                                follow_and_delete_downstream(&steps, &mut func_delete, ctx.clone());
                                func_delete
                                    .incorporate(
                                        terminal.edit(ctx_clone.clone()).delete_recursive(),
                                    );
                            });
                        func_delete.execute().unwrap();
                    };
                    let on_delete_clone = on_delete.clone();
                    let on_save_edited_fn_impl = Callback::new(move |
                        incorporatable: ExistingBuilder<OperativeConcrete, Schema>|
                    {
                        on_delete_clone();
                        match incorporatable.execute() {
                            Ok(_) => {}
                            Err(err) => {
                                leptos::logging::log!("Error executing blueprint: {:#?}", err)
                            }
                        };
                        leptos::logging::log!("executed blueprint");
                        is_adding_impl.set(false);
                    });
                    let func_impl_clone = func_impl.clone();
                    let func_impl_clone_2 = func_impl.clone();
                    let operative_clone = operative.clone();

                    // func_delete.execute().unwrap();

                    view! {
                        <SubSection>
                            <div class="flex">
                                <div class="flex-grow">
                                    <SubSectionHeader>
                                        {move || func_impl_clone_2.get_name()}
                                    </SubSectionHeader>
                                    <LeafSection attr:class="leaf-section dependent">
                                        "Implementation of "
                                        <strong>
                                            {move || func_impl.get_definition_slot().get_name()}
                                        </strong>
                                    </LeafSection>
                                </div>
                                <div>
                                    <Button on:click=move |_| {
                                        is_editing_impl.set(true)
                                    }>Edit Implementation</Button>
                                </div>
                                <div>
                                    <Button on:click=move |_| on_delete()>
                                        Delete Implementation
                                    </Button>
                                </div>
                            </div>
                            <LeafSection>
                                <Show when=move || { is_editing_impl.get() }>
                                    <MethodImplementationBuilder
                                        fn_def=func_impl_clone.get_definition_slot()
                                        operative=operative_clone.clone()
                                        on_save=on_save_edited_fn_impl
                                        on_cancel=Callback::new(move |_na: ()| {
                                            is_editing_impl.set(false)
                                        })
                                        initial_state=func_impl_clone.clone()
                                    />
                                </Show>
                            </LeafSection>
                        </SubSection>
                    }
                }
            />
        </SubSection>
        <SubSection>
            <SubSectionHeader>"Current Ancestor Method Implementations"</SubSectionHeader>
            <For

                each=move || ancestor_ops_with_method_impls.get()
                key=|item| item.get_id().clone()
                children=move |ancestor_op| {
                    let ancestor_op_clone = ancestor_op.clone();
                    view! {
                        <LeafSection>
                            <LeafSectionHeader>
                                "Ancestor: "{ancestor_op.get_name()}
                            </LeafSectionHeader>
                            <ul>
                                <For
                                    each=move || ancestor_op_clone.get_methodimpls_slot()
                                    key=|item| item.get_id().clone()
                                    let:method_impl
                                >
                                    <li>
                                        {method_impl.get_name()} " ("
                                        {method_impl.get_definition_slot().get_name()}")"
                                    </li>
                                </For>
                            </ul>
                        </LeafSection>
                    }
                }
            />
        </SubSection>
    }
}
fn follow_and_delete_downstream(
    steps: &Vec<ImplStepVariantTraitObject>,
    editor: &mut ExistingBuilder<MethodImplementation, Schema>,
    ctx: SharedGraph<Schema>,
) {
    for step in steps {
        match step {
            ImplStepVariantTraitObject::ImplStepCollectionIsEmpty(rgsoconcrete) => {
                editor.incorporate(rgsoconcrete.edit(ctx.clone()).delete_recursive());
                editor.incorporate(
                    rgsoconcrete
                        .get_outputbool_slot()
                        .edit(ctx.clone())
                        .delete_recursive(),
                );
                follow_and_delete_downstream(
                    &rgsoconcrete
                        .get_outputbool_slot()
                        .get_downstreamsteps_slot(),
                    editor,
                    ctx.clone(),
                );
            }
            ImplStepVariantTraitObject::ImplStepCollectionGetNextItem(rgsoconcrete) => {
                editor.incorporate(rgsoconcrete.edit(ctx.clone()).delete_recursive());
                editor.incorporate(
                    rgsoconcrete
                        .get_output_slot()
                        .edit(ctx.clone())
                        .delete_recursive(),
                );
                follow_and_delete_downstream(
                    &rgsoconcrete.get_output_slot().get_downstreamsteps_slot(),
                    editor,
                    ctx.clone(),
                );
            }
            ImplStepVariantTraitObject::ImplStepInvokeMethod(rgsoconcrete) => {
                editor.incorporate(rgsoconcrete.edit(ctx.clone()).delete_recursive());
                let outputs = rgsoconcrete.get_methodoutputs_slot();
                for output in outputs {
                    editor.incorporate(output.edit(ctx.clone()).delete_recursive());
                    follow_and_delete_downstream(
                        &output.get_downstreamsteps_slot(),
                        editor,
                        ctx.clone(),
                    );
                }
            }
            ImplStepVariantTraitObject::ImplStepIdentity(rgsoconcrete) => {
                editor.incorporate(rgsoconcrete.edit(ctx.clone()).delete_recursive());
                editor.incorporate(
                    rgsoconcrete
                        .get_output_slot()
                        .edit(ctx.clone())
                        .delete_recursive(),
                );
                follow_and_delete_downstream(
                    &rgsoconcrete.get_output_slot().get_downstreamsteps_slot(),
                    editor,
                    ctx.clone(),
                );
            }
            ImplStepVariantTraitObject::ImplStepIsType(rgsoconcrete) => {
                editor.incorporate(rgsoconcrete.edit(ctx.clone()).delete_recursive());
                editor.incorporate(
                    rgsoconcrete
                        .get_outputbool_slot()
                        .edit(ctx.clone())
                        .delete_recursive(),
                );
                follow_and_delete_downstream(
                    &rgsoconcrete
                        .get_outputbool_slot()
                        .get_downstreamsteps_slot(),
                    editor,
                    ctx.clone(),
                );
            }
            ImplStepVariantTraitObject::ImplStepCollectionGetLength(rgsoconcrete) => {
                editor.incorporate(rgsoconcrete.edit(ctx.clone()).delete_recursive());
                editor.incorporate(
                    rgsoconcrete
                        .get_outputint_slot()
                        .edit(ctx.clone())
                        .delete_recursive(),
                );
                follow_and_delete_downstream(
                    &rgsoconcrete.get_outputint_slot().get_downstreamsteps_slot(),
                    editor,
                    ctx.clone(),
                );
            }
            ImplStepVariantTraitObject::ImplStepWhileLoop(rgsoconcrete) => {
                editor.incorporate(rgsoconcrete.edit(ctx.clone()).delete_recursive());
                editor.incorporate(
                    rgsoconcrete
                        .get_output_slot()
                        .edit(ctx.clone())
                        .delete_recursive(),
                );
                follow_and_delete_downstream(
                    &rgsoconcrete.get_output_slot().get_downstreamsteps_slot(),
                    editor,
                    ctx.clone(),
                );
            }
            ImplStepVariantTraitObject::ImplStepMathDivide(rgsoconcrete) => {
                editor.incorporate(rgsoconcrete.edit(ctx.clone()).delete_recursive());
                editor.incorporate(
                    rgsoconcrete
                        .get_outputint_slot()
                        .edit(ctx.clone())
                        .delete_recursive(),
                );
                follow_and_delete_downstream(
                    &rgsoconcrete.get_outputint_slot().get_downstreamsteps_slot(),
                    editor,
                    ctx.clone(),
                );
            }
            ImplStepVariantTraitObject::ImplStepBitNot(rgsoconcrete) => {
                editor.incorporate(rgsoconcrete.edit(ctx.clone()).delete_recursive());
                editor.incorporate(
                    rgsoconcrete
                        .get_outputbool_slot()
                        .edit(ctx.clone())
                        .delete_recursive(),
                );
                follow_and_delete_downstream(
                    &rgsoconcrete
                        .get_outputbool_slot()
                        .get_downstreamsteps_slot(),
                    editor,
                    ctx.clone(),
                );
            }
            ImplStepVariantTraitObject::ImplStepCompareEqual(rgsoconcrete) => {
                editor.incorporate(rgsoconcrete.edit(ctx.clone()).delete_recursive());
                editor.incorporate(
                    rgsoconcrete
                        .get_outputbool_slot()
                        .edit(ctx.clone())
                        .delete_recursive(),
                );
                follow_and_delete_downstream(
                    &rgsoconcrete
                        .get_outputbool_slot()
                        .get_downstreamsteps_slot(),
                    editor,
                    ctx.clone(),
                );
            }
            ImplStepVariantTraitObject::ImplStepBitAnd(rgsoconcrete) => {
                editor.incorporate(rgsoconcrete.edit(ctx.clone()).delete_recursive());
                editor.incorporate(
                    rgsoconcrete
                        .get_outputbool_slot()
                        .edit(ctx.clone())
                        .delete_recursive(),
                );
                follow_and_delete_downstream(
                    &rgsoconcrete
                        .get_outputbool_slot()
                        .get_downstreamsteps_slot(),
                    editor,
                    ctx.clone(),
                );
            }
            ImplStepVariantTraitObject::ImplStepMathAdd(rgsoconcrete) => {
                editor.incorporate(rgsoconcrete.edit(ctx.clone()).delete_recursive());
                editor.incorporate(
                    rgsoconcrete
                        .get_outputint_slot()
                        .edit(ctx.clone())
                        .delete_recursive(),
                );
                follow_and_delete_downstream(
                    &rgsoconcrete.get_outputint_slot().get_downstreamsteps_slot(),
                    editor,
                    ctx.clone(),
                );
            }
            ImplStepVariantTraitObject::ImplStepMathModulus(rgsoconcrete) => {
                editor.incorporate(rgsoconcrete.edit(ctx.clone()).delete_recursive());
                editor.incorporate(
                    rgsoconcrete
                        .get_outputint_slot()
                        .edit(ctx.clone())
                        .delete_recursive(),
                );
                follow_and_delete_downstream(
                    &rgsoconcrete.get_outputint_slot().get_downstreamsteps_slot(),
                    editor,
                    ctx.clone(),
                );
            }
            ImplStepVariantTraitObject::ImplStepIf(rgsoconcrete) => {
                editor.incorporate(rgsoconcrete.edit(ctx.clone()).delete_recursive());
                if let Some(output) = rgsoconcrete.get_output_slot().first() {
                    editor.incorporate(output.edit(ctx.clone()).delete_recursive());
                    follow_and_delete_downstream(
                        &output.get_downstreamsteps_slot(),
                        editor,
                        ctx.clone(),
                    );
                }
            }
            ImplStepVariantTraitObject::ImplStepMathSubtract(rgsoconcrete) => {
                editor.incorporate(rgsoconcrete.edit(ctx.clone()).delete_recursive());
                editor.incorporate(
                    rgsoconcrete
                        .get_outputint_slot()
                        .edit(ctx.clone())
                        .delete_recursive(),
                );
                follow_and_delete_downstream(
                    &rgsoconcrete.get_outputint_slot().get_downstreamsteps_slot(),
                    editor,
                    ctx.clone(),
                );
            }
            ImplStepVariantTraitObject::ImplStepCollectionFilter(rgsoconcrete) => {
                editor.incorporate(rgsoconcrete.edit(ctx.clone()).delete_recursive());
                editor.incorporate(
                    rgsoconcrete
                        .get_outputcollection_slot()
                        .edit(ctx.clone())
                        .delete_recursive(),
                );
                follow_and_delete_downstream(
                    &rgsoconcrete
                        .get_outputcollection_slot()
                        .get_downstreamsteps_slot(),
                    editor,
                    ctx.clone(),
                );
            }
            ImplStepVariantTraitObject::ImplStepMutateSlot(rgsoconcrete) => {
                editor.incorporate(rgsoconcrete.edit(ctx.clone()).delete_recursive());
            }
            ImplStepVariantTraitObject::ImplStepMapToOutput(rgsoconcrete) => {
                editor.incorporate(rgsoconcrete.edit(ctx.clone()).delete_recursive());
            }
            ImplStepVariantTraitObject::ImplStepCompareGreaterThan(rgsoconcrete) => {
                editor.incorporate(rgsoconcrete.edit(ctx.clone()).delete_recursive());
                editor.incorporate(
                    rgsoconcrete
                        .get_outputbool_slot()
                        .edit(ctx.clone())
                        .delete_recursive(),
                );
                follow_and_delete_downstream(
                    &rgsoconcrete
                        .get_outputbool_slot()
                        .get_downstreamsteps_slot(),
                    editor,
                    ctx.clone(),
                );
            }
            ImplStepVariantTraitObject::ImplStepGetField(rgsoconcrete) => {
                editor.incorporate(rgsoconcrete.edit(ctx.clone()).delete_recursive());
                editor.incorporate(
                    rgsoconcrete
                        .get_outputfield_slot()
                        .edit(ctx.clone())
                        .delete_recursive(),
                );
                follow_and_delete_downstream(
                    &rgsoconcrete
                        .get_outputfield_slot()
                        .get_downstreamsteps_slot(),
                    editor,
                    ctx.clone(),
                );
            }
            ImplStepVariantTraitObject::ImplStepMutateField(rgsoconcrete) => {
                editor.incorporate(rgsoconcrete.edit(ctx.clone()).delete_recursive());
            }
            ImplStepVariantTraitObject::ImplStepMultiTypeSplitter(rgsoconcrete) => {
                editor.incorporate(rgsoconcrete.edit(ctx.clone()).delete_recursive());
                editor.incorporate(
                    rgsoconcrete
                        .get_output_slot()
                        .edit(ctx.clone())
                        .delete_recursive(),
                );
                follow_and_delete_downstream(
                    &rgsoconcrete.get_output_slot().get_downstreamsteps_slot(),
                    editor,
                    ctx.clone(),
                );
            }
            ImplStepVariantTraitObject::ImplStepCompareLessThan(rgsoconcrete) => {
                editor.incorporate(rgsoconcrete.edit(ctx.clone()).delete_recursive());
                editor.incorporate(
                    rgsoconcrete
                        .get_outputbool_slot()
                        .edit(ctx.clone())
                        .delete_recursive(),
                );
                follow_and_delete_downstream(
                    &rgsoconcrete
                        .get_outputbool_slot()
                        .get_downstreamsteps_slot(),
                    editor,
                    ctx.clone(),
                );
            }
            ImplStepVariantTraitObject::ImplStepCollectionMap(rgsoconcrete) => {
                editor.incorporate(rgsoconcrete.edit(ctx.clone()).delete_recursive());
                editor.incorporate(
                    rgsoconcrete
                        .get_outputcollection_slot()
                        .edit(ctx.clone())
                        .delete_recursive(),
                );
                follow_and_delete_downstream(
                    &rgsoconcrete
                        .get_outputcollection_slot()
                        .get_downstreamsteps_slot(),
                    editor,
                    ctx.clone(),
                );
            }
            ImplStepVariantTraitObject::ImplStepMapFromInput(rgsoconcrete) => {
                editor.incorporate(rgsoconcrete.edit(ctx.clone()).delete_recursive());
                editor.incorporate(
                    rgsoconcrete
                        .get_output_slot()
                        .edit(ctx.clone())
                        .delete_recursive(),
                );
                follow_and_delete_downstream(
                    &rgsoconcrete.get_output_slot().get_downstreamsteps_slot(),
                    editor,
                    ctx.clone(),
                );
            }
            ImplStepVariantTraitObject::ImplStepMathMultiply(rgsoconcrete) => {
                editor.incorporate(rgsoconcrete.edit(ctx.clone()).delete_recursive());
                editor.incorporate(
                    rgsoconcrete
                        .get_outputint_slot()
                        .edit(ctx.clone())
                        .delete_recursive(),
                );
                follow_and_delete_downstream(
                    &rgsoconcrete.get_outputint_slot().get_downstreamsteps_slot(),
                    editor,
                    ctx.clone(),
                );
            }
            ImplStepVariantTraitObject::ImplStepTraverseSlot(rgsoconcrete) => {
                editor.incorporate(rgsoconcrete.edit(ctx.clone()).delete_recursive());
                editor.incorporate(
                    rgsoconcrete
                        .get_outputoperatives_slot()
                        .edit(ctx.clone())
                        .delete_recursive(),
                );
                follow_and_delete_downstream(
                    &rgsoconcrete
                        .get_outputoperatives_slot()
                        .get_downstreamsteps_slot(),
                    editor,
                    ctx.clone(),
                );
            }
            ImplStepVariantTraitObject::ImplStepBitOr(rgsoconcrete) => {
                editor.incorporate(rgsoconcrete.edit(ctx.clone()).delete_recursive());
                editor.incorporate(
                    rgsoconcrete
                        .get_outputbool_slot()
                        .edit(ctx.clone())
                        .delete_recursive(),
                );
                follow_and_delete_downstream(
                    &rgsoconcrete
                        .get_outputbool_slot()
                        .get_downstreamsteps_slot(),
                    editor,
                    ctx.clone(),
                );
            }
        }
    }
}
