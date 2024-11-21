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
    schema_concrete: RGSOConcrete<SchemaConcrete, Schema>,
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
    schema_concrete: RGSOConcrete<SchemaConcrete, Schema>,
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
    schema_concrete: RGSOConcrete<SchemaConcrete, Schema>,
) -> BTreeSet<RGSOConcrete<InstanceConcrete, Schema>> {
    let ops_which_impl = get_all_operatives_which_impl_trait_set(traits, schema_concrete.clone());
    let instances_which_impl = ops_which_impl
        .into_iter()
        .fold(BTreeSet::new(), |mut agg, op| {
            agg.extend(get_all_descendent_instances(op, schema_concrete.clone()));
            agg
        });
    instances_which_impl
}

pub fn get_all_traits_in_specialization(
    specialization: RGSOConcrete<OperativeSlotTraitObjectSpecialization, Schema>,
) -> BTreeSet<RGSOConcrete<TraitConcrete, Schema>> {
    let mut found_terminal = false;
    let mut rolling_trait_set = BTreeSet::new();
    let mut cur_spec =
        SlotSpecializableTraitOperativeTraitObject::OperativeSlotTraitObjectSpecialization(
            specialization,
        );
    while !found_terminal {
        match cur_spec {
            SlotSpecializableTraitOperativeTraitObject::TemplateSlotTraitOperative(terminal) => {
                rolling_trait_set.extend(terminal.get_allowedtraits_slot());
                found_terminal = true;
                break;
            }
            SlotSpecializableTraitOperativeTraitObject::OperativeSlotTraitObjectSpecialization(
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
    schema_concrete: RGSOConcrete<SchemaConcrete, Schema>,
    specializable: SlotSpecializableTraitObject,
) -> BTreeSet<RGSOConcrete<OperativeConcrete, Schema>> {
    match specializable {
        SlotSpecializableTraitObject::TemplateSlotTraitOperative(trait_op) => {
            get_all_operatives_which_impl_trait_set(
                trait_op.get_allowedtraits_slot(),
                schema_concrete,
            )
        }
        SlotSpecializableTraitObject::OperativeSlotMultiSpecialization(multi) => multi
            .get_allowedoperatives_slot()
            .into_iter()
            .collect::<BTreeSet<_>>(),
        SlotSpecializableTraitObject::TemplateSlotMultiOperative(multi) => multi
            .get_allowedoperatives_slot()
            .into_iter()
            .collect::<BTreeSet<_>>(),
        SlotSpecializableTraitObject::OperativeSlotTraitObjectSpecialization(trait_spec) => {
            let trait_set = get_all_traits_in_specialization(trait_spec);

            get_all_operatives_which_impl_trait_set(
                trait_set.into_iter().collect::<Vec<_>>(),
                schema_concrete,
            )
        }
    }
}
