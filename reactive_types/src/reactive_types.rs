use base_types::common::{ConstraintTraits, Uid};
use std::{collections::HashMap, marker::PhantomData};
use strum_macros::{Display, EnumIter, EnumString};

use leptos::signal_prelude::*;

pub trait RCSO<TTypes: ConstraintTraits, TValues: ConstraintTraits> {
    fn get_fields(&self) -> Vec<impl Tagged + FieldInfo<TTypes, TValues>>;
}
pub trait Tagged {
    fn get_tag(&self) -> &RTag;
}
pub trait FieldInfo<TTypes: ConstraintTraits, TValues: ConstraintTraits> {
    fn get_value_type(&self) -> TTypes;
    fn get_value(&self) -> Option<TValues>;
}

macro_rules! apply_tagged {
    ( $($x:ident),* ) => {
        $(impl Tagged for $x {
            fn get_tag(&self) -> &RTag {
               &self.tag
            }
        })*
    };
    ( $($x:ident<$($t:ident),*>),* ) => {
        $(impl<$($t: ConstraintTraits),*> Tagged for $x<$($t),*> {
            fn get_tag(&self) -> &RTag {
               &self.tag
            }
        }),*
    };
}

apply_tagged!(RLibraryTemplate<TTypes,TValues>);
apply_tagged!(RLibraryOperative<TTypes, TValues>);

#[derive(Clone, Debug, PartialEq)]
pub struct RConstraintSchema<TTypes: ConstraintTraits, TValues: ConstraintTraits> {
    pub template_library: RwSignal<HashMap<Uid, RLibraryTemplate<TTypes, TValues>>>,
    pub instance_library: RwSignal<HashMap<Uid, RLibraryOperative<TTypes, TValues>>>,
    pub operative_library: RwSignal<HashMap<Uid, RLibraryOperative<TTypes, TValues>>>,
    pub traits: RwSignal<HashMap<Uid, RTraitDef<TTypes>>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct RTag {
    pub name: RwSignal<String>,
    pub id: RwSignal<Uid>,
}
impl RTag {
    pub fn new<T>(name: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            name: RwSignal::new(name.into()),
            id: RwSignal::new(uuid::Uuid::new_v4().as_u128()),
        }
    }
}

