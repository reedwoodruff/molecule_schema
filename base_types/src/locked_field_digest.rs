use std::{
    borrow::Cow,
    collections::{BTreeMap, HashMap},
};

use crate::{
    common::{ConstraintTraits, Uid},
    constraint_schema::{FieldConstraint, LockedFieldConstraint},
};

#[derive(Clone, Debug)]
pub struct LockedFieldsDigest<'a, TTypes: ConstraintTraits, TValues: ConstraintTraits> {
    pub digest_object_id: Uid,
    pub locked_fields: HashMap<Uid, LockedFieldDigest<TValues>>,
    pub field_constraints: Cow<'a, BTreeMap<Uid, FieldConstraint<TTypes>>>,
}

#[derive(Clone, Debug)]
pub struct LockedFieldDigest<TValues: ConstraintTraits> {
    pub digest_object_id: Uid,
    pub fulfilled_field: LockedFieldConstraint<TValues>,
    pub hosting_element_id: Uid,
}

impl<'a, TTypes: ConstraintTraits, TValues: ConstraintTraits>
    LockedFieldsDigest<'a, TTypes, TValues>
{
    pub fn new(object_id: Uid) -> Self {
        Self {
            locked_fields: HashMap::new(),
            field_constraints: Cow::Owned(BTreeMap::new()),
            digest_object_id: object_id,
        }
    }
    pub fn get_unfulfilled_fields(&self) -> Vec<FieldConstraint<TTypes>> {
        self.field_constraints
            .iter()
            .filter(|(uid, _)| !self.locked_fields.contains_key(uid))
            .map(|(_uid, field_constraint)| field_constraint.clone())
            .collect()
    }
    pub fn get_ancestors_locked_fields(&self) -> Vec<LockedFieldDigest<TValues>> {
        self.locked_fields
            .values()
            .filter(|locked_field_digest| {
                locked_field_digest.hosting_element_id != self.digest_object_id
            })
            .cloned()
            .collect()
    }
}
