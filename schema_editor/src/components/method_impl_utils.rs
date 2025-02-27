use graph_canvas::prelude::*;
use graph_canvas::{NodeTemplate, SlotPosition, SlotType};
use schema_editor_generated_toolkit::prelude::*;
use uuid::Uuid;

use super::method_impl_traversal_utils::{
    analyze_method_implementation, get_step_data_dependencies, ExecutionNode, ExecutionNodeTerminal,
};
use super::utils::get_all_operatives_which_impl_trait_set;

pub(crate) fn constraint_template_to_canvas_template(
    template: &base_types::constraint_schema::LibraryTemplate<
        base_types::primitives::PrimitiveTypes,
        base_types::primitives::PrimitiveValues,
    >,
    keep_fields: bool,
) -> NodeTemplate {
    let template_string_id = uuid::Uuid::from_u128(template.tag.id).to_string();
    let slot_templates = template
        .operative_slots
        .values()
        .map(|slot| {
            let slot_string_id = uuid::Uuid::from_u128(slot.tag.id).to_string();
            let allowed_connections = match &slot.operative_descriptor {
                base_types::constraint_schema::OperativeVariants::LibraryOperative(op) => {
                    vec![CONSTRAINT_SCHEMA
                        .operative_library
                        .get(&op)
                        .unwrap()
                        .tag
                        .name
                        .clone()]
                }
                base_types::constraint_schema::OperativeVariants::TraitOperative(
                    trait_operative,
                ) => CONSTRAINT_SCHEMA
                    .get_all_operatives_which_impl_trait_set(&trait_operative.trait_ids)
                    .iter()
                    .map(|op| op.tag.name.clone())
                    .collect::<Vec<_>>(),
            };
            SlotTemplate {
                id: slot_string_id,
                name: slot.tag.name.clone(),
                position: SlotPosition::Right,
                slot_type: SlotType::Outgoing,
                can_modify_connections: true,
                allowed_connections,
                min_connections: match slot.bounds {
                    base_types::constraint_schema::SlotBounds::Single => 1,
                    base_types::constraint_schema::SlotBounds::LowerBound(min) => min,
                    base_types::constraint_schema::SlotBounds::UpperBound(_) => 0,
                    base_types::constraint_schema::SlotBounds::Range(min, _) => min,
                    base_types::constraint_schema::SlotBounds::LowerBoundOrZero(_) => 0,
                    base_types::constraint_schema::SlotBounds::RangeOrZero(_, _) => 0,
                },
                max_connections: match slot.bounds {
                    base_types::constraint_schema::SlotBounds::Single => Some(1),
                    base_types::constraint_schema::SlotBounds::LowerBound(_) => None,
                    base_types::constraint_schema::SlotBounds::UpperBound(max) => Some(max),
                    base_types::constraint_schema::SlotBounds::Range(_, max) => Some(max),
                    base_types::constraint_schema::SlotBounds::LowerBoundOrZero(_) => None,
                    base_types::constraint_schema::SlotBounds::RangeOrZero(_, max) => Some(max),
                },
            }
        })
        .collect();
    let field_templates = template
        .field_constraints
        .values()
        .map(|field| {
            let field_string_id = uuid::Uuid::from_u128(field.tag.id).to_string();
            FieldTemplate {
                id: field_string_id,
                name: field.tag.name.clone(),
                field_type: match field.value_type {
                    base_types::primitives::PrimitiveTypes::Bool => FieldType::Boolean,
                    base_types::primitives::PrimitiveTypes::Int => FieldType::Integer,
                    base_types::primitives::PrimitiveTypes::String => FieldType::String,
                    // base_types::primitives::PrimitiveTypes::EmptyTuple => todo!(),
                    // base_types::primitives::PrimitiveTypes::Option(primitive_types) => todo!(),
                    // base_types::primitives::PrimitiveTypes::List(primitive_types) => todo!(),
                    _ => todo!(),
                },
                default_value: match field.value_type {
                    base_types::primitives::PrimitiveTypes::Bool => "false".to_string(),
                    base_types::primitives::PrimitiveTypes::Int => "0".to_string(),
                    base_types::primitives::PrimitiveTypes::String => "".to_string(),
                    // base_types::primitives::PrimitiveTypes::EmptyTuple => todo!(),
                    // base_types::primitives::PrimitiveTypes::Option(primitive_types) => todo!(),
                    // base_types::primitives::PrimitiveTypes::List(primitive_types) => todo!(),
                    _ => todo!(),
                },
            }
        })
        .collect::<Vec<_>>();
    NodeTemplate {
        template_id: template_string_id,
        name: template.tag.name.clone(),
        field_templates: if keep_fields {
            field_templates
        } else {
            Vec::new()
        },
        slot_templates,
        ..NodeTemplate::new(&template.tag.name)
    }
}

