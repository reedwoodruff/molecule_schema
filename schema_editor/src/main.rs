use leptos::prelude::*;
mod components;
use components::app::App;

include!(concat!(env!("OUT_DIR"), "/recursive_schema.rs"));

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| view! { <App /> });
}
