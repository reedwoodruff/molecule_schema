use leptos::*;
use molecule_schema::components::App;

pub fn main() {
    constraint_schema::constraint_schema!();
    console_error_panic_hook::set_once();
    mount_to_body(|| view! {<App schema={constraint_schema_generated}/>});
}
