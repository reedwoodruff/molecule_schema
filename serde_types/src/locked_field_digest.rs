use std::collections::HashMap;

use crate::{
    common::{ConstraintTraits, Uid},
    constraint_schema::FulfilledFieldConstraint,
};

pub struct LockedFieldsDigest<TValues: ConstraintTraits>(
    pub HashMap<Uid, LockedFieldDigest<TValues>>,
);

pub struct LockedFieldDigest<TValues: ConstraintTraits> {
    pub fulfilled_field: FulfilledFieldConstraint<TValues>,
    pub hosting_element_id: Uid,
}
