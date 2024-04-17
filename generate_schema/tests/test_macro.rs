use std::collections::HashMap;

use base_types::common::{ConstraintTraits, Uid};
use base_types::constraint_schema::ConstraintSchema;
use base_types::primitives::{PrimitiveTypes, PrimitiveValues};
use base_types::traits::{BaseGraphEnvironment, GraphEnvironment, GSO};
use generate_schema::generate_concrete_schema;

// use super::*;

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
    // struct SampleGraphEnvironment<TSchema: GSO> {
    //     created_instances: HashMap<Uid, TSchema>,
    //     constraint_schema: ConstraintSchema<PrimitiveTypes, PrimitiveValues>,
    // }
    //
    // impl<TTSchema: GSO> GraphEnvironment for SampleGraphEnvironment<TTSchema> {
    //     type Schema = TTSchema;
    //     type Types = PrimitiveTypes;
    //     type Values = PrimitiveValues;
    //
    //     fn get_constraint_schema(&self) -> &ConstraintSchema<Self::Types, Self::Values> {
    //         &self.constraint_schema
    //     }
    //
    //     fn get_element(&self, id: &Uid) -> Option<&Self::Schema> {
    //         self.created_instances.get(id)
    //     }
    //     fn instantiate_element(&mut self, element: Self::Schema) -> Uid {
    //         // let id = uuid::Uuid::new_v4().as_u128();
    //         let id = element.get_id();
    //         self.created_instances.insert(id, element);
    //         id
    //     }
    // }
    constraint_schema::constraint_schema!();
    let mut sge_instance = BaseGraphEnvironment {
        created_instances: HashMap::<Uid, Schema>::new(),
        constraint_schema: constraint_schema_generated,
    };
    generate_concrete_schema!(sge_instance);

    let new_word = WordOp::initiate_build()
        .set_display("Clunk".to_string())
        .build()
        .unwrap();
    let new_word2 = WordOp::initiate_build()
        .set_display("Clip".to_string())
        .build()
        .unwrap();
    let new_linear_displayable = LinearDisplayableOp::initiate_build()
        .add_new_latter(new_word)
        .add_new_former(new_word2)
        .build()
        .unwrap();
    let new_punctuation_op = PunctuationOp::initiate_build()
        .set_display(".".to_string())
        .build()
        .unwrap();
    let new_sen = SentenceOp::initiate_build()
        .add_new_linear_displayable(new_linear_displayable)
        .add_new_final_punctuation(new_punctuation_op)
        .build()
        .unwrap()
        .flatten();

    for element in new_sen {
        println!("{:#?}", element);
    }
    panic!();
    // let mut new_sen = SentenceOp::initiate_build();
    // let mut new_linear_displayable = LinearDisplayableOp::initiate_build();
    // let new_linear_displayable = new_linear_displayable.build().unwrap();
    // new_sen.add_new_linear_displayable(new_linear_displayable);

    // LinearDisplayableOp::initiate_build().
}