pub(crate) fn rgso_to_canvas_template_with_slots(
    item: &RGSOConcrete<OperativeConcrete, Schema>,
    schema: &RGSOConcrete<SchemaConcrete, Schema>,
) -> NodeTemplate {
    let template_string_id = uuid::Uuid::from_u128(item.get_id().clone()).to_string();
    let slot_templates = item
        .get_roottemplate_slot()
        .get_templateslots_slot()
        .into_iter()
        .map(|slot| {
            let slot_string_id = uuid::Uuid::from_u128(slot.get_id().clone()).to_string();
            let allowed_connections = match &slot.get_templateslotvariant_slot() {
                TemplateSlotTypeVariantTraitObject::TemplateSlotTypeTraitOperative(rgsoconcrete) => {
                    // let traits = rgsoconcrete.get_allowedtraits_slot().iter().map(|trait_item| trait_item.get_id()).collect::<Vec<_>>();
                    get_all_operatives_which_impl_trait_set(rgsoconcrete.get_allowedtraits_slot(), schema).into_iter().map(|item| item.get_name()).collect::<Vec<_>>()
                },
                TemplateSlotTypeVariantTraitObject::TemplateSlotTypeSingleOperative(rgsoconcrete) => vec![rgsoconcrete.get_allowedoperative_slot().get_name()],
                TemplateSlotTypeVariantTraitObject::TemplateSlotTypeMultiOperative(rgsoconcrete) => rgsoconcrete.get_allowedoperatives_slot().iter().map(|item| item.get_name()).collect::<Vec<_>>(),
            };
            SlotTemplate {
                id: slot_string_id,
                name: slot.get_name(),
                position: SlotPosition::Right,
                slot_type: SlotType::Outgoing,
                can_modify_connections: true,
                allowed_connections,
                min_connections: match slot.get_slotcardinality_slot() {
                    TemplateSlotCardinalityVariantTraitObject::TemplateSlotCardinalityRangeOrZero(_rgsoconcrete) => 0,
                    TemplateSlotCardinalityVariantTraitObject::TemplateSlotCardinalityLowerBoundOrZero(_rgsoconcrete) => 0,
                    TemplateSlotCardinalityVariantTraitObject::TemplateSlotCardinalityRange(rgsoconcrete) => rgsoconcrete.get_lower_bound_field() as usize,
                    TemplateSlotCardinalityVariantTraitObject::TemplateSlotCardinalityLowerBound(rgsoconcrete) => rgsoconcrete.get_lower_bound_field() as usize,
                    TemplateSlotCardinalityVariantTraitObject::TemplateSlotCardinalitySingle(_rgsoconcrete) => 1,
                },
                max_connections: match slot.get_slotcardinality_slot() {
                    TemplateSlotCardinalityVariantTraitObject::TemplateSlotCardinalityRangeOrZero(rgsoconcrete) => Some(rgsoconcrete.get_upper_bound_field() as usize),
                    TemplateSlotCardinalityVariantTraitObject::TemplateSlotCardinalityLowerBoundOrZero(_rgsoconcrete) => None,
                    TemplateSlotCardinalityVariantTraitObject::TemplateSlotCardinalityRange(rgsoconcrete) => Some(rgsoconcrete.get_upper_bound_field() as usize),
                    TemplateSlotCardinalityVariantTraitObject::TemplateSlotCardinalityLowerBound(_rgsoconcrete) => None,
                    TemplateSlotCardinalityVariantTraitObject::TemplateSlotCardinalitySingle(_rgsoconcrete) => Some(1),
                },
            }
        })
        .collect();
    NodeTemplate {
        template_id: template_string_id,
        name: item.get_name_field().clone(),
        slot_templates,
        ..NodeTemplate::new(&item.get_name_field())
    }
}

