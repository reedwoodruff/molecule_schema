use base_types::common::Uid;
use base_types::primitives::PrimitiveTypes;
use leptos::prelude::*;

use crate::components::{
    app::SchemaContext,
    common::{select_input::SelectInputEnum, text_input::TextInput},
};

use crate::reactive_types::reactive_types::RTraitMethodDef;
#[component]
pub fn EditTrait(id: RwSignal<Uid>) -> impl IntoView {
    let ctx = use_context::<SchemaContext>().unwrap();
    let schema_clone_1 = ctx.schema.clone();

    let trait_info = Memo::new(move |_| {
        ctx.schema
            .traits
            .with(|traits| traits.get(&id.get()).unwrap().clone())
    });
    let templates_which_impl = Memo::new(move |_| {
        ctx.schema.template_library.with(|templates| {
            templates
                .values()
                .filter(|template| template.trait_impls.get().contains_key(&id.get()))
                .cloned()
                // .map(|template| template.tag.name.get())
                .collect::<Vec<_>>()
        })
    });
    let operatives_which_impl = Memo::new(move |_| {
        ctx.schema.operative_library.with(|operatives| {
            operatives
                .values()
                .filter(|operative| operative.trait_impls.get().contains_key(&id.get()))
                .cloned()
                // .map(|operative| operative.tag.name.get())
                .collect::<Vec<_>>()
        })
    });
    // let name = RwSignal::new();
    view! {
        <div class="large-margin med-pad border-gray flex">
            <div class="flex-grow margin-right border-right">
                <button on:click=move |_| ctx.selected_element.set(None)>X</button>
                <button on:click=move |_| {
                    ctx.selected_element.set(None);
                    schema_clone_1
                        .traits
                        .update(|prev| {
                            prev.remove(&id.get());
                        })
                }>delete trait</button>
                <div>Name: <TextInput value=trait_info.get().tag.name /></div>

            </div>

            <div class="flex-grow margin-right border-right">
                <h4>Methods</h4>
                <For
                    each=move || trait_info.get().methods.get()
                    key=|(method_id, _method)| *method_id
                    let:method
                >
                    <div>
                        <TextInput value=method.1.tag.name />
                        " -> "
                        <SelectInputEnum value=method.1.return_type />
                        <button on:click=move |_| {
                            trait_info
                                .get()
                                .methods
                                .update(|prev_methods| {
                                    prev_methods.remove(&method.0);
                                });
                        }>Delete Method</button>
                    </div>
                </For>

                <div>
                    <button on:click=move |_| {
                        trait_info
                            .get()
                            .methods
                            .update(|methods| {
                                let new_method = RTraitMethodDef::<PrimitiveTypes>::new();
                                methods.insert(new_method.tag.id.get(), new_method);
                            })
                    }>Add method</button>
                </div>

            </div>
            <div class="flex-grow margin-right border-right">
                <h4>"Templates which impl this trait"</h4>
                <div>
                    <ul>
                        <For
                            each=move || templates_which_impl.get()
                            key=|item| item.tag.id.clone()
                            let:template
                        >
                            <li>{move || template.tag.name}</li>
                        </For>
                    </ul>
                </div>
                <h4>"Operatives which impl this trait"</h4>
                <div>
                    <ul>
                        <For
                            each=move || operatives_which_impl.get()
                            key=|item| item.tag.id.clone()
                            let:operative
                        >
                            <li>{move || operative.tag.name}</li>
                        </For>
                    </ul>
                </div>
            </div>
        </div>
    }
}
