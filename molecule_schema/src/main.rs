use leptos::*;
use molecule_schema::components::app::App;

pub fn main() {
    let constraint_schema_generated = constraint_schema::constraint_schema!();
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App schema=constraint_schema_generated/> });
}
