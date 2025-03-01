use crate::components::method_impl_utils::{
    constraint_template_to_canvas_template, create_functioninput_complex,
    create_functionoutput_complex, generate_function_input_and_mapstep_complex,
    generate_function_output_and_mapstep_complex, rgso_operative_to_canvas_template,
    rgso_trait_to_canvas_template, setup_existing_fn_impl_in_canvas,
};
use crate::components::workspace::WorkspaceState;
use crate::components::{common::*, graph_editor::GraphEditor};
use graph_canvas::TemplateGroup;
use graph_canvas::{GraphCanvasConfig, InitialConnection, InitialNode};
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
    on_save: Callback<Box<dyn Incorporatable<MethodImplementation, Schema>>>,
    on_cancel: Callback<()>,
    #[prop(optional)] initial_state: Option<RGSOConcrete<MethodImplementation, Schema>>,
) -> impl IntoView {
    let WorkspaceState { .. } = use_context::<WorkspaceState>().unwrap();
    let ctx = use_context::<SharedGraph<Schema>>().unwrap();
    let ctx_clone = ctx.clone();

    let func_impl_name = RwSignal::new(fn_def.get_name() + "_impl");

    let fn_def_clone = fn_def.clone();
    let operative_clone = operative.clone();

    let inner_on_save = move |_| {
        let hairy_boy = MethodImplementation::new(ctx_clone.clone())
            .set_temp_id("new_fn_impl")
            .add_existing_definition(fn_def_clone.get_id(), |na| na)
            .add_existing_implementor(operative_clone.get_id(), |na| na)
            // .add_
            // .add_new_inititialsteps(|init_step| init_step.add_existing_input(existing_item_id, builder_closure))
            .set_name(func_impl_name.get());
        on_save.run(Box::new(hairy_boy));
    };

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
                let int_uid: u128 = u128::from_str(step.get_str("template_id").unwrap()).unwrap();
                let template = CONSTRAINT_SCHEMA.template_library.get(&int_uid).unwrap();
                constraint_template_to_canvas_template(template, true)
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
                let int_uid: u128 =
                    u128::from_str(impl_data.get_str("template_id").unwrap()).unwrap();
                let template = CONSTRAINT_SCHEMA.template_library.get(&int_uid).unwrap();
                let mut canvas_template = constraint_template_to_canvas_template(template, true);
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
            .get_template_by_operative_id(&ImplData::get_operative_id())
            .unwrap();
        let mut impl_data_template =
            constraint_template_to_canvas_template(&impl_data_constraint, false);
        // Manually remove downstream and upstream slots so as to simplify interface
        impl_data_template
            .slot_templates
            .retain(|slot| slot.name != "DownstreamSteps" && slot.name != "UpstreamStep");
        all_templates.push(impl_data_template);
        impl_data_template_names.insert(0, impl_data_constraint.tag.name.clone());

        let function_io_constraint = CONSTRAINT_SCHEMA
            .get_template_by_operative_id(&FunctionIOSelf::get_operative_id())
            .unwrap();
        all_templates.push(constraint_template_to_canvas_template(
            &function_io_constraint,
            false,
        ));

        let function_input_constraint = CONSTRAINT_SCHEMA
            .get_template_by_operative_id(&FunctionInput::get_operative_id())
            .unwrap();
        let mut function_input =
            constraint_template_to_canvas_template(&function_input_constraint, false);
        function_input.can_create = false;
        function_input.can_modify_slots = false;
        function_input.can_delete = false;
        all_templates.push(function_input);

        let function_output_constraint = CONSTRAINT_SCHEMA
            .get_template_by_operative_id(&FunctionOutput::get_operative_id())
            .unwrap();
        let mut function_output =
            constraint_template_to_canvas_template(&function_output_constraint, false);
        function_output.can_create = false;
        function_output.can_delete = false;
        function_output.can_modify_slots = false;
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
            func_impl_name.set("testing here".to_string());
            fn_def
                .get_inputs_slot()
                .into_iter()
                .enumerate()
                .for_each(|(i, input)| {
                    let input_and_mapstep_complex =
                        generate_function_input_and_mapstep_complex(&input, &operative);
                    initial_nodes.extend(input_and_mapstep_complex);
                    // let template_name = match input.get_type_slot() {
                    //     FunctionInputVariantTraitObject::ImplDataMultiOperative(rgsoconcrete) => {
                    //         rgsoconcrete.operative().tag.name.clone()
                    //     }
                    //     FunctionInputVariantTraitObject::FunctionIOSelf(_rgsoconcrete) => {
                    //         operative.get_name().clone()
                    //     }
                    //     FunctionInputVariantTraitObject::ImplDataBool(rgsoconcrete) => {
                    //         rgsoconcrete.operative().tag.name.clone()
                    //     }
                    //     FunctionInputVariantTraitObject::ImplDataInt(rgsoconcrete) => {
                    //         rgsoconcrete.operative().tag.name.clone()
                    //     }
                    //     FunctionInputVariantTraitObject::ImplDataString(rgsoconcrete) => {
                    //         rgsoconcrete.operative().tag.name.clone()
                    //     }
                    //     FunctionInputVariantTraitObject::ImplDataCollection(rgsoconcrete) => {
                    //         rgsoconcrete.operative().tag.name.clone()
                    //     }
                    //     FunctionInputVariantTraitObject::ImplDataSingleOperative(rgsoconcrete) => {
                    //         rgsoconcrete.operative().tag.name.clone()
                    //     }
                    //     FunctionInputVariantTraitObject::ImplDataTraitOperative(rgsoconcrete) => {
                    //         rgsoconcrete.operative().tag.name.clone()
                    //     }
                    // };
                    // let function_input_id = uuid::Uuid::from_u128(*input.get_id()).to_string();
                    // let data_node_id = "DataNode".to_string() + &i.to_string();
                    // let connection_node_id = "InputConnectionNode".to_string() + &i.to_string();
                    // let type_id = "DataNodeType".to_string() + &i.to_string();

                    // let input_node_complex = create_functioninput_complex(input.clone());
                    // initial_nodes.extend(input_node_complex);
                    // initial_nodes.push(InitialNode {
                    //     template_name: CONSTRAINT_SCHEMA
                    //         .get_operative_by_id(&ImplStepMapFromInput::get_operative_id())
                    //         .unwrap()
                    //         .tag
                    //         .name,
                    //     x: 60.0,
                    //     y: 20.0 + (40.0 * i as f64),
                    //     can_delete: false,
                    //     can_move: true,
                    //     initial_connections: vec![
                    //         InitialConnection {
                    //             host_slot_name: "Input".to_string(),
                    //             target_instance_id: function_input_id.clone(),
                    //             can_delete: false,
                    //         },
                    //         InitialConnection {
                    //             host_slot_name: "Output".to_string(),
                    //             target_instance_id: data_node_id.clone(),
                    //             can_delete: false,
                    //         },
                    //     ],
                    //     id: Some(connection_node_id.clone()),
                    //     initial_field_values: vec![],
                    // });
                    // initial_nodes.push(InitialNode {
                    //     template_name: CONSTRAINT_SCHEMA
                    //         .get_operative_by_id(&ImplData::get_operative_id())
                    //         .unwrap()
                    //         .tag
                    //         .name,
                    //     x: 100.0,
                    //     y: 20.0 + (40.0 * i as f64),
                    //     can_delete: false,
                    //     can_move: true,
                    //     initial_connections: vec![
                    //         InitialConnection {
                    //             host_slot_name: "UpstreamStep".to_string(),
                    //             target_instance_id: connection_node_id.clone(),
                    //             can_delete: false,
                    //         },
                    //         InitialConnection {
                    //             host_slot_name: "DataType".to_string(),
                    //             target_instance_id: type_id.clone(),
                    //             can_delete: false,
                    //         },
                    //     ],
                    //     id: Some(data_node_id.clone()),
                    //     initial_field_values: vec![],
                    // });
                    // initial_nodes.push(InitialNode {
                    //     template_name,
                    //     x: 140.0,
                    //     y: 20.0 + (40.0 * i as f64),
                    //     can_delete: false,
                    //     can_move: true,
                    //     initial_connections: vec![],
                    //     id: Some(type_id.clone()),
                    //     initial_field_values: vec![],
                    // });
                });

            fn_def
                .get_outputs_slot()
                .into_iter()
                .enumerate()
                .for_each(|(i, output)| {
                    let output_and_mapstep_complex =
                        generate_function_output_and_mapstep_complex(&output);
                    initial_nodes.extend(output_and_mapstep_complex);
                    //     let function_output_id = uuid::Uuid::from_u128(*output.get_id()).to_string();
                    //     let connection_node_id = "OutputConnectionNode".to_string() + &i.to_string();
                    //     let output_node_complex = create_functionoutput_complex(output.clone());
                    //     initial_nodes.extend(output_node_complex);
                    //     initial_nodes.push(InitialNode {
                    //         template_name: CONSTRAINT_SCHEMA
                    //             .get_operative_by_id(&ImplStepMapToOutput::get_operative_id())
                    //             .unwrap()
                    //             .tag
                    //             .name,
                    //         x: 60.0,
                    //         y: 20.0 + (40.0 * i as f64),
                    //         can_delete: false,
                    //         can_move: true,
                    //         initial_connections: vec![InitialConnection {
                    //             host_slot_name: "Output".to_string(),
                    //             target_instance_id: function_output_id.clone(),
                    //             can_delete: false,
                    //         }],
                    //         id: Some(connection_node_id.clone()),
                    //         initial_field_values: vec![],
                    //     });
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

        // TODO: Make sure all nodes are correctly initialized for inputs and outputs when starting fresh (e.g. recursive collections)

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
                </LeafSection>
            </LeafSection>
            <LeafSection>
                <GraphEditor config=canvas_config />
            </LeafSection>

            <div>
                <Button on:click=inner_on_save>Save</Button>
                " "
                <Button on:click=move |_| on_cancel.run(())>Cancel</Button>
            </div>
        </Provider>
    }
}
