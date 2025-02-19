use std::collections::BTreeSet;

use graph_canvas::prelude::*;
use schema_editor_generated_toolkit::prelude::*;

pub fn get_all_descendent_operators(
    op: RGSOConcrete<OperativeConcrete, Schema>,
    agg: &mut BTreeSet<RGSOConcrete<OperativeConcrete, Schema>>,
) -> &mut BTreeSet<RGSOConcrete<OperativeConcrete, Schema>> {
    op.get_childrenoperatives_slot()
        .into_iter()
        .fold(agg, |mut agg, child| {
            agg.insert(child.clone());
            get_all_descendent_operators(child, &mut agg);
            agg
        })
}
pub fn get_all_descendent_operators_including_own(
    op: RGSOConcrete<OperativeConcrete, Schema>,
    agg: &mut BTreeSet<RGSOConcrete<OperativeConcrete, Schema>>,
) -> &mut BTreeSet<RGSOConcrete<OperativeConcrete, Schema>> {
    let mut ops_to_check = op.get_childrenoperatives_slot();
    ops_to_check.push(op);
    ops_to_check.into_iter().fold(agg, |mut agg, child| {
        agg.insert(child.clone());
        get_all_descendent_operators(child, &mut agg);
        agg
    })
}

pub fn get_all_descendent_instances(
    op: RGSOConcrete<OperativeConcrete, Schema>,
    schema_concrete: &RGSOConcrete<SchemaConcrete, Schema>,
) -> BTreeSet<RGSOConcrete<InstanceConcrete, Schema>> {
    let mut return_set = BTreeSet::new();
    get_all_descendent_operators(op, &mut return_set);
    let all_instances = schema_concrete.get_instances_slot();
    return_set
        .into_iter()
        .fold(BTreeSet::new(), |mut agg, descendent| {
            agg.extend(
                all_instances
                    .iter()
                    .filter(|instance| {
                        instance.get_parentoperative_slot().get_id() == descendent.get_id()
                    })
                    .cloned(),
            );
            agg
        })
}

pub fn get_all_descendent_instances_including_own(
    op: RGSOConcrete<OperativeConcrete, Schema>,
    schema_concrete: &RGSOConcrete<SchemaConcrete, Schema>,
) -> BTreeSet<RGSOConcrete<InstanceConcrete, Schema>> {
    let mut return_set = BTreeSet::new();
    return_set.insert(op.clone());
    get_all_descendent_operators_including_own(op, &mut return_set);
    let all_instances = schema_concrete.get_instances_slot();
    return_set
        .into_iter()
        .fold(BTreeSet::new(), |mut agg, descendent| {
            agg.extend(
                all_instances
                    .iter()
                    .filter(|instance| {
                        instance.get_parentoperative_slot().get_id() == descendent.get_id()
                    })
                    .cloned(),
            );
            agg
        })
}
pub fn get_all_operatives_which_impl_trait_set(
    traits: Vec<RGSOConcrete<TraitConcrete, Schema>>,
    schema_concrete: &RGSOConcrete<SchemaConcrete, Schema>,
) -> BTreeSet<RGSOConcrete<OperativeConcrete, Schema>> {
    // let all_traits = schema_concrete.get_traits_slot();
    let all_ops = schema_concrete.get_operatives_slot();
    all_ops
        .into_iter()
        .filter(|op| {
            traits
                .iter()
                .all(|inner_trait| op.get_traitimpls_slot().contains(inner_trait))
        })
        .collect()
}

pub fn get_all_instances_which_impl_trait_set(
    traits: Vec<RGSOConcrete<TraitConcrete, Schema>>,
    schema_concrete: &RGSOConcrete<SchemaConcrete, Schema>,
) -> BTreeSet<RGSOConcrete<InstanceConcrete, Schema>> {
    let ops_which_impl = get_all_operatives_which_impl_trait_set(traits, schema_concrete);
    let instances_which_impl = ops_which_impl
        .into_iter()
        .fold(BTreeSet::new(), |mut agg, op| {
            agg.extend(get_all_descendent_instances_including_own(
                op,
                schema_concrete,
            ));
            agg
        });
    instances_which_impl
}

