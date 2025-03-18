use std::collections::HashMap;

use crate::components::{common::*, workspace::WorkspaceState};
use leptos::prelude::*;
use schema_editor_generated_toolkit::prelude::*;

#[component]
pub fn OperativeTraitImplementations(
    operative: RGSOConcrete<OperativeConcrete, Schema>,
) -> impl IntoView {
    let ctx = use_context::<SharedGraph<Schema>>().unwrap();
    let WorkspaceState { schema, .. } = use_context::<WorkspaceState>().unwrap();

    let operative_clone = operative.clone();
    let ancestor_ops_with_trait_impls = Memo::new(move |_| {
        let mut ancestor_ops_with_trait_impls = vec![];
        let mut next_operative = operative_clone.get_parentoperative_slot().first().cloned();
        while let Some(confirmed_parent_op) = next_operative {
            if !confirmed_parent_op.get_traitimpls_slot().is_empty() {
                ancestor_ops_with_trait_impls.push(confirmed_parent_op.clone());
            }
            next_operative = confirmed_parent_op
                .get_parentoperative_slot()
                .first()
                .cloned();
        }
        ancestor_ops_with_trait_impls
    });

    let operative_clone = operative.clone();
    let schema_clone = schema.clone();
    let trait_impl_options = Memo::new(move |_| {
        schema_clone
            .get()
            .get_traits_slot()
            .into_iter()
            .filter_map(|trait_def| {
                if !operative_clone
                    .get_traitimpls_slot()
                    .iter()
                    .any(|trait_impl| {
                        trait_impl.get_traitdefinition_slot().get_id() == trait_def.get_id()
                    })
                {
                    Some(trait_def)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    });
    let currently_impling_trait =
        RwSignal::<Option<RGSOConcrete<TraitConcrete, Schema>>>::new(None);
    let tentative_impling_trait =
        RwSignal::<Option<RGSOConcrete<TraitConcrete, Schema>>>::new(None);

    let trait_impl_method_impl_selections = RwSignal::new(HashMap::<
        u128,
        RwSignal<Option<RGSOConcrete<MethodImplementation, Schema>>>,
    >::new());

    let ctx_clone = ctx.clone();
    let ctx_clone_2 = ctx.clone();
    let operative_clone_7 = operative.clone();
    let operative_clone_8 = operative.clone();
    let operative_clone_9 = operative.clone();
    let operative_clone_10 = operative.clone();
    view! {
        <SubSection>
            <SubSectionHeader>"Add A New Trait Implementation"</SubSectionHeader>
            <Show when=move || currently_impling_trait.get().is_none()>
                <SignalSelectRGSOWithOptions
                    options=trait_impl_options
                    value=tentative_impling_trait
                />
                <Button on:click=move |_| {
                    if let Some(tentative) = tentative_impling_trait.get() {
                        currently_impling_trait.set(Some(tentative));
                        tentative_impling_trait.set(None);
                    }
                }>"Begin Implementation"</Button>
            </Show>
            <Show when=move || {
                currently_impling_trait.get().is_some()
            }>
                {
                    let ctx_clone = ctx_clone.clone();
                    let operative_clone_9 = operative_clone_9.clone();
                    let operative_clone_10 = operative_clone_10.clone();
                    let trait_def = currently_impling_trait.get().unwrap();
                    let required_method_defs = trait_def.get_requiredmethods_slot();
                    let required_method_defs_clone = required_method_defs.clone();
                    let is_trait_fulfilled = move || {
                        required_method_defs_clone
                            .iter()
                            .all(|method_def| {
                                trait_impl_method_impl_selections
                                    .get()
                                    .get(method_def.get_id())
                                    .unwrap()
                                    .get()
                                    .is_some()
                            })
                    };
                    let is_trait_fulfilled_clone = is_trait_fulfilled.clone();
                    let trait_def_clone = trait_def.clone();
                    let required_method_defs_clone = required_method_defs.clone();
                    let trait_impl_name = RwSignal::new(
                        operative_clone_9.get_name() + &trait_def_clone.get_name(),
                    );
                    let trait_impl_documentation = RwSignal::new(String::new());
                    let on_save = move || {
                        if !is_trait_fulfilled() {
                            return;
                        }
                        let mut editor = operative_clone_10.edit(ctx_clone.clone());
                        editor.add_temp_traitimpls("new_trait_impl");
                        let new_trait_impl = TraitImplementation::new(ctx_clone.clone())
                            .set_temp_id("new_trait_impl")
                            .set_name(trait_impl_name.get())
                            .set_documentation(trait_impl_documentation.get())
                            .add_existing_traitdefinition(trait_def_clone.get_id(), |na| na)
                            .add_existing_implingoperative(operative_clone_10.get_id(), |na| na);
                        required_method_defs_clone
                            .iter()
                            .for_each(|method_def| {
                                editor
                                    .incorporate(
                                        &new_trait_impl
                                            .clone()
                                            .add_existing_methodimplementations(
                                                trait_impl_method_impl_selections
                                                    .get()
                                                    .get(method_def.get_id())
                                                    .unwrap()
                                                    .get()
                                                    .unwrap()
                                                    .get_id(),
                                                |na| na,
                                            ),
                                    );
                            });
                        editor.execute().unwrap();
                        currently_impling_trait.set(None);
                    };

                    view! {
                        <LeafSection>
                            "Implementation Name: " <SignalTextInput value=trait_impl_name /> <ul>
                                <For
                                    each=move || {
                                        trait_def.get_requiredmethods_slot().into_iter().enumerate()
                                    }
                                    key=move |item| item.1.get_id().clone()
                                    let:method_def
                                >
                                    {
                                        let operative_clone_9 = operative_clone_9.clone();
                                        let select_signal = RwSignal::<
                                            Option<RGSOConcrete<MethodImplementation, Schema>>,
                                        >::new(None);
                                        let method_def_clone = method_def.1.clone();
                                        let method_def_clone_2 = method_def.1.clone();
                                        trait_impl_method_impl_selections
                                            .update(|prev| {
                                                prev.insert(method_def.1.get_id().clone(), select_signal);
                                            });
                                        let method_impl_options = Memo::new(move |_| {
                                            let mut method_impl_options = vec![];
                                            let local_impl = operative_clone_9
                                                .get_methodimpls_slot()
                                                .iter()
                                                .find(|method_impl| {
                                                    method_impl.get_definition_slot().get_id().clone()
                                                        == method_def_clone_2.get_id().clone()
                                                })
                                                .cloned();
                                            if let Some(local_impl) = local_impl {
                                                method_impl_options.push(local_impl.clone());
                                            }
                                            let mut ancestor_impls = vec![];
                                            let mut current_op = operative_clone_9
                                                .get_parentoperative_slot()
                                                .first()
                                                .cloned();
                                            while let Some(parent) = current_op {
                                                let parent_impl = parent
                                                    .get_methodimpls_slot()
                                                    .iter()
                                                    .find(|method_impl| {
                                                        method_impl.get_definition_slot().get_id().clone()
                                                            == method_def.1.get_id().clone()
                                                    })
                                                    .cloned();
                                                if let Some(parent_impl) = parent_impl {
                                                    ancestor_impls.push(parent_impl.clone());
                                                }
                                                current_op = parent
                                                    .get_parentoperative_slot()
                                                    .first()
                                                    .cloned();
                                            }
                                            method_impl_options.extend(ancestor_impls);
                                            method_impl_options
                                        });
                                        view! {
                                            <li>
                                                <span>{method_def_clone.get_name()}</span>
                                                <SignalSelectRGSOWithOptions
                                                    options=method_impl_options
                                                    value=select_signal
                                                    empty_allowed=true
                                                />
                                                " "
                                                {move || {
                                                    if select_signal.get().is_some() { "✅" } else { "☒" }
                                                }}
                                            </li>
                                        }
                                    }
                                </For>
                            </ul>
                            <Button
                                on:click=move |_| { on_save() }
                                prop:disabled=move || !is_trait_fulfilled_clone()
                            >
                                Save
                            </Button>
                            <Button on:click=move |_| {
                                currently_impling_trait.set(None);
                            }>"Cancel"</Button>
                        </LeafSection>
                    }
                }
            </Show>

        </SubSection>
        <SubSection>
            <SubSectionHeader>"Current Local Trait Implementations"</SubSectionHeader>
            <ul>
                <For

                    each=move || operative_clone_8.get_traitimpls_slot()
                    key=|item| item.get_id().clone()
                    children=move |trait_impl| {
                        let trait_impl_clone = trait_impl.clone();
                        let ctx_clone_2 = ctx_clone_2.clone();
                        view! {
                            <li>
                                {trait_impl_clone.get_traitdefinition_slot().get_name()}" "
                                <Button on:click=move |_| {
                                    trait_impl_clone
                                        .edit(ctx_clone_2.clone())
                                        .delete_recursive()
                                        .execute()
                                        .unwrap();
                                }>Delete</Button>
                            </li>
                        }
                    }
                />
            </ul>
        </SubSection>
        <SubSection>
            <SubSectionHeader>"Current Ancestor Trait Implementations"</SubSectionHeader>
            <For

                each=move || ancestor_ops_with_trait_impls.get()
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
                                    each=move || ancestor_op_clone.get_traitimpls_slot()
                                    key=|item| item.get_id().clone()
                                    let:trait_impl
                                >
                                    <li>{trait_impl.get_traitdefinition_slot().get_name()}</li>
                                </For>
                            </ul>
                        </LeafSection>
                    }
                }
            />
        </SubSection>
    }
}
