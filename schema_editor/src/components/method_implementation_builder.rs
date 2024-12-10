use crate::components::common::*;
use schema_editor_generated_toolkit::prelude::*;

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
            .set_name(func_impl_name.get());
        on_save.run(Box::new(hairy_boy));
    };

    // If this is editing an existing impl, set the local signals to match the existing signal at the start
    if let Some(initial_state) = initial_state {
        func_impl_name.set(initial_state.get_name());
    }

    let fn_def_clone = fn_def.clone();
    view! {
        <LeafSection>
            Implementation name: <SignalTextInput value=func_impl_name />
            <LeafSection>
                "Implementation of " <strong>{move || fn_def_clone.get_name()}</strong>
            </LeafSection>
        </LeafSection>
        <div>
            <Button on:click=inner_on_save>Save</Button>
            " "
            <Button on:click=move |_| on_cancel.run(())>Cancel</Button>
        </div>
    }
}
