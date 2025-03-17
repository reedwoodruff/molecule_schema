use std::{fmt::Display, str::FromStr};

use leptos::{
    either::{Either, EitherOf5},
    prelude::*,
};
use schema_editor_generated_toolkit::prelude::{
    GetName, TemplateSlotCardinalityVariantTraitObject, RGSO, *,
};
use web_sys::Event;

#[component]
pub fn DocumentationInput(value: RwSignal<String>) -> impl IntoView {
    view! {
        <LeafSection>
            <LeafSectionHeader>Documentation</LeafSectionHeader>
            <SignalTextInput value=value textarea=true />
        </LeafSection>
    }
}
#[component]
pub fn ToggleManagedDocumentationInput<F, G>(getter: F, setter: G) -> impl IntoView
where
    F: Fn() -> String + Send + Sync + Clone + 'static,
    G: Fn(String) + Send + Sync + Clone + 'static,
{
    let is_editing = RwSignal::new(false);
    let input_ref = NodeRef::<leptos::html::Input>::new();

    let toggle_text = move || match is_editing.get() {
        true => "✔",
        false => "✎",
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
        let getter = getter.clone();
        let setter = setter.clone();
        let toggle_button = view! {
            <Button attr:class="edit-toggle-button" on:click=on_click_toggle>
                {toggle_text}
            </Button>
        };
        match is_editing.get() {
            true => Either::Left(view! {
                <LeafSection>
                    <em>Documentation</em>
                    <LeafSection attr:class="leafsection dependent width-100">
                        <ManagedTextInput textarea=true getter setter input_ref>
                            {toggle_button}
                        </ManagedTextInput>
                    </LeafSection>
                </LeafSection>
            }),
            false => Either::Right(view! {
                <LeafSection>
                    <em>"Documentation "</em>
                    <span style="width: 15px; display: inline-block"></span>
                    {toggle_button}
                    <LeafSection attr:class="leafsection dependent width-100">
                        <span>{getter.clone()}</span>
                    </LeafSection>
                </LeafSection>
            }),
        }
    };
    view
}

#[component]
pub fn SignalTextInput<T>(
    value: RwSignal<T>,
    #[prop(optional)] textarea: Option<bool>,
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
    move || {
        if textarea.is_some_and(|textarea| textarea == true) {
            view! {
                <textarea
                    prop:value=move || value.get().to_string()
                    on:input=on_input
                    class="width-100"
                />
            }
            .into_any()
        } else {
            view! { <input prop:value=move || value.get().to_string() on:input=on_input /> }
                .into_any()
        }
    }
}

#[component]
pub fn ManagedTextInput<F, G>(
    getter: F,
    setter: G,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] input_ref: Option<NodeRef<leptos::html::Input>>,
    #[prop(optional)] textarea: Option<bool>,
) -> impl IntoView
where
    F: Fn() -> String + Send + Sync + Clone + 'static,
    G: Fn(String) + Send + Sync + Clone + 'static,
{
    let children_view = children.map(|children_fn| children_fn().into_any());

    let getter_clone = getter.clone();
    let setter_clone = setter.clone();

    let is_textarea = textarea.is_some_and(|t| t == true);

    if let Some(input_ref) = input_ref {
        if is_textarea {
            view! {
                <textarea
                    class="inner-textarea"
                    prop:value=getter_clone.clone()
                    on:input=move |e| setter_clone.clone()(event_target_value(&e))
                />

                {children_view.unwrap_or_else(|| view! {}.into_any())}
            }
            .into_any()
        } else {
            view! {
                <input
                    node_ref=input_ref.clone()
                    class="inner-text-input"
                    prop:value=getter.clone()
                    on:input=move |e| setter.clone()(event_target_value(&e))
                />

                {children_view.unwrap_or_else(|| view! {}.into_any())}
            }
            .into_any()
        }
    } else {
        if is_textarea {
            view! {
                <textarea
                    class="inner-textarea"
                    prop:value=getter_clone.clone()
                    on:input=move |e| setter_clone.clone()(event_target_value(&e))
                />

                {children_view.unwrap_or_else(|| view! {}.into_any())}
            }
            .into_any()
        } else {
            view! {
                <input
                    class="inner-text-input"
                    prop:value=getter.clone()
                    on:input=move |e| setter.clone()(event_target_value(&e))
                />

                {children_view.unwrap_or_else(|| view! {}.into_any())}
            }
            .into_any()
        }
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
        true => "✔",
        false => "✎",
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
            true => Either::Left(view! {
                <ManagedTextInput input_ref getter=getter.clone() setter=setter.clone()>
                    {toggle_button}
                </ManagedTextInput>
            }),
            false => Either::Right(view! {
                <span>
                    {getter.clone()} <span style="width: 15px; display: inline-block"></span>
                    {toggle_button}
                </span>
            }),
        }
    };
    view
}

