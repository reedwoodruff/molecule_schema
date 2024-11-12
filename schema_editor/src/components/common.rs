use std::{fmt::Display, str::FromStr};

use leptos::{either::Either, prelude::*};
use schema_editor_generated_toolkit::prelude::{GetName, RGSO};
use web_sys::Event;

#[component]
pub fn SignalTextInput<T>(
    value: RwSignal<T>,
    // #[prop(optional)] show_save_button: Option<bool>,
    // on_save: F,
) -> impl IntoView
where
    T: ToString + Send + Sync + Clone + FromStr + 'static,
{
    let on_input = move |e| {
        match T::from_str(&event_target_value(&e)) {
            Ok(val) => value.set(val),
            Err(_) => (),
        };
    };
    view! { <input value=move || value.get().to_string() on:input=on_input/> }
}

#[component]
pub fn ManagedTextInput<F, G>(
    getter: F,
    setter: G,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] input_ref: Option<NodeRef<leptos::html::Input>>, // children: Option<Children>,
) -> impl IntoView
where
    F: Fn() -> String + Send + Sync + 'static,
    G: Fn(String) + Send + Sync + 'static,
{
    let final_postfix = move || {
        if let Some(children) = children {
            Either::Left(children())
        } else {
            Either::Right(view! {})
        }
    };

    if let Some(input_ref) = input_ref {
        // let the_input = input().node_ref(input_ref);
        Either::Left(view! {
           <input node_ref=input_ref class="inner-text-input" prop:value=getter on:input=move |e| setter(event_target_value(&e))/>
           {final_postfix()}
        })
    } else {
        Either::Right(view! {
           <input class="inner-text-input" prop:value=getter on:input=move |e| setter(event_target_value(&e))/>
           {final_postfix()}
        })
    }
}

#[component]
pub fn ToggleManagedTextInput<F, G>(getter: F, setter: G) -> impl IntoView
where
    F: Fn() -> String + Send + Sync + Clone + 'static,
    G: Fn(String) + Send + Sync + Clone + 'static,
{
    let is_editing = RwSignal::new(false);
    let input_ref = NodeRef::<leptos::html::Input>::new();

    let toggle_text = move || match is_editing.get() {
        true => "Finish",
        false => "Edit",
    };

    let on_click_toggle = move |_| {
        is_editing.update(|prev| {
            *prev = !*prev;
        });
    };

    Effect::new(move || {
        if let Some(input_ref) = input_ref.get() {
            input_ref.focus().unwrap();
        }
    });

    let view = move || {
        let toggle_button = view! {
            <Button attr:class="edit-toggle-button" on:click=on_click_toggle>
                {toggle_text}
            </Button>
        };
        match is_editing.get() {
            true => Either::Left(
                view! {<ManagedTextInput input_ref getter=getter.clone() setter=setter.clone() >{toggle_button}</ManagedTextInput>},
            ),
            false => Either::Right(view! {
            <span>{getter.clone()}
            <span style="width: 15px; display: inline-block"></span>
            {toggle_button}
            </span>}),
        }
    };
    view
}

#[component]
pub fn LeafSection(children: Children) -> impl IntoView {
    view! {
        <div class="leafsection">
        {children()}
        </div>
    }
}
#[component]
pub fn LeafSectionHeader(children: Children) -> impl IntoView {
    view! {
        <h4 class="leafsection-header">
        {children()}
        </h4>
    }
}

#[component]
pub fn SubSection(children: Children) -> impl IntoView {
    view! {
        <div class="subsection">
        {children()}
        </div>
    }
}
#[component]
pub fn SubSectionHeader(children: Children) -> impl IntoView {
    view! {
        <h3 class="subsection-header">
        {children()}
        </h3>
    }
}

#[slot]
pub struct SectionHeader {
    // #[prop(optional)]
    children: Children,
}
#[component]
pub fn Section(section_header: SectionHeader, children: Children) -> impl IntoView {
    let is_collapsed = RwSignal::new(false);
    let collapsed_class = move || match is_collapsed.get() {
        true => "collapsed-children",
        false => "",
    };
    let children_div_class = move || format!("{}", collapsed_class());
    view! {
        <section class="section">
        <div class="flex">
            <div class="flex-grow">
            <h2 class="section-header">
            {(section_header.children)()}
            </h2>
            </div>
            <div>
            <Button on:click=move |_| is_collapsed.update(|prev| *prev = !*prev)>{move || match is_collapsed.get() {
                true => "+".to_string(),
                false => "-".to_string(),
            }}
            </Button>
            </div>
        </div>
        <div class=children_div_class>
        {children()}
        </div>
        </section>
    }
}

