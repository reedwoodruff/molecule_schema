use crate::common::*;
use std::{collections::BTreeMap, marker::PhantomData};

pub type SlotId = Uid;
pub type TraitId = Uid;
pub type TraitMethodId = Uid;
pub type FieldId = Uid;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Default)]
pub struct ConstraintSchema<TTypes: ConstraintTraits, TValues: ConstraintTraits> {
    pub template_library: BTreeMap<Uid, LibraryTemplate<TTypes, TValues>>,
    pub instance_library: BTreeMap<Uid, LibraryOperative<TTypes, TValues>>,
    pub operative_library: BTreeMap<Uid, LibraryOperative<TTypes, TValues>>,
    pub traits: BTreeMap<Uid, TraitDef<TTypes>>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct LibraryTemplate<TTypes: ConstraintTraits, TValues: ConstraintTraits> {
    pub tag: Tag,
    pub field_constraints: BTreeMap<FieldId, FieldConstraint<TTypes>>,
    pub operative_slots: BTreeMap<SlotId, OperativeSlot>,
    pub trait_impls: BTreeMap<TraitId, TraitImpl>,
    pub instances: Vec<Uid>,
    #[cfg_attr(feature = "serde", serde(skip))]
    pub _phantom: PhantomData<TValues>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct LibraryOperative<TTypes: ConstraintTraits, TValues: ConstraintTraits> {
    pub tag: Tag,
    pub template_id: Uid,
    // If the operative is based on another operative
    pub parent_operative_id: Option<Uid>,
    pub slotted_instances: BTreeMap<SlotId, SlottedInstances>,
    pub locked_fields: BTreeMap<FieldId, LockedFieldConstraint<TValues>>,
    pub trait_impls: BTreeMap<TraitId, TraitImpl>,
    pub _phantom: PhantomData<TTypes>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct OperativeSlot {
    pub tag: Tag,
    pub operative_descriptor: OperativeVariants,
    pub bounds: SlotBounds,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub enum SlotBounds {
    // Unbounded,
    Single,
    LowerBound(usize),
    UpperBound(usize),
    Range(usize, usize),
    LowerBoundOrZero(usize),
    RangeOrZero(usize, usize),
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub enum OperativeVariants {
    LibraryOperative(Uid),
    TraitOperative(TraitOperative),
}
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct TraitOperative {
    pub trait_ids: Vec<Uid>,
    pub tag: Tag,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct TraitDef<TTypes: ConstraintTraits> {
    pub tag: Tag,
    pub methods: BTreeMap<Uid, TraitMethodDef<TTypes>>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct TraitMethodDef<TTypes: ConstraintTraits> {
    pub tag: Tag,
    pub return_type: TTypes,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct FieldConstraint<TTypes: ConstraintTraits> {
    pub tag: Tag,
    pub value_type: TTypes,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct LockedFieldConstraint<TValues: ConstraintTraits> {
    pub field_constraint_name: String,
    pub field_constraint_id: Uid,
    pub value: TValues,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct SlottedInstances {
    pub operative_slot_id: Uid,
    pub operative_id: Uid,
    pub fulfilling_instance_ids: Vec<Uid>,
}

pub type TraitImpl = BTreeMap<TraitMethodId, Vec<TraitMethodImplPath>>;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub enum TraitMethodImplPath {
    // Denotes that the current path element has a field with id [Uid] which holds the
    // required information.
    Field(Uid),
    // Denotes that the current path element implements a trait with the given method
    // which will return the required information
    TraitMethod { trait_id: Uid, trait_method_id: Uid },
    Constituent(Uid),
}
