use std::collections::HashMap;

use leptos::{RwSignal, *};
use serde_types::{
    common::{ConstraintTraits, Uid},
    constraint_schema::SlotBounds,
};

use super::reactive_types::{ROperativeSlot, RSlotBounds};

#[derive(Clone, Debug, PartialEq)]
pub struct ROperativeDigest {
    pub operative_slots: HashMap<Uid, ROperativeSlotDigest>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ROperativeSlotDigest {
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
        self.operative_slots
            .values()
            .all(|op_slot_status| op_slot_status.get_fulfillment_status())
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
}

impl ROperativeDigest {
    pub fn get_unfulfilled_operative_slots(&self) -> Vec<&ROperativeSlotDigest> {
        self.operative_slots
            .iter()
            .filter_map(|(_slot_id, operative_slot)| {
                if !operative_slot.get_fulfillment_status() {
                    Some(operative_slot)
                } else {
                    None
                }
            })
            .collect()
    }
    pub fn get_fulfilled_operative_slots(&self) -> Vec<&ROperativeSlotDigest> {
        self.operative_slots
            .iter()
            .filter_map(|(_slot_id, operative_slot)| {
                if operative_slot.get_fulfillment_status() {
                    Some(operative_slot)
                } else {
                    None
                }
            })
            .collect()
    }
}