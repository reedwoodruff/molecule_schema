use std::fmt::Display;

use crate::components::{
    configure_schema_object::{
        edit_operative::EditOperative, edit_template::EditTemplate, edit_trait::EditTrait,
    },
    tree_view::TreeRef,
};

use base_types::{
    common::Uid,
    constraint_schema::ConstraintSchema,
    primitives::{PrimitiveTypes, PrimitiveValues},
};
use leptos::prelude::*;
use reactive_types::reactive_types::{
    RConstraintSchema, RLibraryTemplate, RTag, RTraitDef, RTraitOperative,
};

#[derive(Clone, PartialEq, Debug)]
pub enum ListItemTypes {
    Template(RwSignal<Uid>),
    Operative(RwSignal<Uid>),
    Instance(RwSignal<Uid>),
    Trait(RwSignal<Uid>),
}
impl ListItemTypes {
    pub fn get_id(&self) -> RwSignal<Uid> {
        match self {
            ListItemTypes::Template(id) => *id,
            ListItemTypes::Operative(id) => *id,
            ListItemTypes::Instance(id) => *id,
            ListItemTypes::Trait(id) => *id,
        }
    }
}
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
pub struct SchemaContext {
    pub schema: RConstraintSchema<PrimitiveTypes, PrimitiveValues>,
    pub selected_element: RwSignal<Option<ListItemTypes>>,
}
pub fn print_schema_reactive(reactive_schema: &RConstraintSchema<PrimitiveTypes, PrimitiveValues>) {
    let base_schema: ConstraintSchema<PrimitiveTypes, PrimitiveValues> =
        reactive_schema.clone().into();
    let json = serde_json::to_string_pretty(&base_schema).unwrap();
    // std::fs::write(path, json).expect("Unable to write file");
    log!("{}", json);
}

#[component]
pub fn App(schema: ConstraintSchema<PrimitiveTypes, PrimitiveValues>) -> impl IntoView {
    let reactive_schema: RConstraintSchema<PrimitiveTypes, PrimitiveValues> = schema.into();
    let constraint_objects = reactive_schema.template_library;
    let sorted_constraint_objects = create_memo(move |_| {
        let mut values = reactive_schema
            .template_library
            .get()
            .values()
            .cloned()
            .collect::<Vec<_>>();
        values.sort_by(|a, b| a.tag.name.get().cmp(&b.tag.name.get()));
        values
    });
    let instances = reactive_schema.instance_library;
    let sorted_instances = create_memo(move |_| {
        let mut values = reactive_schema
            .instance_library
            .get()
            .values()
            .cloned()
            .collect::<Vec<_>>();
        values.sort_by(|a, b| a.tag.name.get().cmp(&b.tag.name.get()));
        values
    });
    let operatives = reactive_schema.operative_library;
    let sorted_operatives = create_memo(move |_| {
        let mut values = reactive_schema
            .operative_library
            .get()
            .values()
            .cloned()
            .collect::<Vec<_>>();
        values.sort_by(|a, b| a.tag.name.get().cmp(&b.tag.name.get()));
        values
    });
    let traits = reactive_schema.traits;
    let sorted_traits = create_memo(move |_| {
        let mut values = reactive_schema
            .traits
            .get()
            .values()
            .cloned()
            .collect::<Vec<_>>();
        values.sort_by(|a, b| a.tag.name.get().cmp(&b.tag.name.get()));
        values
    });
    let selected_element = RwSignal::new(None);

    provide_context(SchemaContext {
        schema: reactive_schema.clone(),
        selected_element,
    });

    // let handle_list_item_click =
    //     Rc::new(move |id: Uid, source: TreeTypes| selected_element.set(Some(TreeRef(source, id))));
    // let handle_list_item_click2 = handle_list_item_click.clone();
    // let handle_list_item_click3 = handle_list_item_click.clone();
    // let handle_list_item_click4 = handle_list_item_click.clone();

    let click_new_constraint_object = move |_| {
        constraint_objects.update(|prev| {
            let new_constraint_object = RLibraryTemplate::new();
            prev.insert(new_constraint_object.tag.id.get(), new_constraint_object);
        })
    };

    view! {
        <button on:click=move |_| print_schema_reactive(&reactive_schema)>Export Schema</button>
        <div class="flex">
            <div class="flex-grow ">
                <div class="large-margin med-pad border-gray">
                    <h2>Templates <button on:click=click_new_constraint_object>+</button></h2>
                    <For
                        each=move || sorted_constraint_objects.get()
                        key=move |child| child.tag.id
                        children=move |child| {
                            view! {
                                // let clone = handle_list_item_click.clone();
                                <div>
                                    <RootListItem
                                        tag=child.tag
                                        on_click=move |id: Uid| {
                                            selected_element
                                                .set(Some(ListItemTypes::Template(RwSignal::new(id))))
                                        }
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
                        each=move || sorted_operatives.get()
                        key=move |child| child.tag.id
                        children=move |child| {
                            view! {
                                // let clone = handle_list_item_click2.clone();
                                <RootListItem
                                    tag=child.tag
                                    on_click=move |id: Uid| {
                                        selected_element
                                            .set(Some(ListItemTypes::Operative(RwSignal::new(id))))
                                    }
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
                        each=move || sorted_instances.get()
                        key=move |child| child.tag.id
                        children=move |child| {
                            view! {
                                // let clone = handle_list_item_click3.clone();
                                <RootListItem
                                    tag=child.tag
                                    on_click=move |id: Uid| {
                                        selected_element
                                            .set(Some(ListItemTypes::Instance(RwSignal::new(id))))
                                    }
                                />
                            }
                        }
                    />

                </div>
            </div>

            <div class="flex-grow ">
                <div class="large-margin med-pad border-gray">
                    <h2>
                        Traits
                        <button on:click=move |_| {
                            traits
                                .update(|prev_traits| {
                                    let new_trait = RTraitDef::<PrimitiveTypes>::new();
                                    prev_traits.insert(new_trait.tag.id.get(), new_trait);
                                })
                        }>+</button>
                    </h2>
                    <For
                        each=move || sorted_traits.get()
                        key=move |child| child.tag.id
                        children=move |child| {
                            view! {
                                <RootListItem
                                    tag=child.tag
                                    on_click=move |id: Uid| {
                                        selected_element
                                            .set(Some(ListItemTypes::Trait(RwSignal::new(id))))
                                    }
                                />
                            }
                        }
                    />

                </div>
            </div>
        </div>
        <Show when=move || {
            matches!(selected_element.get(), Some(ListItemTypes::Template(_template)))
        }>
            <EditTemplate element=TreeRef(
                TreeTypes::Template,
                selected_element.get().unwrap().get_id().get(),
            )/>
        </Show>
        <Show when=move || {
            matches!(selected_element.get(), Some(ListItemTypes::Operative(_operative)))
        }>
            <EditOperative element=TreeRef(
                TreeTypes::LibraryOperative,
                selected_element.get().unwrap().get_id().get(),
            )/>
        </Show>
        <Show when=move || {
            matches!(selected_element.get(), Some(ListItemTypes::Trait(_trait)))
        }>
            <EditTrait id=selected_element.get().unwrap().get_id()/>
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
            .is_some_and(|item| item.get_id().get() == tag.id.get())
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
