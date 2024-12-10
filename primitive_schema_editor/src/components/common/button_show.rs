use leptos::prelude::*;

#[component]
pub fn ButtonShow(
    children: ChildrenFn,
    #[prop(default = "Show".to_string(), into)] show_text: String,
    #[prop(default = "Hide".to_string(), into)] hide_text: String,
) -> impl IntoView {
    let showing = RwSignal::new(false);
    view! {
        <Show when=move || !showing.get()>
            <button on:click=move |_| showing.set(true)>{show_text.clone()}</button>
        </Show>
        <Show when=move || {
            showing.get()
        }>

            {
                let children = children.clone();
                let hide_text = hide_text.clone();
                view! {
                    {children()}
                    <br />
                    <button on:click=move |_| showing.set(false)>{hide_text}</button>
                }
            }

        </Show>
    }
}