#[component]
pub fn LeafSection(children: Children) -> impl IntoView {
    view! { <div class="leafsection">{children()}</div> }
}
#[component]
pub fn LeafSectionHeader(children: Children) -> impl IntoView {
    view! { <h4 class="leafsection-header">{children()}</h4> }
}

#[component]
pub fn SubSection(children: Children) -> impl IntoView {
    view! { <div class="subsection">{children()}</div> }
}
#[component]
pub fn SubSectionHeader(children: Children) -> impl IntoView {
    view! { <h3 class="subsection-header">{children()}</h3> }
}

#[slot]
pub struct SectionHeader {
    // #[prop(optional)]
    children: Children,
}
#[component]
pub fn Collapsible(
    #[prop(optional)] start_open: Option<bool>,
    children: Children,
) -> impl IntoView {
    let is_collapsed = RwSignal::new(!start_open.unwrap_or(false));
    let collapsed_class = move || match is_collapsed.get() {
        true => "collapsed-children",
        false => "",
    };
    let children_div_class = move || format!("{}", collapsed_class());
    view! {
        <div class="flex">
            <div class="flex-grow">{move || if is_collapsed.get() { "..." } else { "" }}</div>
            <div>
                <Button on:click=move |_| {
                    is_collapsed.update(|prev| *prev = !*prev)
                }>
                    {move || match is_collapsed.get() {
                        true => "+".to_string(),
                        false => "-".to_string(),
                    }}
                </Button>
            </div>
        </div>
        <div class=children_div_class>{children()}</div>
    }
}
#[component]
pub fn Section(
    section_header: SectionHeader,
    children: Children,
    #[prop(optional)] master_collapser: Option<RwSignal<bool>>,
) -> impl IntoView {
    let is_collapsed = RwSignal::new(master_collapser.map_or(false, |inner| inner.get()));
    Effect::new(move || {
        if let Some(master_collapser) = master_collapser {
            is_collapsed.set(master_collapser.get());
        }
    });
    let collapsed_class = move || match is_collapsed.get() {
        true => "collapsed-children",
        false => "",
    };
    let children_div_class = move || format!("{}", collapsed_class());
    view! {
        <section class="section">
            <div class="flex">
                <div class="flex-grow">
                    <h2 class="section-header">{(section_header.children)()}</h2>
                </div>
                <div>
                    <Button on:click=move |_| {
                        is_collapsed.update(|prev| *prev = !*prev)
                    }>
                        {move || match is_collapsed.get() {
                            true => "+".to_string(),
                            false => "-".to_string(),
                        }}
                    </Button>
                </div>
            </div>
            <div class=children_div_class>{children()}</div>
        </section>
    }
}

#[component]
pub fn InfoNote(children: Children) -> impl IntoView {
    view! { <div class="infonote">{children()}</div> }
}

#[component]
pub fn Button(children: Children) -> impl IntoView {
    view! { <button>{children()}</button> }
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
                <option prop:selected=move || {
                    discriminant.to_string() == cur_value()
                }>{discriminant.to_string()}</option>
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
                    view! {
                        <option prop:selected=move || {
                            discriminant == getter_clone()
                        }>{discriminant.to_string()}</option>
                    }
                }
            </For>
        </select>
    }
}

