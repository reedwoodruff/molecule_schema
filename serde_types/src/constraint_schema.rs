use crate::common::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, marker::PhantomData};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConstraintSchema<TTypes: ConstraintTraits, TValues: ConstraintTraits> {
    pub template_library: HashMap<Uid, LibraryTemplate<TTypes, TValues>>,
    pub instance_library: HashMap<Uid, LibraryOperative<TTypes, TValues>>,
    pub operative_library: HashMap<Uid, LibraryOperative<TTypes, TValues>>,
    pub traits: HashMap<Uid, TraitDef<TTypes>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LibraryTemplate<TTypes: ConstraintTraits, TValues: ConstraintTraits> {
    pub tag: Tag,
    pub field_constraints: HashMap<Uid, FieldConstraint<TTypes>>,
    pub operative_slots: HashMap<Uid, OperativeSlot>,
    pub trait_impls: HashMap<Uid, TraitImpl>,
    pub instances: Vec<Uid>,
    pub _phantom: PhantomData<TValues>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LibraryOperative<TTypes: ConstraintTraits, TValues: ConstraintTraits> {
    pub tag: Tag,
    pub template_id: Uid,
    // If the operative is based on another operative
    pub parent_operative_id: Option<Uid>,
    pub slotted_instances: HashMap<Uid, SlottedInstances>,
    pub locked_fields: HashMap<Uid, LockedFieldConstraint<TValues>>,
    pub trait_impls: HashMap<Uid, TraitImpl>,
    pub _phantom: PhantomData<TTypes>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OperativeSlot {
    pub tag: Tag,
    pub operative_descriptor: OperativeVariants,
    pub bounds: SlotBounds,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SlotBounds {
    // Unbounded,
    Single,
    LowerBound(usize),
    UpperBound(usize),
    Range(usize, usize),
    LowerBoundOrZero(usize),
    RangeOrZero(usize, usize),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum OperativeVariants {
    LibraryOperative(Uid),
    TraitOperative(TraitOperative),
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TraitOperative {
    pub trait_ids: Vec<Uid>,
    pub tag: Tag,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TraitDef<TTypes: ConstraintTraits> {
    pub tag: Tag,
    pub methods: HashMap<Uid, TraitMethodDef<TTypes>>,
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
    TraitMethod { trait_id: Uid, trait_method_id: Uid },
    Constituent(Uid),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FieldConstraint<TTypes: ConstraintTraits> {
    pub tag: Tag,
    pub value_type: TTypes,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LockedFieldConstraint<TValues: ConstraintTraits> {
    pub field_constraint_name: String,
    pub field_constraint_id: Uid,
    pub value: TValues,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SlottedInstances {
    pub operative_slot_id: Uid,
    pub operative_id: Uid,
    pub fulfilling_instance_ids: Vec<Uid>,
}
