use crate::{common::ConstraintTraits, constraint::ConstraintObject};

pub struct ConstraintSchema<TTypes: ConstraintTraits, TValues: ConstraintTraits> {
    constraint_objects: Vec<ConstraintObject<TTypes, TValues>>,
}

#[cfg(test)]
mod tests {
    use crate::constraint::FieldConstraint;

    use super::*;

    #[test]
    fn test() {
        #[derive(Clone, Debug, PartialEq)]
        enum TTypesImpl {
            String,
            I32,
        }
        #[derive(Clone, Debug, PartialEq)]
        enum TValuesImpl {
            String(String),
            I32(i32),
        }
        impl ConstraintTraits for TTypesImpl {}
        impl ConstraintTraits for TValuesImpl {}

        let test_schema = ConstraintSchema::<TTypesImpl, TValuesImpl> {
            constraint_objects: vec![ConstraintObject::<TTypesImpl, TValuesImpl> {
                field_constraints: vec![FieldConstraint::<TTypesImpl, TValuesImpl> {
                    id: 0,
                    name: "test".to_string(),
                    value_type: TTypesImpl::String,
                    locked_value: None,
                }],
            }],
        };
        // test_schema.
    }
}
