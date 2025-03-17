use crate::components::{
    common::*,
    workspace::{WorkspaceState, WorkspaceTab},
};
use schema_editor_generated_toolkit::prelude::*;

#[component]
pub fn TraitEditor(trait_concrete: RwSignal<RGSOConcrete<TraitConcrete, Schema>>) -> impl IntoView {
    let ctx = use_context::<SharedGraph<Schema>>().unwrap();
    let WorkspaceState {
        selected_tab,
        schema,
    } = use_context::<WorkspaceState>().unwrap();

    let ctx_clone = ctx.clone();
    let trait_concrete_clone = trait_concrete.clone();
    let delete_trait_concrete = move |_| {
        let ctx_clone = ctx_clone.clone();
        trait_concrete_clone
            .get()
            .edit(ctx_clone)
            .delete()
            .execute()
            .unwrap();
        selected_tab.set(WorkspaceTab::Template(RwSignal::new(None)))
    };
    let trait_concrete_clone = trait_concrete.clone();
    let ctx_clone = ctx.clone();
    let update_name = move |new_val: String| {
        let mut editor = trait_concrete_clone.get().edit(ctx_clone.clone());
        editor.set_name(new_val).execute().unwrap();
    };
    let trait_concrete_clone = trait_concrete.clone();
    let ctx_clone = ctx.clone();
    let update_documentation = move |new_val: String| {
        let mut editor = trait_concrete_clone.get().edit(ctx_clone.clone());
        editor.set_documentation(new_val).execute().unwrap();
    };
    let schema_clone = schema.clone();
    let trait_concrete_clone = trait_concrete.clone();
    let selected_fn_def = RwSignal::<Option<RGSOConcrete<FunctionDefinition, Schema>>>::new(None);
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
                    ) && !trait_concrete_clone
                        .get()
                        .get_requiredmethods_slot()
                        .contains(&fn_def)
                })
            })
            .collect::<Vec<_>>()
    });

    let ctx_clone = ctx.clone();
    let ctx_clone_2 = ctx.clone();
    let trait_concrete_clone = trait_concrete.clone();
    let trait_concrete_clone_2 = trait_concrete.clone();
    let trait_concrete_clone_3 = trait_concrete.clone();

    view! {
        <div>
            <Section>
                <SectionHeader slot>Overview</SectionHeader>
                <SubSection>
                    <SubSectionHeader>
                        "Name: "
                        <ToggleManagedTextInput
                            getter=move || trait_concrete.get().get_name_field()
                            setter=update_name
                        />
                    </SubSectionHeader>
                    <ToggleManagedDocumentationInput
                        getter=move || trait_concrete.get().get_documentation_field()
                        setter=update_documentation
                    />
                </SubSection>
                <SubSection>
                    <Button on:click=delete_trait_concrete>Delete Item</Button>
                </SubSection>
            </Section>
            <Section>
                <SectionHeader slot>Required Methods</SectionHeader>
                <SubSection>
                    <SubSectionHeader>Add Method</SubSectionHeader>

                    <SignalSelectRGSOWithOptions
                        value=selected_fn_def
                        options=Signal::derive(move || fn_def_options.get())
                        empty_allowed=true
                    />
                    <Button on:click=move |_| {
                        if let Some(selected_method) = selected_fn_def.get() {
                            let mut editor = trait_concrete_clone_3.get().edit(ctx_clone_2.clone());
                            editor
                                .add_existing_requiredmethods(selected_method.get_id(), |na| na)
                                .execute()
                                .unwrap();
                        } else {
                            return
                        }
                    }>Add Method to Trait</Button>

                </SubSection>

                <SubSection>
                    <SubSectionHeader>Already Required Methods</SubSectionHeader>
                    <For
                        each=move || trait_concrete_clone.get().get_requiredmethods_slot()
                        key=move |item| item.get_id().clone()
                        let:method
                    >
                        {
                            let method_id = method.get_id().clone();
                            let ctx_clone = ctx_clone.clone();
                            let trait_concrete_clone_2 = trait_concrete_clone_2.clone();
                            view! {
                                <div>
                                    {move || method.get_name()}
                                    <Button on:click=move |_| {
                                        let mut editor = trait_concrete_clone_2
                                            .get()
                                            .edit(ctx_clone.clone());
                                        editor.remove_from_requiredmethods(&method_id);
                                        editor.execute().unwrap();
                                    }>Delete Method</Button>
                                </div>
                            }
                        }
                    </For>
                </SubSection>
            </Section>
        </div>
    }
}
