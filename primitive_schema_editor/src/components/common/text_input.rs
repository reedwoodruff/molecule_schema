use leptos::{attr::AttributeValue, prelude::*};

#[component]
pub fn TextInput(
    value: RwSignal<String>,
    // #[prop(optional)] show_save_button: Option<bool>,
    // on_save: F,
) -> impl IntoView {
    view! { <input prop:value=value on:input=move |e| value.set(event_target_value(&e)) /> }
}

#[component]
pub fn NumberInput2<T>(value: RwSignal<T>) -> impl IntoView
where
    T: Into<usize> + std::str::FromStr + Clone + AttributeValue + 'static + Send + Sync + ToString,
    leptos::prelude::RwSignal<T>: AttributeValue,
{
    view! {
        <input
            prop:value=move || value.get().to_string()
            type="number"
            on:input=move |e| {
                if let Ok(num_val) = event_target_value(&e).parse() {
                    value.set(num_val)
                }
            }
        />
    }
}
