use std::{collections::HashMap, rc::Rc};

use leptos::{logging::log, *};
use serde_types::{
    common::Uid,
    primitives::{PrimitiveTypes, PrimitiveValues},
};

use crate::{
    components::{
        common::{select_input::SelectInputOptional, text_input::TextInput},
        tree_view_revamp::{TreeNodeDataSelectionType, TreeView},
        SchemaContext, TreeTypes,
    },
    utils::{
        reactive_item::RConstraintSchemaItem,
        reactive_types::{RLibraryInstance, RLibraryOperative, RTraitMethodImplPath},
    },
};

use super::super::tree_view_revamp::TreeRef;

#[component]
pub fn EditOperative(element: TreeRef) -> impl IntoView {
    let ctx = use_context::<SchemaContext>().unwrap();

    let active_object = create_memo(move |_| {
        ctx.schema
            .operative_library
            .with(|operatives| operatives.get(&element.1).cloned())
            .unwrap()
    });
    let schema_clone = ctx.schema.clone();
    let schema_clone_2 = ctx.schema.clone();
    let associated_template = create_memo(move |_| {
        ctx.schema
            .template_library
            .with(|templates| {
                templates
                    .get(&active_object.get().template_id.get())
                    .cloned()
            })
            .unwrap()
    });

    let all_field_constraints = move || associated_template.get().field_constraints.get();
    let unfulfilled_field_constraints = create_memo(move |_| {
        active_object
            .get()
            .get_all_unfulfilled_fields(&schema_clone)
    });
    let local_fulfilled_field_constraints =
        create_memo(move |_| active_object.get().get_local_fulfilled_fields());
    let ancestors_fulfilled_field_constraints = create_memo(move |_| {
        active_object
            .get()
            .get_ancestors_fulfilled_fields(&schema_clone_2)
    });

    let _parent_ops_info = create_memo(move |_| {
        let mut fulfilled_operatives = Vec::new();
        let mut fulfilled_fields = Vec::new();
        let mut next_parent_operative = active_object.get().parent_operative_id.get();
        while let Some(parent_op_id) = next_parent_operative {
            let parent_op = ctx
                .schema
                .operative_library
                .with(|ops| ops.get(&parent_op_id).unwrap().clone());
            fulfilled_operatives.extend(parent_op.fulfilled_library_operatives.get());
            fulfilled_fields.extend(parent_op.locked_fields.get());
            next_parent_operative = parent_op.parent_operative_id.get();
        }
        (fulfilled_operatives, fulfilled_fields)
    });

    let constituent_instances = move || {
        associated_template
            .get()
            .instances
            .get()
            .iter()
            .cloned()
            .chain(
                active_object
                    .get()
                    .fulfilled_library_operatives
                    .get()
                    .iter()
                    .map(|fulf_op| fulf_op.fulfilling_instance_id.get()),
            )
            .map(|instance_id| {
                ctx.schema
                    .instance_library
                    .get()
                    .get(&instance_id)
                    .cloned()
                    .unwrap()
            })
            .collect::<Vec<_>>()
    };
    let constituent_library_operatives = move || {
        associated_template
            .get()
            .library_operatives
            .get()
            .iter()
            .filter(|op_id| {
                !active_object
                    .get()
                    .fulfilled_library_operatives
                    .get()
                    .iter()
                    .map(|fulf_op| fulf_op.operative_id.get())
                    .collect::<Vec<_>>()
                    .contains(op_id)
            })
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
        associated_template
            .get()
            .trait_operatives
            .get()
            .iter()
            // .map(|trait_op| trait_op.trait_id.get())
            .filter(|op| {
                !active_object
                    .get()
                    .fulfilled_trait_operatives
                    .get()
                    .iter()
                    .map(|fulf_op| fulf_op.operative_id.get())
                    .collect::<Vec<_>>()
                    .contains(&op.trait_id.get())
            })
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
            .chain(&associated_template.get().trait_impls.get().clone())
            .map(|(trait_id, trait_methods)| {
                (
                    *trait_methods,
                    ctx.schema.traits.get().get(trait_id).cloned().unwrap(),
                )
            })
            .collect::<Vec<_>>()
    };

    let new_operative_name = RwSignal::new("new_operative".to_string());
    let new_instance_name = RwSignal::new("new_instance".to_string());

    let on_click_create_operative = move |_| {
        let new_operative = RLibraryOperative::<PrimitiveTypes, PrimitiveValues>::new(
            associated_template.get().tag.id.get(),
            Some(active_object.get().tag.id.get()),
            new_operative_name.get(),
        );
        ctx.schema.operative_library.update(|lib| {
            lib.insert(new_operative.tag.id.get(), new_operative);
        });
    };
    let on_click_create_instance = move |_| {
        let new_instance = RLibraryInstance::<PrimitiveTypes, PrimitiveValues>::new(
            associated_template.get().tag.id.get(),
            Some(active_object.get().tag.id.get()),
            new_instance_name.get(),
        );
        ctx.schema.instance_library.update(|lib| {
            lib.insert(new_instance.tag.id.get(), new_instance);
        });
    };

    let select_trait_impl_options = ctx.schema.traits.with(|lib| {
        lib.iter()
            .map(|(id, lib_item)| (*id, lib_item.tag.name.get()))
            .collect::<Vec<_>>()
    });
    let add_trait_impl_id = RwSignal::new(None);
    let TypedSelectInputTraitImplSelection = SelectInputOptional::<Uid, String, _, _>;

    let adding_trait_impl = RwSignal::new(false);
    let selecting_trait_impl_path = RwSignal::<Option<u128>>::new(None);
    let active_trait_impl_method_paths = RwSignal::new(HashMap::<
        Uid,
        (
            String,
            PrimitiveTypes,
            RwSignal<Option<Vec<RTraitMethodImplPath>>>,
        ),
    >::new());

    let on_click_tree_data = Rc::new(
        move |last_item: TreeNodeDataSelectionType,
              data_type: PrimitiveTypes,
              path: Rc<Vec<TreeRef>>| {
            log!("clicked {:?}, {:?}, {:?}", last_item, data_type, path);
            if let Some(method_id) = selecting_trait_impl_path.get() {
                let entry = active_trait_impl_method_paths
                    .get()
                    .get(&method_id)
                    .expect("method must exist")
                    .clone();
                if entry.1 == data_type {
                    let mut new_path: Vec<RTraitMethodImplPath> = path
                        .iter()
                        .filter(|item| item.0 != TreeTypes::Template)
                        .map(|item| match item.0.clone() {
                            TreeTypes::Instance => {
                                RTraitMethodImplPath::InstanceConstituent(RwSignal::new(item.1))
                            }
                            TreeTypes::LibraryOperative => {
                                RTraitMethodImplPath::LibraryOperativeConstituent(RwSignal::new(
                                    item.1,
                                ))
                            }
                            TreeTypes::TraitOperative(trait_op) => {
                                RTraitMethodImplPath::TraitOperativeConstituent {
                                    trait_method_id: RwSignal::new(method_id),
                                    trait_operative_id: RwSignal::new(trait_op.tag.id.get()),
                                    trait_id: RwSignal::new(trait_op.trait_id.get()),
                                }
                            }
                            _ => {
                                log!("strange path item");
                                RTraitMethodImplPath::Field(RwSignal::new(item.1))
                            }
                        })
                        .collect();
                    match last_item {
                        TreeNodeDataSelectionType::TraitMethod {
                            method_id,
                            trait_id,
                        } => {
                            new_path.push(RTraitMethodImplPath::TraitMethod {
                                trait_id: RwSignal::new(trait_id),
                                trait_method_id: RwSignal::new(method_id),
                            });
                        }
                        TreeNodeDataSelectionType::Field(field_id) => {
                            new_path.push(RTraitMethodImplPath::Field(RwSignal::new(field_id)));
                        }
                    }
                    entry.2.set(Some(new_path));
                    active_trait_impl_method_paths.update(|prev| {
                        prev.insert(method_id, entry);
                    });
                } else {
                    log!("incorrect data type");
                }
            }
        },
    );

    let on_select_trait_impl = move |trait_impl_id| {
        add_trait_impl_id.set(trait_impl_id);
        if let Some(trait_impl_id) = trait_impl_id {
            let trait_in_question = ctx
                .schema
                .traits
                .with(|items| items.get(&trait_impl_id).cloned())
                .expect("trait must exist");
            let mut hashmap = HashMap::new();
            trait_in_question
                .methods
                .get()
                .iter()
                .for_each(|trait_item| {
                    hashmap.insert(
                        trait_item.tag.id.get(),
                        (
                            trait_item.tag.name.get(),
                            trait_item.return_type.get(),
                            RwSignal::new(None),
                        ),
                    );
                });
            active_trait_impl_method_paths.set(hashmap);
        } else {
            active_trait_impl_method_paths.set(HashMap::new());
        }
    };

    let is_trait_impl_complete = create_memo(move |_| {
        if add_trait_impl_id.get().is_none() {
            return false;
        }
        let mut is_complete = true;
        active_trait_impl_method_paths
            .get()
            .iter()
            .for_each(|item| {
                if item.1 .2.get().is_none() {
                    is_complete = false;
                }
            });
        is_complete
    });

    let on_click_add_trait_impl = move |_| {
        if add_trait_impl_id.get().is_some() {
            let trait_impl_id = add_trait_impl_id.get().unwrap();
            if is_trait_impl_complete.get() {
                let mut trait_impl = HashMap::new();
                active_trait_impl_method_paths.get().iter().for_each(
                    |(method_id, (_, _, path))| {
                        trait_impl.insert(*method_id, RwSignal::new(path.get().unwrap()));
                    },
                );
                active_object.get().trait_impls.update(|prev| {
                    prev.insert(trait_impl_id, RwSignal::new(trait_impl));
                });
                add_trait_impl_id.set(None);
                active_trait_impl_method_paths.set(HashMap::new());
            }
        }
    };

    view! {
        <div class="large-margin med-pad border-gray flex">
            <div class="flex-grow margin-right border-right">
                <button on:click=move |_| ctx.selected_element.set(None)>X</button>
                <button on:click=move |_| {
                    ctx
                        .schema
                        .template_library
                        .update(|prev| {
                            prev.remove(&element.1);
                        })
                }>delete element</button>
                <br/>
                <TextInput
                    initial_value=new_operative_name.get()
                    on_save=move |val: String| {
                        new_operative_name.set(val);
                    }

                    show_save_button=true
                />
                <br/>
                <button on:click=on_click_create_operative>Create Operative</button>
                <br/>
                <TextInput
                    initial_value=new_instance_name.get()
                    on_save=move |val: String| {
                        new_instance_name.set(val);
                    }

                    show_save_button=true
                />
                <br/>
                <button on:click=on_click_create_instance>Create Instance</button>

            </div>

            <div class="flex-grow margin-right border-right">
                <h4>Name</h4>
                <div class="flex">
                    <TextInput
                        initial_value=active_object.get().tag.name.get()
                        on_save=move |val: String| {
                            active_object.get().tag.name.set(val);
                        }
                    />

                </div>
            </div>

            <div class="flex-grow margin-right border-right">
                <h4>Fields</h4>
                <For
                    each=ancestors_fulfilled_field_constraints
                    key=move |item| item.tag.id
                    let:item
                >
                    <div>{item.tag.name} (locked above)</div>
                </For>
                <For each=local_fulfilled_field_constraints key=move |item| item.tag.id let:item>
                    <div>
                        {item.tag.name} : {move || item.value.get().to_string()} (locked)
                        <button on:click=move |_| {
                            active_object
                                .get()
                                .locked_fields
                                .update(|prev| {
                                    let index = prev
                                        .iter()
                                        .position(|prev_item| {
                                            prev_item.tag.id.get() == item.tag.id.get()
                                        })
                                        .unwrap();
                                    prev.remove(index);
                                })
                        }>unlock</button>
                    </div>
                </For>
                <For each=unfulfilled_field_constraints key=move |item| item.tag.id let:item>
                    <div>
                        {item.tag.name}
                        {move || {
                            let initiated_locking = RwSignal::new(false);
                            let value = RwSignal::new("".to_string());
                            let item_clone = item.clone();
                            let on_click_lock = Callback::new(move |_| {
                                let new_val = match item.value_type.get() {
                                    PrimitiveTypes::I32 => {
                                        PrimitiveValues::I32(value.get().parse().unwrap())
                                    }
                                    PrimitiveTypes::U32 => {
                                        PrimitiveValues::U32(value.get().parse().unwrap())
                                    }
                                    PrimitiveTypes::F32 => {
                                        PrimitiveValues::F32(value.get().parse().unwrap())
                                    }
                                    PrimitiveTypes::String => {
                                        PrimitiveValues::String(value.get().parse().unwrap())
                                    }
                                    PrimitiveTypes::Bool => {
                                        PrimitiveValues::Bool(value.get().parse().unwrap())
                                    }
                                    PrimitiveTypes::Char => {
                                        PrimitiveValues::Char(value.get().parse().unwrap())
                                    }
                                    PrimitiveTypes::Option(_) => todo!(),
                                };
                                active_object
                                    .get()
                                    .locked_fields
                                    .update(|prev| {
                                        prev.push(item_clone.fulfill(new_val));
                                    });
                            });
                            view! {
                                <Show when=move || !initiated_locking.get()>
                                    <button on:click=move |_| {
                                        initiated_locking.set(true)
                                    }>Start Lock</button>
                                </Show>
                                <Show when=initiated_locking>
                                    <TextInput
                                        initial_value=""
                                        on_save=move |new_val| {
                                            value.set(new_val.into());
                                        }
                                    />
                                    <button on:click=on_click_lock>Lock</button>
                                    <button on:click=move |_| {
                                        initiated_locking.set(false)
                                    }>Cancel</button>
                                </Show>
                            }
                        }}

                    </div>

                </For>
            </div>

            <div class="flex-grow margin-right border-right">
                <h4>Constituents</h4>
                <strong>Instances</strong>

                <br/>

                <For
                    each=constituent_instances
                    key=move |item| item.tag.id
                    children=move |item| {
                        view! { <div>{item.tag.name} <br/></div> }
                    }
                />

                <br/>

                <strong>Library Operatives</strong>
                <br/>
                <For
                    each=constituent_library_operatives
                    key=move |item| item.tag.id
                    children=move |item| {
                        view! { <div>{item.tag.name} <br/></div> }
                    }
                />

                <br/>

                <strong>Trait Operatives</strong>
                <br/>
                <For
                    each=constituent_trait_operatives
                    key=move |(trait_operative, _trait_def)| trait_operative.tag.id
                    children=move |(trait_operative, trait_def)| {
                        view! {
                            <div>
                                operative name: {trait_operative.tag.name} <br/> trait name:
                                {trait_def.tag.name} <br/>
                            </div>
                        }
                    }
                />

                <br/>

            </div>
            <div class="flex-grow margin-right">
                <h4>
                    Trait Impls
                    <button on:click=move |_| {
                        if adding_trait_impl.get() {
                            adding_trait_impl.set(false);
                        } else {
                            adding_trait_impl.set(true);
                        }
                    }>begin addition / cancel</button>
                </h4>
                New Impl:
                <br/>
                trait:
                <TypedSelectInputTraitImplSelection
                    options=select_trait_impl_options
                    value=add_trait_impl_id
                    on_select=on_select_trait_impl
                />
                <br/>
                <For each=active_trait_impl_method_paths key=move |item| item.0 let:item>

                    {
                        let click_closure = move |_| {
                            if selecting_trait_impl_path
                                .get()
                                .is_some_and(|selected| selected == item.0)
                            {
                                selecting_trait_impl_path.set(None);
                            } else {
                                selecting_trait_impl_path.set(Some(item.0));
                            }
                        };
                        view! {
                            <div class=move || {
                                if item.1.2.get().is_some() { "bg-light-green" } else { "" }
                            }>{item.1.0} : {item.1.1.to_string()}</div>
                            <button
                                disabled=move || add_trait_impl_id.get().is_none()
                                on:click=click_closure
                            >
                                Click Here and then select in the graph view
                            </button>
                        }
                    }

                </For>
                <br/>
                <button
                    on:click=on_click_add_trait_impl
                    disabled=move || !is_trait_impl_complete.get()
                >
                    +
                </button>

                <For
                    each=trait_impls
                    key=move |(_methods, trait_def)| trait_def.tag.id
                    children=move |(methods, trait_def)| {
                        let trait_id = trait_def.tag.id.get();
                        view! {
                            <div>
                                trait name: {trait_def.tag.name} <br/>
                                <button on:click=move |_| {
                                    active_object
                                        .get()
                                        .trait_impls
                                        .update(|prev| {
                                            prev.remove(&trait_id.clone());
                                        })
                                }>delete impl</button> <br/> trait methods:
                                <For
                                    each=methods
                                    key=move |(method_id, _path)| *method_id
                                    children=move |(method_id, path)| {
                                        let method_def = trait_def
                                            .methods
                                            .get()
                                            .iter()
                                            .find(|method| method.tag.id.get() == method_id)
                                            .cloned()
                                            .unwrap();
                                        let method_path = path
                                            .get()
                                            .iter()
                                            .map(|path_item| {
                                                match path_item {
                                                    RTraitMethodImplPath::Field(_item) => "Field".to_string(),
                                                    RTraitMethodImplPath::InstanceConstituent(_item) => {
                                                        "Instance".to_string()
                                                    }
                                                    RTraitMethodImplPath::LibraryOperativeConstituent(_item) => {
                                                        "LibraryOperative".to_string()
                                                    }
                                                    RTraitMethodImplPath::TraitOperativeConstituent { .. } => {
                                                        "TraitOperative".to_string()
                                                    }
                                                    RTraitMethodImplPath::TraitMethod {
                                                        trait_method_id: _,
                                                        trait_id: _,
                                                    } => "TraitMethod".to_string(),
                                                }
                                            })
                                            .collect::<Vec<String>>()
                                            .join("::");
                                        view! {
                                            {method_def.tag.name}
                                            <br/>
                                            {method_path}
                                        }
                                    }
                                />

                            </div>
                        }
                    }
                />

            </div>
        </div>
        <Show when=move || ctx.selected_element.get().is_some()>
            <TreeView
                on_click_tree_data=on_click_tree_data.clone()
                element=ctx.selected_element.get().unwrap()
            />
        </Show>
    }
}