pub fn get_all_operatives_which_satisfy_specializable(
    schema_concrete: &RGSOConcrete<SchemaConcrete, Schema>,
    specializable: OperativeSlotTypeSpecializableTraitObject,
) -> BTreeSet<RGSOConcrete<OperativeConcrete, Schema>> {
    match specializable {
        OperativeSlotTypeSpecializableTraitObject::TemplateSlotTypeTraitOperative(trait_op) => {
            get_all_operatives_which_impl_trait_set(
                trait_op.get_allowedtraits_slot(),
                schema_concrete,
            )
        }
        OperativeSlotTypeSpecializableTraitObject::OperativeSlotTypeMultiSpecialization(multi) => {
            let mut return_list = BTreeSet::new();
            let allowed = multi
                .get_allowedoperatives_slot()
                .into_iter()
                .collect::<BTreeSet<_>>();
            allowed.into_iter().for_each(|allowed_op| {
                get_all_descendent_operators_including_own(allowed_op, &mut return_list);
            });
            return_list
        }
        OperativeSlotTypeSpecializableTraitObject::TemplateSlotTypeMultiOperative(multi) => {
            let mut return_list = BTreeSet::new();
            let allowed = multi
                .get_allowedoperatives_slot()
                .into_iter()
                .collect::<BTreeSet<_>>();
            allowed.into_iter().for_each(|allowed_op| {
                get_all_descendent_operators_including_own(allowed_op, &mut return_list);
            });
            return_list
        }
        OperativeSlotTypeSpecializableTraitObject::OperativeSlotTypeSingleSpecialization(
            single,
        ) => {
            let mut return_list = BTreeSet::new();
            get_all_descendent_operators_including_own(
                single.get_allowedoperative_slot(),
                &mut return_list,
            );
            return_list
        }
        OperativeSlotTypeSpecializableTraitObject::TemplateSlotTypeSingleOperative(single) => {
            let mut return_list = BTreeSet::new();
            get_all_descendent_operators_including_own(
                single.get_allowedoperative_slot(),
                &mut return_list,
            );
            return_list
        }
        OperativeSlotTypeSpecializableTraitObject::OperativeSlotTypeTraitObjectSpecialization(
            trait_spec,
        ) => get_all_operatives_which_impl_trait_set(
            trait_spec.get_allowedtraits_slot(),
            schema_concrete,
        ),
    }
}

pub fn get_all_operatives_which_satisfy_specialization(
    schema_concrete: &RGSOConcrete<SchemaConcrete, Schema>,
    specializable: OperativeSlotTypeSpecializationTraitObject,
) -> BTreeSet<RGSOConcrete<OperativeConcrete, Schema>> {
    match specializable {
        OperativeSlotTypeSpecializationTraitObject::OperativeSlotTypeMultiSpecialization(multi) => {
            multi
                .get_allowedoperatives_slot()
                .into_iter()
                .collect::<BTreeSet<_>>()
        }
        OperativeSlotTypeSpecializationTraitObject::OperativeSlotTypeSingleSpecialization(
            single,
        ) => BTreeSet::from([single.get_allowedoperative_slot()]),
        OperativeSlotTypeSpecializationTraitObject::OperativeSlotTypeTraitObjectSpecialization(
            trait_spec,
        ) => get_all_operatives_which_impl_trait_set(
            trait_spec.get_allowedtraits_slot(),
            schema_concrete,
        ),
    }
}
pub fn get_all_instances_which_satisfy_specialization(
    schema_concrete: &RGSOConcrete<SchemaConcrete, Schema>,
    specializable: OperativeSlotTypeSpecializationTraitObject,
) -> BTreeSet<RGSOConcrete<InstanceConcrete, Schema>> {
    let satisfactory_ops =
        get_all_operatives_which_satisfy_specialization(schema_concrete, specializable);
    satisfactory_ops
        .into_iter()
        .fold(BTreeSet::new(), |mut agg, op| {
            agg.extend(get_all_descendent_instances_including_own(
                op,
                schema_concrete,
            ));
            agg
        })
}

