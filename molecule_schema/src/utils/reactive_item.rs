use leptos::*;
use std::collections::HashMap;

use serde_types::common::{ConstraintTraits, Uid};

use super::{
    operative_digest::{ROperativeDigest, ROperativeSlotDigest, RRelatedInstance},
    reactive_types::{
        RConstraintSchema, RLibraryOperative, RLibraryTemplate, RSlottedInstances, RTag,
        RTraitImpl, Tagged,
    },
    trait_impl_digest::{RRelatedTraitImpl, RTraitImplDigest},
};

pub trait RConstraintSchemaItem: Tagged + PartialEq {
    type TTypes: ConstraintTraits;

    type TValues: ConstraintTraits;

    fn get_template_id(&self) -> Uid;
    fn get_parent_operative_id(&self) -> Option<Uid>;
    fn get_local_trait_impls(&self) -> RwSignal<HashMap<Uid, RTraitImpl>>;
    fn get_local_slotted_instances(&self) -> Option<RwSignal<HashMap<Uid, RSlottedInstances>>>;
    fn get_trait_impl_digest(
        &self,
        schema: &RConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> RTraitImplDigest;
    fn get_operative_digest(
        &self,
        schema: &RConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> ROperativeDigest;
}

impl<TTypes: ConstraintTraits, TValues: ConstraintTraits> RConstraintSchemaItem
    for RLibraryTemplate<TTypes, TValues>
{
    type TTypes = TTypes;
    type TValues = TValues;
    fn get_template_id(&self) -> Uid {
        self.get_tag().id.get()
    }
    fn get_parent_operative_id(&self) -> Option<Uid> {
        None
    }
    fn get_local_trait_impls(&self) -> RwSignal<HashMap<Uid, RTraitImpl>> {
        self.trait_impls
    }
    fn get_local_slotted_instances(&self) -> Option<RwSignal<HashMap<Uid, RSlottedInstances>>> {
        None
    }
    fn get_operative_digest(
        &self,
        schema: &RConstraintSchema<TTypes, TValues>,
    ) -> ROperativeDigest {
        let slot_digest_hashmap = self.operative_slots.with(|operative_slot| {
            operative_slot
                .iter()
                .map(|(slot_id, op_slot)| {
                    (
                        *slot_id,
                        ROperativeSlotDigest {
                            slot: op_slot.clone(),
                            related_instances: vec![],
                        },
                    )
                })
                .collect()
        });
        ROperativeDigest {
            operative_slots: slot_digest_hashmap,
        }
    }
    fn get_trait_impl_digest(
        &self,
        schema: &RConstraintSchema<TTypes, TValues>,
    ) -> RTraitImplDigest {
        RTraitImplDigest(self.trait_impls.with(|trait_impls| {
            trait_impls
                .iter()
                .map(|(trait_id, trait_impl)| {
                    (
                        *trait_id,
                        RRelatedTraitImpl {
                            trait_impl: trait_impl.clone(),
                            hosting_element_id: self.get_tag().id.get(),
                        },
                    )
                })
                .collect()
        }))
    }
}

impl<TTypes: ConstraintTraits, TValues: ConstraintTraits> RConstraintSchemaItem
    for RLibraryOperative<TTypes, TValues>
{
    type TTypes = TTypes;
    type TValues = TValues;
    fn get_template_id(&self) -> Uid {
        self.get_tag().id.get()
    }
    fn get_parent_operative_id(&self) -> Option<Uid> {
        self.parent_operative_id.get()
    }
    fn get_local_trait_impls(&self) -> RwSignal<HashMap<Uid, RTraitImpl>> {
        self.trait_impls
    }
    fn get_local_slotted_instances(&self) -> Option<RwSignal<HashMap<Uid, RSlottedInstances>>> {
        Some(self.slotted_instances)
    }
    fn get_operative_digest<'a>(
        &'a self,
        schema: &'a RConstraintSchema<Self::TTypes, TValues>,
    ) -> ROperativeDigest {
        let related_template = schema.template_library.with(|template_library| {
            template_library
                .get(&self.get_template_id())
                .unwrap()
                .clone()
        });
        let mut aggregate_instances = HashMap::new();

        // by setting the first parent id to the current operative's id, we can avoid special
        // casing this element
        let mut next_parent_id = Some(self.tag.id.get());
        while let Some(parent_id) = next_parent_id {
            let parent_operative = schema
                .operative_library
                .with(|operative_library| operative_library.get(&parent_id).unwrap().clone());
            parent_operative
                .slotted_instances
                .with(|parent_slotted_instances| {
                    for (slot_id, slotted_instances) in parent_slotted_instances.iter() {
                        let related_instances = slotted_instances.fulfilling_instance_ids.with(
                            |fulfilling_instance_ids| {
                                fulfilling_instance_ids
                                    .iter()
                                    .map(|instance_id| RRelatedInstance {
                                        instance_id: *instance_id,
                                        hosting_element_id: parent_id,
                                    })
                                    .collect::<Vec<_>>()
                            },
                        );
                        aggregate_instances
                            .entry(*slot_id)
                            .or_insert_with(|| vec![])
                            .extend(related_instances);
                    }
                });
            next_parent_id = parent_operative.parent_operative_id.get();
        }

        let operative_slots = related_template.operative_slots.with(|operative_slots| {
            operative_slots
                .iter()
                .map(|(slot_id, op_slot)| {
                    (
                        *slot_id,
                        ROperativeSlotDigest {
                            slot: op_slot.clone(),
                            related_instances: aggregate_instances
                                .get(slot_id)
                                .cloned()
                                .unwrap_or_else(|| vec![]),
                        },
                    )
                })
                .collect()
        });

        ROperativeDigest { operative_slots }
    }

    fn get_trait_impl_digest(
        &self,
        schema: &RConstraintSchema<Self::TTypes, TValues>,
    ) -> RTraitImplDigest {
        let mut next_parent_id = Some(self.tag.id.get());
        let mut aggregate_trait_impls = HashMap::new();

        while let Some(parent_id) = next_parent_id {
            let parent_operative = schema
                .operative_library
                .with(|operative_library| operative_library.get(&parent_id).unwrap().clone());
            aggregate_trait_impls.extend(parent_operative.get_local_trait_impls().with(
                |local_trait_impls| {
                    local_trait_impls
                        .iter()
                        .map(|(trait_id, trait_impl)| {
                            (
                                *trait_id,
                                RRelatedTraitImpl {
                                    trait_impl: trait_impl.clone(),
                                    hosting_element_id: self.get_tag().id.get(),
                                },
                            )
                        })
                        .collect::<HashMap<_, _>>()
                },
            ));
            next_parent_id = parent_operative.parent_operative_id.get();
        }

        RTraitImplDigest(aggregate_trait_impls)
    }
}
