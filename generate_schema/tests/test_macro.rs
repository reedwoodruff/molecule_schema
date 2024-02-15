use std::{collections::HashMap, marker::PhantomData};

use generate_schema::generate_concrete_schema;
use output_types::GraphEnvironment;
use output_types::Uid;
use serde_types::common::*;
use serde_types::constraint_schema::*;
use serde_types::primitives::PrimitiveTypes;
use serde_types::primitives::PrimitiveValues;

// use super::*;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};

// #[test]
// fn create_json() {
//     type TTypesImpl = PrimitiveTypes;
//     type TValuesImpl = PrimitiveValues;
//
//     let mut constraint_objects = HashMap::new();
//
//     constraint_objects.insert(
//         0,
//         ConstraintObject {
//             trait_impls: HashMap::new(),
//             tag: Tag {
//                 name: "Person".to_string(),
//                 id: 0,
//             },
//             field_constraints: vec![
//                 FieldConstraint {
//                     tag: Tag {
//                         id: 0,
//                         name: "name".to_string(),
//                     },
//                     value_type: TTypesImpl::String,
//                 },
//                 FieldConstraint {
//                     tag: Tag {
//                         id: 1,
//                         name: "age".to_string(),
//                     },
//                     value_type: TTypesImpl::Option(Box::new(TTypesImpl::U32)),
//                 },
//             ],
//             // edge_constraints: vec![],
//             // constituents: vec![],
//             library_operatives: vec![],
//             trait_operatives: vec![],
//             instances: vec![],
//             _phantom: PhantomData,
//         },
//     );
//     constraint_objects.insert(
//         1,
//         ConstraintObject {
//             trait_impls: HashMap::from([(
//                 0,
//                 HashMap::from([(0, vec![TraitMethodImplPath::Field(0)])]),
//             )]),
//             tag: Tag {
//                 name: "Sock".to_string(),
//                 id: 1,
//             },
//             field_constraints: vec![FieldConstraint {
//                 tag: Tag {
//                     id: 0,
//                     name: "color".to_string(),
//                 },
//                 value_type: TTypesImpl::String,
//             }],
//             // edge_constraints: vec![],
//             // constituents: vec![],
//             library_operatives: vec![],
//             trait_operatives: vec![],
//             instances: vec![],
//             _phantom: PhantomData,
//         },
//     );
//     constraint_objects.insert(
//         2,
//         ConstraintObject {
//             trait_impls: HashMap::new(),
//             tag: Tag {
//                 name: "HasColoredObject".to_string(),
//                 id: 2,
//             },
//             field_constraints: vec![],
//             // edge_constraints: vec![
//             //     FuzzyEdgeDescriptor::new()
//             //         .dir(Dir::Recv)
//             //         .edge_type(EdgeType::Slot(0))
//             //         .add_target_schema_trait(0),
//             //     FuzzyEdgeDescriptor::new()
//             //         .dir(Dir::Emit)
//             //         .edge_type(EdgeType::Slot(1)),
//             // ],
//             // constituents: vec![],
//             library_operatives: vec![900],
//             trait_operatives: vec![TraitOperative {
//                 tag: Tag {
//                     id: 901,
//                     name: "ownee".to_string(),
//                 },
//                 trait_id: 0,
//             }],
//             instances: vec![800],
//             _phantom: PhantomData,
//         },
//     );
//
//     let mut traits = HashMap::new();
//     let color_trait = TraitDef {
//         tag: Tag {
//             id: 0,
//             name: "Color".to_string(),
//         },
//         methods: vec![TraitMethodDef {
//             tag: Tag {
//                 id: 0,
//                 name: "getColor".to_string(),
//             },
//             return_type: TTypesImpl::String,
//         }],
//     };
//     traits.insert(0, color_trait);
//
//     let mut library_operatives = HashMap::new();
//     let locked_age_field = FulfilledFieldConstraint {
//         tag: Tag {
//             id: 1,
//             name: "age".to_string(),
//         },
//         value_type: TTypesImpl::Option(Box::new(TTypesImpl::U32)),
//         value: TValuesImpl::Option(Box::new(TValuesImpl::U32(99))),
//     };
//     let owner_operative = LibraryOperative {
//         operative_library_id: None,
//         constraint_object_id: 0,
//         tag: Tag {
//             id: 900,
//             name: "owner".to_string(),
//         },
//         fulfilled_operatives: vec![],
//         locked_fields: vec![locked_age_field],
//         trait_impls: HashMap::new(),
//     };
//     library_operatives.insert(900, owner_operative);
//
//     let test_schema: ConstraintSchema<TTypesImpl, TValuesImpl> = ConstraintSchema {
//         constraint_objects: constraint_objects,
//         instance_library: Default::default(),
//         operative_library: library_operatives,
//         traits: traits,
//     };
//     println!("{}", to_string_pretty(&test_schema).unwrap());
//     panic!();
// }

#[test]
fn test_macro() {
    // let graph_environment =
    struct SampleGraphEnvironment {};

    impl GraphEnvironment for SampleGraphEnvironment {
        type Schema = Schema;

        fn get_element(&self, id: Uid) -> Option<Self::Schema> {
            None
        }
        fn instantiate_element(&self, element: &Self::Schema) -> Uid {
            0
        }
    }
    let sge_instance = SampleGraphEnvironment {};
    generate_concrete_schema!(sge_instance);
    // println!("{:?}", constraint_schema);
    // panic!();
    let test2 = Sock::initiate_build()
        .set_color("blue".to_string())
        .build()
        .unwrap();
    let test3 = Person::initiate_build()
        .set_name("blubber".to_string())
        .build()
        .unwrap();
    let test = HasColoredObject::initiate_build()
        .set_ownee(test2.id)
        .set_owner(test3.id)
        .build()
        .unwrap();

    println!("{:?}", test);
    println!("{:?}", test2);
    println!("{:?}", test3);
    println!("{:?}", test2.get_constraint_schema_id());
    println!("{:?}", test2.getColor());
    panic!();
}
