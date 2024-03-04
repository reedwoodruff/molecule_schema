use std::rc::Rc;

use leptos::{logging::log, *};
use serde_types::{
    common::{ConstraintTraits, Uid},
    primitives::PrimitiveTypes,
};

use crate::utils::{
    reactive_item::RConstraintSchemaItem,
    reactive_types::{
        FieldInfo, RConstraintSchema, RFieldConstraint, RTag, RTraitDef, RTraitOperative, Tagged,
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
    instance_constituents: Vec<Uid>,
    library_operative_constituents: Vec<Uid>,
    trait_operative_constituents: Vec<RTraitOperative>,
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
        .iter()
        .map(|field| FieldInfoStruct {
            tag: field.get_tag().clone(),
            value_type: <RFieldConstraint<TTypes> as FieldInfo<TTypes, TValues>>::get_value_type(
                field,
            ),
            value: field.get_value(),
        })
        .collect::<Vec<_>>();
    let mut traits_impled = item.get_local_trait_impls();
    traits_impled.extend(item.get_ancestors_trait_impls(schema));
    let traits_impled = traits_impled
        .keys()
        .map(|id| {
            schema
                .traits
                .with(|trait_items| trait_items.get(id).unwrap().clone())
        })
        .collect();

    let instance_constituents = item.get_all_constituent_instance_ids(schema);
    let library_operative_constituents = item.get_all_unfulfilled_library_operatives_ids(schema);
    let trait_operative_constituents = item.get_all_unfulfilled_trait_operatives(schema);

    TreeNodeInfo {
        top_level_type: tree_type,
        tag: element_tag,
        fields,
        trait_impls: traits_impled,
        instance_constituents,
        library_operative_constituents,
        trait_operative_constituents,
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
            let element = move || ctx.schema.traits.with(|el| el.get(id).unwrap().clone());

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
