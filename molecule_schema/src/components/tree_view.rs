use std::collections::HashMap;
use std::sync::Arc;

use base_types::{
    common::{ConstraintTraits, Uid},
    primitives::PrimitiveTypes,
};
use leptos::prelude::*;

use crate::components::app::SchemaContext;
use reactive_types::{
    operative_digest::ROperativeDigest,
    reactive_item::RConstraintSchemaItem,
    reactive_types::{
        FieldInfo, RConstraintSchema, RFieldConstraint, ROperativeVariants, RTag, RTraitDef, Tagged,
    },
};

use super::app::TreeTypes;

#[component]
pub fn TreeView<F>(element: TreeRef, on_click_tree_data: Arc<F>) -> impl IntoView
where
    F: Send + Sync + Fn(TreeNodeDataSelectionType, PrimitiveTypes, Arc<Vec<TreeRef>>) + 'static,
{
    let path = Arc::new(Vec::new());
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
    operative_digest: Memo<ROperativeDigest>,
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
                .with(|trait_items| trait_items.get(id).unwrap().clone())
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
    path: Arc<Vec<TreeRef>>,
    on_click_tree_data: Arc<F>,
) -> impl IntoView
where
    F: Send + Sync + Fn(TreeNodeDataSelectionType, PrimitiveTypes, Arc<Vec<TreeRef>>) + 'static,
{
    let ctx = use_context::<SchemaContext>().unwrap();
    let schema_clone = ctx.schema.clone();
    let element_clone = element.clone();

    let mut ancestor_is_already_displayed = false;
    match &element.0 {
        TreeTypes::Instance => {
            let item = ctx
                .schema
                .instance_library
                .with(|instance_library| instance_library.get(&element.1).unwrap().clone());
            path.iter().for_each(|el| {
                if item.check_ancestry(&ctx.schema, &el.1) {
                    ancestor_is_already_displayed = true;
                }
            })
        }
        TreeTypes::Template => {
            let item = ctx
                .schema
                .template_library
                .with(|template_library| template_library.get(&element.1).unwrap().clone());
            path.iter().for_each(|el| {
                if item.check_ancestry(&ctx.schema, &el.1) {
                    ancestor_is_already_displayed = true;
                }
            })
        }
        TreeTypes::LibraryOperative => {
            let item = ctx
                .schema
                .operative_library
                .with(|operative_library| operative_library.get(&element.1).unwrap().clone());
            path.iter().for_each(|el| {
                if item.check_ancestry(&ctx.schema, &el.1) {
                    ancestor_is_already_displayed = true;
                }
            })
        }
        TreeTypes::TraitOperative(_) => {}
    }
    if ancestor_is_already_displayed {
        return view! { <div class="tree-node-container container_slot_unfulfilled">Recursion Buster</div> }.into_any();
    }

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

            let cloned_id = *id;
            TreeNodeInfo {
                top_level_type: TreeTypes::TraitOperative(trait_op.clone()),
                tag: trait_op.tag.clone(),
                fields: vec![],
                trait_impls,
                template_level_instances: vec![],
                operative_digest: create_memo(move |_| ROperativeDigest {
                    digest_object_id: cloned_id,
                    operative_slots: HashMap::new(),
                }),
            }
        }
    });

    // let mut new_path = (*path).clone();
    let mut new_path = path.as_ref().clone();
    new_path.push(element_clone);
    let new_path = Arc::new(new_path);
    let new_path_2 = new_path.clone();
    let new_path_3 = new_path.clone();
    let new_path_4 = new_path.clone();
    let new_path_5 = new_path.clone();

    let all_slots = create_memo(move |_| {
        tree_element
            .get()
            .operative_digest
            .get()
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
            <For
                each=move || tree_element.get().fields
                key=move |item| (item.tag.id.get(), item.value_type.clone())
                let:item
            >

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
                                key=move |(method_item_id, _method_item)| *method_item_id
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
                each=move || tree_element.get().operative_digest.get().operative_slots
                key=move |(slot_id, _slot)| (*slot_id, _slot.clone())
                let:slot_info
            >

                {
                    let on_click_tree_data_2 = on_click_tree_data_2.clone();
                    view! { <div></div> }
                }

            </For>
            <For each=move || all_slots.get() key=move |item| item.clone() let:child>

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
                    let related_instances_clone = Arc::new(child.related_instances.clone());
                    let show_instances = move || { !child.related_instances.is_empty() };
                    view! {
                        <div>
                            <div class=slot_class>Slot name: {child.slot.tag.name} <br/></div>
                            <div class="flex">
                                <div>
                                    <Show when=show_instances.clone()>Operative: <br/> <hr/></Show>
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

                                <Show when=show_instances>

                                    {
                                        let related_instances_clone = related_instances_clone
                                            .as_ref()
                                            .clone();
                                        view! {
                                            <div>
                                                Slotted instances: <br/>
                                                <For
                                                    each=move || related_instances_clone.clone()
                                                    key=move |instance| instance.clone()
                                                    let:instance
                                                >
                                                    <div>
                                                        {schema_clone
                                                            .instance_library
                                                            .with(|instance_library| {
                                                                instance_library
                                                                    .get(&instance.instance_id)
                                                                    .unwrap()
                                                                    .tag
                                                                    .name
                                                                    .get()
                                                            })}

                                                    </div>
                                                </For>
                                            </div>
                                        }
                                    }

                                </Show>
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
    .into_any()
}
