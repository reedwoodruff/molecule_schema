use leptos::*;
use std::collections::HashMap;

use serde_types::common::{ConstraintTraits, Uid};

use super::reactive_types::{RFieldConstraint, RFulfilledFieldConstraint};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RLockedFieldsDigest<TTypes: ConstraintTraits, TValues: ConstraintTraits> {
    pub object_id: Uid,
    pub locked_fields: HashMap<Uid, RLockedFieldDigest<TValues>>,
    pub field_constraints: RwSignal<HashMap<Uid, RFieldConstraint<TTypes>>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RLockedFieldDigest<TValues: ConstraintTraits> {
    pub fulfilled_field: RFulfilledFieldConstraint<TValues>,
    pub hosting_element_id: Uid,
}

impl<TTypes: ConstraintTraits, TValues: ConstraintTraits> RLockedFieldsDigest<TTypes, TValues> {
    pub fn new(object_id: Uid) -> Self {
        Self {
            locked_fields: HashMap::new(),
            field_constraints: RwSignal::new(HashMap::new()),
            object_id,
        }
    }
    pub fn get_unfulfilled_fields(&self) -> Vec<RFieldConstraint<TTypes>> {
        self.field_constraints.with(|field_constraints| {
            field_constraints
                .iter()
                .filter(|(uid, _)| !self.locked_fields.contains_key(uid))
                .map(|(_uid, field_constraint)| field_constraint.clone())
                .collect()
        })
    }
    pub fn get_ancestors_locked_fields(&self) -> Vec<RLockedFieldDigest<TValues>> {
        self.locked_fields
            .values()
            .filter(|locked_field_digest| locked_field_digest.hosting_element_id != self.object_id)
            .cloned()
            .collect()
    }
}