pub(crate) fn rgso_operative_to_canvas_template(
    item: &RGSOConcrete<OperativeConcrete, Schema>,
) -> NodeTemplate {
    let template_string_id = uuid::Uuid::from_u128(item.get_id().clone()).to_string();
    NodeTemplate {
        template_id: template_string_id,
        name: item.get_name_field().clone(),
        slot_templates: vec![],
        ..NodeTemplate::new(&item.get_name_field())
    }
}
pub(crate) fn rgso_trait_to_canvas_template(
    item: &RGSOConcrete<TraitConcrete, Schema>,
) -> NodeTemplate {
    let template_string_id = uuid::Uuid::from_u128(item.get_id().clone()).to_string();
    NodeTemplate {
        template_id: template_string_id,
        name: item.get_name_field().clone(),
        slot_templates: vec![],
        ..NodeTemplate::new(&item.get_name_field())
    }
}

pub(crate) fn setup_existing_fn_impl_in_canvas(
    existing_impl: &RGSOConcrete<MethodImplementation, Schema>,
) -> Vec<InitialNode> {
    let execution_flow = analyze_method_implementation(existing_impl);

    let mut initial_nodes = Vec::new();

    execution_flow.nodes.values().for_each(|node| match node {
        ExecutionNode::Data(rgsoconcrete) => {
            let data_complex = generate_impldata_complex(rgsoconcrete);
            initial_nodes.extend(data_complex);
        }
        ExecutionNode::Step(impl_step_variant_trait_object) => {
            let step_name = match impl_step_variant_trait_object {
                ImplStepVariantTraitObject::ImplStepBitNot(step) => {
                    step.operative().tag.name.clone()
                }
                ImplStepVariantTraitObject::ImplStepMathDivide(step) => {
                    step.operative().tag.name.clone()
                }
                ImplStepVariantTraitObject::ImplStepBitOr(step) => {
                    step.operative().tag.name.clone()
                }
                ImplStepVariantTraitObject::ImplStepCompareEqual(step) => {
                    step.operative().tag.name.clone()
                }
                ImplStepVariantTraitObject::ImplStepBitAnd(step) => {
                    step.operative().tag.name.clone()
                }
                ImplStepVariantTraitObject::ImplStepMathAdd(step) => {
                    step.operative().tag.name.clone()
                }
                ImplStepVariantTraitObject::ImplStepMathModulus(step) => {
                    step.operative().tag.name.clone()
                }
                ImplStepVariantTraitObject::ImplStepMathMultiply(step) => {
                    step.operative().tag.name.clone()
                }
                ImplStepVariantTraitObject::ImplStepMathSubtract(step) => {
                    step.operative().tag.name.clone()
                }
                ImplStepVariantTraitObject::ImplStepCompareGreaterThan(step) => {
                    step.operative().tag.name.clone()
                }
                ImplStepVariantTraitObject::ImplStepCompareLessThan(step) => {
                    step.operative().tag.name.clone()
                }
                ImplStepVariantTraitObject::ImplStepIf(step) => step.operative().tag.name.clone(),
                ImplStepVariantTraitObject::ImplStepIteratorFilter(step) => {
                    step.operative().tag.name.clone()
                }
                ImplStepVariantTraitObject::ImplStepMutateSlot(step) => {
                    step.operative().tag.name.clone()
                }
                ImplStepVariantTraitObject::ImplStepGetField(step) => {
                    step.operative().tag.name.clone()
                }
                ImplStepVariantTraitObject::ImplStepMutateField(step) => {
                    step.operative().tag.name.clone()
                }
                ImplStepVariantTraitObject::ImplStepMultiTypeSplitter(step) => {
                    step.operative().tag.name.clone()
                }
                ImplStepVariantTraitObject::ImplStepIteratorMap(step) => {
                    step.operative().tag.name.clone()
                }
                ImplStepVariantTraitObject::ImplStepTraverseSlot(step) => {
                    step.operative().tag.name.clone()
                }
                ImplStepVariantTraitObject::ImplStepWhileLoop(step) => {
                    step.operative().tag.name.clone()
                }
                ImplStepVariantTraitObject::ImplStepMapToOutput(step) => {
                    step.operative().tag.name.clone()
                }
                ImplStepVariantTraitObject::ImplStepMapFromInput(step) => {
                    step.operative().tag.name.clone()
                }
            };

            let mut step_node = InitialNode::new(step_name);
            step_node.id =
                Some(Uuid::from_u128(*impl_step_variant_trait_object.get_id()).to_string());

            let edges = get_step_data_dependencies(impl_step_variant_trait_object);
            for edge in edges {
                step_node.initial_connections.push(InitialConnection {
                    can_delete: true,
                    host_slot_name: edge.slot_name,
                    target_instance_id: Uuid::from_u128(*edge.to.get_id()).to_string(),
                });
            }

            initial_nodes.push(step_node);
        }
        ExecutionNode::Terminal(execution_node_terminal) => {
            match execution_node_terminal {
                ExecutionNodeTerminal::Input(input) => {
                    let terminal_name = input.operative().tag.name.clone();
                    let mut terminal_node = InitialNode::new(terminal_name);
                    terminal_node.id =
                        Some(Uuid::from_u128(*execution_node_terminal.get_id()).to_string());

                    // Connect to Data Node
                    terminal_node.initial_connections.push(InitialConnection {
                        can_delete: true,
                        host_slot_name: "Output".to_string(),
                        target_instance_id: Uuid::from_u128(*input.get_output_slot().get_id())
                            .to_string(),
                    });

                    // Create and Connect to FunctionInput Node
                    let rgso_function_input = input.get_input_slot();
                    let function_input_complex =
                        create_functioninput_complex(rgso_function_input.clone());
                    initial_nodes.extend(function_input_complex);

                    // generate_impldata_complex(fi_rgso_data_node)
                    terminal_node.initial_connections.push(InitialConnection {
                        can_delete: true,
                        host_slot_name: "Input".to_string(),
                        target_instance_id: Uuid::from_u128(*rgso_function_input.get_id())
                            .to_string(),
                    });

                    initial_nodes.push(terminal_node);
                }
                ExecutionNodeTerminal::Output(output) => {
                    let terminal_name = output.operative().tag.name.clone();
                    let mut terminal_node = InitialNode::new(terminal_name);
                    terminal_node.id =
                        Some(Uuid::from_u128(*execution_node_terminal.get_id()).to_string());

                    // Connect to Data Node
                    terminal_node.initial_connections.push(InitialConnection {
                        can_delete: true,
                        host_slot_name: "Input".to_string(),
                        target_instance_id: Uuid::from_u128(*output.get_input_slot().get_id())
                            .to_string(),
                    });

                    // Create and Connect to FunctionOutput Node
                    let rgso_function_output = output.get_output_slot();
                    let function_output_complex =
                        create_functionoutput_complex(rgso_function_output.clone());
                    initial_nodes.extend(function_output_complex);

                    terminal_node.initial_connections.push(InitialConnection {
                        can_delete: true,
                        host_slot_name: "Output".to_string(),
                        target_instance_id: Uuid::from_u128(*rgso_function_output.get_id())
                            .to_string(),
                    });

                    initial_nodes.push(terminal_node);
                }
            };
        }
    });

    initial_nodes
}

