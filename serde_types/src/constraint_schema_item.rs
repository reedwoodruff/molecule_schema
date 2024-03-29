use std::collections::HashMap;

use crate::{
    common::{ConstraintTraits, Tag, Uid},
    constraint_schema::{
        ConstraintSchema, LibraryOperative, LibraryTemplate,
        LockedFieldConstraint, SlottedInstances, TraitImpl,
    },
    locked_field_digest::{LockedFieldDigest, LockedFieldsDigest},
    operative_digest::{OperativeDigest, OperativeSlotDigest, RelatedInstance},
    trait_impl_digest::{RelatedTraitImpl, TraitImplDigest},
};
pub trait ConstraintSchemaItem {
    type TTypes: ConstraintTraits;
    type TValues: ConstraintTraits;

    fn get_template_id(&self) -> &Uid;
    fn get_parent_operative_id(&self) -> Option<&Uid>;
    fn get_tag(&self) -> &Tag;
    fn get_local_trait_impls(&self) -> &HashMap<Uid, TraitImpl>;
    fn get_local_slotted_instances(&self) -> Option<&HashMap<Uid, SlottedInstances>>;
    fn get_local_locked_fields(
        &self,
    ) -> Option<&HashMap<Uid, LockedFieldConstraint<Self::TValues>>>;
    fn get_trait_impl_digest<'a>(
        &'a self,
        schema: &'a ConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> TraitImplDigest;
    fn get_operative_digest<'a>(
        &'a self,
        schema: &'a ConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> OperativeDigest;
    fn get_locked_fields_digest<'a>(
        &'a self,
        schema: &'a ConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Option<LockedFieldsDigest<Self::TValues>>;
}

impl<TTypes: ConstraintTraits, TValues: ConstraintTraits> ConstraintSchemaItem
    for LibraryTemplate<TTypes, TValues>
{
    type TTypes = TTypes;
    type TValues = TValues;
    fn get_template_id(&self) -> &Uid {
        &self.get_tag().id
    }
    fn get_parent_operative_id(&self) -> Option<&Uid> {
        None
    }
    fn get_tag(&self) -> &Tag {
        &self.tag
    }
    fn get_local_trait_impls(&self) -> &HashMap<Uid, TraitImpl> {
        &self.trait_impls
    }
    fn get_local_slotted_instances(&self) -> Option<&HashMap<Uid, SlottedInstances>> {
        None
    }
    fn get_local_locked_fields(
        &self,
    ) -> Option<&HashMap<Uid, LockedFieldConstraint<Self::TValues>>> {
        None
    }
    fn get_operative_digest(&self, schema: &ConstraintSchema<TTypes, TValues>) -> OperativeDigest {
        OperativeDigest {
            constituent_operatives: self
                .operative_slots
                .iter()
                .map(|(slot_id, op_slot)| {
                    (
                        *slot_id,
                        OperativeSlotDigest {
                            slot: op_slot,
                            related_instances: vec![],
                        },
                    )
                })
                .collect(),
        }
    }
    fn get_trait_impl_digest<'a>(
        &'a self,
        schema: &'a ConstraintSchema<TTypes, TValues>,
    ) -> TraitImplDigest {
        TraitImplDigest(
            self.trait_impls
                .iter()
                .map(|(trait_id, trait_impl)| {
                    (
                        *trait_id,
                        RelatedTraitImpl {
                            trait_impl,
                            hosting_element_id: self.get_tag().id,
                        },
                    )
                })
                .collect(),
        )
    }
    fn get_locked_fields_digest<'a>(
        &'a self,
        schema: &'a ConstraintSchema<TTypes, TValues>,
    ) -> Option<LockedFieldsDigest<TValues>> {
        None
    }
}

