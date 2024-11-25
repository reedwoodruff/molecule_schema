use std::collections::BTreeSet;

use schema_editor_generated_toolkit::prelude::*;

pub fn get_all_descendent_operators(
    op: RGSOConcrete<OperativeConcrete, Schema>,
    agg: &mut BTreeSet<RGSOConcrete<OperativeConcrete, Schema>>,
) -> &mut BTreeSet<RGSOConcrete<OperativeConcrete, Schema>> {
    agg.insert(op.clone());
    op.get_childrenoperatives_slot()
        .into_iter()
        .fold(agg, |mut agg, child| {
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
            agg.extend(get_all_descendent_instances(op, schema_concrete));
            agg
        });
    instances_which_impl
}

pub fn get_all_traits_in_specialization(
    specialization: RGSOConcrete<OperativeSlotTypeTraitObjectSpecialization, Schema>,
) -> BTreeSet<RGSOConcrete<TraitConcrete, Schema>> {
    let mut found_terminal = false;
    let mut rolling_trait_set = BTreeSet::new();
    let mut cur_spec =
        OperativeSlotTypeSpecializableTraitOperativeTraitObject::OperativeSlotTypeTraitObjectSpecialization(
            specialization,
        );
    while !found_terminal {
        match cur_spec {
            OperativeSlotTypeSpecializableTraitOperativeTraitObject::TemplateSlotTypeTraitOperative(terminal) => {
                rolling_trait_set.extend(terminal.get_allowedtraits_slot());
                found_terminal = true;
                break;
            }
            OperativeSlotTypeSpecializableTraitOperativeTraitObject::OperativeSlotTypeTraitObjectSpecialization(
                spec,
            ) => {
                rolling_trait_set.extend(spec.get_allowedtraits_slot());
                cur_spec = spec.get_specializationtarget_slot();
            }
        }
    }
    rolling_trait_set
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
            multi
                .get_allowedoperatives_slot()
                .into_iter()
                .collect::<BTreeSet<_>>()
        }
        OperativeSlotTypeSpecializableTraitObject::TemplateSlotTypeMultiOperative(multi) => multi
            .get_allowedoperatives_slot()
            .into_iter()
            .collect::<BTreeSet<_>>(),
        OperativeSlotTypeSpecializableTraitObject::OperativeSlotTypeTraitObjectSpecialization(
            trait_spec,
        ) => {
            let trait_set = get_all_traits_in_specialization(trait_spec);

            get_all_operatives_which_impl_trait_set(
                trait_set.into_iter().collect::<Vec<_>>(),
                schema_concrete,
            )
        }
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
        ) => {
            let trait_set = get_all_traits_in_specialization(trait_spec);

            get_all_operatives_which_impl_trait_set(
                trait_set.into_iter().collect::<Vec<_>>(),
                schema_concrete,
            )
        }
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
            agg.extend(get_all_descendent_instances(op, schema_concrete));
            agg
        })
}

pub fn get_childest_specialization_for_op_and_slot(
    op: RGSOConcrete<OperativeConcrete, Schema>,
    slot: RGSOConcrete<TemplateSlot, Schema>,
) -> Option<OperativeSlotTypeSpecializationTraitObject> {
    let specs = op
        .clone()
        .get_slottypespecializations_slot()
        .into_iter()
        .filter(|specialization| match specialization {
            OperativeSlotTypeSpecializationTraitObject::OperativeSlotTypeSingleSpecialization(single) => {
                single.get_roottemplateslot_slot().get_id() == slot.get_id()
            }
            OperativeSlotTypeSpecializationTraitObject::OperativeSlotTypeMultiSpecialization(multi) => {
                multi.get_roottemplateslot_slot().get_id() == slot.get_id()
            }
            OperativeSlotTypeSpecializationTraitObject::OperativeSlotTypeTraitObjectSpecialization(
                traits,
            ) => traits.get_roottemplateslot_slot().get_id() == slot.get_id(),
        })
        .collect::<Vec<_>>();

    if specs.len() == 0 {
        None
    } else if specs.len() == 1 {
        Some(specs.into_iter().next().unwrap())
    } else {
        let all_parent_ids = specs
            .iter()
            .map(|spec| match spec {
                OperativeSlotTypeSpecializationTraitObject::OperativeSlotTypeSingleSpecialization(item) => {
                    *item.get_specializationtarget_slot().get_id()
                }
                OperativeSlotTypeSpecializationTraitObject::OperativeSlotTypeMultiSpecialization(item) => {
                    *item.get_specializationtarget_slot().get_id()
                }
                OperativeSlotTypeSpecializationTraitObject::OperativeSlotTypeTraitObjectSpecialization(
                    item,
                ) => *item.get_specializationtarget_slot().get_id(),
            })
            .collect::<Vec<_>>();
        let childest_spec = specs
            .into_iter()
            .find(|spec| !all_parent_ids.contains(spec.get_id()));
        childest_spec
    }
}