fn generate_impldata_complex(impl_data_node: &RGSOConcrete<ImplData, Schema>) -> Vec<InitialNode> {
    let mut initial_nodes = Vec::new();

    let mut data_node = InitialNode::new(impl_data_node.operative().tag.name.clone());
    data_node.id = Some(Uuid::from_u128(*impl_data_node.get_id()).to_string());

    let data_type = impl_data_node.get_datatype_slot();

    let additional_nodes = generate_impldatavariant_complex(data_type, &mut data_node, "DataType");

    initial_nodes.extend(additional_nodes);
    initial_nodes.push(data_node);
    initial_nodes
}

fn generate_impldatavariant_complex(
    impl_data_type: ImplDataVariantTraitObject,
    node_to_connect: &mut InitialNode,
    slot_name: &str,
) -> Vec<InitialNode> {
    let mut initial_nodes = vec![];

    match impl_data_type {
        ImplDataVariantTraitObject::ImplDataMultiOperative(datatype) => {
            let mut data_type_node = InitialNode::new(datatype.operative().tag.name.clone());
            data_type_node.id = Some(Uuid::from_u128(*datatype.get_id()).to_string());
            let allowed_ops = datatype.get_allowedoperatives_slot();
            allowed_ops.iter().for_each(|allowed_op| {
                let mut op_node = InitialNode::new(allowed_op.operative().tag.name.clone());
                op_node.id = Some(Uuid::from_u128(*allowed_op.get_id()).to_string());
                data_type_node.initial_connections.push(InitialConnection {
                    host_slot_name: "AllowedOperatives".to_string(),
                    target_instance_id: op_node.id.clone().unwrap(),
                    can_delete: true,
                });
                initial_nodes.push(op_node);
            });
            node_to_connect.initial_connections.push(InitialConnection {
                can_delete: true,
                host_slot_name: slot_name.to_string(),
                target_instance_id: data_type_node.id.clone().unwrap(),
            });
            initial_nodes.push(data_type_node);
        }
        ImplDataVariantTraitObject::ImplDataBool(datatype) => {
            let mut data_type_node = InitialNode::new(datatype.operative().tag.name.clone());
            data_type_node.id = Some(Uuid::from_u128(*datatype.get_id()).to_string());
            node_to_connect.initial_connections.push(InitialConnection {
                can_delete: true,
                host_slot_name: slot_name.to_string(),
                target_instance_id: data_type_node.id.clone().unwrap(),
            });
            initial_nodes.push(data_type_node);
        }
        ImplDataVariantTraitObject::ImplDataInt(datatype) => {
            let mut data_type_node = InitialNode::new(datatype.operative().tag.name.clone());
            data_type_node.id = Some(Uuid::from_u128(*datatype.get_id()).to_string());
            node_to_connect.initial_connections.push(InitialConnection {
                can_delete: true,
                host_slot_name: slot_name.to_string(),
                target_instance_id: data_type_node.id.clone().unwrap(),
            });
            initial_nodes.push(data_type_node);
        }
        ImplDataVariantTraitObject::ImplDataString(datatype) => {
            let mut data_type_node = InitialNode::new(datatype.operative().tag.name.clone());
            data_type_node.id = Some(Uuid::from_u128(*datatype.get_id()).to_string());
            node_to_connect.initial_connections.push(InitialConnection {
                can_delete: true,
                host_slot_name: slot_name.to_string(),
                target_instance_id: data_type_node.id.clone().unwrap(),
            });
            initial_nodes.push(data_type_node);
        }
        ImplDataVariantTraitObject::ImplDataManualInt(datatype) => {
            let mut data_type_node = InitialNode::new(datatype.operative().tag.name.clone());
            data_type_node.id = Some(Uuid::from_u128(*datatype.get_id()).to_string());
            node_to_connect.initial_connections.push(InitialConnection {
                can_delete: true,
                host_slot_name: slot_name.to_string(),
                target_instance_id: data_type_node.id.clone().unwrap(),
            });
            let field_id = datatype
                .template()
                .field_constraints
                .values()
                .find(|field| field.tag.name == "value")
                .unwrap()
                .tag
                .id;
            node_to_connect
                .initial_field_values
                .push(InitialFieldValue {
                    field_id: Uuid::from_u128(field_id).to_string(),
                    value: datatype.get_value_field().to_string(),
                });
            initial_nodes.push(data_type_node);
        }
        ImplDataVariantTraitObject::ImplDataManualBool(datatype) => {
            let mut data_type_node = InitialNode::new(datatype.operative().tag.name.clone());
            data_type_node.id = Some(Uuid::from_u128(*datatype.get_id()).to_string());
            node_to_connect.initial_connections.push(InitialConnection {
                can_delete: true,
                host_slot_name: slot_name.to_string(),
                target_instance_id: data_type_node.id.clone().unwrap(),
            });
            let field_id = datatype
                .template()
                .field_constraints
                .values()
                .find(|field| field.tag.name == "value")
                .unwrap()
                .tag
                .id;
            node_to_connect
                .initial_field_values
                .push(InitialFieldValue {
                    field_id: Uuid::from_u128(field_id).to_string(),
                    value: datatype.get_value_field().to_string(),
                });
            initial_nodes.push(data_type_node);
        }
        ImplDataVariantTraitObject::ImplDataCollection(datatype) => {
            let mut data_type_node = InitialNode::new(datatype.operative().tag.name.clone());
            data_type_node.id = Some(Uuid::from_u128(*datatype.get_id()).to_string());
            node_to_connect.initial_connections.push(InitialConnection {
                can_delete: true,
                host_slot_name: slot_name.to_string(),
                target_instance_id: data_type_node.id.clone().unwrap(),
            });
            initial_nodes.push(data_type_node.clone());
            let mut current_collection_node = data_type_node;
            let mut maybe_next_collection = Some(datatype.get_collectiontype_slot());
            while let Some(next_collection) = maybe_next_collection {
                let mut collection_type_node =
                    InitialNode::new(next_collection.operative().tag.name.clone());
                collection_type_node.id =
                    Some(Uuid::from_u128(*next_collection.get_id()).to_string());
                current_collection_node
                    .initial_connections
                    .push(InitialConnection {
                        can_delete: true,
                        host_slot_name: "CollectionType".to_string(),
                        target_instance_id: collection_type_node.id.clone().unwrap(),
                    });
                initial_nodes.push(collection_type_node.clone());
                match next_collection {
                    ImplDataVariantMinusManualsTraitObject::ImplDataCollection(rgsoconcrete) => {
                        maybe_next_collection =
                            Some(ImplDataVariantMinusManualsTraitObject::ImplDataCollection(
                                rgsoconcrete,
                            ));
                        current_collection_node = collection_type_node;
                    }
                    _ => maybe_next_collection = None,
                }
            }
        }
        ImplDataVariantTraitObject::ImplDataSingleOperative(datatype) => {
            let mut data_type_node = InitialNode::new(datatype.operative().tag.name.clone());
            data_type_node.id = Some(Uuid::from_u128(*datatype.get_id()).to_string());
            let allowed_op = datatype.get_allowedoperative_slot();
            let mut op_node = InitialNode::new(allowed_op.operative().tag.name.clone());
            op_node.id = Some(Uuid::from_u128(*allowed_op.get_id()).to_string());
            data_type_node.initial_connections.push(InitialConnection {
                host_slot_name: "AllowedOperatives".to_string(),
                target_instance_id: op_node.id.clone().unwrap(),
                can_delete: true,
            });
            initial_nodes.push(op_node);
            node_to_connect.initial_connections.push(InitialConnection {
                can_delete: true,
                host_slot_name: slot_name.to_string(),
                target_instance_id: data_type_node.id.clone().unwrap(),
            });
            initial_nodes.push(data_type_node);
        }
        ImplDataVariantTraitObject::ImplDataManualString(datatype) => {
            let mut data_type_node = InitialNode::new(datatype.operative().tag.name.clone());
            data_type_node.id = Some(Uuid::from_u128(*datatype.get_id()).to_string());
            node_to_connect.initial_connections.push(InitialConnection {
                can_delete: true,
                host_slot_name: slot_name.to_string(),
                target_instance_id: data_type_node.id.clone().unwrap(),
            });
            let field_id = datatype
                .template()
                .field_constraints
                .values()
                .find(|field| field.tag.name == "value")
                .unwrap()
                .tag
                .id;
            node_to_connect
                .initial_field_values
                .push(InitialFieldValue {
                    field_id: Uuid::from_u128(field_id).to_string(),
                    value: datatype.get_value_field(),
                });
            initial_nodes.push(data_type_node);
        }
        ImplDataVariantTraitObject::ImplDataTraitOperative(datatype) => {
            let mut data_type_node = InitialNode::new(datatype.operative().tag.name.clone());
            data_type_node.id = Some(Uuid::from_u128(*datatype.get_id()).to_string());
            let required_traits = datatype.get_requiredtraits_slot();
            required_traits.iter().for_each(|required_trait| {
                let mut op_node = InitialNode::new(required_trait.operative().tag.name.clone());
                op_node.id = Some(Uuid::from_u128(*required_trait.get_id()).to_string());
                data_type_node.initial_connections.push(InitialConnection {
                    host_slot_name: "RequiredTraits".to_string(),
                    target_instance_id: op_node.id.clone().unwrap(),
                    can_delete: true,
                });
                initial_nodes.push(op_node);
            });
            node_to_connect.initial_connections.push(InitialConnection {
                can_delete: true,
                host_slot_name: slot_name.to_string(),
                target_instance_id: data_type_node.id.clone().unwrap(),
            });
            initial_nodes.push(data_type_node);
        }
    };
    initial_nodes
}

