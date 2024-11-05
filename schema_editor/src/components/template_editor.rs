use std::sync::Arc;

use generated_crate::prelude::*;

use crate::components::common::{ManagedTextInput, ToggleManagedTextInput};

use super::workspace::WorkspaceState;
#[component]
pub fn TemplateEditor(template: RGSOConcrete<TemplateConcrete, Schema>) -> impl IntoView {
    let ctx = use_context::<Arc<RBaseGraphEnvironment<Schema>>>().unwrap();
    let ctx_clone = ctx.clone();

    let template_clone = template.clone();
    let update_name = move |new_val: String| {
        let editor = template_clone.edit(ctx_clone.clone());
        editor.set_name(new_val).execute().unwrap();
    };

    view! {
        <div>
       // <ManagedTextInput getter=move || template.get_name_field() setter=update_name />
        <ToggleManagedTextInput getter=move || template.get_name_field() setter=update_name />
        // <Input value=rw/>
        // <Button>"test"</Button>
        // <input prop:value=move || template.get_name_field() on:input=update_name />
       </div>
    }
}
