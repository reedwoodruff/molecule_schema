use crate::components::{
    common::*, method_impl_step_builder::MethodImplementationStepBuilder,
    method_impl_utils::ExecutionSteps,
};
use leptos::context::Provider;
use schema_editor_generated_toolkit::prelude::*;

#[derive(Clone, Debug)]
pub struct MethodImplBuilderContext {
    pub impling_operative: RGSOConcrete<OperativeConcrete, Schema>,
}

#[component]
pub fn MethodImplementationBuilder(
    fn_def: RGSOConcrete<FunctionDefinition, Schema>,
    operative: RGSOConcrete<OperativeConcrete, Schema>,

    // Will return an executable which contains the new MethodImplementation with a temp_id of "new_fn_impl"
    on_save: Callback<Box<dyn Incorporatable<MethodImplementation, Schema>>>,
    on_cancel: Callback<()>,
    #[prop(optional)] initial_state: Option<RGSOConcrete<MethodImplementation, Schema>>,
) -> impl IntoView {
    let ctx = use_context::<SharedGraph<Schema>>().unwrap();
    let ctx_clone = ctx.clone();

    let func_impl_name = RwSignal::new(fn_def.get_name() + "_impl");

    let fn_def_clone = fn_def.clone();
    let operative_clone = operative.clone();

    let inner_on_save = move |_| {
        let hairy_boy = MethodImplementation::new(ctx_clone.clone())
            .set_temp_id("new_fn_impl")
            .add_existing_definition(fn_def_clone.get_id(), |na| na)
            .add_existing_implementor(operative_clone.get_id(), |na| na)
            // .add_new_inititialsteps(|init_step| init_step.add_existing_input(existing_item_id, builder_closure))
            .set_name(func_impl_name.get());
        on_save.run(Box::new(hairy_boy));
    };

    let fn_def_clone = fn_def.clone();
    let step_lists =
        RwSignal::<Vec<RwSignal<Vec<ExecutionSteps>>>>::new(vec![RwSignal::new(vec![])]);

    // If this is editing an existing impl, set the local signals to match the existing signal at the start
    if let Some(initial_state) = initial_state {
        func_impl_name.set(initial_state.get_name());
    } else {
        let initial_steps = fn_def_clone
            .get_inputs_slot()
            .into_iter()
            .map(|input| RwSignal::new(vec![ExecutionSteps::MapFromInput { input: input }]))
            .collect::<Vec<_>>();
        step_lists.set(initial_steps);
    }

    let fn_def_clone = fn_def.clone();
    let fn_def_clone_2 = fn_def.clone();
    let operative_clone = operative.clone();
    view! {
        <Provider value=MethodImplBuilderContext {
            impling_operative: operative_clone,
        }>
            <LeafSection>
                Implementation name: <SignalTextInput value=func_impl_name />
                <LeafSection>
                    "Implementation of " <strong>{move || fn_def_clone.get_name()}</strong>
                </LeafSection>
            </LeafSection>
            <LeafSection>
                <LeafSectionHeader>Function Inputs</LeafSectionHeader>
                <For
                    each=move || step_lists.get()
                    key=|item| item.clone()
                    children=move |input_row| {
                        view! {
                            <div class="flex">
                                <For
                                    each=move || input_row.get()
                                    key=|item| item.clone()
                                    children=move |step| {
                                        let step_clone = step.clone();
                                        view! {
                                            <MethodImplementationStepBuilder step=Signal::derive(move ||
                                            step_clone.clone()) />
                                        }
                                    }
                                />
                            </div>
                        }
                    }
                />
            </LeafSection>
            // <LeafSection>
            // <LeafSectionHeader>Function Outputs</LeafSectionHeader>
            // <For
            // each=move || fn_def.get_outputs_slot()
            // key=|item| item.get_id().clone()
            // children=move |entry| {
            // let discriminant: GetNameFunctionIOTraitObjectDiscriminants = entry
            // .clone()
            // .into();
            // view! {
            // <div class="flex">
            // <div class="method-impl exit">
            // <div>{move || entry.get_name()}</div>
            // <div>{discriminant.to_string()}</div>
            // </div>
            // </div>
            // }
            // }
            // />
            // </LeafSection>

            <div>
                <Button on:click=inner_on_save>Save</Button>
                " "
                <Button on:click=move |_| on_cancel.run(())>Cancel</Button>
            </div>
        </Provider>
    }
}