fn map_input_types_to_impldatavariant(
    input_type: FunctionInputVariantTraitObject,
) -> ImplDataVariantTraitObject {
    match input_type {
        FunctionInputVariantTraitObject::ImplDataMultiOperative(rgsoconcrete) => {
            ImplDataVariantTraitObject::ImplDataMultiOperative(rgsoconcrete)
        }
        FunctionInputVariantTraitObject::FunctionIOSelf(_rgsoconcrete) => {
            panic!("FunctionIOSelf Should be handled separately")
        }
        FunctionInputVariantTraitObject::ImplDataBool(rgsoconcrete) => {
            ImplDataVariantTraitObject::ImplDataBool(rgsoconcrete)
        }
        FunctionInputVariantTraitObject::ImplDataInt(rgsoconcrete) => {
            ImplDataVariantTraitObject::ImplDataInt(rgsoconcrete)
        }
        FunctionInputVariantTraitObject::ImplDataString(rgsoconcrete) => {
            ImplDataVariantTraitObject::ImplDataString(rgsoconcrete)
        }
        FunctionInputVariantTraitObject::ImplDataCollection(rgsoconcrete) => {
            ImplDataVariantTraitObject::ImplDataCollection(rgsoconcrete)
        }
        FunctionInputVariantTraitObject::ImplDataSingleOperative(rgsoconcrete) => {
            ImplDataVariantTraitObject::ImplDataSingleOperative(rgsoconcrete)
        }
        FunctionInputVariantTraitObject::ImplDataTraitOperative(rgsoconcrete) => {
            ImplDataVariantTraitObject::ImplDataTraitOperative(rgsoconcrete)
        }
    }
}

