use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use graph_canvas::prelude::*;
use graph_canvas::{NodeInstance, NodeTemplate, SlotInstance, SlotPosition, SlotType};
use schema_editor_generated_toolkit::prelude::*;
use uuid::Uuid;

use super::method_impl_traversal_utils::{
    analyze_method_implementation, get_step_data_dependencies, ExecutionNode, ExecutionNodeTerminal,
};

macro_rules! get_field_id {
    ($struct_name:ident, $field_name:expr) => {{
        let operative = CONSTRAINT_SCHEMA
            .operative_library
            .get(&$struct_name::get_operative_id())
            .unwrap();
        let template = CONSTRAINT_SCHEMA
            .template_library
            .get(&operative.template_id)
            .unwrap();
        let field_id = Uuid::from_u128(
            template
                .field_constraints
                .values()
                .find(|field| field.tag.name.contains($field_name))
                .unwrap()
                .tag
                .id,
        )
        .to_string();
        field_id
    }};
}
macro_rules! get_slot_id {
    ($struct_name:ident, $slot_name:expr) => {{
        let operative = CONSTRAINT_SCHEMA
            .operative_library
            .get(&$struct_name::get_operative_id())
            .unwrap();
        let template = CONSTRAINT_SCHEMA
            .template_library
            .get(&operative.template_id)
            .unwrap();
        let slot_id = Uuid::from_u128(
            template
                .operative_slots
                .values()
                .find(|slot| slot.tag.name.contains($slot_name))
                .unwrap()
                .tag
                .id,
        )
        .to_string();
        slot_id
    }};
}

macro_rules! match_impl_step_template {
    ($template_id:expr, $action:ident, $editor:expr, $builder:expr, $step_temp_id:expr) => {{
        let u128 = Uuid::parse_str(&$template_id)
            .expect("Failed to parse UUID")
            .as_u128();

        match u128 {
            id if id == ImplStepBitAnd::get_operative_id() => {
                $editor.incorporate(&$builder.clone().$action::<ImplStepBitAnd>($step_temp_id));
            }
            id if id == ImplStepBitNot::get_operative_id() => {
                $editor.incorporate(&$builder.clone().$action::<ImplStepBitNot>($step_temp_id));
            }
            id if id == ImplStepBitOr::get_operative_id() => {
                $editor.incorporate(&$builder.clone().$action::<ImplStepBitOr>($step_temp_id));
            }
            id if id == ImplStepCompareEqual::get_operative_id() => {
                $editor.incorporate(
                    &$builder
                        .clone()
                        .$action::<ImplStepCompareEqual>($step_temp_id),
                );
            }
            id if id == ImplStepCompareGreaterThan::get_operative_id() => {
                $editor.incorporate(
                    &$builder
                        .clone()
                        .$action::<ImplStepCompareGreaterThan>($step_temp_id),
                );
            }
            id if id == ImplStepCompareLessThan::get_operative_id() => {
                $editor.incorporate(
                    &$builder
                        .clone()
                        .$action::<ImplStepCompareLessThan>($step_temp_id),
                );
            }
            id if id == ImplStepGetField::get_operative_id() => {
                $editor.incorporate(&$builder.clone().$action::<ImplStepGetField>($step_temp_id));
            }
            id if id == ImplStepIf::get_operative_id() => {
                $editor.incorporate(&$builder.clone().$action::<ImplStepIf>($step_temp_id));
            }
            id if id == ImplStepIteratorFilter::get_operative_id() => {
                $editor.incorporate(
                    &$builder
                        .clone()
                        .$action::<ImplStepIteratorFilter>($step_temp_id),
                );
            }
            id if id == ImplStepIteratorMap::get_operative_id() => {
                $editor.incorporate(
                    &$builder
                        .clone()
                        .$action::<ImplStepIteratorMap>($step_temp_id),
                );
            }
            id if id == ImplStepMapFromInput::get_operative_id() => {
                $editor.incorporate(
                    &$builder
                        .clone()
                        .$action::<ImplStepMapFromInput>($step_temp_id),
                );
            }
            id if id == ImplStepMapToOutput::get_operative_id() => {
                $editor.incorporate(
                    &$builder
                        .clone()
                        .$action::<ImplStepMapToOutput>($step_temp_id),
                );
            }
            id if id == ImplStepMathAdd::get_operative_id() => {
                $editor.incorporate(&$builder.clone().$action::<ImplStepMathAdd>($step_temp_id));
            }
            id if id == ImplStepMathDivide::get_operative_id() => {
                $editor.incorporate(
                    &$builder
                        .clone()
                        .$action::<ImplStepMathDivide>($step_temp_id),
                );
            }
            id if id == ImplStepMathModulus::get_operative_id() => {
                $editor.incorporate(
                    &$builder
                        .clone()
                        .$action::<ImplStepMathModulus>($step_temp_id),
                );
            }
            id if id == ImplStepMathMultiply::get_operative_id() => {
                $editor.incorporate(
                    &$builder
                        .clone()
                        .$action::<ImplStepMathMultiply>($step_temp_id),
                );
            }
            id if id == ImplStepMathSubtract::get_operative_id() => {
                $editor.incorporate(
                    &$builder
                        .clone()
                        .$action::<ImplStepMathSubtract>($step_temp_id),
                );
            }
            id if id == ImplStepMultiTypeSplitter::get_operative_id() => {
                $editor.incorporate(
                    &$builder
                        .clone()
                        .$action::<ImplStepMultiTypeSplitter>($step_temp_id),
                );
            }
            id if id == ImplStepMutateField::get_operative_id() => {
                $editor.incorporate(
                    &$builder
                        .clone()
                        .$action::<ImplStepMutateField>($step_temp_id),
                );
            }
            id if id == ImplStepMutateSlot::get_operative_id() => {
                $editor.incorporate(
                    &$builder
                        .clone()
                        .$action::<ImplStepMutateSlot>($step_temp_id),
                );
            }
            id if id == ImplStepTraverseSlot::get_operative_id() => {
                $editor.incorporate(
                    &$builder
                        .clone()
                        .$action::<ImplStepTraverseSlot>($step_temp_id),
                );
            }
            id if id == ImplStepWhileLoop::get_operative_id() => {
                $editor.incorporate(&$builder.clone().$action::<ImplStepWhileLoop>($step_temp_id));
            }
            _ => {}
        };
    }};
}

macro_rules! match_impl_data_minus_manuals_template {
    ($template_id:expr, $action:ident, $builder:expr, $step_temp_id:expr) => {{
        let u128 = Uuid::parse_str(&$template_id)
            .expect("Failed to parse UUID")
            .as_u128();

        match u128 {
            id if id == ImplDataBool::get_operative_id() => {
                $builder.incorporate($builder.clone().$action::<ImplDataBool>($step_temp_id));
            }
            id if id == ImplDataCollection::get_operative_id() => {
                $builder.incorporate(
                    $builder
                        .clone()
                        .$action::<ImplDataCollection>($step_temp_id),
                );
            }
            id if id == ImplDataInt::get_operative_id() => {
                $builder.incorporate($builder.clone().$action::<ImplDataInt>($step_temp_id));
            }
            id if id == ImplDataInt::get_operative_id() => {
                $builder.incorporate($builder.clone().$action::<ImplDataInt>($step_temp_id));
            }
            id if id == ImplDataMultiOperative::get_operative_id() => {
                $builder.incorporate(
                    $builder
                        .clone()
                        .$action::<ImplDataMultiOperative>($step_temp_id),
                );
            }
            id if id == ImplDataMultiOperative::get_operative_id() => {
                $builder.incorporate(
                    $builder
                        .clone()
                        .$action::<ImplDataMultiOperative>($step_temp_id),
                );
            }
            id if id == ImplDataSingleOperative::get_operative_id() => {
                $builder.incorporate(
                    $builder
                        .clone()
                        .$action::<ImplDataSingleOperative>($step_temp_id),
                );
            }
            id if id == ImplDataString::get_operative_id() => {
                $builder.incorporate($builder.clone().$action::<ImplDataString>($step_temp_id));
            }
            id if id == ImplDataTraitOperative::get_operative_id() => {
                $builder.incorporate(
                    $builder
                        .clone()
                        .$action::<ImplDataTraitOperative>($step_temp_id),
                );
            }
            _ => {}
        };
    }};
}
macro_rules! match_impl_data_template {
    ($template_id:expr, $action:ident, $editor:expr, $builder:expr, $step_temp_id:expr) => {{
        let u128 = Uuid::parse_str(&$template_id)
            .expect("Failed to parse UUID")
            .as_u128();

        match u128 {
            id if id == ImplDataBool::get_operative_id() => {
                $editor.incorporate(&$builder.clone().$action::<ImplDataBool>($step_temp_id));
            }
            id if id == ImplDataManualBool::get_operative_id() => {
                $editor.incorporate(
                    &$builder
                        .clone()
                        .$action::<ImplDataManualBool>($step_temp_id),
                );
            }
            id if id == ImplDataCollection::get_operative_id() => {
                $editor.incorporate(
                    &$builder
                        .clone()
                        .$action::<ImplDataCollection>($step_temp_id),
                );
            }
            id if id == ImplDataInt::get_operative_id() => {
                $editor.incorporate(&$builder.clone().$action::<ImplDataInt>($step_temp_id));
            }
            id if id == ImplDataManualInt::get_operative_id() => {
                $editor.incorporate(&$builder.clone().$action::<ImplDataManualInt>($step_temp_id));
            }
            id if id == ImplDataInt::get_operative_id() => {
                $editor.incorporate(&$builder.clone().$action::<ImplDataInt>($step_temp_id));
            }
            id if id == ImplDataMultiOperative::get_operative_id() => {
                $editor.incorporate(
                    &$builder
                        .clone()
                        .$action::<ImplDataMultiOperative>($step_temp_id),
                );
            }
            id if id == ImplDataMultiOperative::get_operative_id() => {
                $editor.incorporate(
                    &$builder
                        .clone()
                        .$action::<ImplDataMultiOperative>($step_temp_id),
                );
            }
            id if id == ImplDataSingleOperative::get_operative_id() => {
                $editor.incorporate(
                    &$builder
                        .clone()
                        .$action::<ImplDataSingleOperative>($step_temp_id),
                );
            }
            id if id == ImplDataString::get_operative_id() => {
                $editor.incorporate(&$builder.clone().$action::<ImplDataString>($step_temp_id));
            }
            id if id == ImplDataManualString::get_operative_id() => {
                $editor.incorporate(
                    &$builder
                        .clone()
                        .$action::<ImplDataManualString>($step_temp_id),
                );
            }
            id if id == ImplDataTraitOperative::get_operative_id() => {
                $editor.incorporate(
                    &$builder
                        .clone()
                        .$action::<ImplDataTraitOperative>($step_temp_id),
                );
            }
            _ => {}
        };
    }};
}

