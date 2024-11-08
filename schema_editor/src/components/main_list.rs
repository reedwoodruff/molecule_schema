use leptos::either::EitherOf4;
use schema_editor_generated_toolkit::prelude::*;

use crate::components::common::Button;

use super::workspace::{WorkspaceState, WorkspaceTab};

#[component]
pub fn MainList() -> impl IntoView {
    let ctx = use_context::<SharedGraph<Schema>>().unwrap();
    let WorkspaceState {
        schema,
        selected_tab,
    } = use_context::<WorkspaceState>().unwrap();

    fn list_item_view<T: RootConstraints>(
        list_item: RGSOConcrete<T, Schema>,
        tab_state: RwSignal<Option<RGSOConcrete<T, Schema>>>,
    ) -> impl IntoView
    where
        RGSOConcrete<T, Schema>: GetName,
    {
        let list_item2 = list_item.clone();
        let active_string = move || {
            tab_state
                .get()
                .filter(|selected_list_item| selected_list_item.get_id() == list_item2.get_id())
                .map_or("", |_| "active")
        };
        let list_item = list_item.clone();
        let list_item2 = list_item.clone();
        let class_string = move || format!("clickable-list-item {}", active_string());
        view! {
            <div>
                <a class=class_string
                    on:click=move |_| {let list_item = list_item.clone(); tab_state.set(Some(list_item))}>
                    {move || list_item2.get_name()}
                </a>
            </div>
        }
    }

    let schema_clone = schema.clone();
    let create_new_button_view = move |variant: <SchemaConcrete as HasSlotEnum>::SlotEnum| {
        let on_click_button = move |_| {
            match variant {
                SchemaConcreteAllSlots::Operatives => unreachable!(),
                SchemaConcreteAllSlots::Instances => todo!(),
                SchemaConcreteAllSlots::Templates => schema_clone
                    .edit(ctx.clone())
                    .add_new_templates(|new_template| new_template.set_name("new".to_string()))
                    .execute()
                    .unwrap(),
                SchemaConcreteAllSlots::Traits => schema_clone
                    .edit(ctx.clone())
                    .add_new_traits(|new_trait| new_trait.set_name("new".to_string()))
                    .execute()
                    .unwrap(),
            };
        };
        view! {
            <Button on:click=on_click_button>Create New</Button>
        }
    };

    let list = move || {
        let schema = schema.clone();
        let create_new_button_view = create_new_button_view.clone();
        match selected_tab.get() {
            WorkspaceTab::Template(tab_state) => EitherOf4::A(view! {
                {move || create_new_button_view.clone()(SchemaConcreteAllSlots::Templates)}
                <For
                each=move || schema.get_templates_slot()
                key=|item| item.get_id().clone()
                children=move |item| list_item_view(item, tab_state.clone())
                >
                </For>
            }),
            WorkspaceTab::Operative(tab_state) => EitherOf4::B(view! {
                <For
                each=move || schema.get_operatives_slot()
                key=|item| item.get_id().clone()
                children=move |item| list_item_view(item, tab_state.clone())
                >
                </For>
            }),
            WorkspaceTab::Instance(tab_state) => EitherOf4::C(view! {
                <For
                each=move || schema.get_instances_slot()
                key=|item| item.get_id().clone()
                children=move |item| list_item_view(item, tab_state.clone())
                >
                </For>
            }),
            WorkspaceTab::Trait(tab_state) => EitherOf4::D(view! {
                {move || create_new_button_view.clone()(SchemaConcreteAllSlots::Traits)}
                <For
                each=move || schema.get_traits_slot()
                key=|item| item.get_id().clone()
                children=move |item| list_item_view(item, tab_state.clone())
                >
                </For>
            }),
        }
    };

    view! {
        <div class="list-container">
            {list}
        </div>
    }
}
