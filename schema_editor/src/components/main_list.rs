use generated_crate::prelude::*;
use leptos::either::EitherOf4;

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

    let list = move || {
        let schema = schema.clone();
        match selected_tab.get() {
            WorkspaceTab::Template(tab_state) => EitherOf4::A(view! {
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
