use std::rc::Rc;

use crate::utils::{
    export_schema,
    reactive_types::{
        RConstraintObject, RConstraintSchema, RLibraryInstance, RLibraryOperative, RTag,
    },
};
use leptos::{logging::log, *};
use serde_types::{
    common::Uid,
    constraint_schema::ConstraintSchema,
    primitives::{PrimitiveTypes, PrimitiveValues},
};

enum ListSource {
    Instance,
    ConstraintObject,
    Operative,
}

#[derive(Clone)]
struct SchemaContext {
    selected_constraint_object: RwSignal<Option<Uid>>,
    selected_instance: RwSignal<Option<Uid>>,
    selected_operative: RwSignal<Option<Uid>>,
}

#[component]
pub fn App(schema: ConstraintSchema<PrimitiveTypes, PrimitiveValues>) -> impl IntoView {
    let reactive_schema: RConstraintSchema<PrimitiveTypes, PrimitiveValues> = schema.into();
    let constraint_objects = reactive_schema.constraint_objects;
    let instances = reactive_schema.instance_library;
    let operatives = reactive_schema.operative_library;

    let selected_constraint_object = RwSignal::<Option<Uid>>::new(None);
    let selected_instance = RwSignal::<Option<Uid>>::new(None);
    let selected_operative = RwSignal::<Option<Uid>>::new(None);

    provide_context(SchemaContext {
        selected_constraint_object: selected_constraint_object.clone(),
        selected_instance: selected_instance.clone(),
        selected_operative: selected_operative.clone(),
    });

    let handle_list_item_click = Rc::new(move |id: Uid, source: ListSource| {
        log!("{}", id);
        match source {
            ListSource::Instance => selected_instance.set(Some(id)),
            ListSource::ConstraintObject => selected_constraint_object.set(Some(id)),
            ListSource::Operative => selected_operative.set(Some(id)),
        }
    });

    let handle_list_item_click2 = handle_list_item_click.clone();
    let handle_list_item_click3 = handle_list_item_click.clone();

    view! {
        // <button on:click=move |_| export_schema(&reactive_schema)>Export Schema</button>
        <div class="flex">
            <div class="flex-grow ">
            <div class="large-margin med-pad half-height border-gray">
            <h2>Constraint Objects</h2>
            <For
                each=constraint_objects
                key=move |(id, _child)| id.clone()
                children=move |(_id, child)| {
                    let clone = handle_list_item_click.clone();
                    view!{<RootListItem tag={child.tag} on_click={move |id: Uid| clone(id, ListSource::ConstraintObject)}/>}
                }
            />
            </div>
            <Show when=move ||selected_constraint_object.get().is_some()>
            <div class="large-margin med-pad half-height border-gray">
                   <div on:click= move |_| selected_constraint_object.set(None)>X</div>
            </div>
            </Show>

            </div>


            <div class="flex-grow ">
            <div class="large-margin med-pad half-height border-gray">
            <h2>Operatives</h2>
            <For
                each=operatives
                key=move |(id, _child)| id.clone()
                children=move |(_id, child)| {
                    let clone = handle_list_item_click2.clone();
                    view!{<RootListItem tag={child.tag} on_click={move |id: Uid| clone(id, ListSource::Operative)}/>}
                }
            />
            </div>
            </div>

            <div class="flex-grow ">
            <div class="large-margin med-pad half-height border-gray">
            <h2>Instances</h2>
            <For
                each=instances
                key=move |(id, _child)| id.clone()
                children=move |(_id, child)| {
                    let clone = handle_list_item_click3.clone();
                    view!{<RootListItem tag={child.tag} on_click={move |id:Uid| clone(id, ListSource::Instance)}/>}
                }
            />
            </div>
            </div>
        </div>
    }
}

#[component]
pub fn RootListItem<F>(tag: RTag, on_click: F) -> impl IntoView
where
    F: Fn(Uid) + 'static,
{
    let ctx = use_context::<SchemaContext>().unwrap();

    let class = create_memo(move |_| {
        if ctx.selected_constraint_object.get() == Some(tag.id.get())
            || ctx.selected_operative.get() == Some(tag.id.get())
            || ctx.selected_instance.get() == Some(tag.id.get())
        {
            "border-red"
        } else {
            "border-invisible"
        }
    });

    view! {
        <div on:click=move|_e| on_click(tag.id.get()) class=class>
            {tag.name}
        </div>
    }
}
