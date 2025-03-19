use crate::components::method_impl_utils::{
    build_schemaful_representation_of_graph, constraint_template_to_canvas_template,
    generate_function_input_and_mapstep_complex, generate_function_output_and_mapstep_complex,
    rgso_operative_to_canvas_template, rgso_trait_to_canvas_template,
    setup_existing_fn_impl_in_canvas,
};
use crate::components::workspace::WorkspaceState;
use crate::components::{common::*, graph_editor::GraphEditor};
use graph_canvas::{GraphCanvas, TemplateGroup};
use graph_canvas::{GraphCanvasConfig, LayoutType};
use leptos::context::Provider;
use leptos::logging::log;
use schema_editor_generated_toolkit::prelude::*;
use std::str::FromStr;
use strum::EnumProperty;
use strum::IntoEnumIterator;

#[derive(Clone, Debug)]
pub struct MethodImplBuilderContext {
    pub _impling_operative: RGSOConcrete<OperativeConcrete, Schema>,
}

#[component]
pub fn MethodImplementationBuilder(
    fn_def: RGSOConcrete<FunctionDefinition, Schema>,
    operative: RGSOConcrete<OperativeConcrete, Schema>,

    // Will return an executable which contains the new MethodImplementation with a temp_id of "new_fn_impl"
    on_save: Callback<ExistingBuilder<OperativeConcrete, Schema>>,
    on_cancel: Callback<()>,
    #[prop(optional)] initial_state: Option<RGSOConcrete<MethodImplementation, Schema>>,
) -> impl IntoView {
    let WorkspaceState { .. } = use_context::<WorkspaceState>().unwrap();
    let ctx = use_context::<SharedGraph<Schema>>().unwrap();
    let ctx_clone = ctx.clone();

    let func_impl_name = RwSignal::new(fn_def.get_name() + "_impl");
    let func_impl_documentation = RwSignal::new("".to_string());

    let fn_def_clone = fn_def.clone();
    let operative_clone = operative.clone();

    let graph_handle: RwSignal<Option<GraphCanvas>> = RwSignal::new(None);
    Effect::new(move |_| match graph_handle.get() {
        Some(mut graph) => {
            match graph.apply_layout(LayoutType::ForceDirected) {
                Ok(_) => {
                    // Layout applied successfully
                }
                Err(err) => {
                    leptos::logging::log!("Error applying layout: {}", err);
                }
            }
        }
        None => {}
    });

    let inner_on_save = move |_| {
        leptos::logging::log!("Starting inner_on_save");
        if graph_handle.get().is_none() {
            return;
        }
        let graph_state = graph_handle.get().unwrap().save();
        // leptos::logging::log!("{:#?}", graph_state);
        match graph_state {
            Ok(graph) => {
                let blueprint = build_schemaful_representation_of_graph(
                    &graph,
                    &fn_def_clone,
                    &operative_clone,
                    ctx_clone.clone(),
                    func_impl_name.get(),
                    func_impl_documentation.get(),
                );
                let instantiables = blueprint
                    .get_inner_builder()
                    .instantiables
                    .get()
                    .iter()
                    .map(|instantiable| {
                        instantiable.lock().unwrap().get_template().tag.name.clone()
                    })
                    .collect::<Vec<String>>();
                let outgoing = blueprint.get_inner_builder().add_outgoing_updates.get();
                let incoming = blueprint.get_inner_builder().add_incoming_updates.get();
                leptos::logging::log!(
                    "Successful blueprint build. Instantiables: {:#?}, Outgoing Updates: {:#?}, Incoming Updates: {:#?}",
                    instantiables, outgoing, incoming
                );
                on_save.run(blueprint);
            }
            Err(_err) => {
                // Handle error
            }
        }
    };

    let operative_clone = operative.clone();
    let canvas_config = {
        let mut all_templates = vec![];

        let created_operatives = ctx
            .created_instances
            .get()
            .values()
            .filter_map(|item| match item {
                Schema::OperativeConcrete(inner) => Some(inner.clone()),
                _ => None,
            })
            .collect::<Vec<_>>();
        created_operatives.iter().for_each(|instance| {
            all_templates.push(rgso_operative_to_canvas_template(instance));
        });
        let created_instance_names = created_operatives
            .iter()
            .map(|instance| instance.get_name().clone())
            .collect::<Vec<String>>();

        let created_traits = ctx
            .created_instances
            .get()
            .values()
            .filter_map(|item| match item {
                Schema::TraitConcrete(inner) => Some(inner.clone()),
                _ => None,
            })
            .collect::<Vec<_>>();
        created_traits.iter().for_each(|instance| {
            log!("{:?}", instance);
            all_templates.push(rgso_trait_to_canvas_template(instance));
        });
        let created_trait_names = created_traits
            .iter()
            .map(|instance| instance.get_name().clone())
            .collect::<Vec<String>>();

        let step_templates = ImplStepVariantTraitObjectDiscriminants::iter()
            .map(|step| {
                let int_op_id: u128 =
                    u128::from_str(step.get_str("operative_id").unwrap()).unwrap();
                let operative = CONSTRAINT_SCHEMA.operative_library.get(&int_op_id).unwrap();
                let mut template = constraint_template_to_canvas_template(operative, true);
                // Manually change the default color of map_to_output and map_from_input
                match step {
                    ImplStepVariantTraitObjectDiscriminants::ImplStepMapFromInput => {
                        template.default_color = "#7BA7E1".to_string();
                    }
                    ImplStepVariantTraitObjectDiscriminants::ImplStepMapToOutput => {
                        template.default_color = "#B88CD1".to_string();
                    }
                    _ => {
                        template.default_color = "#FF8C42".to_string();
                    }
                }
                template
            })
            .collect::<Vec<_>>();
        let step_template_names = step_templates
            .iter()
            .map(|step| step.name.clone())
            .collect::<Vec<_>>();
        all_templates.extend(step_templates);

        // Allow DataVariants to connect to currently created instances and traits as requisite
        let impl_data_templates = ImplDataVariantTraitObjectDiscriminants::iter()
            .map(|impl_data| {
                let op_int_id: u128 =
                    u128::from_str(impl_data.get_str("operative_id").unwrap()).unwrap();
                let operative = CONSTRAINT_SCHEMA.operative_library.get(&op_int_id).unwrap();
                let mut canvas_template = constraint_template_to_canvas_template(operative, true);
                canvas_template.default_color = "#D0D3D9".to_string();

                match impl_data {
                    ImplDataVariantTraitObjectDiscriminants::ImplDataMultiOperative => {
                        canvas_template
                            .slot_templates
                            .iter_mut()
                            .for_each(|slot_template| {
                                slot_template.allowed_connections = created_instance_names.clone();
                            });
                    }
                    ImplDataVariantTraitObjectDiscriminants::ImplDataSingleOperative => {
                        canvas_template
                            .slot_templates
                            .iter_mut()
                            .for_each(|slot_template| {
                                slot_template.allowed_connections = created_instance_names.clone();
                            });
                    }
                    ImplDataVariantTraitObjectDiscriminants::ImplDataTraitOperative => {
                        canvas_template
                            .slot_templates
                            .iter_mut()
                            .for_each(|slot_template| {
                                slot_template.allowed_connections = created_trait_names.clone();
                            });
                    }
                    _ => {}
                };
                canvas_template
            })
            .collect::<Vec<_>>();
        let mut impl_data_template_names = impl_data_templates
            .iter()
            .map(|template| template.name.clone())
            .collect::<Vec<_>>();
        all_templates.extend(impl_data_templates);

        let impl_data_constraint = CONSTRAINT_SCHEMA
            .get_operative_by_id(&ImplData::get_operative_id())
            .unwrap();
        let mut impl_data_template =
            constraint_template_to_canvas_template(&impl_data_constraint, false);
        // Manually remove downstream and upstream slots so as to simplify interface
        impl_data_template
            .slot_templates
            .retain(|slot| slot.name != "DownstreamSteps" && slot.name != "UpstreamStep");
        impl_data_template.default_color = "#36B37E".to_string();
        all_templates.push(impl_data_template);
        impl_data_template_names.insert(0, impl_data_constraint.tag.name.clone());

        let function_io_constraint = CONSTRAINT_SCHEMA
            .get_operative_by_id(&FunctionIOSelf::get_operative_id())
            .unwrap();
        all_templates.push(constraint_template_to_canvas_template(
            &function_io_constraint,
            false,
        ));

        let function_input_constraint = CONSTRAINT_SCHEMA
            .get_operative_by_id(&FunctionInput::get_operative_id())
            .unwrap();
        let mut function_input =
            constraint_template_to_canvas_template(&function_input_constraint, true);
        function_input.can_create = false;
        function_input.can_modify_slots = false;
        function_input.can_delete = false;
        function_input.can_modify_fields = false;
        function_input.default_color = "#4285F4".to_string();
        all_templates.push(function_input);

        let function_output_constraint = CONSTRAINT_SCHEMA
            .get_operative_by_id(&FunctionOutput::get_operative_id())
            .unwrap();
        let mut function_output =
            constraint_template_to_canvas_template(&function_output_constraint, true);
        function_output.can_create = false;
        function_output.can_delete = false;
        function_output.can_modify_slots = false;
        function_output.can_modify_fields = false;
        function_output.default_color = "#9C27B0".to_string();
        all_templates.push(function_output);

        let template_groups = vec![
            TemplateGroup {
                description: None,
                id: "steps".to_string(),
                name: "Steps".to_string(),
                templates: step_template_names,
            },
            TemplateGroup {
                description: None,
                id: "data_types".to_string(),
                name: "Impl Data".to_string(),
                templates: impl_data_template_names,
            },
            TemplateGroup {
                description: None,
                id: "created_instances".to_string(),
                name: "Created Instances".to_string(),
                templates: created_instance_names,
            },
            TemplateGroup {
                description: None,
                id: "created_traits".to_string(),
                name: "Created Traits".to_string(),
                templates: created_trait_names,
            },
        ];

        let mut initial_nodes = vec![];
        // If this is editing an existing impl, set up the config to match the existing impl
        if let Some(initial_state) = initial_state {
            func_impl_name.set(initial_state.get_name());
            initial_nodes = setup_existing_fn_impl_in_canvas(&initial_state);
        }
        // Otherwise set it up with just the inputs and outputs
        else {
            func_impl_name.set(operative_clone.get_name() + &fn_def.get_name());
            fn_def
                .get_inputs_slot()
                .into_iter()
                .enumerate()
                .for_each(|(_i, input)| {
                    let input_and_mapstep_complex =
                        generate_function_input_and_mapstep_complex(&input, &operative);
                    initial_nodes.extend(input_and_mapstep_complex);
                });

            fn_def
                .get_outputs_slot()
                .into_iter()
                .enumerate()
                .for_each(|(_i, output)| {
                    let output_and_mapstep_complex =
                        generate_function_output_and_mapstep_complex(&output);
                    initial_nodes.extend(output_and_mapstep_complex);
                });
        }
        leptos::logging::log!(
            "{:#?}",
            GraphCanvasConfig {
                node_templates: all_templates.clone(),
                initial_nodes: initial_nodes.clone(),
                template_groups: template_groups.clone(),
                ..GraphCanvasConfig::new()
            }
        );

        GraphCanvasConfig {
            node_templates: all_templates,
            initial_nodes,
            template_groups,
            ..GraphCanvasConfig::new()
        }
    };
    let fn_def_clone = fn_def.clone();
    let operative_clone = operative.clone();
    view! {
        <Provider value=MethodImplBuilderContext {
            _impling_operative: operative_clone,
        }>
            <LeafSection>
                Implementation name: <SignalTextInput value=func_impl_name />
                <LeafSection>
                    "Implementation of " <strong>{move || fn_def_clone.get_name()}</strong>
                </LeafSection> <DocumentationInput value=func_impl_documentation />

            </LeafSection>
            <LeafSection>
                <GraphEditor
                    config=canvas_config
                    on_mount=Callback::new(move |graph| {
                        graph_handle.set(Some(graph));
                    })
                />
            </LeafSection>

            <div>
                <Button on:click=inner_on_save>Save</Button>
                " "
                <Button on:click=move |_| on_cancel.run(())>Cancel</Button>
            </div>
        </Provider>
    }
}
