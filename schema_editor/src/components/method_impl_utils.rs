use graph_canvas::{prelude::*, FieldTemplate, FieldType};
use graph_canvas::{NodeTemplate, SlotPosition, SlotType};
use schema_editor_generated_toolkit::prelude::*;

use super::utils::get_all_operatives_which_impl_trait_set;

pub(crate) fn constraint_template_to_canvas_template(
    template: &base_types::constraint_schema::LibraryTemplate<
        base_types::primitives::PrimitiveTypes,
        base_types::primitives::PrimitiveValues,
    >,
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
    let field_templates = template.field_constraints.values().map(|field| {
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
    }).collect::<Vec<_>>();
    NodeTemplate {
        template_id: template_string_id,
        name: template.tag.name.clone(),
        field_templates,
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