// Constraint Objects --------------------------------------------------------
#[derive(Clone, Debug, PartialEq)]
pub struct RLibraryTemplate<TTypes: ConstraintTraits, TValues: ConstraintTraits> {
    pub tag: RTag,
    pub field_constraints: RwSignal<HashMap<Uid, RFieldConstraint<TTypes>>>,
    pub operative_slots: RwSignal<HashMap<Uid, ROperativeSlot>>,
    pub instances: RwSignal<Vec<Uid>>,
    pub trait_impls: RwSignal<HashMap<Uid, RTraitImpl>>,
    pub _phantom: PhantomData<TValues>,
}
// impl<TTypes: ConstraintTraits, TValues: ConstraintTraits> RCSO<TTypes, TValues>
//     for RLibraryTemplate<TTypes, TValues>
// {
//     fn get_fields(&self) -> Vec<impl Tagged + FieldInfo<TTypes, TValues>> {
//         self.field_constraints.get()
//     }
// }
impl<TTypes: ConstraintTraits, TValues: ConstraintTraits> Default
    for RLibraryTemplate<TTypes, TValues>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<TTypes: ConstraintTraits, TValues: ConstraintTraits> RLibraryTemplate<TTypes, TValues> {
    pub fn new() -> Self {
        Self {
            field_constraints: RwSignal::new(HashMap::new()),
            operative_slots: RwSignal::new(HashMap::new()),
            instances: RwSignal::new(vec![]),
            trait_impls: RwSignal::new(HashMap::new()),
            tag: RTag::new("NewConstraintObject"),
            _phantom: PhantomData,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ROperativeSlot {
    pub tag: RTag,
    pub operative_descriptor: ROperativeVariants,
    pub bounds: RwSignal<RSlotBounds>,
}
impl ROperativeSlot {
    pub fn new(operative_id: ROperativeVariants, name: &str) -> Self {
        Self {
            tag: RTag::new(name),
            operative_descriptor: operative_id,
            bounds: RwSignal::new(RSlotBounds::default()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default, EnumIter, Display, EnumString)]
pub enum RSlotBounds {
    // Unbounded,
    #[default]
    Single,
    LowerBound(RwSignal<usize>),
    UpperBound(RwSignal<usize>),
    Range(RwSignal<usize>, RwSignal<usize>),
    LowerBoundOrZero(RwSignal<usize>),
    RangeOrZero(RwSignal<usize>, RwSignal<usize>),
}

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, EnumString, Hash)]
pub enum ROperativeVariants {
    LibraryOperative(RwSignal<Uid>),
    TraitOperative(RTraitOperative),
}

#[derive(Clone, Debug, PartialEq)]
pub struct RFieldConstraint<TTypes: ConstraintTraits> {
    pub tag: RTag,
    pub value_type: RwSignal<TTypes>,
}
impl<TTypes: ConstraintTraits, TValues: ConstraintTraits> FieldInfo<TTypes, TValues>
    for RFieldConstraint<TTypes>
{
    fn get_value_type(&self) -> TTypes {
        self.value_type.get()
    }

    fn get_value(&self) -> Option<TValues> {
        None
    }
}
apply_tagged!(RFieldConstraint<TTypes>);

impl<TTypes: ConstraintTraits> RFieldConstraint<TTypes> {
    pub fn fulfill<TValues: ConstraintTraits>(
        &self,
        value: TValues,
    ) -> RLockedFieldConstraint<TValues> {
        RLockedFieldConstraint {
            field_constraint_name: RwSignal::new(self.tag.name.get()),
            field_constraint_id: RwSignal::new(self.tag.id.get()),
            value: RwSignal::new(value),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct RTraitOperative {
    pub trait_ids: RwSignal<Vec<Uid>>,
    pub tag: RTag,
}
apply_tagged!(RTraitOperative);

pub type RTraitImpl = RwSignal<HashMap<Uid, RwSignal<Vec<RTraitMethodImplPath>>>>;

#[derive(Clone, Debug, PartialEq)]
pub enum RTraitMethodImplPath {
    // Denotes that the current path element has a field with id [Uid] which holds the
    // required information.
    Field(RwSignal<Uid>),
    // Denotes that the current path element implements a trait with the given method
    // which will return the required information
    TraitMethod {
        trait_id: RwSignal<Uid>,
        trait_method_id: RwSignal<Uid>,
    },
    Constituent(RwSignal<Uid>),
}

// Instance --------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub struct RSlottedInstances {
    pub operative_id: RwSignal<Uid>,
    pub operative_slot_id: RwSignal<Uid>,
    pub fulfilling_instance_ids: RwSignal<Vec<Uid>>,
}
impl RSlottedInstances {
    pub fn new(
        operative_slot_id: Uid,
        operative_id: Uid,
        fulfilling_instance_ids: Vec<Uid>,
    ) -> Self {
        Self {
            operative_slot_id: RwSignal::new(operative_slot_id),
            operative_id: RwSignal::new(operative_id),
            fulfilling_instance_ids: RwSignal::new(fulfilling_instance_ids),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RLockedFieldConstraint<TValues: ConstraintTraits> {
    pub field_constraint_id: RwSignal<Uid>,
    pub field_constraint_name: RwSignal<String>,
    pub value: RwSignal<TValues>,
}
impl<TValues: ConstraintTraits> RLockedFieldConstraint<TValues> {
    pub fn new(field_constraint_id: Uid, name: &str, value: TValues) -> Self {
        Self {
            field_constraint_name: RwSignal::new(name.to_string()),
            field_constraint_id: RwSignal::new(field_constraint_id),
            value: RwSignal::new(value),
        }
    }
}

// Operatives -------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq)]
pub struct RLibraryOperative<TTypes: ConstraintTraits, TValues: ConstraintTraits> {
    pub template_id: RwSignal<Uid>,
    // If the operative is based on another operative
    pub parent_operative_id: RwSignal<Option<Uid>>,
    pub tag: RTag,
    pub slotted_instances: RwSignal<HashMap<Uid, RSlottedInstances>>,
    pub locked_fields: RwSignal<HashMap<Uid, RLockedFieldConstraint<TValues>>>,
    pub trait_impls: RwSignal<HashMap<Uid, RTraitImpl>>,
    pub _phantom: PhantomData<TTypes>,
}
// impl<TValues: ConstraintTraits> RCSO<TValues> for RLibraryOperative<TTypes, TValues> {
//     fn get_fields(&self) -> Vec<impl Tagged + FieldInfo<TTypes, TValues>> {
//         Vec::<RFieldConstraint<TTypes>>::new()
//     }
// }
impl<TTypes: ConstraintTraits, TValues: ConstraintTraits> RLibraryOperative<TTypes, TValues> {
    pub fn new<T>(template_id: Uid, parent_operative_id: Option<Uid>, name: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            template_id: RwSignal::new(template_id),
            parent_operative_id: RwSignal::new(parent_operative_id),
            tag: RTag::new(name),
            slotted_instances: RwSignal::new(HashMap::new()),
            locked_fields: RwSignal::new(HashMap::new()),
            trait_impls: RwSignal::new(HashMap::new()),
            _phantom: PhantomData,
        }
    }
}
// Traits --------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq)]
pub struct RTraitDef<TTypes: ConstraintTraits> {
    pub tag: RTag,
    pub methods: RwSignal<HashMap<Uid, RTraitMethodDef<TTypes>>>,
}
impl<TTypes: ConstraintTraits> Default for RTraitDef<TTypes> {
    fn default() -> Self {
        Self::new()
    }
}

impl<TTypes: ConstraintTraits> RTraitDef<TTypes> {
    pub fn new() -> Self {
        Self {
            tag: RTag::new("new_trait"),
            methods: RwSignal::new(HashMap::new()),
        }
    }
}
apply_tagged!(RTraitDef<TTypes>);

#[derive(Clone, Debug, PartialEq)]
pub struct RTraitMethodDef<TTypes: ConstraintTraits> {
    pub tag: RTag,
    pub return_type: RwSignal<TTypes>,
}
impl<TTypes: ConstraintTraits> Default for RTraitMethodDef<TTypes> {
    fn default() -> Self {
        Self::new()
    }
}

impl<TTypes: ConstraintTraits> RTraitMethodDef<TTypes> {
    pub fn new() -> Self {
        Self {
            tag: RTag::new("new_method"),
            return_type: RwSignal::new(TTypes::default()),
        }
    }
}
apply_tagged!(RTraitMethodDef<TTypes>);