// pub fn get_childest_type_specialization_for_op_and_slot(
//     op: RGSOConcrete<OperativeConcrete, Schema>,
//     slot: RGSOConcrete<TemplateSlot, Schema>,
// ) -> Option<OperativeSlotTypeSpecializationTraitObject> {
//     op.get_slotspecializations_slot()
//         .into_iter()
//         .filter(|spec| spec.get_roottemplateslot_slot().get_id() == slot.get_id())
//         .next()
//         .map(|spec| spec.get_typespecialization_slot().first().cloned())
//         .flatten()
// }

// pub fn get_childest_cardinality_specialization_for_op_and_slot(
//     op: RGSOConcrete<OperativeConcrete, Schema>,
//     slot: RGSOConcrete<TemplateSlot, Schema>,
// ) -> Option<OperativeSlotCardinalitySpecializationTraitObject> {
//     op.get_slotspecializations_slot()
//         .into_iter()
//         .filter(|spec| spec.get_roottemplateslot_slot().get_id() == slot.get_id())
//         .next()
//         .map(|spec| spec.get_cardinalityspecialization_slot().first().cloned())
//         .flatten()
// }

// // Returns the most restrictive min, max, and zero_allowed
// // They might not all be from the same specialization
// pub fn get_childest_cardinality_info_downstream(
//     op: RGSOConcrete<OperativeConcrete, Schema>,
//     slot: RGSOConcrete<TemplateSlot, Schema>,
// ) -> Option<CardinalityInfo> {
//     let mut desc_ops_and_self = BTreeSet::new();
//     desc_ops_and_self.insert(op.clone());
//     get_all_descendent_operators(op.clone(), &mut desc_ops_and_self);

//     let card_specs = desc_ops_and_self
//         .into_iter()
//         .filter_map(|desc_op| {
//             if let Some(spec) =
//                 get_childest_cardinality_specialization_for_op_and_slot(desc_op, slot.clone())
//             {
//                 Some(CardinalityInfo::from_card_spec(spec))
//             } else {
//                 None
//             }
//         })
//         .collect::<Vec<_>>();

//     if card_specs.len() == 0 {
//         return None;
//     }
//     let mut iter = card_specs.into_iter();
//     let mut most_restrictive = iter.next().unwrap();
//     iter.for_each(|spec| {
//         most_restrictive.min = most_restrictive.min.max(spec.min);
//         if let Some(new_max) = spec.max {
//             if let Some(old_max) = most_restrictive.max {
//                 most_restrictive.max = Some(old_max.min(new_max));
//             } else {
//                 most_restrictive.max = Some(new_max);
//             }
//         }
//         if spec.zero_allowed == false {
//             most_restrictive.zero_allowed = false;
//         }
//     });

//     Some(most_restrictive)
// }

pub fn get_deepest_downstream_specializations(
    op: RGSOConcrete<OperativeConcrete, Schema>,
    template_slot_id: &Uid,
    include_self: bool,
) -> BTreeSet<RGSOConcrete<OperativeSlotSpecialized, Schema>> {
    fn recurse(
        op: RGSOConcrete<OperativeConcrete, Schema>,
        template_slot_id: &Uid,
    ) -> BTreeSet<RGSOConcrete<OperativeSlotSpecialized, Schema>> {
        let children = op.get_childrenoperatives_slot();
        if children.is_empty() {
            op.get_slotspecializations_slot()
                .into_iter()
                .filter(|spec| spec.get_roottemplateslot_slot().get_id() == template_slot_id)
                .collect::<BTreeSet<_>>()
        } else {
            children
                .into_iter()
                .fold(BTreeSet::new(), |mut agg, child| {
                    agg.extend(recurse(child, template_slot_id));
                    agg
                })
        }
    }
    if include_self && op.get_childrenoperatives_slot().is_empty() {
        return op
            .get_slotspecializations_slot()
            .into_iter()
            .filter(|spec| spec.get_roottemplateslot_slot().get_id() == template_slot_id)
            .collect();
    }
    recurse(op, template_slot_id)
}