impl<TTypes: ConstraintTraits, TValues: ConstraintTraits> ConstraintSchemaItem
    for LibraryOperative<TTypes, TValues>
{
    type TTypes = TTypes;
    type TValues = TValues;
    fn get_template_id(&self) -> &Uid {
        &self.template_id
    }
    fn get_parent_operative_id(&self) -> Option<&Uid> {
        self.parent_operative_id.as_ref()
    }
    fn get_tag(&self) -> &Tag {
        &self.tag
    }
    fn get_local_trait_impls(&self) -> &HashMap<Uid, TraitImpl> {
        &self.trait_impls
    }
    fn get_local_slotted_instances(&self) -> Option<&HashMap<Uid, SlottedInstances>> {
        Some(&self.slotted_instances)
    }
    fn get_local_locked_fields(
        &self,
    ) -> Option<&HashMap<Uid, LockedFieldConstraint<Self::TValues>>> {
        Some(&self.locked_fields)
    }
    fn get_operative_digest<'a>(
        &'a self,
        schema: &'a ConstraintSchema<Self::TTypes, TValues>,
    ) -> OperativeDigest {
        let related_template = schema.template_library.get(self.get_template_id()).unwrap();
        let mut aggregate_instances = HashMap::new();

        // by setting the first parent id to the current operative's id, we can avoid special
        // casing this element
        let mut next_parent_id = Some(self.tag.id);
        while let Some(parent_id) = next_parent_id {
            // let parent_operative = schema.operative_library.get(&parent_id).unwrap();
            let parent_operative =
                if let Some(parent_operative) = schema.operative_library.get(&parent_id) {
                    parent_operative
                } else {
                    schema.instance_library.get(&parent_id).unwrap()
                };
            for (slot_id, slotted_instances) in &parent_operative.slotted_instances {
                aggregate_instances
                    .entry(*slot_id)
                    .or_insert_with(std::vec::Vec::new)
                    .extend(
                        slotted_instances
                            .fulfilling_instance_ids
                            .iter()
                            .map(|instance_id| RelatedInstance {
                                instance_id: *instance_id,
                                hosting_element_id: parent_id,
                            }),
                    );
            }
            next_parent_id = parent_operative.parent_operative_id;
        }

        let constituent_operatives = related_template
            .operative_slots
            .iter()
            .map(|(slot_id, op_slot)| {
                (
                    *slot_id,
                    OperativeSlotDigest {
                        slot: op_slot,
                        related_instances: aggregate_instances
                            .get(slot_id)
                            .cloned()
                            .unwrap_or_else(std::vec::Vec::new),
                    },
                )
            })
            .collect();

        OperativeDigest {
            constituent_operatives,
        }
    }

    fn get_trait_impl_digest<'a>(
        &'a self,
        schema: &'a ConstraintSchema<Self::TTypes, TValues>,
    ) -> TraitImplDigest {
        let mut next_parent_id = Some(self.tag.id);
        let mut aggregate_trait_impls = HashMap::new();

        let template = schema
            .template_library
            .get(self.get_template_id())
            .unwrap();

        aggregate_trait_impls.extend(
            template
                .get_local_trait_impls()
                .iter()
                .map(|(trait_id, trait_impl)| {
                    (
                        *trait_id,
                        RelatedTraitImpl {
                            trait_impl,
                            hosting_element_id: self.get_tag().id,
                        },
                    )
                })
                .collect::<HashMap<_, _>>(),
        );

        while let Some(parent_id) = next_parent_id {
            // let parent_operative = schema.operative_library.get(&parent_id).unwrap();
            let parent_operative =
                if let Some(parent_operative) = schema.operative_library.get(&parent_id) {
                    parent_operative
                } else {
                    schema.instance_library.get(&parent_id).unwrap()
                };
            aggregate_trait_impls.extend(
                parent_operative
                    .get_local_trait_impls()
                    .iter()
                    .map(|(trait_id, trait_impl)| {
                        (
                            *trait_id,
                            RelatedTraitImpl {
                                trait_impl,
                                hosting_element_id: self.get_tag().id,
                            },
                        )
                    })
                    .collect::<HashMap<_, _>>(),
            );
            next_parent_id = parent_operative.parent_operative_id;
        }

        TraitImplDigest(aggregate_trait_impls)
    }
    fn get_locked_fields_digest<'a>(
        &'a self,
        schema: &'a ConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Option<LockedFieldsDigest<Self::TValues>> {
        let mut next_parent_id = Some(self.tag.id);
        let mut aggregate_locked_fields = HashMap::new();

        while let Some(parent_id) = next_parent_id {
            // let parent_operative = schema.operative_library.get(&parent_id).unwrap();
            let parent_operative =
                if let Some(parent_operative) = schema.operative_library.get(&parent_id) {
                    parent_operative
                } else {
                    schema.instance_library.get(&parent_id).unwrap()
                };
            if let Some(new_locked_fields) = parent_operative.get_local_locked_fields() {
                aggregate_locked_fields.extend(
                    new_locked_fields
                        .iter()
                        .map(|(field_id, field_constraint)| {
                            (
                                *field_id,
                                LockedFieldDigest {
                                    fulfilled_field: field_constraint.clone(),
                                    hosting_element_id: parent_id,
                                },
                            )
                        })
                        .collect::<HashMap<_, _>>(),
                );
            }
            next_parent_id = parent_operative.parent_operative_id;
        }
        Some(LockedFieldsDigest(aggregate_locked_fields))
    }
}
