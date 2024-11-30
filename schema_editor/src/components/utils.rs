use std::collections::{BTreeSet, HashSet};

use schema_editor_generated_toolkit::prelude::*;

use super::slot_cardinality_specialization_builder::CardinalityInfo;

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
            allowed.into_iter().map(|allowed_op| {
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
            allowed.into_iter().map(|allowed_op| {
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

pub fn get_childest_type_specialization_for_op_and_slot(
    op: RGSOConcrete<OperativeConcrete, Schema>,
    slot: RGSOConcrete<TemplateSlot, Schema>,
) -> Option<OperativeSlotTypeSpecializationTraitObject> {
    op.get_slotspecializations_slot()
        .into_iter()
        .filter(|spec| spec.get_roottemplateslot_slot().get_id() == slot.get_id())
        .next()
        .map(|spec| spec.get_typespecialization_slot().first().cloned())
        .flatten()
}

pub fn get_childest_cardinality_specialization_for_op_and_slot(
    op: RGSOConcrete<OperativeConcrete, Schema>,
    slot: RGSOConcrete<TemplateSlot, Schema>,
) -> Option<OperativeSlotCardinalitySpecializationTraitObject> {
    op.get_slotspecializations_slot()
        .into_iter()
        .filter(|spec| spec.get_roottemplateslot_slot().get_id() == slot.get_id())
        .next()
        .map(|spec| spec.get_cardinalityspecialization_slot().first().cloned())
        .flatten()
}

// Returns the most restrictive min, max, and zero_allowed
// They might not all be from the same specialization
pub fn get_childest_cardinality_info_downstream(
    op: RGSOConcrete<OperativeConcrete, Schema>,
    slot: RGSOConcrete<TemplateSlot, Schema>,
) -> Option<CardinalityInfo> {
    let mut desc_ops_and_self = BTreeSet::new();
    desc_ops_and_self.insert(op.clone());
    get_all_descendent_operators(op.clone(), &mut desc_ops_and_self);

    let card_specs = desc_ops_and_self
        .into_iter()
        .filter_map(|desc_op| {
            if let Some(spec) =
                get_childest_cardinality_specialization_for_op_and_slot(desc_op, slot.clone())
            {
                Some(CardinalityInfo::from_card_spec(spec))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    if card_specs.len() == 0 {
        return None;
    }
    let mut iter = card_specs.into_iter();
    let mut most_restrictive = iter.next().unwrap();
    iter.for_each(|spec| {
        most_restrictive.min = most_restrictive.min.max(spec.min);
        if let Some(new_max) = spec.max {
            if let Some(old_max) = most_restrictive.max {
                most_restrictive.max = Some(old_max.min(new_max));
            } else {
                most_restrictive.max = Some(new_max);
            }
        }
        if spec.zero_allowed == false {
            most_restrictive.zero_allowed = false;
        }
    });

    Some(most_restrictive)
}

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
