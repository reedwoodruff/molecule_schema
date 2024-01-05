use std::marker::PhantomData;

use crate::common::*;
use molecule_types::*;

#[derive(Clone)]
pub struct ConstraintObject<TTypes: ConstraintTraits, TValues: ConstraintTraits> {
    pub field_constraints: Vec<FieldConstraint<TTypes, TValues>>,
    // edge_constraints: Vec<EdgeConstraint>,
    // tag: SchemaTag,
}
#[derive(Clone)]
pub struct FieldConstraint<TTypes: ConstraintTraits, TValues: ConstraintTraits> {
    pub id: Uid,
    pub name: String,
    pub value_type: TTypes,
    pub locked_value: Option<TValues>,
}

impl<TTypes: ConstraintTraits, TValues: ConstraintTraits> FieldConstraint<TTypes, TValues> {
    fn lock(&self, val: TValues) -> Self {
        // TODO add error if already locked
        Self {
            locked_value: Some(val),
            id: self.id,
            name: self.name.clone(),
            value_type: self.value_type.clone(),
        }
    }
}

// #[derive(Clone)]
// struct EdgeConstraint {
//     id: Uid,
//     edge_type: EdgeType,
//     dir: Dir,
//     target_constraint: Option<TargetConstraint>,
// }

// #[derive(Clone)]
// struct TargetConstraint {
//     allowed_tags: Vec<SchemaTag>,
// }

// impl EdgeConstraint {
//     fn fulfill(&self, target: Uid) -> FulfilledEdgeConstraint {
//         FulfilledEdgeConstraint {
//             ref_constraint: self.clone(),
//             target: target,
//         }
//     }
// }

// struct FulfilledEdgeConstraint {
//     pub ref_constraint: EdgeConstraint,
//     pub target: Uid,
// }
