use std::collections::HashMap;

use crate::{
    common::{ConstraintTraits, Uid},
    constraint_schema::OperativeSlot,
};

#[derive(Clone, Debug)]
pub struct OperativeDigest<'a> {
    pub digest_object_id: Uid,
    pub operative_slots: HashMap<Uid, OperativeSlotDigest<'a>>,
}

#[derive(Clone, Debug)]
pub struct OperativeSlotDigest<'a> {
    pub digest_object_id: Uid,
    pub slot: &'a OperativeSlot,
    pub related_instances: Vec<RelatedInstance>,
}
#[derive(Clone, Debug)]
pub struct RelatedInstance {
    pub instance_id: Uid,
    pub hosting_element_id: Uid,
}

impl<'a> OperativeDigest<'a> {
    pub fn is_fulfilled<TTypes: ConstraintTraits, TValues: ConstraintTraits>(&self) -> bool {
        self.operative_slots
            .values()
            .all(|op_slot_status| op_slot_status.get_fulfillment_status())
    }
}

impl<'a> OperativeSlotDigest<'a> {
    pub fn get_fulfillment_status(&self) -> bool {
        match self.slot.bounds {
            // crate::constraint_schema::SlotBounds::Unbounded => true,
            crate::constraint_schema::SlotBounds::Single => self.related_instances.len() == 1,
            crate::constraint_schema::SlotBounds::LowerBound(lower_bound) => {
                self.related_instances.len() >= lower_bound
            }
            crate::constraint_schema::SlotBounds::UpperBound(upper_bound) => {
                self.related_instances.len() <= upper_bound
            }
            crate::constraint_schema::SlotBounds::Range(lower, upper) => {
                lower <= self.related_instances.len() && self.related_instances.len() <= upper
            }
            crate::constraint_schema::SlotBounds::LowerBoundOrZero(lower_bound) => {
                self.related_instances.is_empty() || self.related_instances.len() >= lower_bound
            }
            crate::constraint_schema::SlotBounds::RangeOrZero(lower, upper) => {
                self.related_instances.is_empty()
                    || (lower <= self.related_instances.len()
                        && self.related_instances.len() <= upper)
            }
        }
    }
    pub fn get_local_related_instances(&self) -> Vec<RelatedInstance> {
        self.related_instances
            .iter()
            .filter(|related_instance| related_instance.hosting_element_id == self.digest_object_id)
            .cloned()
            .collect()
    }
    pub fn get_ancestors_related_instances(&self) -> Vec<RelatedInstance> {
        self.related_instances
            .iter()
            .filter(|related_instance| related_instance.hosting_element_id != self.digest_object_id)
            .cloned()
            .collect()
    }
}

impl<'a> OperativeDigest<'a> {
    pub fn get_unfulfilled_operative_slots(&self) -> Vec<OperativeSlotDigest> {
        let self_clone = self.clone();
        self_clone
            .operative_slots
            .iter()
            .filter_map(|(_slot_id, operative_slot)| {
                if !operative_slot.get_fulfillment_status() {
                    Some(operative_slot.clone())
                } else {
                    None
                }
            })
            .collect()
    }
    pub fn get_fulfilled_operative_slots(&self) -> Vec<OperativeSlotDigest> {
        let self_clone = self.clone();
        self_clone
            .operative_slots
            .iter()
            .filter_map(|(_slot_id, operative_slot)| {
                if operative_slot.get_fulfillment_status() {
                    Some(operative_slot.clone())
                } else {
                    None
                }
            })
            .collect()
    }
}
