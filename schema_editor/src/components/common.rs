use leptos::{either::Either, prelude::*};

#[component]
pub fn SignalTextInput(
    value: RwSignal<String>,
    // #[prop(optional)] show_save_button: Option<bool>,
    // on_save: F,
) -> impl IntoView {
    view! { <input value=value on:input=move |e| value.set(event_target_value(&e))/> }
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
    let toggle_button = view! {
        <button class="edit-toggle-button" on:click=on_click_toggle>
            {toggle_text}
        </button>
    };

    Effect::new(move || {
        if let Some(input_ref) = input_ref.get() {
            input_ref.focus().unwrap();
        }
    });

    let view = move || {
        let toggle_button = toggle_button.clone();
        match is_editing.get() {
            true => Either::Left(
                view! {<ManagedTextInput input_ref getter=getter.clone() setter=setter.clone() >{toggle_button}</ManagedTextInput>},
            ),
            false => Either::Right(view! {<p>{getter.clone()}
                         {toggle_button.clone()}
            </p>}),
        }
    };
    view
}
