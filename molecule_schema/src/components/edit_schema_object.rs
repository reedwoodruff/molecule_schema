use std::{collections::HashMap, rc::Rc};

use leptos::{ev::submit, logging::log, *};
use serde_types::{
    common::Uid,
    constraint_schema::TraitOperative,
    primitives::{PrimitiveTypes, PrimitiveValues},
};
use web_sys::SubmitEvent;

use crate::{
    components::{
        common::{
            select_input::{SelectInput, SelectInputOptional},
            text_input::TextInput,
        },
        tree_view::{TreeNodeDataSelectionType, TreeView},
        SchemaContext, TreeTypes,
    },
    utils::reactive_types::{
        RFieldConstraint, RLibraryInstance, RLibraryOperative, RTag, RTraitMethodImplPath,
        RTraitOperative,
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

    let new_operative_name = RwSignal::new("new_operative".to_string());
    let new_instance_name = RwSignal::new("new_instance".to_string());

    let on_click_create_operative = move |_| {
        let new_operative = RLibraryOperative::<PrimitiveTypes, PrimitiveValues>::new(
            element.1,
            None,
            new_operative_name.get(),
        );
        ctx.schema.operative_library.update(|lib| {
            lib.insert(new_operative.tag.id.get(), new_operative);
        });
    };
    let on_click_create_instance = move |_| {
        let new_instance = RLibraryInstance::<PrimitiveTypes, PrimitiveValues>::new(
            element.1,
            None,
            new_instance_name.get(),
        );
        ctx.schema.instance_library.update(|lib| {
            lib.insert(new_instance.tag.id.get(), new_instance);
        });
    };

    // let field_type_options = RwSignal::new(PrimitiveTypes::get_type_options().into_iter());
    let field_type_options = PrimitiveTypes::get_type_options();

    let select_instance_options = ctx.schema.instance_library.with(|lib| {
        lib.iter()
            .map(|(id, lib_item)| (*id, lib_item.tag.name.get()))
            .collect::<Vec<_>>()
    });

    let add_instance_id = RwSignal::new(None);
    let add_instance_id_clone = add_instance_id.clone();

    let on_click_add_instance = move |_| {
        if let Some(instance_id) = add_instance_id.get() {
            println!("{}", instance_id);
        }
    };
    // let derived_instance_selection = Signal::derive(move || add_instance_id.get().unwrap_or(0));
    let TypedSelectInputInstanceSelection = SelectInputOptional::<Uid, String, _, _>;
    let delete_instance = move |id: Uid| {
        move |_| {
            active_object
                .get()
                .instances
                .update(|lib| lib.retain(|item| item != &id));
        }
    };

    let select_operative_options = ctx.schema.operative_library.with(|lib| {
        lib.iter()
            .map(|(id, lib_item)| (*id, lib_item.tag.name.get()))
            .collect::<Vec<_>>()
    });
    let add_operative_id = RwSignal::new(None);
    let on_click_add_operative = move |_| {
        if let Some(operative_id) = add_operative_id.get() {
            active_object
                .get()
                .library_operatives
                .update(|prev| prev.push(operative_id));
        }
    };
    // let derived_operative_selection = Signal::derive(move || add_operative_id.get().unwrap_or(0));
    let TypedSelectInputOperativeSelection = SelectInputOptional::<Uid, String, _, _>;
    let delete_operative = move |id: Uid| {
        move |_| {
            active_object
                .get()
                .library_operatives
                .update(|lib| lib.retain(|item| item != &id));
        }
    };

    let select_trait_operative_options = ctx.schema.traits.with(|lib| {
        lib.iter()
            .map(|(id, lib_item)| (*id, lib_item.tag.name.get()))
            .collect::<Vec<_>>()
    });
    let add_trait_operative_id = RwSignal::new(None);
    let new_trait_operative_name = RwSignal::new("new_trait_operative".to_string());
    let on_click_add_trait_operative = move |_| {
        if let Some(trait_operative_id) = add_trait_operative_id.get() {
            println!("{}", trait_operative_id);
            let new_trait_op = RTraitOperative {
                trait_id: RwSignal::new(trait_operative_id),
                tag: RTag::new(new_trait_operative_name.get()),
            };
            active_object
                .get()
                .trait_operatives
                .update(|prev| prev.push(new_trait_op));
        }
    };
    let TypedSelectInputTraitOperativeSelection = SelectInputOptional::<Uid, String, _, _>;

    let delete_trait_operative = move |id: Uid| {
        move |_| {
            active_object
                .get()
                .trait_operatives
                .update(|lib| lib.retain(|item| item.tag.id.get() != id));
        }
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
                        .filter(|item| item.0 != TreeTypes::ConstraintObject)
                        .map(|item| {
                            let new_trait_method_impl_path_item = match item.0.clone() {
                                TreeTypes::Instance => {
                                    RTraitMethodImplPath::InstanceConstituent(RwSignal::new(item.1))
                                }
                                TreeTypes::LibraryOperative => {
                                    RTraitMethodImplPath::LibraryOperativeConstituent(
                                        RwSignal::new(item.1),
                                    )
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
                            };
                            new_trait_method_impl_path_item
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
            } else {
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
                <button on:click=move |_| ctx.schema.constraint_objects.update(|prev| {prev.remove(&element.1);})>delete element</button>
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
                <h4>Fields <button on:click=add_field>+</button></h4>
                <For each=field_constraints key=move |item| item.tag.id let:item>

                    {
                        let change_field_type = move |v: &PrimitiveTypes| {
                            item.value_type.set(v.clone())
                        };
                        let field_type_options: HashMap<PrimitiveTypes, String> = field_type_options
                            .clone();
                        let TypedSelectInput = SelectInput::<
                            PrimitiveTypes,
                            String,
                            _,
                            HashMap<PrimitiveTypes, String>,
                        >;
                        view! {
                            <div class="flex">
                                <TextInput
                                    initial_value=item.tag.name.get()
                                    on_save=move |val: String| {
                                        item.tag.name.set(val);
                                    }

                                    show_save_button=true
                                />

                                <TypedSelectInput
                                    options=field_type_options
                                    on_select=change_field_type
                                    value=item.value_type
                                />
                            </div>
                        }
                    }

                </For>
            </div>

            <div class="flex-grow margin-right border-right">
                <h4>Constituents</h4>
                <strong>Instances</strong>

                <br/>
                <TypedSelectInputInstanceSelection
                    options=select_instance_options
                    value=add_instance_id
                    on_select=move |instance_id| add_instance_id.set(instance_id)
                />
                <button
                    on:click=on_click_add_instance
                    disabled=move || add_instance_id.get() == None
                >
                    +
                </button>

                <For
                    each=constituent_instances
                    key=move |item| item.tag.id
                    children=move |item| {
                        view! {
                            <div>
                                {item.tag.name} <br/>
                                <button on:click=delete_instance(item.tag.id.get())>Delete</button>
                            </div>
                        }
                    }
                />

                <br/>

                <strong>Library Operatives</strong>
                <br/>
                <TypedSelectInputOperativeSelection
                    options=select_operative_options
                    value=add_operative_id
                    on_select=move |operative_id| add_operative_id.set(operative_id)
                />
                <button
                    on:click=on_click_add_operative
                    disabled=move || add_operative_id.get() == None
                >
                    +
                </button>
                <For
                    each=constituent_library_operatives
                    key=move |item| item.tag.id
                    children=move |item| {
                        view! {
                            <div>
                                {item.tag.name} <br/>
                                <button on:click=delete_operative(item.tag.id.get())>Delete</button>
                            </div>
                        }
                    }
                />

                <br/>

                <strong>Trait Operatives</strong>
                <br/>
                <TypedSelectInputTraitOperativeSelection
                    options=select_trait_operative_options
                    value=add_trait_operative_id
                    on_select=move |trait_operative_id| {
                        add_trait_operative_id.set(trait_operative_id)
                    }
                />

                <TextInput
                    initial_value=new_trait_operative_name.get()
                    on_save=move |val: String| {
                        new_trait_operative_name.set(val);
                    }
                />

                <button
                    on:click=on_click_add_trait_operative
                    disabled=move || add_trait_operative_id.get() == None
                >
                    +
                </button>
                <For
                    each=constituent_trait_operatives
                    key=move |(trait_operative, trait_def)| trait_operative.tag.id
                    children=move |(trait_operative, trait_def)| {
                        view! {
                            <div>
                                operative name: {trait_operative.tag.name} <br/> trait name:
                                {trait_def.tag.name} <br/>
                                <button on:click=delete_trait_operative(
                                    trait_operative.tag.id.get(),
                                )>Delete</button>
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
                    key=move |(methods, trait_def)| trait_def.tag.id
                    children=move |(methods, trait_def)| {
                        let trait_id = trait_def.tag.id.get().clone();
                        view! {
                            <div>
                                trait name: {trait_def.tag.name}
                                <br/>
                                <button on:click=move |_| {active_object.get().trait_impls.update(|prev| {prev.remove(&trait_id.clone());})}>delete impl</button>
                                <br/> trait methods:
                                <For
                                    each=methods
                                    key=move |(method_id, path)| *method_id
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
                                                    RTraitMethodImplPath::Field(item) => "Field".to_string(),
                                                    RTraitMethodImplPath::InstanceConstituent(item) => {
                                                        "Instance".to_string()
                                                    }
                                                    RTraitMethodImplPath::LibraryOperativeConstituent(item) => {
                                                        "LibraryOperative".to_string()
                                                    }
                                                    RTraitMethodImplPath::TraitOperativeConstituent {
                                                        trait_operative_id,
                                                        ..
                                                    } => "TraitOperative".to_string(),
                                                    RTraitMethodImplPath::TraitMethod {
                                                        trait_method_id,
                                                        trait_id,
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
