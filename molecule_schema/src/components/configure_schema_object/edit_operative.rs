use leptos::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

use base_types::{
    common::Uid,
    primitives::{PrimitiveTypes, PrimitiveValues},
};

use crate::components::{
    app::{ListItemTypes, SchemaContext, TreeTypes},
    common::{button_show::ButtonShow, select_input::SelectInputOptional, text_input::TextInput},
    tree_view::{TreeNodeDataSelectionType, TreeRef, TreeView},
};
use reactive_types::{
    reactive_item::RConstraintSchemaItem,
    reactive_types::{
        RLibraryOperative, ROperativeVariants, RSlottedInstances, RTraitMethodImplPath,
    },
};

#[component]
pub fn EditOperative(element: TreeRef) -> impl IntoView {
    let ctx = use_context::<SchemaContext>().unwrap();

    let schema_clone = Arc::new(ctx.schema);
    let schema_clone_2 = schema_clone.clone();
    let schema_clone_3 = schema_clone.clone();
    let schema_clone_4 = schema_clone.clone();
    let schema_clone_5 = schema_clone.clone();
    let schema_clone_6 = schema_clone.clone();
    let schema_clone_7 = schema_clone.clone();
    let schema_clone_8 = schema_clone.clone();
    let schema_clone_9 = schema_clone.clone();
    let schema_clone_10 = schema_clone.clone();
    let schema_clone_11 = schema_clone.clone();
    let schema_clone_12 = schema_clone.clone();
    let schema_clone_13 = schema_clone.clone();
    let schema_clone_14 = schema_clone.clone();
    let schema_clone_15 = schema_clone.clone();
    let schema_clone_16 = schema_clone.clone();
    let schema_clone_17 = schema_clone.clone();
    let schema_clone_18 = schema_clone.clone();
    let schema_clone_19 = schema_clone.clone();
    let schema_clone_20 = schema_clone.clone();
    let schema_clone_21 = schema_clone.clone();
    let schema_clone_22 = schema_clone.clone();
    let schema_clone_23 = schema_clone.clone();
    let schema_clone_24 = schema_clone.clone();
    let schema_clone_25 = schema_clone.clone();
    let schema_clone_26 = schema_clone.clone();
    let schema_clone_27 = schema_clone.clone();

    let active_object = create_memo(move |_| {
        schema_clone_16
            .operative_library
            .with(|operatives| operatives.get(&element.1).cloned())
            .unwrap()
    });
    let associated_template = create_memo(move |_| {
        schema_clone_20
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
            .get_locked_fields_digest(&schema_clone)
            .get_unfulfilled_fields()
    });
    let local_fulfilled_field_constraints = move || {
        active_object
            .get()
            .get_local_locked_fields()
            .get()
            .values()
            .cloned()
            .collect::<Vec<_>>()
    };
    let ancestors_fulfilled_field_constraints = create_memo(move |_| {
        active_object
            .get()
            .get_locked_fields_digest(&schema_clone_2)
            .get_ancestors_locked_fields()
    });

    let local_trait_impls = move || {
        active_object
            .get()
            .trait_impls
            .get()
            .clone()
            .iter()
            .map(|(trait_id, trait_methods)| {
                (
                    *trait_methods,
                    schema_clone_8.traits.get().get(trait_id).cloned().unwrap(),
                )
            })
            .collect::<Vec<_>>()
    };
    let ancestors_trait_impls = move || {
        active_object
            .get()
            .get_trait_impl_digest(&schema_clone_13)
            .get_local_trait_impls()
            .iter()
            .map(|(trait_id, related_trait_impl)| {
                (
                    related_trait_impl.clone(),
                    schema_clone_10.traits.get().get(trait_id).cloned().unwrap(),
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
        schema_clone_17.clone().operative_library.update(|lib| {
            lib.insert(new_operative.tag.id.get(), new_operative);
        });
    };
    let on_click_create_instance = move |_| {
        let new_instance = RLibraryOperative::<PrimitiveTypes, PrimitiveValues>::new(
            associated_template.get().tag.id.get(),
            Some(active_object.get().tag.id.get()),
            new_instance_name.get(),
        );
        schema_clone_7.instance_library.update(|lib| {
            lib.insert(new_instance.tag.id.get(), new_instance);
        });
    };

    let select_trait_impl_options = schema_clone_5.traits.with(|lib| {
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

    let on_click_tree_data = Arc::new(
        move |last_item: TreeNodeDataSelectionType,
              data_type: PrimitiveTypes,
              path: Arc<Vec<TreeRef>>| {
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
            let trait_in_question = schema_clone_6
                .traits
                .with(|items| items.get(&trait_impl_id).cloned())
                .expect("trait must exist");
            let mut hashmap = HashMap::new();
            trait_in_question
                .methods
                .get()
                .iter()
                .for_each(|(_id, trait_item)| {
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
    let ancestry_breadcrumb = {
        let mut next_parent = active_object.get().parent_operative_id.get();
        let mut breadcrumb = Vec::new();
        while let Some(parent) = next_parent {
            let operative = schema_clone_4
                .operative_library
                .with(|operative_library| operative_library.get(&parent).unwrap().clone());
            breadcrumb.insert(
                0,
                (
                    operative.tag.id.get(),
                    operative.tag.name.get().clone(),
                    ListItemTypes::Operative(operative.tag.id),
                ),
            );
            next_parent = operative.parent_operative_id.get();
        }
        let template = schema_clone_3.template_library.with(|template_library| {
            template_library
                .get(&active_object.get().template_id.get())
                .unwrap()
                .clone()
        });
        breadcrumb.insert(
            0,
            (
                template.tag.id.get(),
                template.tag.name.get().clone() + " (root template)",
                ListItemTypes::Template(template.tag.id),
            ),
        );
        breadcrumb
    };
    let on_click_ancestor_breadcrumb = move |clicked_id: ListItemTypes| {
        ctx.selected_element.set(Some(clicked_id.clone()));
    };

    view! {
        <div class="large-margin med-pad border-gray flex">
            <div class="flex-grow margin-right border-right">
                <button on:click=move |_| ctx.selected_element.set(None)>X</button>
                <button on:click=move |_| {
                    ctx.selected_element.set(None);
                    let stored_element_id = element.1;
                    schema_clone_14
                        .operative_library
                        .update(|prev| {
                            prev.remove(&stored_element_id);
                        })
                }>delete element</button>
                <br/>
                <br/>
                Ancestry:
                <br/>
                <div class="flex">
                    {ancestry_breadcrumb
                        .into_iter()
                        .map(|(ancestor_id, ancestor_name, item_type)| {
                            view! {
                                <div on:click=move |_| on_click_ancestor_breadcrumb(
                                    item_type.clone(),
                                )>" " {ancestor_name} " > "</div>
                            }
                        })
                        .collect::<Vec<_>>()}

                </div>
                <strong>Name</strong>
                <br/>
                <div class="flex">
                    <TextInput value=active_object.get().tag.name/>

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
                <h4>Fields</h4>
                <For
                    each=move || ancestors_fulfilled_field_constraints.get()
                    key=move |item| item.fulfilled_field.field_constraint_id.get()
                    let:item
                >
                    <div>{item.fulfilled_field.field_constraint_name} (locked above)</div>
                </For>
                <For
                    each=local_fulfilled_field_constraints
                    key=move |item| item.field_constraint_id.get()
                    let:item
                >
                    <div>
                        {item.field_constraint_name} : {move || item.value.get().to_string()}
                        (locked)
                        <button on:click=move |_| {
                            active_object
                                .get()
                                .locked_fields
                                .update(|prev| {
                                    prev.remove(&item.field_constraint_id.get());
                                })
                        }>unlock</button>
                    </div>
                </For>
                <For
                    each=move || unfulfilled_field_constraints.get()
                    key=move |item| item.tag.id
                    let:item
                >
                    <div>
                        {item.tag.name} <ButtonShow show_text="Begin Lock" hide_text="Cancel">

                            {
                                let item = item.clone();
                                move || {
                                    let value = RwSignal::new("".to_string());
                                    let item_clone = item.clone();
                                    let on_click_lock = Callback::new(move |_| {
                                        let new_val = match item.value_type.get() {
                                            PrimitiveTypes::EmptyTuple => {
                                                PrimitiveValues::Option(Box::new(None))
                                            }
                                            PrimitiveTypes::Int => {
                                                PrimitiveValues::Int(value.get().parse().unwrap())
                                            }
                                            PrimitiveTypes::String => {
                                                PrimitiveValues::String(value.get().parse().unwrap())
                                            }
                                            PrimitiveTypes::Bool => {
                                                PrimitiveValues::Bool(value.get().parse().unwrap())
                                            }
                                            PrimitiveTypes::Option(_) => todo!(),
                                            PrimitiveTypes::List(_) => todo!(),
                                        };
                                        active_object
                                            .get()
                                            .locked_fields
                                            .update(|prev| {
                                                prev.insert(
                                                    item_clone.tag.id.get(),
                                                    item_clone.fulfill(new_val),
                                                );
                                            });
                                    });
                                    view! {
                                        // PrimitiveTypes::Char => {
                                        // PrimitiveValues::Char(value.get().parse().unwrap())
                                        // }
                                        // PrimitiveTypes::Float => {
                                        // PrimitiveValues::Float(value.get().parse().unwrap())
                                        // }

                                        <TextInput value=value/>

                                        <button on:click=move |e| {
                                            on_click_lock.run(e)
                                        }>Lock</button>
                                    }
                                }
                            }

                        </ButtonShow>

                    </div>

                </For>
            </div>

            <div class="flex-grow margin-right border-right">
                <h4>Operative Slots</h4>

                <For
                    each=move || {
                        active_object
                            .get()
                            .get_operative_digest(&schema_clone_12)
                            .get()
                            .operative_slots
                    }

                    key=move |(id, _item)| { (*id, _item.clone()) }
                    let:operative_slot
                >

                    {
                        let operative_describing_string = match operative_slot
                            .1
                            .slot
                            .operative_descriptor
                            .clone()
                        {
                            ROperativeVariants::TraitOperative(trait_op) => {
                                format!("Trait Operative: {}", trait_op.tag.name.get())
                            }
                            ROperativeVariants::LibraryOperative(lib_op_id) => {
                                format!(
                                    "Library Operative: {}",
                                    schema_clone_11
                                        .operative_library
                                        .with(|operative_library| {
                                            operative_library
                                                .get(&lib_op_id.get())
                                                .unwrap()
                                                .tag
                                                .name
                                                .get()
                                        }),
                                )
                            }
                        };
                        let operative_slot_clone = operative_slot.1.clone();
                        let operative_slot_clone_2 = operative_slot.1.clone();
                        let operative_slot_clone_3 = operative_slot.1.clone();
                        let schema_clone_15 = schema_clone_15.clone();
                        let schema_clone_16 = schema_clone_15.clone();
                        view! {
                            <br/>
                            <hr/>
                            <div>
                                {operative_slot.1.slot.tag.name} -- fulfilled:
                                {move || operative_slot_clone.get_fulfillment_status()} <br/>
                                {operative_describing_string} <br/> Current Instances:
                                <For
                                    each=move || {
                                        operative_slot_clone_3.get_ancestors_related_instances()
                                    }

                                    key=move |item| item.instance_id
                                    let:item
                                >
                                    <div>
                                        {schema_clone_16
                                            .instance_library
                                            .with(|lib| {
                                                lib.get(&item.instance_id).unwrap().tag.name.get()
                                            })}

                                    </div>
                                </For>
                                <For
                                    each=move || {
                                        operative_slot_clone_2.get_local_related_instances().clone()
                                    }

                                    key=move |item| item.instance_id
                                    let:related_instance
                                >

                                    {
                                        let instance_name = schema_clone_15
                                            .instance_library
                                            .with(|lib| {
                                                lib.get(&related_instance.instance_id)
                                                    .unwrap()
                                                    .tag
                                                    .name
                                                    .get()
                                            });
                                        view! {
                                            <div>
                                                {instance_name}
                                                <button on:click=move |_| {
                                                    let slotted_instances = active_object
                                                        .get()
                                                        .slotted_instances
                                                        .with(|slotted_instances| {
                                                            slotted_instances
                                                                .get(&operative_slot.1.slot.tag.id.get())
                                                                .cloned()
                                                        });
                                                    let mut delete_entry = false;
                                                    if let Some(slotted_instances) = slotted_instances {
                                                        slotted_instances
                                                            .fulfilling_instance_ids
                                                            .update(|prev_instance_ids| {
                                                                prev_instance_ids
                                                                    .retain(|instance_id| {
                                                                        instance_id != &related_instance.instance_id
                                                                    });
                                                                if prev_instance_ids.is_empty() {
                                                                    delete_entry = true;
                                                                }
                                                            });
                                                    }
                                                }>remove</button>
                                            </div>
                                        }
                                    }

                                </For> <br/> Add Instance:
                                {
                                    let TypedSelectInputFulfillingInstanceSelection = SelectInputOptional::<
                                        Uid,
                                        String,
                                        _,
                                        _,
                                    >;
                                    let select_fulfilling_instance_options = match operative_slot
                                        .1
                                        .slot
                                        .operative_descriptor
                                    {
                                        ROperativeVariants::TraitOperative(ref trait_op) => {
                                            schema_clone_19
                                                .instance_library
                                                .with(|lib| {
                                                    lib.iter()
                                                        .filter_map(|(id, lib_item)| {
                                                            let trait_impl_digest = lib_item
                                                                .get_trait_impl_digest(&schema_clone_19);
                                                            let contains_trait_impls = trait_op
                                                                .trait_ids
                                                                .get()
                                                                .iter()
                                                                .all(|required_trait_id| {
                                                                    trait_impl_digest
                                                                        .trait_impls
                                                                        .contains_key(required_trait_id)
                                                                });
                                                            if contains_trait_impls {
                                                                Some((*id, lib_item.tag.name.get()))
                                                            } else {
                                                                None
                                                            }
                                                        })
                                                        .collect::<Vec<_>>()
                                                })
                                        }
                                        ROperativeVariants::LibraryOperative(lib_op_id) => {
                                            schema_clone_19
                                                .instance_library
                                                .with(|lib| {
                                                    lib.iter()
                                                        .filter_map(|(id, lib_item)| {
                                                            if lib_item
                                                                .check_ancestry(&schema_clone_22, &lib_op_id.get())
                                                            {
                                                                Some((*id, lib_item.tag.name.get()))
                                                            } else {
                                                                None
                                                            }
                                                        })
                                                        .collect::<Vec<_>>()
                                                })
                                        }
                                    };
                                    let selected_instance_id = RwSignal::new(None);
                                    let operative_slot = operative_slot.1.slot.clone();
                                    let operative_id = move || match &operative_slot
                                        .operative_descriptor
                                    {
                                        ROperativeVariants::TraitOperative(trait_op) => {
                                            trait_op.tag.id.get()
                                        }
                                        ROperativeVariants::LibraryOperative(lib_op_id) => {
                                            lib_op_id.get()
                                        }
                                    };
                                    view! {
                                        <TypedSelectInputFulfillingInstanceSelection
                                            options=select_fulfilling_instance_options.into()
                                            value=add_trait_impl_id
                                            on_select=move |return_val| {
                                                selected_instance_id.set(return_val)
                                            }
                                        />

                                        <button
                                            disabled=move || selected_instance_id.get().is_none()
                                            on:click=move |_| {
                                                let slot_id = operative_slot.tag.id.get();
                                                let current_slot_instances = active_object
                                                    .get()
                                                    .slotted_instances
                                                    .with(|slotted_instances| {
                                                        slotted_instances.get(&slot_id).cloned()
                                                    });
                                                if let Some(current_slot_instances) = current_slot_instances {
                                                    current_slot_instances
                                                        .fulfilling_instance_ids
                                                        .update(|prev_instance_ids| {
                                                            prev_instance_ids.push(selected_instance_id.get().unwrap());
                                                        });
                                                } else {
                                                    let new_slotted_instances = RSlottedInstances::new(
                                                        slot_id,
                                                        operative_id(),
                                                        vec![selected_instance_id.get().unwrap()],
                                                    );
                                                    active_object
                                                        .get()
                                                        .slotted_instances
                                                        .update(|prev_slotted_instance_map| {
                                                            prev_slotted_instance_map
                                                                .insert(slot_id, new_slotted_instances);
                                                        });
                                                }
                                            }
                                        >

                                            +
                                        </button>
                                    }
                                }

                            </div>
                        }
                    }

                </For>
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
                <For
                    each=move || active_trait_impl_method_paths.get()
                    key=move |item| item.0
                    let:item
                >

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
                    each=ancestors_trait_impls

                    key=move |(_methods, trait_def)| trait_def.tag.id
                    children=move |(methods, trait_def)| {
                        let trait_id = trait_def.tag.id.get();
                        view! {
                            <div>
                                trait name: {trait_def.tag.name} <br/> trait methods:
                                <For
                                    each=move || methods.trait_impl.get()
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
                                                    RTraitMethodImplPath::Constituent(_) => {
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

                <For
                    each=local_trait_impls
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
                                    each=move || methods.get()
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
