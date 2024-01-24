use crate::common::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, marker::PhantomData};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConstraintSchema<TTypes: ConstraintTraits, TValues: ConstraintTraits> {
    pub constraint_objects: HashMap<Uid, ConstraintObject<TTypes>>,
    pub instance_library: HashMap<Uid, LibraryInstance<TTypes, TValues>>,
    pub operative_library: HashMap<Uid, LibraryOperative<TTypes, TValues>>,
    pub traits: HashMap<Uid, TraitDef<TTypes>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LibraryInstance<TTypes: ConstraintTraits, TValues: ConstraintTraits> {
    ref_id: Uid,
    id: Uid,
    other_edges: Vec<LibraryEdgeInstance>,
    // fulfilled_edges: Vec<FulfilledEdgeConstraint>,
    fulfilled_operatives: Vec<FulfilledOperative>,
    data: Vec<FulfilledFieldConstraint<TTypes, TValues>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum LibraryOperative<TTypes: ConstraintTraits, TValues: ConstraintTraits> {
    TemplateOperative {
        constraint_object_id: Uid,
        id: Uid,
        // other_edges: Vec<LibraryEdgeInstance>,
        // fulfilled_edges: Vec<FulfilledEdgeConstraint>,
        fulfilled_operatives: Vec<FulfilledOperative>,
        locked_fields: Vec<FulfilledFieldConstraint<TTypes, TValues>>,
        // operative_edges: Vec<FuzzyEdgeDescriptor>,
        // operative_fields: Vec<FieldConstraint<TTypes>>,
    },
    TraitOperative {
        trait_id: Uid,
    },
}
// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub struct LibraryOperative<TTypes: ConstraintTraits, TValues: ConstraintTraits> {
//     ref_id: Uid,
//     id: Uid,
//     other_edges: Vec<LibraryEdgeInstance>,
//     // fulfilled_edges: Vec<FulfilledEdgeConstraint>,
//     fulfilled_operatives: Vec<FulfilledOperative>,
//     locked_fields: Vec<FulfilledFieldConstraint<TTypes, TValues>>,
//     // operative_edges: Vec<FuzzyEdgeDescriptor>,
//     operative_fields: Vec<FieldConstraint<TTypes>>,
// }
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConstraintObject<TTypes: ConstraintTraits> {
    pub field_constraints: Vec<FieldConstraint<TTypes>>,
    // pub edge_constraints: Vec<FuzzyEdgeDescriptor>,
    // pub constituents: Vec<LibraryReference>,
    pub operatives: Vec<Uid>,
    pub instances: Vec<Uid>,
    pub trait_impls: HashMap<Uid, TraitImpl>,
    pub tag: Tag,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TraitDef<TTypes: ConstraintTraits> {
    pub tag: Tag,
    pub methods: Vec<TraitMethodDef<TTypes>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TraitMethodDef<TTypes: ConstraintTraits> {
    // pub trait_id: Uid,
    pub tag: Tag,
    pub return_type: TTypes,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TraitImpl {
    // Not unique
    pub trait_id: Uid,
    pub methods: HashMap<Uid, TraitMethodImpl>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TraitMethodImpl {
    pub trait_method_id: Uid,
    // Some way to describe where in either its structure or its native fields it will get the
    // information to return the required data type. Seems like this will likely be some kind of
    // DSL to describe structural locations, which will then be used upon instantiation to create
    // an actual method which will use those locations to generate a value when called.
    // The difficult thing I'm running into is how you might handle this for a single return type
    // which you may want to build from multiple locations.
    pub fulfillment_path: Vec<TraitPath>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TraitPath {
    // Denotes that the current path element has a field with id [Uid] which holds the
    // required information.
    Field(Uid),
    // Denotes jumping to a constituent element in the structure
    InstanceConstituent(Uid),
    OperativeConstituent(Uid),
    // Denotes that the current path element implements a trait of id [Uid1], which has a method of
    // id [Uid2] which, when invoked,
    // will return the required information
    Trait(Uid, Uid),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FieldConstraint<TTypes: ConstraintTraits> {
    pub tag: Tag,
    pub value_type: TTypes,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FulfilledFieldConstraint<TTypes: ConstraintTraits, TValues: ConstraintTraits> {
    pub constraint_tag: Tag,
    pub value_type: TTypes,
    pub value: TValues,
}

// #[derive(Serialize, Deserialize, Clone)]
// pub struct EdgeConstraint {
//     pub id: Uid,
//     pub edge_descriptor: FuzzyEdgeDescriptor,
// }

// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub struct FulfilledEdgeConstraint {
//     pub constraint_id: Uid,
//     pub edge: LibraryEdgeInstance,
// }
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FulfilledOperative {
    operative_id: Uid,
    fulfilling_instance_id: Uid,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum LibraryReference {
    Instance(Uid),
    Operative(Uid),
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LibraryEdgeInstance {
    pub dir: Dir,
    pub host: LibraryReference,
    pub target: LibraryReference,
    pub edge_type: EdgeType,
}
