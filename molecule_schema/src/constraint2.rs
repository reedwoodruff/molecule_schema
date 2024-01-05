use std::marker::PhantomData;

use crate::common::*;
use molecule_types::*;

/// Two possible states for each individual constraint:
/// 1. Unfulfilled
/// 2. Locked
///
/// Several possible states for the overall ConstraintObject
/// Totally unfulfilled
/// Partially locked
/// Totally locked would equate to being instantiated, and that construct should live in the instantiated library
/// #[derive(Clone)]
struct ConstraintObject<TTypes: ConstraintTraits, TValues: ConstraintTraits> {
    field_constraints: Vec<FieldConstraint<TTypes, TValues>>,
    edge_constraints: Vec<EdgeConstraint>,
    tag: SchemaTag,
}
// impl ConstraintObject {}

// trait TypedValue {
//     type ValueType;
// }

// macro_rules! create_typed_constraints {

//     ($($type:ident => $value:ty),*) => {
//         $(
//             #[derive(Clone)]
//             struct $type {
//                 phantom: PhantomData<$value>,
//             }
//         )*
//         // Generate the enum variants
//         enum TTypesImpl {
//             $($type($type)),*
//         }

//         // Implement the trait for each variant
//         $(
//             impl TypedValue for $type {
//                 type ValueType = $value;
//             }
//         )*
//     };
// }

// // Using the macro
// create_typed_constraints! {
//     String => String,
//     I32 => i32
//     // ... other types
// }

#[derive(Clone)]
struct FieldConstraint<TTypes: ConstraintTraits, TValues: ConstraintTraits> {
    id: Uid,
    name: String,
    value_type: TTypes,
    locked_value: Option<TValues>,
}

trait FieldConstraintTrait {
    type ValueType;
}

impl<TTypes: ConstraintTraits, TValues: ConstraintTraits> FieldConstraint<TTypes, TValues> {
    // fn fulfill(&self, val: TValues) -> FulfilledFieldConstraint<TTypes, TValues> {
    //     // TODO add error if already locked
    //     FulfilledFieldConstraint::<TTypes, TValues> {
    //         constraint_id: self.id,
    //         name: self.name.clone(),
    //         value_type: self.value_type.clone(),
    //         value: val,
    //     }
    // }
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
// struct FulfilledFieldConstraint<TTypes: ConstraintTraits, TValues: ConstraintTraits> {
//     pub constraint_id: Uid,
//     pub name: String,
//     pub value_type: TTypes,
//     pub value: TValues,
// }

#[derive(Clone)]
struct EdgeConstraint {
    id: Uid,
    edge_type: EdgeType,
    dir: Dir,
    target_constraint: Option<TargetConstraint>,
}

#[derive(Clone)]
struct TargetConstraint {
    allowed_tags: Vec<SchemaTag>,
}

impl EdgeConstraint {
    fn fulfill(&self, target: Uid) -> FulfilledEdgeConstraint {
        FulfilledEdgeConstraint {
            ref_constraint: self.clone(),
            target: target,
        }
    }
}

struct FulfilledEdgeConstraint {
    pub ref_constraint: EdgeConstraint,
    pub target: Uid,
}
