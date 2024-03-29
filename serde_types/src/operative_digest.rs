use std::collections::HashMap;

use crate::{
    common::{ConstraintTraits, Uid},
    constraint_schema::{OperativeSlot},
};

#[derive(Clone, Debug)]
pub struct OperativeDigest<'a> {
    pub constituent_operatives: HashMap<Uid, OperativeSlotDigest<'a>>,
}

#[derive(Clone, Debug)]
pub struct OperativeSlotDigest<'a> {
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
        self.constituent_operatives
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
}