#[component]
pub fn SignalSelectRGSOWithOptions<T>(
    value: RwSignal<Option<T>>,
    #[prop(into)] options: Signal<Vec<T>>,
    #[prop(optional)] empty_allowed: bool,
) -> impl IntoView
where
    // I: IntoIterator<Item = T> + Clone + 'static + Send + Sync,
    T: GetName + RGSO + Send + Sync + Clone + 'static,
    // V: Send + Sync + 'static,
{
    Effect::new(move || {
        options.track();
        if empty_allowed {
            value.set(None);
        } else {
            if let Some(first_option) = options.get().into_iter().next() {
                value.set(Some(first_option))
            } else {
                value.set(None);
            }
        }
    });
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
            <For
                each=move || options_clone()
                key=|item| match item {
                    Some(item) => item.get_id().clone().to_string() + &item.get_name(),
                    None => "0".to_string(),
                }
                let:discriminant
            >
                {
                    let discriminant_string = discriminant
                        .clone()
                        .map_or("None".to_string(), |item| item.get_name());
                    view! {
                        <option
                            prop:value=discriminant.map_or(0, |item| item.get_id().clone())
                            prop:selected=move || discriminant_string.clone() == cur_value()
                        >
                            {discriminant_string.clone()}
                        </option>
                    }
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
    T: ToString + Send + Sync + Clone + 'static,
{
    Effect::new(move || {
        options.track();
        if empty_allowed {
            value.set(None);
        } else {
            if let Some(first_option) = options.get().into_iter().next() {
                value.set(Some(first_option))
            } else {
                value.set(None);
            }
        }
    });
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
        Some(item) => item.to_string(),
        None => "None".to_string(),
    };
    let options_clone = options.clone();
    let on_change_value = move |e| {
        let id = event_target_value(&e);
        let return_val = if id == "None" {
            None
        } else {
            options_clone()
                .into_iter()
                .find(|item| {
                    if let Some(item) = item {
                        item.to_string() == id
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
            <For
                each=move || options_clone()
                key=|item| match item {
                    Some(item) => item.to_string(),
                    None => "None".to_string(),
                }
                let:discriminant
            >
                {
                    let discriminant_string = discriminant
                        .clone()
                        .map_or("None".to_string(), |item| item.to_string());
                    view! {
                        <option
                            prop:value=discriminant
                                .map_or("None".to_string(), |item| item.to_string())
                            prop:selected=move || discriminant_string.clone() == cur_value()
                        >
                            {discriminant_string.clone()}
                        </option>
                    }
                }
            </For>
        </select>
    }
}

#[component]
pub fn MultiSelectRGSO<T>(
    list: RwSignal<Vec<T>>,
    #[prop(into)] options: Signal<Vec<T>>,
) -> impl IntoView
where
    T: GetName + RGSO + Send + Sync + Clone + 'static,
{
    let selected_single_item = RwSignal::<Option<T>>::new(None);
    view! {
        <LeafSection>
            <LeafSection>
                <LeafSectionHeader>Item To Add To List:</LeafSectionHeader>
                <SignalSelectRGSOWithOptions
                    empty_allowed=true
                    value=selected_single_item
                    options=Signal::derive(move || {
                        let local_list = list.get();
                        options
                            .get()
                            .into_iter()
                            .filter(|op_item| {
                                local_list
                                    .iter()
                                    .find(|list_item| list_item.get_id() == op_item.get_id())
                                    .is_none()
                            })
                            .collect()
                    })
                />
                <Button
                    prop:disabled=move || { selected_single_item.get().is_none() }
                    on:click=move |_| {
                        if let Some(selected_item) = selected_single_item.get() {
                            list.update(|prev| prev.push(selected_item));
                        }
                        selected_single_item.set(None);
                    }
                >
                    Add
                </Button>
            </LeafSection>
            <LeafSection>
                <LeafSectionHeader>Currently Selected:</LeafSectionHeader>
                <ul>
                    <For each=move || list.get() key=|item| item.get_id().clone() let:item>
                        {
                            let item_clone = item.clone();
                            view! {
                                <li>
                                    {move || item_clone.get_name()}" "
                                    <Button on:click=move |_| {
                                        list.update(|prev| {
                                            prev.retain(|inner_item| {
                                                inner_item.get_id() != item.get_id()
                                            })
                                        })
                                    }>X</Button>
                                </li>
                            }
                        }
                    </For>
                </ul>
            </LeafSection>
        </LeafSection>
    }
}

#[component]
pub fn CardinalityView(cardinality: TemplateSlotCardinalityVariantTraitObject) -> impl IntoView {
    match cardinality {
        TemplateSlotCardinalityVariantTraitObject::TemplateSlotCardinalityRangeOrZero(inner) => {
            EitherOf5::B(move || {
                format!(
                    "Lower Bound: {}, Upper Bound: {}",
                    inner.get_lower_bound_field(),
                    inner.get_upper_bound_field()
                )
            })
        }
        TemplateSlotCardinalityVariantTraitObject::TemplateSlotCardinalityLowerBoundOrZero(
            inner,
        ) => EitherOf5::C(move || format!("Lower Bound: {}", inner.get_lower_bound_field(),)),
        TemplateSlotCardinalityVariantTraitObject::TemplateSlotCardinalityRange(inner) => {
            EitherOf5::D(move || {
                format!(
                    "Lower Bound: {}, Upper Bound: {}",
                    inner.get_lower_bound_field(),
                    inner.get_upper_bound_field()
                )
            })
        }
        TemplateSlotCardinalityVariantTraitObject::TemplateSlotCardinalityLowerBound(inner) => {
            EitherOf5::E(move || format!("Lower Bound: {}", inner.get_lower_bound_field(),))
        }
        TemplateSlotCardinalityVariantTraitObject::TemplateSlotCardinalitySingle(_inner) => {
            EitherOf5::A(move || "Exactly 1")
        }
    }
}
