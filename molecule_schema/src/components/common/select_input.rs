use std::{collections::HashMap, rc::Rc};

use leptos::{logging::log, *};


#[component]
pub fn SelectInput<K, V, F, I>(
    options: I,
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
    let map = Rc::new(
        options
            .clone()
            .into_iter()
            .map(|(k, v)| (k, v.into()))
            .collect::<HashMap<K, String>>(),
    );
    let map2 = map.clone();

    let options = RwSignal::new(options);

    let cur_value = move || {
        log!("{:?}", value.get());
        if let Some(val) = map.clone().get(&value.get()) {
            val.clone()
        } else {
            "".to_string()
        }
    };

    let callback = Rc::new(on_select);
    let on_change = move |e| {
        let map = map2.clone();
        let return_val = event_target_value(&e);
        log!("{}", return_val);
        let key_val_pair = map.iter().find(|(_key, value)| **value == return_val);
        if let Some(pair) = key_val_pair {
            callback(pair.0);
        }
    };

    // create_effect(|_| {
    //     if let Some(select_ref) = select_ref.get() {
    //         select_ref.set_value(cur_value);
    //     }
    // });
    view! {
        <select ref=select_ref value=move || cur_value() on:change=on_change>
            <For each=options key=move |item| item.0.clone() let:item>
                <option value=item.1.clone().into()>{item.1.into()}</option>
            </For>
        // {move || options.get().into_iter().map(|item| {
        // view!{<option value=item.1.clone().into()>{item.1.into()}</option>}
        // }).collect::<Vec<_>>()}
        </select>
    }
}

#[component]
pub fn SelectInputOptional<K, V, F, I>(
    options: I,
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
    let map = Rc::new(
        options
            .clone()
            .into_iter()
            .map(|(k, v)| (k, v.into()))
            .collect::<HashMap<K, String>>(),
    );
    let map2 = map.clone();

    let options = RwSignal::new(options);

    let cur_value = move || {
        if let Some(val) = value.get() {
            map.clone().get(&val).cloned()
        } else {
            None
        }
    };

    let callback = Rc::new(on_select);
    let on_change = move |e| {
        let map = map2.clone();

        let return_val = event_target_value(&e);
        log!("{}", return_val);
        if return_val == "NoneOption" {
            callback(None);
        } else {
            let key_val_pair = map.iter().find(|(_key, value)| **value == return_val);
            if let Some(pair) = key_val_pair {
                callback(Some(pair.0.clone()));
            }
        }
    };

    view! {
        <select value=move || cur_value() on:change=on_change>
            <option value="NoneOption" id="NoneOption"></option>
            <For each=options key=move |item| item.0.clone() let:item>
                <option value=item.1.clone().into()>{item.1.into()}</option>
            </For>
        // {move || options.get().into_iter().map(|item| {
        // view!{<option value=item.1.clone().into()>{item.1.into()}</option>}
        // }).collect::<Vec<_>>()}
        </select>
    }
}