#[component]
pub fn Button(children: Children) -> impl IntoView {
    view! {
        <button>
        {children()}
        </button>
    }
}

#[component]
pub fn SignalEnumSelect<T>(
    value: RwSignal<T>,
    #[prop(optional)] hook: Option<Callback<T>>,
) -> impl IntoView
where
    T: Send + Sync + Clone + Display + strum::IntoEnumIterator + FromStr + 'static,
    <T as strum::IntoEnumIterator>::Iterator: Send + Sync,
{
    let cur_value = move || value.get().to_string();
    let on_change_value = move |e| {
        let return_val = event_target_value(&e);
        value.set(T::from_str(&return_val).ok().unwrap());
        if let Some(hook) = hook {
            hook.run(T::from_str(&return_val).ok().unwrap());
        }
    };

    view! {
        <select prop:value=cur_value on:change=on_change_value>
        <For each=move || T::iter() key=|item| item.to_string() let:discriminant>
            <option prop:selected = move || discriminant.to_string() == cur_value()>{discriminant.to_string()}</option>
        </For>
        </select>
    }
}
#[component]
pub fn ManagedEnumSelect<T, F>(getter: F, setter: Callback<T>) -> impl IntoView
where
    T: Send + Sync + Clone + Display + strum::IntoEnumIterator + FromStr + 'static + PartialEq,
    <T as strum::IntoEnumIterator>::Iterator: Send + Sync,
    F: Fn() -> T + Send + Sync + Clone + 'static,
{
    // let cur_value = move || value.get().to_string();
    let on_change_value = move |e: Event| {
        let return_val = event_target_value(&e);
        // value.set(T::from_str(&return_val).ok().unwrap());
        setter.run(T::from_str(&return_val).ok().unwrap());
    };

    let getter_clone = getter.clone();
    view! {
        <select prop:value=move || getter_clone.clone()().to_string() on:change=on_change_value>
        <For each=move || T::iter() key=|item| item.to_string() let:discriminant>
            {
            let getter_clone = getter.clone();
            view!{<option prop:selected = move || discriminant == getter_clone()>{discriminant.to_string()}</option>}
            }
        </For>
        </select>
    }
}

#[component]
pub fn SignalSelectWithOptions<T>(
    value: RwSignal<Option<T>>,
    #[prop(into)] options: Signal<Vec<T>>,
    #[prop(optional)] empty_allowed: bool,
) -> impl IntoView
where
    // I: IntoIterator<Item = T> + Clone + 'static + Send + Sync,
    T: GetName + RGSO + Send + Sync + Clone + 'static,
    // V: Send + Sync + 'static,
{
    let options = move || {
        let mut formatted_options = options
            .get()
            .into_iter()
            .map(|item| Some(item))
            .collect::<Vec<_>>();
        if empty_allowed {
            formatted_options.push(None)
        }
        formatted_options
    };
    let cur_value = move || match value.get() {
        Some(item) => item.get_name(),
        None => "None".to_string(),
    };
    let options_clone = options.clone();
    let on_change_value = move |e| {
        let id: u128 = u128::from_str(&event_target_value(&e)).unwrap();
        let return_val = if id == 0 {
            None
        } else {
            options_clone()
                .into_iter()
                .find(|item| {
                    if let Some(item) = item {
                        *item.get_id() == id
                    } else {
                        false
                    }
                })
                .unwrap()
        };
        value.set(return_val);
    };
    let options_clone = options.clone();
    view! {
        <select prop:value=cur_value on:change=on_change_value>
        <For each=move || options_clone()
            key=|item| match item  {Some(item) => item.get_id().clone().to_string() + &item.get_name(), None => "0".to_string()}
            let:discriminant>
            {
                let discriminant_string = discriminant.clone().map_or("None".to_string(), |item| item.get_name());
            view!{<option
                prop:value=discriminant.map_or(0, |item| item.get_id().clone())
                prop:selected = move || discriminant_string.clone() == cur_value()>
                    {discriminant_string.clone()}
                </option>}
            }
        </For>
        </select>

    }
}
