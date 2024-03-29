use std::collections::HashMap;

use crate::{
    common::{ConstraintTraits, Uid},
    constraint_schema::LockedFieldConstraint,
};

pub struct LockedFieldsDigest<TValues: ConstraintTraits>(
    pub HashMap<Uid, LockedFieldDigest<TValues>>,
);

pub struct LockedFieldDigest<TValues: ConstraintTraits> {
    pub fulfilled_field: LockedFieldConstraint<TValues>,
    pub hosting_element_id: Uid,
}