pub fn restructure_slot_specialization_to_delete_input(
    editor: &mut ExistingBuilder<SchemaConcrete, Schema>,
    ctx: SharedGraph<Schema>,
    specialized_slot_to_delete: RGSOConcrete<OperativeSlotSpecialized, Schema>,
) {
    leptos::logging::log!("Running delete slot_spec");
    editor.incorporate(specialized_slot_to_delete.edit(ctx.clone()).delete());
    let operative = specialized_slot_to_delete.get_specializer_slot();
    let mut descendents = BTreeSet::new();
    get_all_descendent_operators_including_own(operative, &mut descendents);

    let upstream_node = specialized_slot_to_delete.get_upstreamslotdescription_slot();

    descendents.into_iter().for_each(|descendent| {
        if let Some(desc_spec_node) = descendent
            .get_slotspecializations_slot()
            .into_iter()
            .filter(|desc_spec_node| {
                desc_spec_node.get_roottemplateslot_slot().get_id()
                    == specialized_slot_to_delete
                        .get_roottemplateslot_slot()
                        .get_id()
            })
            .next()
        {
            // If the descendent's specialized slot is the one being deleted, handle updating it to an upstream
            if desc_spec_node.get_id() == specialized_slot_to_delete.get_id() {
                leptos::logging::log!("removing from an operative");
                editor.incorporate(
                    descendent
                        .edit(ctx.clone())
                        .remove_from_slotspecializations(specialized_slot_to_delete.get_id()),
                );
                match &upstream_node {
                    // If upstream is the template, then there is no need for any specialization
                    SlotDescriptionTraitObject::TemplateSlot(_) => {}
                    SlotDescriptionTraitObject::OperativeSlotSpecialized(item) => editor
                        .incorporate(
                            &descendent
                                .edit(ctx.clone())
                                .add_existing_slotspecializations(item.get_id(), |na| na),
                        ),
                }
            }
            // Otherwise, if the descendent's specialized slot pointed to the slot-to-delete as its upstream,
            // then it needs to change its upstream to the new upstream
            else if desc_spec_node.get_upstreamslotdescription_slot().get_id()
                == specialized_slot_to_delete.get_id()
            {
                editor.incorporate(
                    desc_spec_node
                        .edit(ctx.clone())
                        .remove_from_upstreamslotdescription(specialized_slot_to_delete.get_id()),
                );
                match &upstream_node {
                    SlotDescriptionTraitObject::TemplateSlot(item) => editor.incorporate(
                        &desc_spec_node
                            .edit(ctx.clone())
                            .add_existing_upstreamslotdescription::<TemplateSlot>(
                                item.get_id(),
                                |na| na,
                            ),
                    ),
                    SlotDescriptionTraitObject::OperativeSlotSpecialized(item) => editor
                        .incorporate(
                            &desc_spec_node
                                .edit(ctx.clone())
                                .add_existing_upstreamslotdescription::<OperativeSlotSpecialized>(
                                    item.get_id(),
                                    |na| na,
                                ),
                        ),
                }
            }
        }
    });
}

pub(crate) fn constraint_to_canvas_template(
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
    NodeTemplate {
        template_id: template_string_id,
        name: template.tag.name.clone(),
        slot_templates,
        ..NodeTemplate::new(&template.tag.name) // max_instances: None,
                                                // min_instances: None,
                                                // can_delete: true,
                                                // can_create: true,
                                                // default_width: true,
                                                // default_height: todo!(),
    }
}

// fn get_allowed_step_connections(step: ImplStepVariantTraitObject) => Vec<ImplDataVariantTraitObject> {
//     match step {

//     }
// }
