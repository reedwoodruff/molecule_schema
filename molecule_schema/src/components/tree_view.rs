use leptos::*;
use serde_types::common::{ConstraintTraits, Uid};

use crate::utils::reactive_types::{
    ROperativeVariants, RTag, RTraitDef, RTraitOperative, Tagged, RCSO,
};

use super::{SchemaContext, TreeTypes};

#[component]
pub fn TreeView(element: TreeRef) -> impl IntoView {
    let path = Vec::new();
    view! {<TreeNode element=element path=path />}
}

#[derive(Clone, PartialEq)]
pub struct TreeRef(pub TreeTypes, pub Uid);

#[derive(Clone, PartialEq)]
struct TreeNodeInfo<TTypes: ConstraintTraits> {
    top_level_type: TreeTypes,
    tag: RTag,
    fields: Vec<RTag>,
    trait_impls: Vec<RTraitDef<TTypes>>,
    instance_constituents: Vec<Uid>,
    library_operative_constituents: Vec<Uid>,
    trait_operative_constituents: Vec<RTraitOperative>,
}

#[component]
pub fn TreeNode(element: TreeRef, path: Vec<TreeRef>) -> impl IntoView {
    let ctx = use_context::<SchemaContext>().unwrap();
    let element_clone = element.clone();

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
                .map(|field| field.get_tag().clone())
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
                .map(|field| field.get_tag().clone())
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
                .map(|field| field.get_tag().clone())
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
            TreeNodeInfo {
                top_level_type: TreeTypes::TraitOperative(trait_op.clone()),
                tag: trait_op.tag.clone(),
                fields: vec![],
                trait_impls: vec![],
                instance_constituents: vec![],
                library_operative_constituents: vec![],
                trait_operative_constituents: vec![],
            }
        }
    });

    let mut new_path = path.clone();
    new_path.push(element_clone);
    let new_path_2 = new_path.clone();
    let new_path_3 = new_path.clone();

    view! {
        <div class=move || "tree-node-container container_".to_string() + &tree_element.get().top_level_type.to_string()>
            <h3 class="small-title">
                {move || tree_element.get().tag.name}
            </h3>
            <For each=move ||tree_element.get().fields key=move |item| item.id let:item>
                <div>{item.name}</div>
            </For>
        // {move || tree_element.get().fields.iter().map(|el| el.name).collect::<Vec<_>>()}
        </div>
        <div class="flex">
            <For each=move || tree_element.get().instance_constituents key=move |&item| item let:child>
            <div><TreeNode element={TreeRef(TreeTypes::Instance, child)} path=new_path.clone() /></div>
            </For>
            <For each=move || tree_element.get().library_operative_constituents key=move |&item| item let:child>
            <div><TreeNode element={TreeRef(TreeTypes::LibraryOperative, child)} path=new_path_2.clone() /></div>
            </For>
            <For each=move || tree_element.get().trait_operative_constituents key=move |item| item.trait_id.clone() let:child>
            <div><TreeNode element={TreeRef(TreeTypes::TraitOperative(child.clone()), child.trait_id.get())} path=new_path_3.clone() /></div>
            </For>
        </div>
    }
}
