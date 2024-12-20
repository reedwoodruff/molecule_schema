use leptos::prelude::*;
use std::collections::HashMap;

use base_types::common::{ConstraintTraits, Uid};

use super::{
    locked_field_digest::{RLockedFieldDigest, RLockedFieldsDigest},
    operative_digest::{ROperativeDigest, ROperativeSlotDigest, RRelatedInstance},
    reactive_types::{
        RConstraintSchema, RLibraryOperative, RLibraryTemplate, RLockedFieldConstraint,
        RSlottedInstances, RTraitImpl, Tagged,
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
    fn check_ancestry(
        &self,
        schema: &RConstraintSchema<Self::TTypes, Self::TValues>,
        ancestor_id_in_question: &Uid,
    ) -> bool {
        if self.get_tag().id.get() == *ancestor_id_in_question {
            return true;
        }

        let template = schema.template_library.with(|template_library| {
            template_library
                .get(&self.get_template_id())
                .unwrap()
                .clone()
        });
        if template.get_template_id() == *ancestor_id_in_question {
            return true;
        }
        let mut next_parent_id = self.get_parent_operative_id();
        while let Some(parent_id) = next_parent_id {
            if let Some(parent_operative) = schema
                .operative_library
                .with(|operative_library| operative_library.get(&parent_id).cloned())
            {
                if parent_operative.get_tag().id.get() == *ancestor_id_in_question {
                    return true;
                }
                next_parent_id = parent_operative.get_parent_operative_id();
            } else {
                panic!("Ancestor not found in schema");
            }
        }
        false
    }
    fn get_local_locked_fields(
        &self,
    ) -> RwSignal<HashMap<Uid, RLockedFieldConstraint<Self::TValues>>>;
    fn get_trait_impl_digest(
        &self,
        schema: &RConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> RTraitImplDigest;
    fn get_operative_digest(
        &self,
        schema: &RConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Memo<ROperativeDigest>;
    fn get_locked_fields_digest(
        &self,
        schema: &RConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> RLockedFieldsDigest<Self::TTypes, Self::TValues>;
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
    fn get_locked_fields_digest(
        &self,
        _schema: &RConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> RLockedFieldsDigest<Self::TTypes, Self::TValues> {
        RLockedFieldsDigest {
            digest_object_id: self.get_tag().id.get(),
            locked_fields: HashMap::new(),
            field_constraints: self.field_constraints,
        }
    }
    fn get_operative_digest(
        &self,
        _schema: &RConstraintSchema<TTypes, TValues>,
    ) -> Memo<ROperativeDigest> {
        let self_clone = self.clone();
        create_memo(move |_| {
            let slot_digest_hashmap = self_clone.operative_slots.with(|operative_slot| {
                operative_slot
                    .iter()
                    .map(|(slot_id, op_slot)| {
                        (
                            *slot_id,
                            ROperativeSlotDigest {
                                digest_object_id: self_clone.get_tag().id.get(),
                                slot: op_slot.clone(),
                                related_instances: vec![],
                            },
                        )
                    })
                    .collect()
            });
            ROperativeDigest {
                digest_object_id: self_clone.get_tag().id.get(),
                operative_slots: slot_digest_hashmap,
            }
        })
    }
    fn get_trait_impl_digest(
        &self,
        _schema: &RConstraintSchema<TTypes, TValues>,
    ) -> RTraitImplDigest {
        RTraitImplDigest {
            object_id: self.get_tag().id.get(),
            trait_impls: self.trait_impls.with(|trait_impls| {
                trait_impls
                    .iter()
                    .map(|(trait_id, trait_impl)| {
                        (
                            *trait_id,
                            RRelatedTraitImpl {
                                trait_impl: *trait_impl,
                                hosting_element_id: self.get_tag().id.get(),
                            },
                        )
                    })
                    .collect()
            }),
        }
    }
    fn get_local_locked_fields(
        &self,
    ) -> RwSignal<HashMap<Uid, RLockedFieldConstraint<Self::TValues>>> {
        RwSignal::new(HashMap::new())
    }
}

impl<TTypes: ConstraintTraits, TValues: ConstraintTraits> RConstraintSchemaItem
    for RLibraryOperative<TTypes, TValues>
{
    type TTypes = TTypes;
    type TValues = TValues;
    fn get_template_id(&self) -> Uid {
        self.template_id.get()
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
    fn get_local_locked_fields(
        &self,
    ) -> RwSignal<HashMap<Uid, RLockedFieldConstraint<Self::TValues>>> {
        self.locked_fields
    }
    fn get_operative_digest(
        &self,
        schema: &RConstraintSchema<Self::TTypes, TValues>,
    ) -> Memo<ROperativeDigest> {
        let self_id = self.get_tag().id.get();
        let self_clone = self.clone();
        let schema_clone = schema.clone();

        create_memo(move |_| {
            let related_template = schema_clone.template_library.with(|template_library| {
                template_library
                    .get(&self_clone.get_template_id())
                    .unwrap()
                    .clone()
            });
            let mut aggregate_instances = HashMap::new();

            // by setting the first parent id to the current operative's id, we can avoid special
            // casing this element
            let mut next_parent_id = Some(self_id);
            while let Some(parent_id) = next_parent_id {
                let parent_operative = schema_clone.operative_library.with(|operative_library| {
                    if let Some(parent_operative) = operative_library.get(&parent_id) {
                        parent_operative.clone()
                    } else {
                        schema_clone.instance_library.with(|instance_library| {
                            instance_library.get(&parent_id).unwrap().clone()
                        })
                    }
                });
                // let parent_operative = schema_clone
                //     .operative_library
                //     .with(|operative_library| operative_library.get(&parent_id).unwrap().clone());
                parent_operative
                    .slotted_instances
                    .with(|parent_slotted_instances| {
                        for (slot_id, slotted_instances) in parent_slotted_instances.iter() {
                            let related_instances = slotted_instances.fulfilling_instance_ids.with(
                                |fulfilling_instance_ids| {
                                    fulfilling_instance_ids
                                        .iter()
                                        .map(|instance_id| RRelatedInstance {
                                            digest_object_id: self_id,
                                            instance_id: *instance_id,
                                            hosting_element_id: parent_id,
                                        })
                                        .collect::<Vec<_>>()
                                },
                            );
                            aggregate_instances
                                .entry(*slot_id)
                                .or_insert_with(std::vec::Vec::new)
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
                                digest_object_id: self_id,
                                slot: op_slot.clone(),
                                related_instances: aggregate_instances
                                    .get(slot_id)
                                    .cloned()
                                    .unwrap_or_else(std::vec::Vec::new),
                            },
                        )
                    })
                    .collect()
            });

            ROperativeDigest {
                digest_object_id: self_id,
                operative_slots,
            }
        })
    }

    fn get_trait_impl_digest(
        &self,
        schema: &RConstraintSchema<Self::TTypes, TValues>,
    ) -> RTraitImplDigest {
        let mut next_parent_id = Some(self.tag.id.get());
        let mut aggregate_trait_impls = HashMap::new();
        let template = schema.template_library.with(|template_library| {
            template_library
                .get(&self.get_template_id())
                .unwrap()
                .clone()
        });

        aggregate_trait_impls.extend(template.get_local_trait_impls().with(|local_trait_impls| {
            local_trait_impls
                .iter()
                .map(|(trait_id, trait_impl)| {
                    (
                        *trait_id,
                        RRelatedTraitImpl {
                            trait_impl: *trait_impl,
                            hosting_element_id: template.get_tag().id.get(),
                        },
                    )
                })
                .collect::<HashMap<_, _>>()
        }));

        while let Some(parent_id) = next_parent_id {
            let parent_operative = schema.operative_library.with(|operative_library| {
                if let Some(parent_operative) = operative_library.get(&parent_id) {
                    parent_operative.clone()
                } else {
                    schema
                        .instance_library
                        .with(|instance_library| instance_library.get(&parent_id).unwrap().clone())
                }
            });
            // let parent_operative = schema
            //     .operative_library
            //     .with(|operative_library| operative_library.get(&parent_id).unwrap().clone());
            aggregate_trait_impls.extend(parent_operative.get_local_trait_impls().with(
                |local_trait_impls| {
                    local_trait_impls
                        .iter()
                        .map(|(trait_id, trait_impl)| {
                            (
                                *trait_id,
                                RRelatedTraitImpl {
                                    trait_impl: *trait_impl,
                                    hosting_element_id: parent_id,
                                },
                            )
                        })
                        .collect::<HashMap<_, _>>()
                },
            ));
            next_parent_id = parent_operative.parent_operative_id.get();
        }

        RTraitImplDigest {
            object_id: self.get_tag().id.get(),
            trait_impls: aggregate_trait_impls,
        }
    }

    fn get_locked_fields_digest(
        &self,
        schema: &RConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> RLockedFieldsDigest<Self::TTypes, Self::TValues> {
        let mut next_parent_id = Some(self.tag.id.get());
        let mut aggregate_locked_fields = HashMap::new();

        while let Some(parent_id) = next_parent_id {
            let parent_operative = schema.operative_library.with(|operative_library| {
                if let Some(parent_operative) = operative_library.get(&parent_id) {
                    parent_operative.clone()
                } else {
                    schema
                        .instance_library
                        .with(|instance_library| instance_library.get(&parent_id).unwrap().clone())
                }
            });
            // let parent_operative = schema
            //     .operative_library
            //     .with(|operative_library| operative_library.get(&parent_id).unwrap().clone());
            aggregate_locked_fields.extend(parent_operative.get_local_locked_fields().with(
                |new_locked_fields| {
                    new_locked_fields
                        .iter()
                        .map(|(field_id, field_constraint)| {
                            (
                                *field_id,
                                RLockedFieldDigest {
                                    digest_object_id: self.get_tag().id.get(),
                                    fulfilled_field: field_constraint.clone(),
                                    hosting_element_id: parent_id,
                                },
                            )
                        })
                        .collect::<HashMap<_, _>>()
                },
            ));
            next_parent_id = parent_operative.parent_operative_id.get();
        }
        let template = schema.template_library.with(|template_library| {
            template_library
                .get(&self.template_id.get())
                .unwrap()
                .clone()
        });
        RLockedFieldsDigest {
            digest_object_id: self.tag.id.get(),
            locked_fields: aggregate_locked_fields,
            field_constraints: template.field_constraints,
        }
    }
}
