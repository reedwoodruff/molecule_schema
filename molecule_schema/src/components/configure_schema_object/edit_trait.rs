use leptos::{*};
use serde_types::{common::Uid, primitives::PrimitiveTypes};

use crate::{
    components::{
        app::SchemaContext,
        common::{select_input::SelectInputEnum, text_input::TextInput},
    },
    utils::reactive_types::RTraitMethodDef,
};

#[component]
pub fn EditTrait(id: RwSignal<Uid>) -> impl IntoView {
    let ctx = use_context::<SchemaContext>().unwrap();
    let schema_clone_1 = ctx.schema.clone();

    let trait_info = create_memo(move |_| {
        ctx.schema
            .traits
            .with(|traits| traits.get(&id.get()).unwrap().clone())
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
                <div>Name: <TextInput value=trait_info.get().tag.name/></div>


            </div>

                <div class="flex-grow margin-right border-right">
                    <h4>Methods</h4>
                    <For each=trait_info.get().methods key=|(method_id, _method)| *method_id let:method>
                       <div><TextInput value=method.1.tag.name /> " -> " <SelectInputEnum  value=method.1.return_type />
                        <button on:click=move |_|{trait_info.get().methods.update(|prev_methods| {prev_methods.remove(&method.0);});}>Delete Method</button>
                    </div>
                    </For>

                <div>
            <button on:click=move|_| trait_info.get().methods.update(|methods| {
                    let new_method = RTraitMethodDef::<PrimitiveTypes>::new();
                    methods.insert(new_method.tag.id.get(), new_method);
                })
            >Add method</button>
            </div>

                </div>
        </div>
    }
}
