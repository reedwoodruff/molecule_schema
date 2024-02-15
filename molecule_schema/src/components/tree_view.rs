use std::{marker::PhantomData, rc::Rc};

use leptos::*;
use serde_types::{
    common::{ConstraintTraits, Uid},
    primitives::PrimitiveTypes,
};

use crate::utils::reactive_types::{
    FieldInfo, ROperativeVariants, RTag, RTraitDef, RTraitOperative, Tagged, RCSO,
};

use super::{SchemaContext, TreeTypes};

#[component]
pub fn TreeView<F>(element: TreeRef, on_click_tree_data: Rc<F>) -> impl IntoView
where
    F: Fn(TreeNodeDataSelectionType, PrimitiveTypes, Rc<Vec<TreeRef>>) + 'static,
{
    let path = Rc::new(Vec::new());
    view! { <TreeNode element=element path=path on_click_tree_data/> }
}

#[derive(Clone, PartialEq, Debug)]
pub struct TreeRef(pub TreeTypes, pub Uid);

#[derive(Clone, PartialEq)]
struct FieldInfoStruct<TTypes: ConstraintTraits, TValues: ConstraintTraits> {
    tag: RTag,
    value_type: TTypes,
    value: Option<TValues>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum TreeNodeDataSelectionType {
    Field(Uid),
    TraitMethod { trait_id: Uid, method_id: Uid },
}

#[derive(Clone, PartialEq)]
struct TreeNodeInfo<TTypes: ConstraintTraits, TValues: ConstraintTraits> {
    top_level_type: TreeTypes,
    tag: RTag,
    fields: Vec<FieldInfoStruct<TTypes, TValues>>,
    trait_impls: Vec<RTraitDef<TTypes>>,
    instance_constituents: Vec<Uid>,
    library_operative_constituents: Vec<Uid>,
    trait_operative_constituents: Vec<RTraitOperative>,
}

#[component]
pub fn TreeNode<F>(
    element: TreeRef,
    path: Rc<Vec<TreeRef>>,
    on_click_tree_data: Rc<F>,
) -> impl IntoView
where
    F: Fn(TreeNodeDataSelectionType, PrimitiveTypes, Rc<Vec<TreeRef>>) + 'static,
{
    let ctx = use_context::<SchemaContext>().unwrap();
    let element_clone = element.clone();

    let on_click_tree_data_1 = on_click_tree_data.clone();
    let on_click_tree_data_2 = on_click_tree_data_1.clone();
    let on_click_tree_data_3 = on_click_tree_data_1.clone();
    let on_click_tree_data_4 = on_click_tree_data_1.clone();
    let on_click_tree_data_5 = on_click_tree_data_1.clone();

    let tree_element = create_memo(move |_| match &element {
        TreeRef(TreeTypes::ConstraintObject, id) => {
            let element = move || {
                ctx.schema
                    .constraint_objects
                    .with(|o| o.get(&id).unwrap().clone())
            };
            let element_tag = element().get_tag().clone();
            let fields = element()
                .get_fields()
                .iter()
                .map(|field| FieldInfoStruct {
                    tag: field.get_tag().clone(),
                    value_type: field.get_value_type(),
                    value: field.get_value(),
                })
                .collect::<Vec<_>>();
            let traits_impled = element()
                .trait_impls
                .get()
                .iter()
                .map(|(id, _)| {
                    ctx.schema
                        .traits
                        .with(|trait_items| trait_items.get(&id).unwrap().clone())
                })
                .collect();

            TreeNodeInfo {
                top_level_type: TreeTypes::ConstraintObject,
                tag: element_tag,
                fields,
                trait_impls: traits_impled,
                instance_constituents: element().instances.get(),
                library_operative_constituents: element().library_operatives.get(),
                trait_operative_constituents: element().trait_operatives.get(),
            }
        }
        TreeRef(TreeTypes::Instance, id) => {
            let element = move || {
                ctx.schema
                    .instance_library
                    .with(|o| o.get(&id).unwrap().clone())
            };
            let element_tag = element().get_tag().clone();
            let fields = element()
                .get_fields()
                .iter()
                .map(|field| FieldInfoStruct {
                    tag: field.get_tag().clone(),
                    value_type: field.get_value_type(),
                    value: field.get_value(),
                })
                .collect::<Vec<_>>();
            let parent_constraint_object_instances = ctx
                .schema
                .constraint_objects
                .with(|o| {
                    o.get(&element().constraint_object_id.get())
                        .unwrap()
                        .instances
                })
                .get();

            let traits_impled = element()
                .trait_impls
                .get()
                .iter()
                .map(|(id, _)| {
                    ctx.schema
                        .traits
                        .with(|trait_items| trait_items.get(&id).unwrap().clone())
                })
                .collect();

            let instances = move || {
                element().fulfilled_operatives.with(|el| {
                    el.iter()
                        .map(|item| item.fulfilling_instance_id.get())
                        .collect::<Vec<_>>()
                })
            };
            let mut instances = instances();
            instances.extend(parent_constraint_object_instances);

            TreeNodeInfo {
                top_level_type: TreeTypes::Instance,
                tag: element_tag,
                fields,
                trait_impls: traits_impled,
                instance_constituents: instances,
                library_operative_constituents: Vec::new(),
                trait_operative_constituents: Vec::new(),
            }
        }
        TreeRef(TreeTypes::LibraryOperative, id) => {
            let element = move || {
                ctx.schema
                    .operative_library
                    .with(|o| o.get(&id).unwrap().clone())
            };
            let element_tag = element().get_tag().clone();
            let parent_constraint_object = move || {
                ctx.schema.constraint_objects.with(|o| {
                    o.get(&element().constraint_object_id.get())
                        .unwrap()
                        .clone()
                })
            };

            let fields = parent_constraint_object()
                .get_fields()
                .iter()
                .map(|field| FieldInfoStruct {
                    tag: field.get_tag().clone(),
                    value_type: field.get_value_type(),
                    value: field.get_value(),
                })
                .collect::<Vec<_>>();

            let traits_impled = element()
                .trait_impls
                .get()
                .iter()
                .map(|(id, _)| {
                    ctx.schema
                        .traits
                        .with(|trait_items| trait_items.get(&id).unwrap().clone())
                })
                .collect();

            let parent_constraint_object_instances = parent_constraint_object().instances;

            let mut instances = move || {
                element().fulfilled_operatives.with(|el| {
                    el.iter()
                        .map(|item| item.fulfilling_instance_id.get())
                        .collect::<Vec<_>>()
                })
            };
            let mut fulfilled_ops = move || {
                element().fulfilled_operatives.with(|el| {
                    let mut fulfilled_trait_ops = vec![];
                    let mut fulfilled_library_ops = vec![];
                    el.iter().for_each(|item| match item.operative_id.get() {
                        ROperativeVariants::TraitOperative(id_signal) => {
                            fulfilled_trait_ops.push(id_signal.get())
                        }
                        ROperativeVariants::LibraryOperative(id_signal) => {
                            fulfilled_library_ops.push(id_signal.get())
                        }
                    });
                    (fulfilled_library_ops, fulfilled_trait_ops)
                })
            };
            let final_instances = instances();

            let mut library_operatives = parent_constraint_object().library_operatives.get();
            library_operatives.retain(|item| !fulfilled_ops().0.contains(item));

            let mut trait_operatives = parent_constraint_object().trait_operatives.get();
            trait_operatives.retain(|item| {
                // !fulfilled_ops().1.contains(item)
                if let Some(thing) = fulfilled_ops()
                    .1
                    .iter()
                    .find(|fulfilled_id| &item.tag.id.get() == *fulfilled_id)
                {
                    false
                } else {
                    true
                }
            });

            TreeNodeInfo {
                top_level_type: TreeTypes::LibraryOperative,
                tag: element_tag,
                fields,
                trait_impls: traits_impled,
                instance_constituents: final_instances,
                library_operative_constituents: library_operatives,
                trait_operative_constituents: trait_operatives,
            }
        }

        TreeRef(TreeTypes::TraitOperative(trait_op), id) => {
            let element = move || ctx.schema.traits.with(|el| el.get(&id).unwrap().clone());

            // let traits_impled = element()
            //     .trait_impls
            //     .get()
            //     .iter()
            //     .map(|(id, _)| {
            //         ctx.schema
            //             .traits
            //             .with(|trait_items| trait_items.get(&id).unwrap().clone())
            //     })
            //     .collect();

            TreeNodeInfo {
                top_level_type: TreeTypes::TraitOperative(trait_op.clone()),
                tag: trait_op.tag.clone(),
                fields: vec![],
                trait_impls: vec![element()],
                instance_constituents: vec![],
                library_operative_constituents: vec![],
                trait_operative_constituents: vec![],
            }
        }
    });

    // let mut new_path = (*path).clone();
    let mut new_path = path.as_ref().clone();
    new_path.push(element_clone);
    let new_path = Rc::new(new_path);
    let new_path_2 = new_path.clone();
    let new_path_3 = new_path.clone();
    let new_path_4 = new_path.clone();
    let new_path_5 = new_path.clone();

    view! {
        <div class=move || {
            "tree-node-container container_".to_string()
                + &tree_element.get().top_level_type.to_string()
        }>
            <h3 class="small-title">{move || tree_element.get().tag.name}</h3>
            <For each=move || tree_element.get().fields key=move |item| item.tag.id.get() let:item>

                {
                    let on_click_tree_data_4 = on_click_tree_data_4.clone();
                    let new_path = new_path_4.clone();
                    let item_value_type_2 = item.value_type.clone();
                    view! {
                        <div on:click=move |_| on_click_tree_data_4(
                            TreeNodeDataSelectionType::Field(item.tag.id.get()),
                            item_value_type_2.clone(),
                            new_path.clone(),
                        )>{move || item.tag.name.get()} : {item.value_type.to_string()}</div>
                    }
                }

            </For>
            <For
                each=move || tree_element.get().trait_impls
                key=move |item| item.tag.id.get()
                let:item
            >

                {
                    let on_click_tree_data_5 = on_click_tree_data_5.clone();
                    let new_path_5 = new_path_5.clone();
                    view! {
                        <hr/>
                        <div>
                            implements trait: {item.tag.name} <br/>
                            <For
                                each=move || item.methods.get()
                                key=move |method_item| method_item.tag.id.get()
                                let:method_item
                            >

                                {
                                    let on_click_tree_data = on_click_tree_data_5.clone();
                                    let new_path = new_path_5.clone();
                                    let on_click_closure = move |_| on_click_tree_data(
                                        TreeNodeDataSelectionType::TraitMethod {
                                            trait_id: item.tag.id.get(),
                                            method_id: method_item.tag.id.get(),
                                        },
                                        method_item.return_type.get(),
                                        new_path.clone(),
                                    );
                                    view! {
                                        <div on:click=on_click_closure>
                                            {method_item.tag.name} :
                                            {move || method_item.return_type.get().to_string()}
                                        </div>
                                    }
                                }

                            </For>
                        </div>
                    }
                }

            </For>
        </div>
        <div class="flex">
            <For
                each=move || tree_element.get().instance_constituents
                key=move |&item| item
                let:child
            >

                {
                    let on_click_tree_data_1 = on_click_tree_data_1.clone();
                    view! {
                        <div>
                            <TreeNode
                                on_click_tree_data=on_click_tree_data_1
                                element=TreeRef(TreeTypes::Instance, child)
                                path=new_path.clone()
                            />
                        </div>
                    }
                }

            </For>
            <For
                each=move || tree_element.get().library_operative_constituents
                key=move |&item| item
                let:child
            >

                {
                    let on_click_tree_data_2 = on_click_tree_data_2.clone();
                    view! {
                        <div>
                            <TreeNode
                                on_click_tree_data=on_click_tree_data_2
                                element=TreeRef(TreeTypes::LibraryOperative, child)
                                path=new_path_2.clone()
                            />
                        </div>
                    }
                }

            </For>
            <For
                each=move || tree_element.get().trait_operative_constituents
                key=move |item| item.trait_id
                let:child
            >

                {
                    let on_click_tree_data_3 = on_click_tree_data_3.clone();
                    view! {
                        <div>
                            <TreeNode
                                on_click_tree_data=on_click_tree_data_3
                                element=TreeRef(
                                    TreeTypes::TraitOperative(child.clone()),
                                    child.trait_id.get(),
                                )

                                path=new_path_3.clone()
                            />
                        </div>
                    }
                }

            </For>
        </div>
    }
}
