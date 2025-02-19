use crate::components::{editing_space::EditingSpace, main_list::MainList};
use leptos::{context::Provider, either::Either};
use schema_editor_generated_toolkit::prelude::*;

#[derive(Clone, Debug)]
pub enum WorkspaceTab {
    Template(RwSignal<Option<RGSOConcrete<TemplateConcrete, Schema>>>),
    Operative(RwSignal<Option<RGSOConcrete<OperativeConcrete, Schema>>>),
    Instance(RwSignal<Option<RGSOConcrete<InstanceConcrete, Schema>>>),
    Trait(RwSignal<Option<RGSOConcrete<TraitConcrete, Schema>>>),
    Function(RwSignal<Option<RGSOConcrete<FunctionDefinition, Schema>>>),
}
#[derive(Clone)]
pub struct WorkspaceState {
    pub selected_tab: RwSignal<WorkspaceTab>,
    pub schema: Signal<RGSOConcrete<SchemaConcrete, Schema>>,
}

#[component]
pub fn Workspace(schema_final_id: RwSignal<Option<Uid>>) -> impl IntoView {
    let ctx = use_context::<SharedGraph<Schema>>().unwrap();

    let maybe_schema = Signal::derive(move || match schema_final_id.get() {
        Some(id) => {
            let schema = ctx.get(&id).unwrap();
            match schema {
                Schema::SchemaConcrete(inner) => Some(inner.clone()),
                _ => None,
            }
        }
        None => None,
    });

    let selected_tab = RwSignal::new(WorkspaceTab::Template(RwSignal::new(None)));

    // Extra closure here to fix shadowed context

    move || match maybe_schema.get() {
        Some(schema_existent) => Either::Left({
            let schema_clone = schema_existent.clone();
            view! {
                <Provider value=WorkspaceState {
                    schema: Signal::derive(move || schema_clone.clone()),
                    selected_tab: selected_tab.clone(),
                }>
                    <div class="workspace-container">
                        <div class="tabs-container">
                            <For
                                each=move || {
                                    schema_existent
                                        .clone()
                                        .outgoing_slots_with_enum()
                                        .clone()
                                        .into_values()
                                }
                                key=move |item| item.base.slot.tag.id.clone()
                                let:slot
                                children=move |slot| {
                                    let slot_enum_clone = slot.slot_enum.clone();
                                    let is_active = move || {
                                        if <WorkspaceTab as Into<
                                            SchemaConcreteAllSlots,
                                        >>::into(selected_tab.get()) == slot_enum_clone
                                        {
                                            "active"
                                        } else {
                                            ""
                                        }
                                    };
                                    let class = move || format!("tab-link {}", is_active());
                                    let slot_enum_clone = slot.slot_enum.clone();
                                    view! {
                                        <a
                                            class=class
                                            on:click=move |_| {
                                                selected_tab.set(slot_enum_clone.clone().into())
                                            }
                                        >
                                            {slot.slot.tag.name.clone()}
                                        </a>
                                    }
                                }
                            ></For>

                        </div>
                        <MainList />
                        <EditingSpace />
                    </div>
                </Provider>
            }
        }),
        None => Either::Right(view! {
            <div>
                <p>No Schema</p>
            </div>
        }),
    }
}

impl From<WorkspaceTab> for SchemaConcreteAllSlots {
    fn from(value: WorkspaceTab) -> Self {
        match value {
            WorkspaceTab::Instance(_) => SchemaConcreteAllSlots::Instances,
            WorkspaceTab::Template(_) => SchemaConcreteAllSlots::Templates,
            WorkspaceTab::Operative(_) => SchemaConcreteAllSlots::Operatives,
            WorkspaceTab::Trait(_) => SchemaConcreteAllSlots::Traits,
            WorkspaceTab::Function(_) => SchemaConcreteAllSlots::Functions,
        }
    }
}
impl From<SchemaConcreteAllSlots> for WorkspaceTab {
    fn from(value: SchemaConcreteAllSlots) -> Self {
        match value {
            SchemaConcreteAllSlots::Functions => WorkspaceTab::Function(RwSignal::new(None)),
            SchemaConcreteAllSlots::Instances => WorkspaceTab::Instance(RwSignal::new(None)),
            SchemaConcreteAllSlots::Templates => WorkspaceTab::Template(RwSignal::new(None)),
            SchemaConcreteAllSlots::Operatives => WorkspaceTab::Operative(RwSignal::new(None)),
            SchemaConcreteAllSlots::Traits => WorkspaceTab::Trait(RwSignal::new(None)),
        }
    }
}
