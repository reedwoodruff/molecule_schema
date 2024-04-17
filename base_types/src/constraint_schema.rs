use crate::common::*;
use std::{collections::HashMap, marker::PhantomData};

pub type SlotId = Uid;
pub type TraitId = Uid;
pub type TraitMethodId = Uid;
pub type FieldId = Uid;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct ConstraintSchema<TTypes: ConstraintTraits, TValues: ConstraintTraits> {
    pub template_library: HashMap<Uid, LibraryTemplate<TTypes, TValues>>,
    pub instance_library: HashMap<Uid, LibraryOperative<TTypes, TValues>>,
    pub operative_library: HashMap<Uid, LibraryOperative<TTypes, TValues>>,
    pub traits: HashMap<Uid, TraitDef<TTypes>>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct LibraryTemplate<TTypes: ConstraintTraits, TValues: ConstraintTraits> {
    pub tag: Tag,
    pub field_constraints: HashMap<FieldId, FieldConstraint<TTypes>>,
    pub operative_slots: HashMap<SlotId, OperativeSlot>,
    pub trait_impls: HashMap<TraitId, TraitImpl>,
    pub instances: Vec<Uid>,
    pub _phantom: PhantomData<TValues>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct LibraryOperative<TTypes: ConstraintTraits, TValues: ConstraintTraits> {
    pub tag: Tag,
    pub template_id: Uid,
    // If the operative is based on another operative
    pub parent_operative_id: Option<Uid>,
    pub slotted_instances: HashMap<SlotId, SlottedInstances>,
    pub locked_fields: HashMap<FieldId, LockedFieldConstraint<TValues>>,
    pub trait_impls: HashMap<TraitId, TraitImpl>,
    pub _phantom: PhantomData<TTypes>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct OperativeSlot {
    pub tag: Tag,
    pub operative_descriptor: OperativeVariants,
    pub bounds: SlotBounds,
}

#[cfg(feature = "to_tokens")]
impl quote::ToTokens for OperativeSlot {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let tag = &self.tag;
        let bounds = &self.bounds;
        let operative_descriptor = self.operative_descriptor.clone();
        tokens.extend(quote::quote! {
            base_types::constraint_schema::OperativeSlot {
                tag: #tag,
                operative_descriptor: #operative_descriptor,
                bounds: #bounds,
            }
        })
    }
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

#[cfg(feature = "to_tokens")]
impl quote::ToTokens for SlotBounds {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ts = match *self {
            SlotBounds::Single => {
                quote::quote! {base_types::constraint_schema::SlotBounds::Single}
            }
            SlotBounds::LowerBound(lb) => {
                quote::quote! {base_types::constraint_schema::SlotBounds::LowerBound(#lb)}
            }
            SlotBounds::UpperBound(ub) => {
                quote::quote! {base_types::constraint_schema::SlotBounds::UpperBound(#ub)}
            }
            SlotBounds::Range(lb, ub) => {
                quote::quote! {base_types::constraint_schema::SlotBounds::Range(#lb,#ub)}
            }
            SlotBounds::LowerBoundOrZero(lb) => {
                quote::quote! {base_types::constraint_schema::SlotBounds::LowerBoundOrZero(#lb)}
            }
            SlotBounds::RangeOrZero(lb, ub) => {
                quote::quote! {base_types::constraint_schema::SlotBounds::RangeOrZero(#lb,#ub)}
            }
        };
        ts.to_tokens(tokens);
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub enum OperativeVariants {
    LibraryOperative(Uid),
    TraitOperative(TraitOperative),
}
#[cfg(feature = "to_tokens")]
impl quote::ToTokens for OperativeVariants {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ts = match self {
            OperativeVariants::TraitOperative(trait_op) => {
                quote::quote! {base_types::constraint_schema::OperativeVariants::TraitOperative(#trait_op)}
            }
            OperativeVariants::LibraryOperative(id) => {
                quote::quote! {base_types::constraint_schema::OperativeVariants::LibraryOperative(#id)}
            }
        };
        ts.to_tokens(tokens);
    }
}
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct TraitOperative {
    pub trait_ids: Vec<Uid>,
    pub tag: Tag,
}
#[cfg(feature = "to_tokens")]
impl quote::ToTokens for TraitOperative {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let trait_ids = &self.trait_ids;
        let tag = &self.tag;
        tokens.extend(quote::quote! {
            base_types::constraint_schema::TraitOperative {
                trait_ids: vec![#(#trait_ids,)*],
                tag: #tag,
            }
        })
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct TraitDef<TTypes: ConstraintTraits> {
    pub tag: Tag,
    pub methods: HashMap<Uid, TraitMethodDef<TTypes>>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct TraitMethodDef<TTypes: ConstraintTraits> {
    // pub trait_id: Uid,
    pub tag: Tag,
    pub return_type: TTypes,
}

pub type TraitImpl = HashMap<TraitMethodId, Vec<TraitMethodImplPath>>;

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
