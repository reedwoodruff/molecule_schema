use leptos::{ev::submit, logging::log, *};
use serde_types::{
    common::Uid,
    primitives::{PrimitiveTypes, PrimitiveValues},
};
use web_sys::SubmitEvent;

use crate::{
    components::{common::text_input::TextInput, SchemaContext},
    utils::reactive_types::{
        RFieldConstraint, RLibraryInstance, RLibraryOperative, RTag, RTraitMethodImplPath,
    },
};

use super::tree_view::TreeRef;

#[component]
pub fn EditSchemaObject(element: TreeRef) -> impl IntoView {
    let ctx = use_context::<SchemaContext>().unwrap();

    let active_object = create_memo(move |_| {
        ctx.schema
            .constraint_objects
            .with(|co| co.get(&element.1).cloned())
            .unwrap()
    });

    let field_constraints = move || active_object.get().field_constraints.get();

    let add_field = move |_| {
        let new_field = RFieldConstraint::<PrimitiveTypes> {
            tag: RTag::new("New Field".to_string()),
            value_type: RwSignal::new(PrimitiveTypes::String),
        };
        active_object().field_constraints.update(|prev| {
            prev.push(new_field);
        });
    };

    let constituent_instances = move || {
        active_object
            .get()
            .instances
            .get()
            .iter()
            .map(|instance_id| {
                // log!("{}", instance_id);
                ctx.schema
                    .instance_library
                    .get()
                    .get(instance_id)
                    .cloned()
                    .unwrap()
            })
            .collect::<Vec<_>>()
    };
    let constituent_library_operatives = move || {
        active_object
            .get()
            .library_operatives
            .get()
            .iter()
            .map(|library_operative_id| {
                ctx.schema
                    .operative_library
                    .get()
                    .get(library_operative_id)
                    .cloned()
                    .unwrap()
            })
            .collect::<Vec<_>>()
    };
    let constituent_trait_operatives = move || {
        active_object
            .get()
            .trait_operatives
            .get()
            .iter()
            .map(|trait_operative| {
                (
                    trait_operative.clone(),
                    ctx.schema
                        .traits
                        .get()
                        .get(&trait_operative.trait_id.get())
                        .cloned()
                        .unwrap(),
                )
            })
            .collect::<Vec<_>>()
    };
    let trait_impls = move || {
        active_object
            .get()
            .trait_impls
            .get()
            .clone()
            .iter()
            .map(|(trait_id, trait_methods)| {
                (
                    trait_methods.clone(),
                    ctx.schema.traits.get().get(&trait_id).cloned().unwrap(),
                )
            })
            .collect::<Vec<_>>()
    };

    let on_click_create_operative = move |_| {
        let new_operative =
            RLibraryOperative::<PrimitiveTypes, PrimitiveValues>::new(element.1, None);
        ctx.schema.operative_library.update(|lib| {
            lib.insert(new_operative.tag.id.get(), new_operative);
        });
    };
    let on_click_create_instance = move |_| {
        let new_instance =
            RLibraryInstance::<PrimitiveTypes, PrimitiveValues>::new(element.1, None);
        ctx.schema.instance_library.update(|lib| {
            lib.insert(new_instance.tag.id.get(), new_instance);
        });
    };

    view! {
        <div class="large-margin med-pad border-gray flex">
            <div>
            <button on:click= move |_| ctx.selected_element.set(None)>X</button>
            <br/>
            <button on:click=on_click_create_operative>Create Operative</button>
            <br/>
            <button on:click=on_click_create_instance>Create Instance</button>

            </div>

            <div class="flex-grow">
            <h4>Name </h4>
                <div class="flex">
                    <TextInput
                        initial_value=active_object.get().tag.name.get()
                        on_save= move |val: String| {
                            active_object.get().tag.name.set(val);
                    }/>
                </div>
            </div>

            <div class="flex-grow">
            <h4>Fields <button on:click=add_field>+</button></h4>
            <For
                each=field_constraints
                key=move |item| item.tag.id
                let:item
            >
                <div class="flex">
                <TextInput
                    initial_value=item.tag.name.get()
                    on_save= move |val: String| {
                        item.tag.name.set(val);
                }/>
                <select>

                </select>
                </div>
            </For>
            </div>

            <div class="flex-grow">
            <h4>Constituents</h4>
            <strong>Instances <button on:click=add_field>+</button></strong>
            <For
                each=constituent_instances
                key= move |item| item.tag.id
                children=move |item| {
                    view!{<div>
                    {item.tag.name}
                        </div>
                    }
                }
            />
            <br/>

            <strong>Library Operatives <button on:click=add_field>+</button></strong>
            <For
                each=constituent_library_operatives
                key= move |item| item.tag.id
                children=move |item| {
                    view!{<div>
                    {item.tag.name}
                        </div>
                    }
                }
            />
            <br/>


            <strong>Trait Operatives <button on:click=add_field>+</button></strong>
            <For
                each=constituent_trait_operatives
                key= move |(trait_operative, trait_def)| trait_operative.tag.id
                children=move |(trait_operative, trait_def)| {
                    view!{<div>
                    operative name: {trait_operative.tag.name}
                    <br/>
                    trait name: {trait_def.tag.name}
                        </div>
                    }
                }
            />
            <br/>

            </div>
            <div class="flex-grow">
            <h4>Trait Impls<button on:click=add_field>+</button></h4>
            <For
                each=trait_impls
                key= move |(methods, trait_def)| trait_def.tag.id
                children=move |(methods, trait_def)| {
                    view!{<div>
                    trait name: {trait_def.tag.name}
                        <br/>
                    trait methods:
                            <For each=methods key=move |(method_id, path)| *method_id
                                children=move |(method_id, path)| {
                                let method_def = trait_def.methods.get().iter().find(|method| method.tag.id.get() == method_id).cloned().unwrap();
                                let method_path = path.get().iter().map(|path_item| {
                                                match path_item {
                                                RTraitMethodImplPath::Field(item) => {
                                                "Field".to_string()
                                                }
                                                RTraitMethodImplPath::InstanceConstituent(item) => {
                                                "Instance".to_string()
                                                }
                                                RTraitMethodImplPath::LibraryOperativeConstituent(item) => {
                                                "LibraryOperative".to_string()
                                                }
                                                RTraitMethodImplPath::TraitOperativeConstituent{trait_operative_id, ..} => {
                                                "TraitOperative".to_string()
                                                }
                                                RTraitMethodImplPath::TraitMethod{trait_method_id, trait_id} => {
                                                "TraitMethod".to_string()
                                                }
                                                }
                                                }).collect::<Vec<String>>().join("::");
                                view!{{method_def.tag.name}<br/>{method_path}}
                                }
                                />
                        </div>
                    }
                }
            />
            </div>
        </div>
        // <Show when=move || field_constraints.is_some()>
        // </Show>
    }
}
