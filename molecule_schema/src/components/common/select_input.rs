use std::{collections::HashMap, rc::Rc, str::FromStr};

use leptos::{*};

#[component]
pub fn SelectInput<K, V, F, I>(
    options: MaybeSignal<I>,
    on_select: F,
    #[prop(into)] value: Signal<K>,
) -> impl IntoView
where
    K: Eq + std::hash::Hash + Clone + std::fmt::Debug + 'static,
    // V: Display + Clone + 'static,
    V: Into<String> + Clone + 'static,
    F: Fn(&K) + 'static,
    I: IntoIterator<Item = (K, V)> + Clone + 'static,
{
    // let cloned_options = options.into_iter().cloned().collect::<Vec<(K, V)>>();
    // let map: HashMap<K, V> = HashMap::from(options.into_iter().collect::<Vec<_>>());
    let select_ref = create_node_ref();
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
    let map2 = map;

    let cur_value = move || {
        if let Some(val) = map.get().clone().get(&value.get()) {
            val.clone()
        } else {
            "".to_string()
        }
    };

    let callback = Rc::new(on_select);
    let on_change = move |e| {
        let map = map2;
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
        <select ref=select_ref value=cur_value on:change=on_change>
            <For each=move || options2.get() key=move |item| item.0.clone() let:item>
                <option value=item.1.clone().into() selected=value.get() == item.0>
                    {item.1.into()}
                </option>
            </For>
        // {move || options.get().into_iter().map(|item| {
        // view!{<option value=item.1.clone().into()>{item.1.into()}</option>}
        // }).collect::<Vec<_>>()}
        </select>
    }
}

#[component]
pub fn SelectInputOptional<K, V, F, I>(
    options: MaybeSignal<I>,
    on_select: F,
    #[prop(into)] value: Signal<Option<K>>,
) -> impl IntoView
where
    K: Eq + std::hash::Hash + Clone + std::fmt::Debug + 'static,
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
                .collect::<HashMap<K, String>>()
        })
    });
    let map2 = map;

    let cur_value = move || {
        if let Some(val) = value.get() {
            map.get().get(&val).cloned()
        } else {
            None
        }
    };

    let callback = Rc::new(on_select);
    let on_change = move |e| {
        let map = map2;

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
        <select value=cur_value on:change=on_change>
            <option value="NoneOption" id="NoneOption"></option>
            <For each=move || options2.get() key=move |item| item.0.clone() let:item>
                <option value=item.1.clone().into()>{item.1.into()}</option>
            </For>
        // {move || options.get().into_iter().map(|item| {
        // view!{<option value=item.1.clone().into()>{item.1.into()}</option>}
        // }).collect::<Vec<_>>()}
        </select>
    }
}

use strum::IntoEnumIterator;
#[component]
pub fn SelectInputEnum<T>(value: RwSignal<T>) -> impl IntoView
where
    T: IntoEnumIterator + FromStr + ToString + Default + std::fmt::Debug + Clone + 'static,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
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
                <option selected=value.get().to_string()
                    == item.to_string()>{move || item.to_string()}</option>
            </For>
        </select>
    }
}