fn map_output_types_to_impldatavariant(
    output_type: ImplDataVariantMinusManualsTraitObject,
) -> ImplDataVariantTraitObject {
    match output_type {
        ImplDataVariantMinusManualsTraitObject::ImplDataBool(rgsoconcrete) => {
            ImplDataVariantTraitObject::ImplDataBool(rgsoconcrete)
        }
        ImplDataVariantMinusManualsTraitObject::ImplDataInt(rgsoconcrete) => {
            ImplDataVariantTraitObject::ImplDataInt(rgsoconcrete)
        }
        ImplDataVariantMinusManualsTraitObject::ImplDataString(rgsoconcrete) => {
            ImplDataVariantTraitObject::ImplDataString(rgsoconcrete)
        }
        ImplDataVariantMinusManualsTraitObject::ImplDataCollection(rgsoconcrete) => {
            ImplDataVariantTraitObject::ImplDataCollection(rgsoconcrete)
        }
        ImplDataVariantMinusManualsTraitObject::ImplDataSingleOperative(rgsoconcrete) => {
            ImplDataVariantTraitObject::ImplDataSingleOperative(rgsoconcrete)
        }
        ImplDataVariantMinusManualsTraitObject::ImplDataTraitOperative(rgsoconcrete) => {
            ImplDataVariantTraitObject::ImplDataTraitOperative(rgsoconcrete)
        }
        ImplDataVariantMinusManualsTraitObject::ImplDataMultiOperative(rgsoconcrete) => {
            ImplDataVariantTraitObject::ImplDataMultiOperative(rgsoconcrete)
        }
    }
}

