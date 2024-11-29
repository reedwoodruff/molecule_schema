use std::collections::HashMap;

use crate::components::common::*;
use from_reactive::FromStandalone;
use schema_editor_generated_toolkit::prelude::*;

#[component]
pub fn ControlPanel(schema_id: RwSignal<Option<Uid>>) -> impl IntoView {
    let ctx = use_context::<SharedGraph<Schema>>().unwrap();
    let ctx_clone = ctx.clone();
    let import_value = RwSignal::new(String::new());
    let delete_all = move |_| {
        ctx_clone.created_instances.set(HashMap::new());
        schema_id.set(None);
    };

    let ctx_clone = ctx.clone();
    let process_import = move |_| {
        let standalones = serde_json::from_str::<Vec<StandaloneRGSOConcrete>>(&import_value.get());
        if standalones.is_err() {
            leptos::logging::log!("error parsing json");
            return;
        }
        let hydrated = standalones
            .unwrap()
            .into_iter()
            .map(|standalone| {
                (
                    standalone.id,
                    Schema::from_standalone(standalone, ctx_clone.clone()),
                )
            })
            .collect::<HashMap<_, _>>();

        ctx_clone.created_instances.set(hydrated);
        let new_schema_id = ctx_clone
            .created_instances
            .get()
            .values()
            .find(|instance| instance.operative().tag.id == SchemaConcrete::get_operative_id())
            .unwrap()
            .get_id()
            .clone();
        schema_id.set(Some(new_schema_id));
    };
    view! {
        <Section>
        <SectionHeader slot>Graph Control</SectionHeader>
        <SubSection>
         <SubSectionHeader>Reset</SubSectionHeader>
         <Button on:click=delete_all>Reset</Button>
        </SubSection>
        <SubSection>
         <SubSectionHeader>Import</SubSectionHeader>
         <SignalTextInput value=import_value></SignalTextInput>
         <Button on:click=process_import>Import</Button>
        </SubSection>

        </Section>
    }
}
