use std::collections::HashMap;

use leptos::{*};
use serde_types::{
    common::{ConstraintTraits, Uid},
};

use super::reactive_types::{ROperativeSlot, RSlotBounds};

#[derive(Clone, Debug, PartialEq)]
pub struct ROperativeDigest {
    pub digest_object_id: Uid,
    pub operative_slots: HashMap<Uid, ROperativeSlotDigest>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ROperativeSlotDigest {
    pub digest_object_id: Uid,
    pub slot: ROperativeSlot,
    pub related_instances: Vec<RRelatedInstance>,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RRelatedInstance {
    pub instance_id: Uid,
    pub hosting_element_id: Uid,
}

impl ROperativeDigest {
    pub fn is_fulfilled<TTypes: ConstraintTraits, TValues: ConstraintTraits>(&self) -> bool {
        self.
        // operative_slots.with(|operative_slots| {
            operative_slots
                .values()
                .all(|operative_slot| operative_slot.get_fulfillment_status())
        // })
    }
}

impl ROperativeSlotDigest {
    pub fn get_fulfillment_status(&self) -> bool {
        let len = self.related_instances.len();
        match self.slot.bounds.get() {
            // RSlotBounds::Unbounded => true,
            RSlotBounds::Single => len == 1,
            RSlotBounds::LowerBound(lower_bound) => len >= lower_bound.get(),
            RSlotBounds::UpperBound(upper_bound) => len <= upper_bound.get(),
            RSlotBounds::Range(lower, upper) => lower.get() <= len && len <= upper.get(),
            RSlotBounds::LowerBoundOrZero(lower_bound) => len == 0 || len >= lower_bound.get(),
            RSlotBounds::RangeOrZero(lower, upper) => {
                len == 0 || (lower.get() <= len && len <= upper.get())
            }
        }
    }
    pub fn get_local_related_instances(&self) -> Vec<RRelatedInstance> {
        self.related_instances
            .iter()
            .filter(|related_instance| related_instance.hosting_element_id == self.digest_object_id)
            .cloned()
            .collect()
    }
    pub fn get_ancestors_related_instances(&self) -> Vec<RRelatedInstance> {
        self.related_instances
            .iter()
            .filter(|related_instance| related_instance.hosting_element_id != self.digest_object_id)
            .cloned()
            .collect()
    }
}

impl ROperativeDigest {
    pub fn get_unfulfilled_operative_slots(&self) -> Vec<ROperativeSlotDigest> {
        let self_clone = self.clone();
        // create_memo(move |_| {
        self_clone
            // .operative_slots.with(|operative_slots| {
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
        // })
        // })
    }
    pub fn get_fulfilled_operative_slots(&self) -> Vec<ROperativeSlotDigest> {
        let self_clone = self.clone();
        // create_memo(move |_| {
        self_clone
            // .operative_slots.with(|operative_slots| {
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
        // })
        // })
    }
    // pub fn get_local_relat
}
