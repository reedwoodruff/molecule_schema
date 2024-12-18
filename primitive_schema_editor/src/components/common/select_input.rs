use std::{
    collections::{BTreeMap, HashMap},
    rc::Rc,
    str::FromStr,
};

use leptos::prelude::*;

#[component]
pub fn SelectInput<K: Send + Sync, V: Send + Sync, F, I: Send + Sync>(
    options: MaybeSignal<I>,
    on_select: F,
    #[prop(into)] value: Signal<K>,
) -> impl IntoView
where
    K: Eq + std::hash::Hash + Ord + Clone + std::fmt::Debug + 'static,
    // V: Display + Clone + 'static,
    V: Into<String> + Clone + 'static,
    F: Fn(&K) + 'static,
    I: IntoIterator<Item = (K, V)> + Clone + 'static,
{
    // let cloned_options = options.into_iter().cloned().collect::<Vec<(K, V)>>();
    // let map: HashMap<K, V> = HashMap::from(options.into_iter().collect::<Vec<_>>());
    let select_ref = NodeRef::new();
    let options2 = options.clone();
    let map = create_memo(move |_| {
        options.clone().with(|options| {
            options
                .clone()
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect::<HashMap<K, String>>()
        })
    });
    let map_clone = map.clone();
    let ordered_options = Memo::new(move |_| {
        let mut ordered = map_clone.get().into_iter().collect::<Vec<(K, String)>>();
        ordered.sort_by(|a, b| b.1.cmp(&a.1));
        ordered
    });

    let cur_value = move || {
        if let Some(val) = map.get().clone().get(&value.get()) {
            val.clone()
        } else {
            "".to_string()
        }
    };

    let map_clone = map.clone();
    let callback = Rc::new(on_select);
    let on_change = move |e| {
        let map = map_clone;
        let return_val = event_target_value(&e);
        let key_val_pair = map
            .get()
            .into_iter()
            .find(|(_key, value)| **value == return_val);
        if let Some(pair) = key_val_pair {
            callback(&pair.0);
        }
    };

    view! {
        <select node_ref=select_ref prop:value=cur_value on:change=on_change>
            <For each=move || ordered_options.get() key=move |item| item.0.clone() let:item>
                <option value=item.1.clone() selected=value.get() == item.0>
                    {item.1.clone()}
                </option>
            </For>
        // {move || options.get().into_iter().map(|item| {
        // view!{<option value=item.1.clone().into()>{item.1.into()}</option>}
        // }).collect::<Vec<_>>()}
        </select>
    }
}

#[component]
pub fn SelectInputOptional<K: Send + Sync, V: Send + Sync, F, I: Send + Sync>(
    options: MaybeSignal<I>,
    on_select: F,
    #[prop(into)] value: Signal<Option<K>>,
) -> impl IntoView
where
    K: Eq + std::hash::Hash + Clone + std::fmt::Debug + Ord + 'static,
    // V: Display + Clone + 'static,
    V: Into<String> + Clone + 'static,
    F: Fn(Option<K>) + 'static,
    I: IntoIterator<Item = (K, V)> + Clone + 'static,
{
    let options2 = options.clone();
    let map = create_memo(move |_| {
        options.clone().with(|options| {
            options
                .clone()
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect::<BTreeMap<K, String>>()
        })
    });
    let map_clone = map.clone();
    let ordered_options = Memo::new(move |_| {
        let mut ordered = map_clone.get().into_iter().collect::<Vec<(K, String)>>();
        ordered.sort_by(|a, b| a.1.cmp(&b.1));
        ordered
    });

    let cur_value = move || {
        if let Some(val) = value.get() {
            map.get().get(&val).cloned()
        } else {
            None
        }
    };

    let map_clone = map.clone();
    let callback = Rc::new(on_select);
    let on_change = move |e| {
        let map = map_clone;

        let return_val = event_target_value(&e);
        if return_val == "NoneOption" {
            callback(None);
        } else {
            let key_val_pair = map
                .get()
                .iter()
                .find(|(_key, value)| **value == return_val)
                .map(|(key, value)| (key.clone(), value.clone()));
            if let Some(pair) = key_val_pair {
                callback(Some(pair.0.clone()));
            }
        }
    };

    view! {
        <select on:change=on_change prop:value=cur_value>
            <option value="NoneOption" id="NoneOption"></option>
            <For each=move || ordered_options.get() key=move |item| item.0.clone() let:item>
                <option value=item.1.clone()>{item.1.clone()}</option>
            </For>
        // {move || options.get().into_iter().map(|item| {
        // view!{<option value=item.1.clone().into()>{item.1.into()}</option>}
        // }).collect::<Vec<_>>()}
        </select>
    }
}

use strum::IntoEnumIterator;
#[component]
pub fn SelectInputEnum<T: Send + Sync>(value: RwSignal<T>) -> impl IntoView
where
    T: IntoEnumIterator + FromStr + ToString + Default + std::fmt::Debug + Clone + 'static,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
    <T as IntoEnumIterator>::Iterator: Send,
{
    let options = T::iter()
        .map(|item| (item.clone(), item.to_string()))
        .collect::<Vec<_>>();
    view! {
        <select on:change=move |e| {
            let return_val = event_target_value(&e);
            if let Ok(return_val) = T::from_str(&return_val) {
                value.set(return_val);
            }
        }>

            <For each=move || T::iter() key=move |item| item.to_string() let:item>
                {move || {
                    let item = item.clone();
                    let item2 = item.clone();
                    view! {
                        <option selected=value.get().to_string()
                            == item2.to_string()>{move || item.to_string()}</option>
                    }
                }}
            </For>
        </select>
    }
}
