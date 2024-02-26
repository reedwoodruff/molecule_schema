use crate::common::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, marker::PhantomData};
use strum_macros::{AsRefStr, Display};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConstraintSchema<TTypes: ConstraintTraits, TValues: ConstraintTraits> {
    pub template_library: HashMap<Uid, LibraryTemplate<TTypes, TValues>>,
    pub instance_library: HashMap<Uid, LibraryInstance<TTypes, TValues>>,
    pub operative_library: HashMap<Uid, LibraryOperative<TTypes, TValues>>,
    pub traits: HashMap<Uid, TraitDef<TTypes>>,
}

#[derive(Display, AsRefStr)]
pub enum ConstraintSchemaInstantiableType {
    Template,
    Instance,
    Operative,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LibraryTemplate<TTypes: ConstraintTraits, TValues: ConstraintTraits> {
    pub field_constraints: Vec<FieldConstraint<TTypes>>,
    pub library_operatives: Vec<Uid>,
    pub trait_operatives: Vec<TraitOperative>,
    pub instances: Vec<Uid>,
    pub trait_impls: HashMap<Uid, TraitImpl>,
    pub tag: Tag,
    pub _phantom: PhantomData<TValues>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LibraryOperative<TTypes: ConstraintTraits, TValues: ConstraintTraits> {
    pub template_id: Uid,
    // If the operative is based on another operative
    pub parent_operative_id: Option<Uid>,
    pub tag: Tag,
    pub fulfilled_library_operatives: Vec<FulfilledOperative>,
    pub fulfilled_trait_operatives: Vec<FulfilledOperative>,
    pub locked_fields: Vec<FulfilledFieldConstraint<TTypes, TValues>>,
    pub trait_impls: HashMap<Uid, TraitImpl>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LibraryInstance<TTypes: ConstraintTraits, TValues: ConstraintTraits> {
    pub template_id: Uid,
    // If the instance is of a particular operative
    pub parent_operative_id: Option<Uid>,
    pub tag: Tag,
    // pub other_edges: Vec<LibraryEdgeInstance>,
    pub other_edges: Vec<FulfilledOperative>,
    pub fulfilled_library_operatives: Vec<FulfilledOperative>,
    pub fulfilled_trait_operatives: Vec<FulfilledOperative>,
    pub data: Vec<FulfilledFieldConstraint<TTypes, TValues>>,
    pub trait_impls: HashMap<Uid, TraitImpl>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TraitOperative {
    pub trait_id: Uid,
    pub tag: Tag,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum OperativeVariants {
    LibraryOperative(Uid),
    TraitOperative(Uid),
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

pub type TraitImpl = HashMap<Uid, Vec<TraitMethodImplPath>>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TraitMethodImplPath {
    // Denotes that the current path element has a field with id [Uid] which holds the
    // required information.
    Field(Uid),
    // Denotes that the current path element implements a trait with the given method
    // which will return the required information
    TraitMethod {
        trait_id: Uid,
        trait_method_id: Uid,
    },
    // Denotes jumping to a constituent element in the structure
    InstanceConstituent(Uid),
    LibraryOperativeConstituent(Uid),
    // Denotes that the current path element has an operative element [Uid1]
    // which implements a trait of id [Uid2], which has a method of
    // id [Uid2] which, when invoked,
    // will return the required information
    TraitOperativeConstituent {
        trait_operative_id: Uid,
        trait_id: Uid,
        trait_method_id: Uid,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FieldConstraint<TTypes: ConstraintTraits> {
    pub tag: Tag,
    pub value_type: TTypes,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FulfilledFieldConstraint<TTypes: ConstraintTraits, TValues: ConstraintTraits> {
    pub tag: Tag,
    pub value_type: TTypes,
    pub value: TValues,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FulfilledOperative {
    pub operative_id: Uid,
    pub fulfilling_instance_id: Uid,
}
