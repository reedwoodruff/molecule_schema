use std::{fmt::Display, rc::Rc};

use crate::{
    components::configure_schema_object::edit_template::EditTemplate,
    utils::{
        export_schema,
        reactive_types::{RConstraintSchema, RLibraryTemplate, RTag, RTraitOperative},
    },
};
use leptos::{logging::log, *};
use serde_types::{
    common::Uid,
    constraint_schema::ConstraintSchema,
    primitives::{PrimitiveTypes, PrimitiveValues},
};

use self::tree_view_revamp::TreeRef;

pub mod common;
pub mod configure_schema_object;
pub mod tree_view;
pub mod tree_view_revamp;

#[derive(Clone, PartialEq, Debug)]
pub enum TreeTypes {
    // Trait,
    Instance,
    Template,
    LibraryOperative,
    TraitOperative(RTraitOperative),
}
impl Display for TreeTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TreeTypes::Instance => write!(f, "instance"),
            TreeTypes::Template => write!(f, "constraint_object"),
            TreeTypes::LibraryOperative => write!(f, "library_operative"),
            TreeTypes::TraitOperative(_) => write!(f, "trait_operative"),
        }
    }
}

#[derive(Clone)]
struct SchemaContext {
    schema: RConstraintSchema<PrimitiveTypes, PrimitiveValues>,
    selected_element: RwSignal<Option<TreeRef>>,
}

#[component]
pub fn App(schema: ConstraintSchema<PrimitiveTypes, PrimitiveValues>) -> impl IntoView {
    let reactive_schema: RConstraintSchema<PrimitiveTypes, PrimitiveValues> = schema.into();
    let constraint_objects = reactive_schema.template_library;
    let instances = reactive_schema.instance_library;
    let operatives = reactive_schema.operative_library;
    let _traits = reactive_schema.traits;
    let selected_element = RwSignal::new(None);

    provide_context(SchemaContext {
        schema: reactive_schema.clone(),
        selected_element,
    });

    let handle_list_item_click =
        Rc::new(move |id: Uid, source: TreeTypes| selected_element.set(Some(TreeRef(source, id))));
    let handle_list_item_click2 = handle_list_item_click.clone();
    let handle_list_item_click3 = handle_list_item_click.clone();
    let _handle_list_item_click4 = handle_list_item_click.clone();

    let click_new_constraint_object = move |_| {
        constraint_objects.update(|prev| {
            let new_constraint_object = RLibraryTemplate::new();
            prev.insert(new_constraint_object.tag.id.get(), new_constraint_object);
        })
    };

    view! {
        <button on:click=move |_| export_schema(&reactive_schema)>Export Schema</button>
        <div class="flex">
            <div class="flex-grow ">
                <div class="large-margin med-pad border-gray">
                    <h2>Templates <button on:click=click_new_constraint_object>+</button></h2>
                    <For
                        each=constraint_objects
                        key=move |(id, _child)| *id
                        children=move |(_el_id, child)| {
                            let clone = handle_list_item_click.clone();
                            view! {
                                <div>
                                    <RootListItem
                                        tag=child.tag
                                        on_click=move |id: Uid| clone(id, TreeTypes::Template)
                                    />
                                </div>
                            }
                        }
                    />

                </div>

            </div>

            <div class="flex-grow ">
                <div class="large-margin med-pad border-gray">
                    <h2>Operatives</h2>
                    <For
                        each=operatives
                        key=move |(id, _child)| *id
                        children=move |(_id, child)| {
                            let clone = handle_list_item_click2.clone();
                            view! {
                                <RootListItem
                                    tag=child.tag
                                    on_click=move |id: Uid| clone(id, TreeTypes::LibraryOperative)
                                />
                            }
                        }
                    />

                </div>
            </div>

            <div class="flex-grow ">
                <div class="large-margin med-pad border-gray">
                    <h2>Instances</h2>
                    <For
                        each=instances
                        key=move |(id, _child)| *id
                        children=move |(_id, child)| {
                            let clone = handle_list_item_click3.clone();
                            view! {
                                <RootListItem
                                    tag=child.tag
                                    on_click=move |id: Uid| clone(id, TreeTypes::Instance)
                                />
                            }
                        }
                    />

                </div>
            </div>

        // <div class="flex-grow ">
        // <div class="large-margin med-pad half-height border-gray">
        // <h2>Traits</h2>
        // <For
        // each=traits
        // key=move |(id, _child)| id.clone()
        // children=move |(_id, child)| {
        // let clone = handle_list_item_click4.clone();
        // view!{<RootListItem tag={child.tag} on_click={move |id:Uid| clone(id, TopLevelType::Trait)}/>}
        // }
        // />
        // </div>
        // </div>
        </div>
        <Show when=move || {
            match selected_element.get() {
                Some(TreeRef(TreeTypes::Template, _id_)) => true,
                _ => false,
            }
        }>
            <EditTemplate element=selected_element.get().unwrap()/>
        </Show>
        <Show when=move || {
            match selected_element.get() {
                Some(TreeRef(TreeTypes::LibraryOperative, _id_)) => true,
                _ => false,
            }
        }>
            <configure_schema_object::edit_operative::EditOperative element=selected_element
                .get()
                .unwrap()></configure_schema_object::edit_operative::EditOperative>
        </Show>
    }
}

#[component]
pub fn RootListItem<F>(tag: RTag, on_click: F) -> impl IntoView
where
    F: Fn(Uid) + 'static,
{
    let ctx = use_context::<SchemaContext>().unwrap();

    let class = create_memo(move |_| {
        if ctx
            .selected_element
            .get()
            .is_some_and(|TreeRef(_top_level_type, id)| id == tag.id.get())
        {
            "border-red"
        } else {
            "border-invisible"
        }
    });

    view! {
        <div on:click=move |_e| on_click(tag.id.get()) class=class>
            {tag.name}
        </div>
    }
}