pub(crate) fn constraint_template_to_canvas_template(
    operative: &base_types::constraint_schema::LibraryOperative<
        base_types::primitives::PrimitiveTypes,
        base_types::primitives::PrimitiveValues,
    >,
    keep_fields: bool,
) -> NodeTemplate {
    let operative_string_id = uuid::Uuid::from_u128(operative.tag.id).to_string();
    let template = CONSTRAINT_SCHEMA
        .template_library
        .get(&operative.template_id)
        .unwrap();
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
        template_id: operative_string_id,
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

// pub(crate) fn rgso_to_canvas_template_with_slots(
//     item: &RGSOConcrete<OperativeConcrete, Schema>,
//     schema: &RGSOConcrete<SchemaConcrete, Schema>,
// ) -> NodeTemplate {
//     let template_string_id = uuid::Uuid::from_u128(item.get_id().clone()).to_string();
//     let slot_templates = item
//         .get_roottemplate_slot()
//         .get_templateslots_slot()
//         .into_iter()
//         .map(|slot| {
//             let slot_string_id = uuid::Uuid::from_u128(slot.get_id().clone()).to_string();
//             let allowed_connections = match &slot.get_templateslotvariant_slot() {
//                 TemplateSlotTypeVariantTraitObject::TemplateSlotTypeTraitOperative(rgsoconcrete) => {
//                     // let traits = rgsoconcrete.get_allowedtraits_slot().iter().map(|trait_item| trait_item.get_id()).collect::<Vec<_>>();
//                     get_all_operatives_which_impl_trait_set(rgsoconcrete.get_allowedtraits_slot(), schema).into_iter().map(|item| item.get_name()).collect::<Vec<_>>()
//                 },
//                 TemplateSlotTypeVariantTraitObject::TemplateSlotTypeSingleOperative(rgsoconcrete) => vec![rgsoconcrete.get_allowedoperative_slot().get_name()],
//                 TemplateSlotTypeVariantTraitObject::TemplateSlotTypeMultiOperative(rgsoconcrete) => rgsoconcrete.get_allowedoperatives_slot().iter().map(|item| item.get_name()).collect::<Vec<_>>(),
//             };
//             SlotTemplate {
//                 id: slot_string_id,
//                 name: slot.get_name(),
//                 position: SlotPosition::Right,
//                 slot_type: SlotType::Outgoing,
//                 can_modify_connections: true,
//                 allowed_connections,
//                 min_connections: match slot.get_slotcardinality_slot() {
//                     TemplateSlotCardinalityVariantTraitObject::TemplateSlotCardinalityRangeOrZero(_rgsoconcrete) => 0,
//                     TemplateSlotCardinalityVariantTraitObject::TemplateSlotCardinalityLowerBoundOrZero(_rgsoconcrete) => 0,
//                     TemplateSlotCardinalityVariantTraitObject::TemplateSlotCardinalityRange(rgsoconcrete) => rgsoconcrete.get_lower_bound_field() as usize,
//                     TemplateSlotCardinalityVariantTraitObject::TemplateSlotCardinalityLowerBound(rgsoconcrete) => rgsoconcrete.get_lower_bound_field() as usize,
//                     TemplateSlotCardinalityVariantTraitObject::TemplateSlotCardinalitySingle(_rgsoconcrete) => 1,
//                 },
//                 max_connections: match slot.get_slotcardinality_slot() {
//                     TemplateSlotCardinalityVariantTraitObject::TemplateSlotCardinalityRangeOrZero(rgsoconcrete) => Some(rgsoconcrete.get_upper_bound_field() as usize),
//                     TemplateSlotCardinalityVariantTraitObject::TemplateSlotCardinalityLowerBoundOrZero(_rgsoconcrete) => None,
//                     TemplateSlotCardinalityVariantTraitObject::TemplateSlotCardinalityRange(rgsoconcrete) => Some(rgsoconcrete.get_upper_bound_field() as usize),
//                     TemplateSlotCardinalityVariantTraitObject::TemplateSlotCardinalityLowerBound(_rgsoconcrete) => None,
//                     TemplateSlotCardinalityVariantTraitObject::TemplateSlotCardinalitySingle(_rgsoconcrete) => Some(1),
//                 },
//             }
//         })
//         .collect();
//     NodeTemplate {
//         template_id: template_string_id,
//         name: item.get_name_field().clone(),
//         slot_templates,
//         ..NodeTemplate::new(&item.get_name_field())
//     }
// }

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
    leptos::logging::log!("{:#?}", execution_flow);

    let mut initial_nodes = Vec::new();

    execution_flow.nodes.values().for_each(|node| match node {
        ExecutionNode::Data(rgsoconcrete) => {
            let data_complex = generate_impldata_complex(rgsoconcrete);
            initial_nodes.extend(data_complex);
        }
        ExecutionNode::Step(impl_step_variant_trait_object) => {
            let step_operative_id = match impl_step_variant_trait_object {
                ImplStepVariantTraitObject::ImplStepBitNot(step) => step.operative().tag.id.clone(),
                ImplStepVariantTraitObject::ImplStepMathDivide(step) => {
                    step.operative().tag.id.clone()
                }
                ImplStepVariantTraitObject::ImplStepBitOr(step) => step.operative().tag.id.clone(),
                ImplStepVariantTraitObject::ImplStepCompareEqual(step) => {
                    step.operative().tag.id.clone()
                }
                ImplStepVariantTraitObject::ImplStepBitAnd(step) => step.operative().tag.id.clone(),
                ImplStepVariantTraitObject::ImplStepMathAdd(step) => {
                    step.operative().tag.id.clone()
                }
                ImplStepVariantTraitObject::ImplStepMathModulus(step) => {
                    step.operative().tag.id.clone()
                }
                ImplStepVariantTraitObject::ImplStepMathMultiply(step) => {
                    step.operative().tag.id.clone()
                }
                ImplStepVariantTraitObject::ImplStepMathSubtract(step) => {
                    step.operative().tag.id.clone()
                }
                ImplStepVariantTraitObject::ImplStepCompareGreaterThan(step) => {
                    step.operative().tag.id.clone()
                }
                ImplStepVariantTraitObject::ImplStepCompareLessThan(step) => {
                    step.operative().tag.id.clone()
                }
                ImplStepVariantTraitObject::ImplStepIf(step) => step.operative().tag.id.clone(),
                ImplStepVariantTraitObject::ImplStepIteratorFilter(step) => {
                    step.operative().tag.id.clone()
                }
                ImplStepVariantTraitObject::ImplStepMutateSlot(step) => {
                    step.operative().tag.id.clone()
                }
                ImplStepVariantTraitObject::ImplStepGetField(step) => {
                    step.operative().tag.id.clone()
                }
                ImplStepVariantTraitObject::ImplStepMutateField(step) => {
                    step.operative().tag.id.clone()
                }
                ImplStepVariantTraitObject::ImplStepMultiTypeSplitter(step) => {
                    step.operative().tag.id.clone()
                }
                ImplStepVariantTraitObject::ImplStepIteratorMap(step) => {
                    step.operative().tag.id.clone()
                }
                ImplStepVariantTraitObject::ImplStepTraverseSlot(step) => {
                    step.operative().tag.id.clone()
                }
                ImplStepVariantTraitObject::ImplStepWhileLoop(step) => {
                    step.operative().tag.id.clone()
                }
                ImplStepVariantTraitObject::ImplStepMapToOutput(step) => {
                    step.operative().tag.id.clone()
                }
                ImplStepVariantTraitObject::ImplStepMapFromInput(step) => {
                    step.operative().tag.id.clone()
                }
            };

            let mut step_node = InitialNode::new(step_operative_id.into());
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
                    let terminal_operative_id = input.operative().tag.id.clone();
                    let mut terminal_node = InitialNode::new(terminal_operative_id.into());
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
                    let terminal_operative_id = output.operative().tag.id.clone();
                    let mut terminal_node = InitialNode::new(terminal_operative_id.into());
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

    let mut data_node = InitialNode::new(impl_data_node.operative().tag.id.clone().into());
    data_node.id = Some(Uuid::from_u128(*impl_data_node.get_id()).to_string());

    let data_type = impl_data_node.get_datatype_slot();

    let additional_nodes =
        generate_impldatavariant_complex(data_type, &mut data_node, "DataType", false);

    initial_nodes.extend(additional_nodes);
    initial_nodes.push(data_node);
    initial_nodes
}

fn generate_impldatavariant_complex(
    impl_data_type: ImplDataVariantTraitObject,
    node_to_connect: &mut InitialNode,
    slot_name: &str,
    use_new_ids: bool,
) -> Vec<InitialNode> {
    let mut initial_nodes = vec![];

    match impl_data_type {
        ImplDataVariantTraitObject::ImplDataMultiOperative(datatype) => {
            let mut data_type_node = InitialNode::new(datatype.operative().tag.id.clone().into());
            data_type_node.id = if use_new_ids {
                Some(Uuid::new_v4().to_string())
            } else {
                Some(Uuid::from_u128(*datatype.get_id()).to_string())
            };
            let allowed_ops = datatype.get_allowedoperatives_slot();
            allowed_ops.iter().for_each(|allowed_op| {
                let mut op_node = InitialNode::new(allowed_op.get_id().clone().into());
                op_node.id = if use_new_ids {
                    Some(Uuid::new_v4().to_string())
                } else {
                    Some(Uuid::from_u128(*allowed_op.get_id()).to_string())
                };
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
            let mut data_type_node = InitialNode::new(datatype.operative().tag.id.clone().into());
            data_type_node.id = if use_new_ids {
                Some(Uuid::new_v4().to_string())
            } else {
                Some(Uuid::from_u128(*datatype.get_id()).to_string())
            };
            node_to_connect.initial_connections.push(InitialConnection {
                can_delete: true,
                host_slot_name: slot_name.to_string(),
                target_instance_id: data_type_node.id.clone().unwrap(),
            });
            initial_nodes.push(data_type_node);
        }
        ImplDataVariantTraitObject::ImplDataInt(datatype) => {
            let mut data_type_node = InitialNode::new(datatype.operative().tag.id.clone().into());
            data_type_node.id = if use_new_ids {
                Some(Uuid::new_v4().to_string())
            } else {
                Some(Uuid::from_u128(*datatype.get_id()).to_string())
            };
            node_to_connect.initial_connections.push(InitialConnection {
                can_delete: true,
                host_slot_name: slot_name.to_string(),
                target_instance_id: data_type_node.id.clone().unwrap(),
            });
            initial_nodes.push(data_type_node);
        }
        ImplDataVariantTraitObject::ImplDataString(datatype) => {
            let mut data_type_node = InitialNode::new(datatype.operative().tag.id.clone().into());
            data_type_node.id = if use_new_ids {
                Some(Uuid::new_v4().to_string())
            } else {
                Some(Uuid::from_u128(*datatype.get_id()).to_string())
            };
            node_to_connect.initial_connections.push(InitialConnection {
                can_delete: true,
                host_slot_name: slot_name.to_string(),
                target_instance_id: data_type_node.id.clone().unwrap(),
            });
            initial_nodes.push(data_type_node);
        }
        ImplDataVariantTraitObject::ImplDataManualInt(datatype) => {
            let mut data_type_node = InitialNode::new(datatype.operative().tag.id.clone().into());
            data_type_node.id = if use_new_ids {
                Some(Uuid::new_v4().to_string())
            } else {
                Some(Uuid::from_u128(*datatype.get_id()).to_string())
            };
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
            data_type_node.initial_field_values.push(InitialFieldValue {
                field_template_id: Uuid::from_u128(field_id).to_string(),
                value: datatype.get_value_field().to_string(),
            });
            initial_nodes.push(data_type_node);
        }
        ImplDataVariantTraitObject::ImplDataManualBool(datatype) => {
            let mut data_type_node = InitialNode::new(datatype.operative().tag.id.clone().into());
            data_type_node.id = if use_new_ids {
                Some(Uuid::new_v4().to_string())
            } else {
                Some(Uuid::from_u128(*datatype.get_id()).to_string())
            };
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
            data_type_node.initial_field_values.push(InitialFieldValue {
                field_template_id: Uuid::from_u128(field_id).to_string(),
                value: datatype.get_value_field().to_string(),
            });
            initial_nodes.push(data_type_node);
        }
        ImplDataVariantTraitObject::ImplDataCollection(datatype) => {
            let mut data_type_node = InitialNode::new(datatype.operative().tag.id.clone().into());
            data_type_node.id = if use_new_ids {
                Some(Uuid::new_v4().to_string())
            } else {
                Some(Uuid::from_u128(*datatype.get_id()).to_string())
            };
            node_to_connect.initial_connections.push(InitialConnection {
                can_delete: true,
                host_slot_name: slot_name.to_string(),
                target_instance_id: data_type_node.id.clone().unwrap(),
            });
            let next_item_nodes = generate_impldatavariant_complex(
                map_output_types_to_impldatavariant(datatype.get_collectiontype_slot()),
                &mut data_type_node,
                "CollectionType",
                use_new_ids,
            );

            initial_nodes.push(data_type_node.clone());
            initial_nodes.extend(next_item_nodes);
        }
        ImplDataVariantTraitObject::ImplDataSingleOperative(datatype) => {
            let mut data_type_node = InitialNode::new(datatype.operative().tag.id.clone().into());
            data_type_node.id = if use_new_ids {
                Some(Uuid::new_v4().to_string())
            } else {
                Some(Uuid::from_u128(*datatype.get_id()).to_string())
            };
            let allowed_op = datatype.get_allowedoperative_slot();
            let mut op_node = InitialNode::new(allowed_op.get_id().clone().into());
            op_node.id = if use_new_ids {
                Some(Uuid::new_v4().to_string())
            } else {
                Some(Uuid::from_u128(*allowed_op.get_id()).to_string())
            };
            data_type_node.initial_connections.push(InitialConnection {
                host_slot_name: "AllowedOperative".to_string(),
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
            let mut data_type_node = InitialNode::new(datatype.operative().tag.id.clone().into());
            data_type_node.id = if use_new_ids {
                Some(Uuid::new_v4().to_string())
            } else {
                Some(Uuid::from_u128(*datatype.get_id()).to_string())
            };
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
            let field_string_id = Uuid::from_u128(field_id).to_string();
            data_type_node.initial_field_values.push(InitialFieldValue {
                field_template_id: field_string_id,
                value: datatype.get_value_field(),
            });
            initial_nodes.push(data_type_node);
        }
        ImplDataVariantTraitObject::ImplDataTraitOperative(datatype) => {
            let mut data_type_node = InitialNode::new(datatype.operative().tag.id.clone().into());
            data_type_node.id = if use_new_ids {
                Some(Uuid::new_v4().to_string())
            } else {
                Some(Uuid::from_u128(*datatype.get_id()).to_string())
            };
            let required_traits = datatype.get_requiredtraits_slot();
            required_traits.iter().for_each(|required_trait| {
                let mut op_node = InitialNode::new(required_trait.get_id().clone().into());
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

    let mut function_input_node =
        InitialNode::new(function_input.operative().tag.id.clone().into());
    function_input_node.id = Some(Uuid::from_u128(*function_input.get_id()).to_string());
    let name_field_id = function_input
        .template()
        .field_constraints
        .values()
        .find(|field| field.tag.name.contains("name"))
        .unwrap()
        .tag
        .id;
    let name_field_id_string = Uuid::from_u128(name_field_id).to_string();
    function_input_node.initial_field_values = vec![InitialFieldValue {
        field_template_id: name_field_id_string,
        value: function_input.get_name_field().clone(),
    }];

    // Map input types to data types, handling the `self` case
    let fi_rgso_data_node = function_input.get_type_slot();
    if matches!(
        fi_rgso_data_node,
        FunctionInputVariantTraitObject::FunctionIOSelf(_)
    ) {
        match &fi_rgso_data_node {
            FunctionInputVariantTraitObject::FunctionIOSelf(rgsoconcrete) => {
                let mut self_input_node =
                    InitialNode::new(rgsoconcrete.operative().tag.id.clone().into());
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
            generate_impldatavariant_complex(mapped_input, &mut function_input_node, "Type", false);
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
    let mut function_output_node =
        InitialNode::new(function_output.operative().tag.id.clone().into());
    function_output_node.id = Some(Uuid::from_u128(*function_output.get_id()).to_string());
    function_output_node.id = Some(Uuid::from_u128(*function_output.get_id()).to_string());
    let name_field_id = function_output
        .template()
        .field_constraints
        .values()
        .find(|field| field.tag.name.contains("name"))
        .unwrap()
        .tag
        .id;
    let name_field_id_string = Uuid::from_u128(name_field_id).to_string();
    function_output_node.initial_field_values = vec![InitialFieldValue {
        field_template_id: name_field_id_string,
        value: function_output.get_name_field().clone(),
    }];

    // Map output types to data types, handling the `self` case
    let fo_rgso_data_node = function_output.get_type_slot();
    let mapped_input = map_output_types_to_impldatavariant(fo_rgso_data_node);
    let mut data_complex_initial_nodes =
        generate_impldatavariant_complex(mapped_input, &mut function_output_node, "Type", false);
    data_complex_initial_nodes.iter_mut().for_each(|node| {
        node.can_delete = false;
    });
    initial_nodes.push(function_output_node);
    initial_nodes.extend(data_complex_initial_nodes);
    initial_nodes
}

pub(crate) fn generate_function_input_and_mapstep_complex(
    input: &RGSOConcrete<FunctionInput, Schema>,
    impling_operative: &RGSOConcrete<OperativeConcrete, Schema>,
) -> Vec<InitialNode> {
    let mut initial_nodes = vec![];
    let function_input_id = uuid::Uuid::from_u128(*input.get_id()).to_string();
    let data_node_id = uuid::Uuid::new_v4().to_string();

    let function_input_complex = create_functioninput_complex(input.clone());
    initial_nodes.extend(function_input_complex);

    // Create map_from_input step
    let map_step_node = InitialNode {
        template_identifier: CONSTRAINT_SCHEMA
            .get_operative_by_id(&ImplStepMapFromInput::get_operative_id())
            .unwrap()
            .tag
            .id
            .into(),
        x: 0.0,
        y: 0.0,
        can_delete: false,
        can_move: true,
        initial_connections: vec![
            InitialConnection {
                host_slot_name: "Input".to_string(),
                target_instance_id: function_input_id.clone(),
                can_delete: false,
            },
            InitialConnection {
                host_slot_name: "Output".to_string(),
                target_instance_id: data_node_id.clone(),
                can_delete: false,
            },
        ],
        id: Some(Uuid::new_v4().to_string()),
        initial_field_values: vec![],
    };

    // If it is a `self` input, handle separately
    if matches!(
        input.get_type_slot(),
        FunctionInputVariantTraitObject::FunctionIOSelf(_)
    ) {
        let impl_data_node_id = data_node_id.clone();
        let data_type_node_id = Uuid::new_v4().to_string();
        let allowed_operative_node_id = Uuid::new_v4().to_string();
        let impl_data_node = InitialNode {
            template_identifier: ImplData::get_operative_id().into(),
            x: 0.0,
            y: 0.0,
            can_delete: false,
            can_move: true,
            initial_connections: vec![InitialConnection {
                host_slot_name: "DataType".to_string(),
                target_instance_id: data_type_node_id.clone(),
                can_delete: false,
            }],
            id: Some(impl_data_node_id.clone()),
            initial_field_values: vec![],
        };

        let data_type_node = InitialNode {
            template_identifier: ImplDataSingleOperative::get_operative_id().into(),
            x: 0.0,
            y: 0.0,
            can_delete: false,
            can_move: true,
            initial_connections: vec![InitialConnection {
                host_slot_name: "AllowedOperative".to_string(),
                target_instance_id: allowed_operative_node_id.clone(),
                can_delete: false,
            }],
            id: Some(data_type_node_id.clone()),
            initial_field_values: vec![],
        };
        let allowed_op = impling_operative;
        let mut op_node = InitialNode::new(allowed_op.get_id().clone().into());
        op_node.id = Some(allowed_operative_node_id.clone());

        initial_nodes.push(impl_data_node);
        initial_nodes.push(data_type_node);
        initial_nodes.push(op_node);
    } else {
        let mapped_type = map_input_types_to_impldatavariant(input.get_type_slot());

        let mut data_node = InitialNode::new(ImplData::get_operative_id().into());
        data_node.id = Some(data_node_id.clone());
        data_node.can_delete = false;

        let additional_nodes =
            generate_impldatavariant_complex(mapped_type, &mut data_node, "DataType", true);

        initial_nodes.extend(additional_nodes);
        initial_nodes.push(data_node);
    }

    initial_nodes.push(map_step_node);
    initial_nodes
}

pub(crate) fn generate_function_output_and_mapstep_complex(
    output: &RGSOConcrete<FunctionOutput, Schema>,
) -> Vec<InitialNode> {
    let mut initial_nodes = vec![];
    let function_output_id = uuid::Uuid::from_u128(*output.get_id()).to_string();
    let data_node_id = uuid::Uuid::new_v4().to_string();

    let function_output_complex = create_functionoutput_complex(output.clone());
    initial_nodes.extend(function_output_complex);

    // Create map_to_output step
    let map_step_node = InitialNode {
        template_identifier: CONSTRAINT_SCHEMA
            .get_operative_by_id(&ImplStepMapToOutput::get_operative_id())
            .unwrap()
            .tag
            .id
            .into(),
        x: 0.0,
        y: 0.0,
        can_delete: false,
        can_move: true,
        initial_connections: vec![
            InitialConnection {
                host_slot_name: "Output".to_string(),
                target_instance_id: function_output_id.clone(),
                can_delete: false,
            },
            InitialConnection {
                host_slot_name: "Input".to_string(),
                target_instance_id: data_node_id.clone(),
                can_delete: false,
            },
        ],
        id: Some(Uuid::new_v4().to_string()),
        initial_field_values: vec![],
    };

    let mapped_type = map_output_types_to_impldatavariant(output.get_type_slot());

    let mut data_node = InitialNode::new(
        CONSTRAINT_SCHEMA
            .get_operative_by_id(&ImplData::get_operative_id())
            .unwrap()
            .tag
            .id
            .into(),
    );
    data_node.id = Some(data_node_id.clone());
    data_node.can_delete = false;

    let additional_nodes =
        generate_impldatavariant_complex(mapped_type, &mut data_node, "DataType", true);

    initial_nodes.extend(additional_nodes);
    initial_nodes.push(data_node);
    initial_nodes.push(map_step_node);
    initial_nodes
}

pub(crate) fn build_schemaful_representation_of_graph(
    graph: &Graph,
    fn_def: &RGSOConcrete<FunctionDefinition, Schema>,
    operative: &RGSOConcrete<OperativeConcrete, Schema>,
    ctx: SharedGraph<Schema>,
    fn_impl_name: String,
) -> ExistingBuilder<OperativeConcrete, Schema> {
    leptos::logging::log!("Building schemaful representation of graph");
    // Map from visual node IDs to schema node temp IDs

    // Create the method implementation builder
    let mut operative_editor = operative.edit(ctx.clone());
    operative_editor.add_temp_functionimpls("new_fn_impl");
    let mut method_impl_editor = MethodImplementation::new(ctx.clone())
        .set_temp_id("new_fn_impl")
        .add_existing_definition(fn_def.get_id(), |na| na)
        .add_existing_implementor(operative.get_id(), |na| na)
        .set_name(fn_impl_name);
    operative_editor.incorporate(&method_impl_editor);
    leptos::logging::log!("Spot 1");

    // Find all function input and output nodes in the graph
    let function_input_template_id = Uuid::from_u128(FunctionInput::get_operative_id()).to_string();
    let function_output_template_id =
        Uuid::from_u128(FunctionOutput::get_operative_id()).to_string();

    // Map of node IDs to corresponding schema objects
    let mut function_input_nodes = HashMap::new();
    let mut function_output_nodes = HashMap::new();

    // Gather function input/output nodes
    for node in graph.node_instances.values() {
        if node.template_id == function_input_template_id {
            // Find the corresponding schema FunctionInput
            let schema_input = fn_def
                .get_inputs_slot()
                .iter()
                .find(|input| {
                    // Carrot!
                    Uuid::from_str(&node.instance_id).unwrap().as_u128() == *input.get_id()
                })
                .unwrap()
                .clone();
            function_input_nodes.insert(node.instance_id.clone(), schema_input);
        } else if node.template_id == function_output_template_id {
            // Find the corresponding schema FunctionOutput
            let schema_output = fn_def
                .get_outputs_slot()
                .iter()
                .find(|output| {
                    Uuid::from_str(&node.instance_id).unwrap().as_u128() == *output.get_id()
                })
                .unwrap()
                .clone();
            function_output_nodes.insert(node.instance_id.clone(), schema_output);
        }
    }

    // Process MapToOutput and MapFromInput terminals
    let map_to_output_template_id =
        Uuid::from_u128(ImplStepMapToOutput::get_operative_id()).to_string();
    let map_from_input_template_id =
        Uuid::from_u128(ImplStepMapFromInput::get_operative_id()).to_string();

    // First pass - process terminals (inputs and outputs)
    for node in graph.node_instances.values() {
        if node.template_id == map_to_output_template_id {
            leptos::logging::log!("processing output terminal");
            // This is a MapToOutput node
            let constraint_input_slot_id = get_slot_id!(ImplStepMapToOutput, "Input");
            let constraint_output_slot_id = get_slot_id!(ImplStepMapToOutput, "Output");

            let output_slot = node
                .slots
                .iter()
                .find(|s| s.slot_template_id == constraint_output_slot_id)
                .unwrap();
            let output_node_id = output_slot
                .connections
                .first()
                .unwrap()
                .target_node_id
                .clone();
            let output_node = function_output_nodes.get(&output_node_id).unwrap();

            let input_slot = node
                .slots
                .iter()
                .find(|s| s.slot_template_id == constraint_input_slot_id)
                .unwrap();
            let data_node_id = input_slot
                .connections
                .first()
                .unwrap()
                .target_node_id
                .clone();

            // Create a temp ID for the data node
            let data_temp_id = data_node_id.clone();

            // Create a temp ID for the terminal node
            let terminal_temp_id = node.instance_id.clone();

            leptos::logging::log!("about to incorporate output");
            // Add terminal to editor
            operative_editor.incorporate(&method_impl_editor.clone().add_new_maptooutputs(
                |terminal| {
                    terminal
                        .set_temp_id(&terminal_temp_id)
                        .add_existing_output(output_node.get_id(), |na| na)
                        .add_temp_input(&data_temp_id)
                },
            ));
        } else if node.template_id == map_from_input_template_id {
            leptos::logging::log!("processing input terminal");
            // This is a MapFromInput node
            let constraint_input_slot_id = get_slot_id!(ImplStepMapFromInput, "Input");
            let constraint_output_slot_id = get_slot_id!(ImplStepMapFromInput, "Output");

            let input_slot = node
                .slots
                .iter()
                .find(|s| s.slot_template_id == constraint_input_slot_id)
                .unwrap();
            let input_node_id = input_slot
                .connections
                .first()
                .unwrap()
                .target_node_id
                .clone();
            let input_node = function_input_nodes.get(&input_node_id).unwrap();

            let output_slot = node
                .slots
                .iter()
                .find(|s| s.slot_template_id == constraint_output_slot_id)
                .unwrap();
            let data_node_id = output_slot
                .connections
                .first()
                .unwrap()
                .target_node_id
                .clone();

            // Create a temp ID for the data node
            let data_temp_id = data_node_id.clone();

            // Create a temp ID for the terminal node
            let terminal_temp_id = node.instance_id.clone();

            leptos::logging::log!("about to incorporate input");
            // Add terminal to editor

            operative_editor.incorporate(&method_impl_editor.clone().add_new_mapfrominputs(
                |terminal| {
                    terminal
                        .set_temp_id(&terminal_temp_id)
                        .add_existing_input(input_node.get_id(), |na| na)
                        .add_temp_output(&data_temp_id)
                },
            ));
        }
    }
    leptos::logging::log!("Step 1 finished: Processing terminals");

    // Second pass - process all step nodes (that aren't terminals)
    for node in graph.node_instances.values() {
        // Skip if this is a terminal node
        if node.template_id == map_to_output_template_id
            || node.template_id == map_from_input_template_id
        {
            continue;
        }

        // Skip if not a step node
        if !is_impl_step_template_id(&node.template_id) {
            continue;
        }

        // Generate a unique temp ID for this step
        let step_temp_id = node.instance_id.clone();

        // Collect data dependencies
        let mut data_deps = HashMap::new();
        for slot in &node.slots {
            if slot.connections.is_empty() {
                continue;
            }

            let slot_name = get_slot_name_from_id(&slot.slot_template_id, node, graph);
            let target_node_id = &slot.connections[0].target_node_id;

            data_deps.insert(slot_name.to_string(), target_node_id.clone());
        }

        // ImplStepBitNot
        if node.template_id == Uuid::from_u128(ImplStepBitNot::get_operative_id()).to_string() {
            let input_id = data_deps.get("InputBool").unwrap();
            let output_id = data_deps.get("OutputBool").unwrap();
            let builder = ImplStepBitNot::new(ctx.clone())
                .set_temp_id(&step_temp_id)
                .add_temp_inputbool(input_id)
                .add_temp_outputbool(output_id);
            operative_editor.incorporate(&builder);
        }
        // ImplStepMathDivide
        else if node.template_id
            == Uuid::from_u128(ImplStepMathDivide::get_operative_id()).to_string()
        {
            let arg1_id = data_deps.get("ArgumentOne").unwrap();
            let arg2_id = data_deps.get("ArgumentTwo").unwrap();
            let output_id = data_deps.get("OutputInt").unwrap();
            let builder = ImplStepMathDivide::new(ctx.clone())
                .set_temp_id(&step_temp_id)
                .add_temp_argumentone(arg1_id)
                .add_temp_argumenttwo(arg2_id)
                .add_temp_outputint(output_id);
            operative_editor.incorporate(&builder);
        }
        // ImplStepBitOr
        else if node.template_id == Uuid::from_u128(ImplStepBitOr::get_operative_id()).to_string()
        {
            let arg1_id = data_deps.get("ArgumentOne").unwrap();
            let arg2_id = data_deps.get("ArgumentTwo").unwrap();
            let output_id = data_deps.get("OutputBool").unwrap();
            let builder = ImplStepBitOr::new(ctx.clone())
                .set_temp_id(&step_temp_id)
                .add_temp_argumentone(arg1_id)
                .add_temp_argumenttwo(arg2_id)
                .add_temp_outputbool(output_id);
            operative_editor.incorporate(&builder);
        }
        // ImplStepCompareEqual
        else if node.template_id
            == Uuid::from_u128(ImplStepCompareEqual::get_operative_id()).to_string()
        {
            let arg1_id = data_deps.get("ArgumentOne").unwrap();
            let arg2_id = data_deps.get("ArgumentTwo").unwrap();
            let output_id = data_deps.get("OutputBool").unwrap();
            let builder = ImplStepCompareEqual::new(ctx.clone())
                .set_temp_id(&step_temp_id)
                .add_temp_argumentone(arg1_id)
                .add_temp_argumenttwo(arg2_id)
                .add_temp_outputbool(output_id);
            operative_editor.incorporate(&builder);
        }
        // ImplStepBitAnd
        else if node.template_id
            == Uuid::from_u128(ImplStepBitAnd::get_operative_id()).to_string()
        {
            let arg1_id = data_deps.get("ArgumentOne").unwrap();
            let arg2_id = data_deps.get("ArgumentTwo").unwrap();
            let output_id = data_deps.get("OutputBool").unwrap();
            let builder = ImplStepBitAnd::new(ctx.clone())
                .set_temp_id(&step_temp_id)
                .add_temp_argumentone(arg1_id)
                .add_temp_argumenttwo(arg2_id)
                .add_temp_outputbool(output_id);
            operative_editor.incorporate(&builder);
        }
        // ImplStepMathAdd
        else if node.template_id
            == Uuid::from_u128(ImplStepMathAdd::get_operative_id()).to_string()
        {
            let arg1_id = data_deps.get("ArgumentOne").unwrap();
            let arg2_id = data_deps.get("ArgumentTwo").unwrap();
            let output_id = data_deps.get("OutputInt").unwrap();
            let builder = ImplStepMathAdd::new(ctx.clone())
                .set_temp_id(&step_temp_id)
                .add_temp_argumentone(arg1_id)
                .add_temp_argumenttwo(arg2_id)
                .add_temp_outputint(output_id);
            operative_editor.incorporate(&builder);
        }
        // ImplStepMathModulus
        else if node.template_id
            == Uuid::from_u128(ImplStepMathModulus::get_operative_id()).to_string()
        {
            let arg1_id = data_deps.get("ArgumentOne").unwrap();
            let arg2_id = data_deps.get("ArgumentTwo").unwrap();
            let output_id = data_deps.get("OutputInt").unwrap();
            let builder = ImplStepMathModulus::new(ctx.clone())
                .set_temp_id(&step_temp_id)
                .add_temp_argumentone(arg1_id)
                .add_temp_argumenttwo(arg2_id)
                .add_temp_outputint(output_id);
            operative_editor.incorporate(&builder);
        }
        // ImplStepMathMultiply
        else if node.template_id
            == Uuid::from_u128(ImplStepMathMultiply::get_operative_id()).to_string()
        {
            let arg1_id = data_deps.get("ArgumentOne").unwrap();
            let arg2_id = data_deps.get("ArgumentTwo").unwrap();
            let output_id = data_deps.get("OutputInt").unwrap();
            let builder = ImplStepMathMultiply::new(ctx.clone())
                .set_temp_id(&step_temp_id)
                .add_temp_argumentone(arg1_id)
                .add_temp_argumenttwo(arg2_id)
                .add_temp_outputint(output_id);
            operative_editor.incorporate(&builder);
        }
        // ImplStepMathSubtract
        else if node.template_id
            == Uuid::from_u128(ImplStepMathSubtract::get_operative_id()).to_string()
        {
            let arg1_id = data_deps.get("ArgumentOne").unwrap();
            let arg2_id = data_deps.get("ArgumentTwo").unwrap();
            let output_id = data_deps.get("OutputInt").unwrap();
            let builder = ImplStepMathSubtract::new(ctx.clone())
                .set_temp_id(&step_temp_id)
                .add_temp_argumentone(arg1_id)
                .add_temp_argumenttwo(arg2_id)
                .add_temp_outputint(output_id);
            operative_editor.incorporate(&builder);
        }
        // ImplStepCompareGreaterThan
        else if node.template_id
            == Uuid::from_u128(ImplStepCompareGreaterThan::get_operative_id()).to_string()
        {
            let arg1_id = data_deps.get("ArgumentOne").unwrap();
            let arg2_id = data_deps.get("ArgumentTwo").unwrap();
            let output_id = data_deps.get("OutputBool").unwrap();
            let builder = ImplStepCompareGreaterThan::new(ctx.clone())
                .set_temp_id(&step_temp_id)
                .add_temp_argumentone(arg1_id)
                .add_temp_argumenttwo(arg2_id)
                .add_temp_outputbool(output_id);
            operative_editor.incorporate(&builder);
        }
        // ImplStepCompareLessThan
        else if node.template_id
            == Uuid::from_u128(ImplStepCompareLessThan::get_operative_id()).to_string()
        {
            let arg1_id = data_deps.get("ArgumentOne").unwrap();
            let arg2_id = data_deps.get("ArgumentTwo").unwrap();
            let output_id = data_deps.get("OutputBool").unwrap();
            let builder = ImplStepCompareLessThan::new(ctx.clone())
                .set_temp_id(&step_temp_id)
                .add_temp_argumentone(arg1_id)
                .add_temp_argumenttwo(arg2_id)
                .add_temp_outputbool(output_id);
            operative_editor.incorporate(&builder);
        }
        // ImplStepIf
        else if node.template_id == Uuid::from_u128(ImplStepIf::get_operative_id()).to_string() {
            let condition_id = data_deps.get("Condition").unwrap();
            let true_branch_id = data_deps.get("TrueBranch").unwrap();
            let false_branch_id = data_deps.get("FalseBranch").unwrap();
            let output_id = data_deps.get("Output").unwrap_or(&step_temp_id); // Optional output
            let mut builder = ImplStepIf::new(ctx.clone())
                .set_temp_id(&step_temp_id)
                .add_temp_condition(condition_id)
                .add_temp_truebranch(true_branch_id)
                .add_temp_falsebranch(false_branch_id);

            // Add output if available
            if data_deps.contains_key("Output") {
                builder.incorporate(builder.clone().add_temp_output(output_id));
            }
            operative_editor.incorporate(&builder);
        }
        // ImplStepIteratorFilter
        else if node.template_id
            == Uuid::from_u128(ImplStepIteratorFilter::get_operative_id()).to_string()
        {
            let input_collection_id = data_deps.get("InputCollection").unwrap();
            let iteration_start_item_id = data_deps.get("IterationStartItem").unwrap();
            let iteration_end_bool_id = data_deps.get("IterationEndBool").unwrap();
            let output_collection_id = data_deps.get("OutputCollection").unwrap();
            let builder = ImplStepIteratorFilter::new(ctx.clone())
                .set_temp_id(&step_temp_id)
                .add_temp_inputcollection(input_collection_id)
                .add_temp_iterationstartitem(iteration_start_item_id)
                .add_temp_iterationendbool(iteration_end_bool_id)
                .add_temp_outputcollection(output_collection_id);
            operative_editor.incorporate(&builder);
        }
        // ImplStepMutateSlot
        else if node.template_id
            == Uuid::from_u128(ImplStepMutateSlot::get_operative_id()).to_string()
        {
            let mutated_operative_id = data_deps.get("MutatedOperative").unwrap();
            let slot_name_id = data_deps.get("SlotName").unwrap();
            let new_value_id = data_deps.get("NewValue").unwrap();
            let builder = ImplStepMutateSlot::new(ctx.clone())
                .set_temp_id(&step_temp_id)
                .add_temp_mutatedoperative(mutated_operative_id)
                .add_temp_slotname(slot_name_id)
                .add_temp_newvalue(new_value_id);
            operative_editor.incorporate(&builder);
        }
        // ImplStepGetField
        else if node.template_id
            == Uuid::from_u128(ImplStepGetField::get_operative_id()).to_string()
        {
            let input_operative_id = data_deps.get("InputOperative").unwrap();
            let field_name_id = data_deps.get("FieldName").unwrap();
            let output_field_id = data_deps.get("OutputField").unwrap();
            let builder = ImplStepGetField::new(ctx.clone())
                .set_temp_id(&step_temp_id)
                .add_temp_inputoperative(input_operative_id)
                .add_temp_fieldname(field_name_id)
                .add_temp_outputfield(output_field_id);
            operative_editor.incorporate(&builder);
        }
        // ImplStepMutateField
        else if node.template_id
            == Uuid::from_u128(ImplStepMutateField::get_operative_id()).to_string()
        {
            let mutated_operative_id = data_deps.get("MutatedOperative").unwrap();
            let field_name_id = data_deps.get("FieldName").unwrap();
            let new_value_id = data_deps.get("NewValue").unwrap();
            let builder = ImplStepMutateField::new(ctx.clone())
                .set_temp_id(&step_temp_id)
                .add_temp_mutatedoperative(mutated_operative_id)
                .add_temp_fieldname(field_name_id)
                .add_temp_newvalue(new_value_id);
            operative_editor.incorporate(&builder);
        }
        // ImplStepMultiTypeSplitter
        else if node.template_id
            == Uuid::from_u128(ImplStepMultiTypeSplitter::get_operative_id()).to_string()
        {
            let input_multi_operative_id = data_deps.get("InputMultiOperative").unwrap();
            let convergence_id = data_deps.get("Convergence").unwrap();
            let discriminant_starts_id = data_deps.get("DiscriminantStarts").unwrap();
            let output_id = data_deps.get("Output").unwrap();
            let builder = ImplStepMultiTypeSplitter::new(ctx.clone())
                .set_temp_id(&step_temp_id)
                .add_temp_inputmultioperative(input_multi_operative_id)
                .add_temp_convergence(convergence_id)
                .add_temp_discriminantstarts(discriminant_starts_id)
                .add_temp_output(output_id);
            operative_editor.incorporate(&builder);
        }
        // ImplStepIteratorMap
        else if node.template_id
            == Uuid::from_u128(ImplStepIteratorMap::get_operative_id()).to_string()
        {
            let input_collection_id = data_deps.get("InputCollection").unwrap();
            let iteration_start_item_id = data_deps.get("IterationStartItem").unwrap();
            let iteration_end_item_id = data_deps.get("IterationEndItem").unwrap();
            let output_collection_id = data_deps.get("OutputCollection").unwrap();
            let builder = ImplStepIteratorMap::new(ctx.clone())
                .set_temp_id(&step_temp_id)
                .add_temp_inputcollection(input_collection_id)
                .add_temp_iterationstartitem(iteration_start_item_id)
                .add_temp_iterationenditem(iteration_end_item_id)
                .add_temp_outputcollection(output_collection_id);
            operative_editor.incorporate(&builder);
        }
        // ImplStepTraverseSlot
        else if node.template_id
            == Uuid::from_u128(ImplStepTraverseSlot::get_operative_id()).to_string()
        {
            let input_operative_id = data_deps.get("InputOperative").unwrap();
            let slot_name_id = data_deps.get("SlotName").unwrap();
            let output_operatives_id = data_deps.get("OutputOperatives").unwrap();
            let builder = ImplStepTraverseSlot::new(ctx.clone())
                .set_temp_id(&step_temp_id)
                .add_temp_inputoperative(input_operative_id)
                .add_temp_slotname(slot_name_id)
                .add_temp_outputoperatives(output_operatives_id);
            operative_editor.incorporate(&builder);
        }
        // ImplStepWhileLoop
        else if node.template_id
            == Uuid::from_u128(ImplStepWhileLoop::get_operative_id()).to_string()
        {
            // While loop has special handling that would depend on your specific implementation
            let builder = ImplStepWhileLoop::new(ctx.clone()).set_temp_id(&step_temp_id);
            operative_editor.incorporate(&builder);
        }
    }

    leptos::logging::log!("Step 2 finished: Processing Steps");

    // Third pass - process all data nodes and their types recursively
    let data_template_id = Uuid::from_u128(ImplData::get_operative_id()).to_string();

    // Process ImplData nodes
    let mut processed_data_nodes = HashSet::new();

    // Now process all data nodes
    for node in graph.node_instances.values() {
        if node.template_id == data_template_id {
            process_data_node(
                &node.instance_id,
                graph,
                &ctx,
                &mut operative_editor,
                &mut processed_data_nodes,
            );
        }
    }
    leptos::logging::log!("Step 3 finished: Processing Data Nodes");

    operative_editor
}

// Helper function to check if a template ID belongs to an ImplStep
fn is_impl_step_template_id(template_id: &str) -> bool {
    // Create a set of ImplStep template IDs
    let step_ids = [
        ImplStepBitNot::get_operative_id(),
        ImplStepMathDivide::get_operative_id(),
        ImplStepBitOr::get_operative_id(),
        ImplStepCompareEqual::get_operative_id(),
        ImplStepBitAnd::get_operative_id(),
        ImplStepMathAdd::get_operative_id(),
        ImplStepMathModulus::get_operative_id(),
        ImplStepMathMultiply::get_operative_id(),
        ImplStepMathSubtract::get_operative_id(),
        ImplStepCompareGreaterThan::get_operative_id(),
        ImplStepCompareLessThan::get_operative_id(),
        ImplStepIf::get_operative_id(),
        ImplStepIteratorFilter::get_operative_id(),
        ImplStepMutateSlot::get_operative_id(),
        ImplStepGetField::get_operative_id(),
        ImplStepMutateField::get_operative_id(),
        ImplStepMultiTypeSplitter::get_operative_id(),
        ImplStepIteratorMap::get_operative_id(),
        ImplStepTraverseSlot::get_operative_id(),
        ImplStepWhileLoop::get_operative_id(),
        ImplStepMapToOutput::get_operative_id(),
        ImplStepMapFromInput::get_operative_id(),
    ];

    step_ids
        .iter()
        .any(|&id| template_id == Uuid::from_u128(id).to_string())
}

// Helper function to get slot name from slot template ID
fn get_slot_name_from_id(slot_template_id: &str, node: &NodeInstance, graph: &Graph) -> String {
    let caps = graph.get_node_capabilities(&node.instance_id).unwrap();
    caps.template
        .slot_templates
        .iter()
        .find(|slot_template| slot_template.id == slot_template_id)
        .unwrap()
        .name
        .clone()
}

// Helper function to recursively process data nodes
fn process_data_node(
    node_id: &str,
    graph: &Graph,
    ctx: &SharedGraph<Schema>,
    editor: &mut ExistingBuilder<OperativeConcrete, Schema>,
    processed_data_nodes: &mut HashSet<String>,
) {
    leptos::logging::log!("Processing a data node");
    // Skip if already processed
    if processed_data_nodes.contains(node_id) {
        return;
    }

    let node = match graph.node_instances.get(node_id) {
        Some(n) => n,
        None => return, // Node doesn't exist
    };

    let temp_id = node_id.clone();
    // Mark as processed
    processed_data_nodes.insert(node_id.to_string());

    let constraint_datatype_slot_id = get_slot_id!(ImplData, "DataType");
    // Find the data type node
    let data_type_slot = node
        .slots
        .iter()
        .find(|s| s.slot_template_id == constraint_datatype_slot_id);
    let impl_data_builder = ImplData::new(ctx.clone()).set_temp_id(&temp_id);

    if let Some(slot) = data_type_slot {
        if slot.connections.is_empty() {
            return; // No data type connected
        }

        let data_type_node_id = &slot.connections[0].target_node_id;
        let data_type_node = match graph.node_instances.get(data_type_node_id) {
            Some(n) => n,
            None => return, // Data type node doesn't exist
        };

        let data_type_template_id = data_type_node.template_id.clone();
        match_impl_data_template!(
            data_type_template_id,
            add_temp_datatype,
            editor,
            impl_data_builder,
            data_type_node_id
        );
        // Process based on data type
        process_data_type(data_type_node, graph, ctx, editor, processed_data_nodes);
    }

    // Find upstream and downstream steps
    let upstream_steps = find_upstream_steps(node_id, graph);
    let downstream_steps = find_downstream_steps(node_id, graph);

    // Apply the upstream and downstream connections
    leptos::logging::log!("About to add upstream steps");
    // Add upstream steps
    for (step_temp_id, step_viz_template_id) in upstream_steps {
        match_impl_step_template!(
            step_viz_template_id,
            add_temp_upstreamstep,
            editor,
            impl_data_builder,
            &step_temp_id
        );
    }

    leptos::logging::log!("About to add downstream steps");
    // Add downstream steps
    for (step_temp_id, step_viz_template_id) in downstream_steps {
        match_impl_step_template!(
            step_viz_template_id,
            add_temp_downstreamsteps,
            editor,
            impl_data_builder,
            &step_temp_id
        );
    }
}

// Helper function to find upstream step IDs
fn find_upstream_steps(
    data_node_id: &str,
    graph: &Graph,
    // (temp_id, viz_node_template_id)
) -> Vec<(String, String)> {
    let mut upstream_steps = Vec::new();

    // Find all step nodes that connect their output to this data node
    for node in graph.node_instances.values() {
        // Skip if not a step node
        if !is_impl_step_template_id(&node.template_id) {
            continue;
        }

        // Check if any output slot connects to our data node
        for slot in &node
            .slots
            .iter()
            .filter_map(|slot| {
                if is_step_output_slot(slot, graph) {
                    Some(slot)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
        {
            for connection in &slot.connections {
                if connection.target_node_id == data_node_id {
                    upstream_steps.push((node.instance_id.clone(), node.template_id.clone()));
                    break;
                }
            }
        }
    }

    upstream_steps
}

// Helper function to find downstream step IDs
fn find_downstream_steps(
    data_node_id: &str,
    graph: &Graph,
    // (temp_id, viz_node_template_id)
) -> Vec<(String, String)> {
    let mut downstream_steps = Vec::new();

    // Find all step nodes that connect their output to this data node
    for node in graph.node_instances.values() {
        // Skip if not a step node
        if !is_impl_step_template_id(&node.template_id) {
            continue;
        }

        // Check if any output slot connects to our data node
        for slot in &node
            .slots
            .iter()
            .filter_map(|slot| {
                if is_step_input_slot(slot, graph) {
                    Some(slot)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
        {
            for connection in &slot.connections {
                if connection.target_node_id == data_node_id {
                    downstream_steps.push((node.instance_id.clone(), node.template_id.clone()));
                    break;
                }
            }
        }
    }

    downstream_steps
}

// Helper function to process data type node
fn process_data_type(
    data_type_node: &NodeInstance,
    graph: &Graph,
    ctx: &SharedGraph<Schema>,
    editor: &mut ExistingBuilder<OperativeConcrete, Schema>,
    processed_data_nodes: &mut HashSet<String>,
) {
    leptos::logging::log!("Processing a data type");
    let temp_id = &data_type_node.instance_id;
    // ImplDataBool
    if data_type_node.template_id == Uuid::from_u128(ImplDataBool::get_operative_id()).to_string() {
        editor.incorporate(&ImplDataBool::new(ctx.clone()).set_temp_id(temp_id));
    }
    // ImplDataInt
    else if data_type_node.template_id
        == Uuid::from_u128(ImplDataInt::get_operative_id()).to_string()
    {
        editor.incorporate(&ImplDataInt::new(ctx.clone()).set_temp_id(temp_id));
    }
    // ImplDataString
    else if data_type_node.template_id
        == Uuid::from_u128(ImplDataString::get_operative_id()).to_string()
    {
        editor.incorporate(&ImplDataString::new(ctx.clone()).set_temp_id(temp_id));
    }
    // ImplDataManualBool
    else if data_type_node.template_id
        == Uuid::from_u128(ImplDataManualBool::get_operative_id()).to_string()
    {
        let constraint_field_value_id = get_field_id!(ImplDataManualBool, "value");
        let value = data_type_node
            .fields
            .iter()
            .find(|f| f.field_template_id == constraint_field_value_id)
            .map(|f| f.value == "true")
            .unwrap_or(false);

        editor.incorporate(
            &ImplDataManualBool::new(ctx.clone())
                .set_temp_id(temp_id)
                .set_value(value),
        );
    }
    // ImplDataManualInt
    else if data_type_node.template_id
        == Uuid::from_u128(ImplDataManualInt::get_operative_id()).to_string()
    {
        let constraint_field_value_id = get_field_id!(ImplDataManualInt, "value");
        let value = data_type_node
            .fields
            .iter()
            .find(|f| f.field_template_id == constraint_field_value_id)
            .map(|f| f.value.parse::<u32>().unwrap_or(0))
            .unwrap_or(0);

        editor.incorporate(
            &ImplDataManualInt::new(ctx.clone())
                .set_temp_id(temp_id)
                .set_value(value),
        );
    }
    // ImplDataManualString
    else if data_type_node.template_id
        == Uuid::from_u128(ImplDataManualString::get_operative_id()).to_string()
    {
        let constraint_field_value_id = get_field_id!(ImplDataManualString, "value");
        let value = data_type_node
            .fields
            .iter()
            .find(|f| f.field_template_id == constraint_field_value_id)
            .map(|f| f.value.clone())
            .unwrap_or_default();

        editor.incorporate(
            &ImplDataManualString::new(ctx.clone())
                .set_temp_id(temp_id)
                .set_value(value),
        );
    }
    // ImplDataCollection (recursive case)
    else if data_type_node.template_id
        == Uuid::from_u128(ImplDataCollection::get_operative_id()).to_string()
    {
        let constraint_collection_type_id = get_slot_id!(ImplDataCollection, "CollectionType");
        let collection_type_slot = data_type_node
            .slots
            .iter()
            .find(|s| s.slot_template_id == constraint_collection_type_id);

        if let Some(slot) = collection_type_slot {
            if slot.connections.is_empty() {
                return; // No collection type connected
            }

            let collection_type_node_id = &slot.connections[0].target_node_id;
            let collection_type_node = match graph.node_instances.get(collection_type_node_id) {
                Some(n) => n,
                None => return, // Collection type node doesn't exist
            };

            let collection_type_temp_id = collection_type_node.instance_id.clone();

            let mut collection_builder = ImplDataCollection::new(ctx.clone()).set_temp_id(temp_id);

            process_data_type(
                collection_type_node,
                graph,
                ctx,
                editor,
                processed_data_nodes,
            );

            let node_template_id = data_type_node.template_id.clone();
            match_impl_data_minus_manuals_template!(
                node_template_id,
                add_temp_collectiontype,
                collection_builder,
                &collection_type_temp_id
            );
            editor.incorporate(&collection_builder.clone());
        }
    }
    // ImplDataSingleOperative
    else if data_type_node.template_id
        == Uuid::from_u128(ImplDataSingleOperative::get_operative_id()).to_string()
    {
        let constraint_allowed_operative_id =
            get_slot_id!(ImplDataSingleOperative, "AllowedOperative");
        let allowed_op_slot = data_type_node
            .slots
            .iter()
            .find(|s| s.slot_template_id == constraint_allowed_operative_id);

        if let Some(slot) = allowed_op_slot {
            if slot.connections.is_empty() {
                return; // No allowed operative connected
            }

            let allowed_op_node_id = &slot.connections[0].target_node_id;
            let allowed_op_node = match graph.node_instances.get(allowed_op_node_id) {
                Some(n) => n,
                None => return, // Allowed operative node doesn't exist
            };

            // Get the operative ID from the template
            let operative_id = Uuid::parse_str(&allowed_op_node.template_id)
                .unwrap_or(Uuid::nil())
                .as_u128();

            editor.incorporate(
                &ImplDataSingleOperative::new(ctx.clone())
                    .set_temp_id(temp_id)
                    .add_existing_allowedoperative(&operative_id, |na| na),
            );
        }
    }
    // ImplDataMultiOperative
    else if data_type_node.template_id
        == Uuid::from_u128(ImplDataMultiOperative::get_operative_id()).to_string()
    {
        let constraint_allowed_operatives_id =
            get_slot_id!(ImplDataMultiOperative, "AllowedOperatives");
        let allowed_ops_slot = data_type_node
            .slots
            .iter()
            .find(|s| s.slot_template_id == constraint_allowed_operatives_id);

        if let Some(slot) = allowed_ops_slot {
            let impl_data_multi_operative =
                ImplDataMultiOperative::new(ctx.clone()).set_temp_id(temp_id);
            // Add each connected operative
            for connection in &slot.connections {
                let allowed_op_node_id = &connection.target_node_id;
                let allowed_op_node = match graph.node_instances.get(allowed_op_node_id) {
                    Some(n) => n,
                    None => continue, // Skip if node doesn't exist
                };

                // Get the operative ID from the template
                let operative_id = Uuid::parse_str(&allowed_op_node.template_id)
                    .unwrap_or(Uuid::nil())
                    .as_u128();

                editor.incorporate(
                    &impl_data_multi_operative
                        .clone()
                        .add_existing_allowedoperatives(&operative_id, |na| na),
                );
            }
        }
    }
    // ImplDataTraitOperative
    else if data_type_node.template_id
        == Uuid::from_u128(ImplDataTraitOperative::get_operative_id()).to_string()
    {
        let constraint_required_traits_id = get_slot_id!(ImplDataTraitOperative, "RequiredTraits");
        let required_traits_slot = data_type_node
            .slots
            .iter()
            .find(|s| s.slot_template_id == constraint_required_traits_id);

        if let Some(slot) = required_traits_slot {
            let todt = ImplDataTraitOperative::new(ctx.clone()).set_temp_id(temp_id);

            // Add each connected trait
            for connection in &slot.connections {
                let trait_node_id = &connection.target_node_id;
                let trait_node = match graph.node_instances.get(trait_node_id) {
                    Some(n) => n,
                    None => continue, // Skip if node doesn't exist
                };

                // Get the trait ID from the template
                let trait_id = Uuid::parse_str(&trait_node.template_id)
                    .unwrap_or(Uuid::nil())
                    .as_u128();

                editor.incorporate(&todt.clone().add_existing_requiredtraits(&trait_id, |na| na));
            }
        }
    }
}

fn is_step_output_slot(slot: &SlotInstance, graph: &Graph) -> bool {
    let caps = graph.get_node_capabilities(&slot.node_instance_id).unwrap();
    let slot_template = caps
        .template
        .slot_templates
        .iter()
        .find(|slot_template| slot_template.id == slot.slot_template_id)
        .unwrap();
    slot_template.name.contains("Output")
}

fn is_step_input_slot(slot: &SlotInstance, graph: &Graph) -> bool {
    if is_step_output_slot(slot, graph) {
        return false;
    }
    let caps = graph.get_node_capabilities(&slot.node_instance_id).unwrap();
    let slot_template = caps
        .template
        .slot_templates
        .iter()
        .find(|slot_template| slot_template.id == slot.slot_template_id)
        .unwrap();
    let name = &slot_template.name;
    if name.contains("IterationStartItem")
        || name.contains("IterationEndBool")
        || name.contains("IterationEndItem")
        || name.contains("Convergence")
        || name.contains("DiscriminantStarts")
        || name.contains("ContinueWhileBool")
        || name.contains("LoopExitState")
        || name.contains("LoopStateIngestor")
        || name.contains("LoopStateStub")
    {
        return false;
    }
    true
}
