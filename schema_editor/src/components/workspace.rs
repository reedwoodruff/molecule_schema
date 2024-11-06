use std::sync::Arc;

use crate::components::{editing_space::EditingSpace, main_list::MainList};
use generated_crate::prelude::*;

#[derive(Clone)]
pub enum WorkspaceTab {
    Template(RwSignal<Option<RGSOConcrete<TemplateConcrete, Schema>>>),
    Operative(RwSignal<Option<RGSOConcrete<OperativeConcrete, Schema>>>),
    Instance(RwSignal<Option<RGSOConcrete<InstanceConcrete, Schema>>>),
    Trait(RwSignal<Option<RGSOConcrete<TraitConcrete, Schema>>>),
}
#[derive(Clone)]
pub struct WorkspaceState {
    pub selected_tab: RwSignal<WorkspaceTab>,
    pub schema: RGSOConcrete<SchemaConcrete, Schema>,
}

#[component]
pub fn Workspace(schema_final_id: u128) -> impl IntoView {
    let ctx = use_context::<SharedGraph<Schema>>().unwrap();

    let schema = match ctx.get(&schema_final_id).unwrap() {
        Schema::SchemaConcrete(inner) => inner,
        _ => panic!(),
    };

    let selected_tab = RwSignal::new(WorkspaceTab::Template(RwSignal::new(None)));
    provide_context(WorkspaceState {
        schema: schema.clone(),
        selected_tab: selected_tab.clone(),
    });

    let schema_clone = schema.clone();

    view! {
        <div>
            <div class="tabs-container">
                <For each=move || schema.outgoing_slots_with_enum().clone().into_values()
                    key=move |item| item.base.slot.tag.id.clone()
                    let:slot
                    children= move |slot| {
                        let slot_enum_clone = slot.slot_enum.clone();
                        let is_active = move || if <WorkspaceTab as Into<SchemaConcreteAllSlots>>::into(selected_tab.get()) == slot_enum_clone {
                            "active"
                            } else {
                                ""
                        };
                        let class=move || format!("tab-link {}", is_active());
                        let slot_enum_clone = slot.slot_enum.clone();
                        view!{
                            <a class=class on:click=move |_| selected_tab.set(slot_enum_clone.clone().into()) >
                                {slot.slot.tag.name.clone()}
                            </a>
                        }
                    }
                >
                </For>

            </div>
            <MainList />
            <EditingSpace />
        </div>
    }
}

impl From<WorkspaceTab> for SchemaConcreteAllSlots {
    fn from(value: WorkspaceTab) -> Self {
        match value {
            WorkspaceTab::Instance(_) => SchemaConcreteAllSlots::Instances,
            WorkspaceTab::Template(_) => SchemaConcreteAllSlots::Templates,
            WorkspaceTab::Operative(_) => SchemaConcreteAllSlots::Operatives,
            WorkspaceTab::Trait(_) => SchemaConcreteAllSlots::Traits,
        }
    }
}
impl From<SchemaConcreteAllSlots> for WorkspaceTab {
    fn from(value: SchemaConcreteAllSlots) -> Self {
        match value {
            SchemaConcreteAllSlots::Instances => WorkspaceTab::Instance(RwSignal::new(None)),
            SchemaConcreteAllSlots::Templates => WorkspaceTab::Template(RwSignal::new(None)),
            SchemaConcreteAllSlots::Operatives => WorkspaceTab::Operative(RwSignal::new(None)),
            SchemaConcreteAllSlots::Traits => WorkspaceTab::Trait(RwSignal::new(None)),
        }
    }
}
