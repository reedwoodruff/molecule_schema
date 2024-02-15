use std::rc::Rc;

use leptos::*;
use web_sys::{MouseEvent, SubmitEvent};

#[component]
pub fn TextInput<F>(
    #[prop(into)] initial_value: String,
    #[prop(optional)] show_save_button: Option<bool>,
    on_save: F,
) -> impl IntoView
where
    F: Fn(String) + 'static,
{
    let name_signal = RwSignal::<String>::new(initial_value.into());

    let on_save = Rc::new(on_save);
    // let save = Rc::new(move || {
    //     on_save();
    // });
    // let save2 = move |_| {
    //     save.clone()();
    // };
    let save2 = on_save.clone();
    let submit_form = move |e: SubmitEvent| {
        e.prevent_default();
        // save.clone()();
        on_save(name_signal.get());
    };

    view! {
        <form on:submit=submit_form>
            <input value=name_signal on:input=move |e| name_signal.set(event_target_value(&e))/>
            <Show when=move || {
                show_save_button.is_some_and(|ssb| ssb == true)
            }>

                {
                    let save2 = save2.clone();
                    view! { <button on:click=move |_| save2(name_signal.get())>Save</button> }
                }

            </Show>
        </form>
    }
}
