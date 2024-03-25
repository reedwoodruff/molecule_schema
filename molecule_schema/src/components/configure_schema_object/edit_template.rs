use std::{collections::HashMap, rc::Rc};

use leptos::{logging::log, *};
use serde_types::{
    common::Uid,
    primitives::{PrimitiveTypes, PrimitiveValues},
};
use web_sys::MouseEvent;

use crate::{
    components::{
        app::{SchemaContext, TreeTypes},
        common::{
            select_input::{SelectInput, SelectInputEnum, SelectInputOptional},
            text_input::{NumberInput2, TextInput},
        },
        tree_view::{TreeNodeDataSelectionType, TreeRef, TreeView},
    },
    utils::{
        reactive_item::RConstraintSchemaItem,
        reactive_types::{
            RFieldConstraint, RLibraryOperative, ROperativeSlot, ROperativeVariants, RSlotBounds,
            RTag, RTraitMethodImplPath, RTraitOperative,
        },
    },
};

#[component]
pub fn EditTemplate(element: TreeRef) -> impl IntoView {
    let ctx = use_context::<SchemaContext>().unwrap();

    let active_object = create_memo(move |_| {
        ctx.schema
            .template_library
            .with(|co| co.get(&element.1).cloned())
            .unwrap()
    });

    let field_constraints = move || {
        active_object
            .get()
            .field_constraints
            .get()
            .values()
            .cloned()
            .collect::<Vec<_>>()
    };

    let add_field = move |_| {
        let new_field = RFieldConstraint::<PrimitiveTypes> {
            tag: RTag::new("New Field".to_string()),
            value_type: RwSignal::new(PrimitiveTypes::String),
        };
        active_object().field_constraints.update(|prev| {
            prev.insert(new_field.tag.id.get(), new_field);
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

    let trait_impls = move || {
        active_object
            .get()
            .trait_impls
            .get()
            .clone()
            .iter()
            .map(|(trait_id, trait_methods)| {
                (
                    *trait_methods,
                    ctx.schema.traits.get().get(trait_id).cloned().unwrap(),
                )
            })
            .collect::<Vec<_>>()
    };

    let operative_slot_view = view! {};

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
        let new_instance = RLibraryOperative::<PrimitiveTypes, PrimitiveValues>::new(
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
            .filter_map(|(id, lib_item)| {
                if lib_item.template_id.get() == active_object.get().tag.id.get() {
                    None
                } else {
                    Some((*id, lib_item.tag.name.get()))
                }
            })
            .collect::<Vec<_>>()
    });

    let add_instance_id = RwSignal::new(None);
    let _add_instance_id_clone = add_instance_id;

    let on_click_add_instance = move |_| {
        if let Some(instance_id) = add_instance_id.get() {
            active_object
                .get()
                .instances
                .update(|prev| prev.push(instance_id));
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

    let schema_clone = ctx.schema.clone();
    let select_operative_options = create_memo(move |_| {
        ctx.schema.operative_library.with(|lib| {
            lib.iter()
                .filter_map(|(id, lib_item)| {
                    if lib_item.check_ancestry(&schema_clone, &active_object.get().tag.id.get()) {
                        None
                    } else {
                        Some((*id, lib_item.tag.name.get()))
                    }
                })
                .collect::<Vec<_>>()
        })
    });
    let add_operative_id = RwSignal::<Option<Uid>>::new(None);
    let on_click_add_operative = move |_| {
        if let Some(operative_id) = add_operative_id.get() {
            active_object.get().operative_slots.update(|prev| {
                let new_slot = ROperativeSlot::new(
                    ROperativeVariants::LibraryOperative(RwSignal::new(operative_id)),
                    "New_Slot",
                );
                prev.insert(new_slot.tag.id.get(), new_slot);
            });
        }
        add_operative_id.set(None);
    };

    let TypedSelectInputOperativeSelection = SelectInputOptional::<Uid, String, _, _>;
    let delete_operative_slot = move |id: Uid| {
        move |_: MouseEvent| {
            active_object
                .get()
                .operative_slots
                .update(|prev| prev.retain(|slot_id, _slot| slot_id != &id));
        }
    };

    let select_trait_operative_options = create_memo(move |_| {
        ctx.schema.traits.with(|lib| {
            lib.iter()
                .map(|(id, lib_item)| (*id, lib_item.tag.name.get()))
                .collect::<Vec<_>>()
        })
    });
    let add_trait_operative_ids = RwSignal::<Option<Vec<Uid>>>::new(None);
    let add_trait_operative_id = RwSignal::<Option<Uid>>::new(None);
    let new_trait_operative_name = RwSignal::new("new_trait_operative".to_string());
    let on_click_add_trait_operative = move |_| {
        if let Some(trait_operative_id) = add_trait_operative_ids.get() {
            println!("{:?}", trait_operative_id);
            let new_trait_op = RTraitOperative {
                trait_ids: RwSignal::new(trait_operative_id),
                tag: RTag::new(new_trait_operative_name.get()),
            };
            active_object.get().operative_slots.update(|prev| {
                let new_slot = ROperativeSlot::new(
                    ROperativeVariants::TraitOperative(new_trait_op),
                    "New_Slot",
                );
                prev.insert(new_slot.tag.id.get(), new_slot);
            })
        }
    };
    let TypedSelectInputTraitOperativeSelection = SelectInputOptional::<Uid, String, _, _>;

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
                                RTraitMethodImplPath::Constituent(RwSignal::new(item.1))
                            }
                            TreeTypes::LibraryOperative => {
                                RTraitMethodImplPath::Constituent(RwSignal::new(item.1))
                            }
                            TreeTypes::TraitOperative(trait_op) => {
                                RTraitMethodImplPath::Constituent(RwSignal::new(
                                    trait_op.tag.id.get(),
                                ))
                                // RTraitMethodImplPath::TraitOperativeConstituent {
                                //     trait_method_id: RwSignal::new(method_id),
                                //     trait_operative_id: RwSignal::new(trait_op.tag.id.get()),
                                //     trait_id: RwSignal::new(trait_op.trait_ids.get()),
                                // }
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
            trait_in_question.methods.get().iter().for_each(
                |(trait_method_id, trait_method_def)| {
                    hashmap.insert(
                        *trait_method_id,
                        (
                            trait_method_def.tag.name.get(),
                            trait_method_def.return_type.get(),
                            RwSignal::new(None),
                        ),
                    );
                },
            );
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
                    ctx.selected_element.set(None);
                    ctx.schema
                        .template_library
                        .update(|prev| {
                            prev.remove(&element.1);
                        })
                }>delete element</button>
                <br/>
                <br/>

                <strong>Name</strong>
                <div class="flex">
                    <TextInput value=active_object.get().tag.name />

                </div>
                <hr/>

                <div>
                    <TextInput value=new_operative_name/>
                    <button on:click=on_click_create_operative>Create Operative</button>
                </div>
                <br/>
                <div>
                    <TextInput value=new_instance_name/>
                    <button on:click=on_click_create_instance>Create Instance</button>
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
                                <TextInput value=item.tag.name />

                                <TypedSelectInput
                                    options=field_type_options.into()
                                    on_select=change_field_type
                                    value=item.value_type
                                />
                            </div>
                        }
                    }

                </For>
            </div>

            <div class="flex-grow margin-right border-right">
                <h4>Instances</h4>

                <TypedSelectInputInstanceSelection
                    options=select_instance_options.into()
                    value=add_instance_id
                    on_select=move |instance_id| add_instance_id.set(instance_id)
                />
                <button
                    on:click=on_click_add_instance
                    disabled=move || add_instance_id.get().is_none()
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

                <h4>Operative Slots</h4>

                <For
                    each=active_object.get().operative_slots
                    key=move |op_slot| op_slot.0
                    let:op_slot
                >
                    <div>
                        Slot name: <TextInput value=op_slot.1.tag.name/>
                        <button on:click=delete_operative_slot(op_slot.0)>Delete Slot</button> <br/>
                        Operative name:
                        {match op_slot.1.operative_descriptor {
                            ROperativeVariants::TraitOperative(trait_op) => trait_op.tag.name.get(),
                            ROperativeVariants::LibraryOperative(op_id) => {
                                ctx.schema
                                    .operative_library
                                    .get()
                                    .get(&op_id.get())
                                    .unwrap()
                                    .tag
                                    .name
                                    .get()
                            }
                        }}
                        <br/> Slot Range: <br/> <SelectInputEnum value=op_slot.1.bounds/> <br/>
                        {move || match op_slot.1.bounds.get() {
                            RSlotBounds::LowerBound(val) => {
                                view! {
                                    Lower bound:
                                    <NumberInput2 value=val/>
                                }
                            }
                            RSlotBounds::UpperBound(val) => {
                                view! {
                                    Upper bound:
                                    <NumberInput2 value=val/>
                                }
                            }
                            RSlotBounds::Range(lower_range, upper_range) => {
                                view! {
                                    Lower range:
                                    <NumberInput2 value=lower_range/>
                                    <br/>
                                    Upper range:
                                    <NumberInput2 value=upper_range/>
                                    <br/>
                                }
                            }
                            RSlotBounds::LowerBoundOrZero(val) => {
                                view! {
                                    Lower bound:
                                    <NumberInput2 value=val/>
                                }
                            }
                            RSlotBounds::RangeOrZero(lower_range, upper_range) => {
                                view! {
                                    Lower range:
                                    <NumberInput2 value=lower_range/>
                                    <br/>
                                    Upper range:
                                    <NumberInput2 value=upper_range/>
                                    <br/>
                                }
                            }
                            _ => {
                                view! {
                                    idk
                                    <br/>
                                }
                            }
                        }}
                        <br/>

                    </div>
                </For>

                <strong>Add Library Operative</strong>
                <br/>

                {
                    let casted_options = <Memo<
                        Vec<(u128, std::string::String)>,
                    > as Into<
                        leptos::MaybeSignal<Vec<(u128, std::string::String)>>,
                    >>::into(select_operative_options);
                    view! {
                        <TypedSelectInputOperativeSelection
                            options=casted_options
                            value=add_operative_id
                            on_select=move |operative_id| add_operative_id.set(operative_id)
                        />
                    }
                }

                <button
                    on:click=on_click_add_operative
                    disabled=move || add_operative_id.get().is_none()
                >
                    +
                </button>

                <br/>

                <strong>Add Trait Operative</strong>
                <br/>

                {
                    let casted_options = <Memo<
                        Vec<(u128, std::string::String)>,
                    > as Into<
                        leptos::MaybeSignal<Vec<(u128, std::string::String)>>,
                    >>::into(select_trait_operative_options);
                    view! {
                        <TypedSelectInputTraitOperativeSelection
                            options=casted_options
                            value=add_trait_operative_id
                            on_select=move |trait_operative_id| {
                                add_trait_operative_id.set(trait_operative_id)
                            }
                        />
                    }
                }

                <button on:click=move |_| {
                    if let Some(current_operative_id) = add_trait_operative_id.get() {
                        if let Some(previously_set_types) = add_trait_operative_ids.get() {
                            let mut new_trait_ids = previously_set_types.clone();
                            new_trait_ids.push(current_operative_id);
                            add_trait_operative_ids.set(Some(new_trait_ids));
                        } else {
                            add_trait_operative_ids.set(Some(vec![current_operative_id]));
                        }
                    }
                }>

                    Add trait to list
                </button>
                <br/>
                Currently selected traits:
                {move || {
                    add_trait_operative_ids
                        .get()
                        .map(|item| {
                            item.iter().map(|item| item.to_string()).collect::<Vec<_>>().join(", ")
                        })
                        .unwrap_or("None".to_string())
                }}

                <button on:click=move |_| {
                    add_trait_operative_ids.set(None)
                }>Clear selected traits</button>

                <TextInput value=new_trait_operative_name />

                <button
                    on:click=on_click_add_trait_operative
                    disabled=move || add_trait_operative_ids.get().is_none()
                >
                    +
                </button>

                <br/>

            </div>
            <div class="flex-grow margin-right">
                <h4>Trait Impls</h4>
                New Impl:
                <br/>
                trait:
                <TypedSelectInputTraitImplSelection
                    options=select_trait_impl_options.into()
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
                                trait name: {trait_def.tag.name}
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
                                            .values()
                                            .find(|method| method.tag.id.get() == method_id)
                                            .cloned()
                                            .unwrap();
                                        let method_path = path
                                            .get()
                                            .iter()
                                            .map(|path_item| {
                                                match path_item {
                                                    RTraitMethodImplPath::Field(_item) => "Field".to_string(),
                                                    RTraitMethodImplPath::Constituent(_item) => {
                                                        "Constituent".to_string()
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
                                            <div>
                                                <strong>{method_def.tag.name}</strong>

                                                ()
                                                <br/>
                                                Fulfillment path:
                                                {method_path}
                                            </div>
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
            <TreeView on_click_tree_data=on_click_tree_data.clone() element=element.clone()/>
        </Show>
    }
}
