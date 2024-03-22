use std::{collections::HashMap, rc::Rc};

use leptos::{logging::log, *};
use serde_types::{
    common::{ConstraintTraits, Uid},
    primitives::PrimitiveTypes,
};

use crate::utils::{
    operative_digest::{ROperativeDigest, ROperativeSlotDigest},
    reactive_item::RConstraintSchemaItem,
    reactive_types::{
        FieldInfo, RConstraintSchema, RFieldConstraint, ROperativeVariants, RTag, RTraitDef,
        RTraitOperative, Tagged,
    },
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
    operative_digest: ROperativeDigest,
    template_level_instances: Vec<Uid>, // instance_constituents: Vec<Uid>,
}

fn get_tree_node_info<TTypes: ConstraintTraits, TValues: ConstraintTraits>(
    tree_type: TreeTypes,
    item: impl RConstraintSchemaItem<TTypes = TTypes, TValues = TValues>,
    schema: &RConstraintSchema<TTypes, TValues>,
) -> TreeNodeInfo<TTypes, TValues> {
    let element_tag = item.get_tag().clone();
    let parent_template = schema
        .template_library
        .with(|templates| templates.get(&item.get_template_id()).unwrap().clone());
    let fields = parent_template
        .field_constraints
        .get()
        .values()
        .map(|field| FieldInfoStruct {
            tag: field.get_tag().clone(),
            value_type: <RFieldConstraint<TTypes> as FieldInfo<TTypes, TValues>>::get_value_type(
                field,
            ),
            value: field.get_value(),
        })
        .collect::<Vec<_>>();
    let traits_impled_ids = item
        .get_trait_impl_digest(schema)
        .trait_impls
        .keys()
        .cloned()
        .collect::<Vec<_>>();
    let traits_impled_defs = traits_impled_ids
        .iter()
        .map(|id| {
            schema
                .traits
                .with(|trait_items| trait_items.get(&id).unwrap().clone())
        })
        .collect();

    let operative_digest = item.get_operative_digest(schema);
    let template_level_instances = parent_template.instances.get();

    TreeNodeInfo {
        top_level_type: tree_type,
        tag: element_tag,
        fields,
        trait_impls: traits_impled_defs,
        operative_digest,
        template_level_instances,
    }
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
        TreeRef(TreeTypes::Template, id) => {
            let element = move || {
                ctx.schema
                    .template_library
                    .with(|o| o.get(id).unwrap().clone())
            };
            get_tree_node_info(TreeTypes::Template, element(), &ctx.schema)
        }
        TreeRef(TreeTypes::Instance, id) => {
            let element = move || {
                ctx.schema
                    .instance_library
                    .with(|o| o.get(id).unwrap().clone())
            };
            get_tree_node_info(TreeTypes::Instance, element(), &ctx.schema)
        }
        TreeRef(TreeTypes::LibraryOperative, id) => {
            let element = move || {
                ctx.schema
                    .operative_library
                    .with(|o| o.get(id).unwrap().clone())
            };
            get_tree_node_info(TreeTypes::LibraryOperative, element(), &ctx.schema)
        }

        TreeRef(TreeTypes::TraitOperative(trait_op), id) => {
            let trait_impls = trait_op.trait_ids.with(|trait_op_trait_ids| {
                trait_op_trait_ids
                    .iter()
                    .map(|trait_op_trait_id| {
                        ctx.schema
                            .traits
                            .with(|el| el.get(trait_op_trait_id).unwrap().clone())
                    })
                    .collect::<Vec<_>>()
            });

            TreeNodeInfo {
                top_level_type: TreeTypes::TraitOperative(trait_op.clone()),
                tag: trait_op.tag.clone(),
                fields: vec![],
                trait_impls: trait_impls,
                template_level_instances: vec![],
                operative_digest: ROperativeDigest {
                    digest_object_id: *id,
                    operative_slots: HashMap::new(),
                },
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

    let all_slots = create_memo(move |_| {
        tree_element
            .get()
            .operative_digest
            .operative_slots
            .values()
            .cloned()
            .collect::<Vec<_>>()
    });

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
                                key=move |(method_item_id, _method_item)| method_item_id.clone()
                                let:method_item
                            >

                                {
                                    let on_click_tree_data = on_click_tree_data_5.clone();
                                    let new_path = new_path_5.clone();
                                    let on_click_closure = move |_| on_click_tree_data(
                                        TreeNodeDataSelectionType::TraitMethod {
                                            trait_id: item.tag.id.get(),
                                            method_id: method_item.0,
                                        },
                                        method_item.1.return_type.get(),
                                        new_path.clone(),
                                    );
                                    view! {
                                        <div on:click=on_click_closure>
                                            {method_item.1.tag.name} :
                                            {move || method_item.1.return_type.get().to_string()}
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
                each=move || tree_element.get().template_level_instances
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
                each=move || tree_element.get().operative_digest.operative_slots
                key=move |(slot_id, _slot)| slot_id.clone()
                let:slot_info
            >

                {
                    let on_click_tree_data_2 = on_click_tree_data_2.clone();
                    view! { <div></div> }
                }

            </For>
            <For each=all_slots key=move |item| item.slot.tag.id.get() let:child>

                {
                    let on_click_tree_data_3 = on_click_tree_data_3.clone();
                    let child_clone_2 = child.clone();
                    let slot_class = move || {
                        "tree-node-container width-100 container_slot_".to_string()
                            + if child_clone_2.get_fulfillment_status() {
                                "fulfilled"
                            } else {
                                "unfulfilled"
                            }
                    };
                    view! {
                        <div>
                            <div class=slot_class>Slot name: {child.slot.tag.name} <br/></div>
                            <div class="flex">
                                <div>
                                    Operative <br/> <hr/>
                                    <TreeNode
                                        on_click_tree_data=on_click_tree_data_3
                                        element=match child.slot.operative_descriptor {
                                            ROperativeVariants::LibraryOperative(id) => {
                                                TreeRef(TreeTypes::LibraryOperative, id.get())
                                            }
                                            ROperativeVariants::TraitOperative(trait_op) => {
                                                TreeRef(
                                                    TreeTypes::TraitOperative(trait_op.clone()),
                                                    trait_op.tag.id.get(),
                                                )
                                            }
                                        }

                                        path=new_path_2.clone()
                                    />
                                </div>
                                <Show when=move || {
                                    child.related_instances.len() > 0
                                }>INSTANCE</Show>
                            </div>
                        </div>
                    }
                }

            </For>
        // <For
        // each=move || tree_element.get().trait_operative_constituents
        // key=move |item| item.tag.id.get()
        // let:child
        // >
        //
        // {
        // let on_click_tree_data_3 = on_click_tree_data_3.clone();
        // view! {
        // <div>
        // <TreeNode
        // on_click_tree_data=on_click_tree_data_3
        // element=TreeRef(
        // TreeTypes::TraitOperative(child.clone()),
        // child.tag.id.get(),
        // )
        //
        // path=new_path_3.clone()
        // />
        // </div>
        // }
        // }
        //
        // </For>
        </div>
    }
}