pub(crate) fn create_functioninput_complex(
    function_input: RGSOConcrete<FunctionInput, Schema>,
) -> Vec<InitialNode> {
    let mut initial_nodes = Vec::new();

    let mut function_input_node = InitialNode::new(function_input.operative().tag.name.clone());
    function_input_node.id = Some(Uuid::from_u128(*function_input.get_id()).to_string());

    // Map input types to data types, handling the `self` case
    let fi_rgso_data_node = function_input.get_type_slot();
    if matches!(
        fi_rgso_data_node,
        FunctionInputVariantTraitObject::FunctionIOSelf(_)
    ) {
        match &fi_rgso_data_node {
            FunctionInputVariantTraitObject::FunctionIOSelf(rgsoconcrete) => {
                let mut self_input_node =
                    InitialNode::new(rgsoconcrete.operative().tag.name.clone());
                self_input_node.can_delete = false;
                self_input_node.id = Some(Uuid::new_v4().to_string());
                function_input_node
                    .initial_connections
                    .push(InitialConnection {
                        can_delete: false,
                        host_slot_name: "Type".to_string(),
                        target_instance_id: self_input_node.id.clone().unwrap(),
                    });
                initial_nodes.push(self_input_node);
            }
            _ => panic!(),
        }
    } else {
        let mapped_input = map_input_types_to_impldatavariant(fi_rgso_data_node);
        let mut data_complex_initial_nodes =
            generate_impldatavariant_complex(mapped_input, &mut function_input_node, "Type");
        data_complex_initial_nodes.iter_mut().for_each(|node| {
            node.can_delete = false;
        });

        initial_nodes.extend(data_complex_initial_nodes);
    }
    initial_nodes.push(function_input_node);
    initial_nodes
}
pub(crate) fn create_functionoutput_complex(
    function_output: RGSOConcrete<FunctionOutput, Schema>,
) -> Vec<InitialNode> {
    let mut initial_nodes = Vec::new();
    let mut function_output_node = InitialNode::new(function_output.operative().tag.name.clone());
    function_output_node.id = Some(Uuid::from_u128(*function_output.get_id()).to_string());

    // Map output types to data types, handling the `self` case
    let fo_rgso_data_node = function_output.get_type_slot();
    let mapped_input = map_output_types_to_impldatavariant(fo_rgso_data_node);
    let mut data_complex_initial_nodes =
        generate_impldatavariant_complex(mapped_input, &mut function_output_node, "Type");
    data_complex_initial_nodes.iter_mut().for_each(|node| {
        node.can_delete = false;
    });
    initial_nodes.push(function_output_node);
    initial_nodes.extend(data_complex_initial_nodes);
    initial_nodes
}
