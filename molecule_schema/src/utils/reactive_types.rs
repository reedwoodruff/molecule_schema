use std::{collections::HashMap, marker::PhantomData};
use strum_macros::{Display, EnumIter, EnumString};

use leptos::signal_prelude::*;

use serde_types::{
    common::{ConstraintTraits, Tag, Uid},
    constraint_schema::{
        ConstraintSchema, FieldConstraint, LibraryOperative, LibraryTemplate,
        LockedFieldConstraint, OperativeSlot, OperativeVariants, SlotBounds, SlottedInstances,
        TraitDef, TraitMethodDef, TraitMethodImplPath, TraitOperative,
    },
};

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
impl<TTypes: ConstraintTraits, TValues: ConstraintTraits> From<ConstraintSchema<TTypes, TValues>>
    for RConstraintSchema<TTypes, TValues>
{
    fn from(value: ConstraintSchema<TTypes, TValues>) -> Self {
        Self {
            template_library: RwSignal::new(
                value
                    .template_library
                    .iter()
                    .map(|(index, item)| (*index, item.clone().into()))
                    .collect(),
            ),
            instance_library: RwSignal::new(
                value
                    .instance_library
                    .iter()
                    .map(|(index, item)| (*index, item.clone().into()))
                    .collect(),
            ),
            operative_library: RwSignal::new(
                value
                    .operative_library
                    .iter()
                    .map(|(index, item)| (*index, item.clone().into()))
                    .collect(),
            ),
            traits: RwSignal::new(
                value
                    .traits
                    .iter()
                    .map(|(index, item)| (*index, item.clone().into()))
                    .collect(),
            ),
        }
    }
}
impl<TTypes: ConstraintTraits, TValues: ConstraintTraits> From<RConstraintSchema<TTypes, TValues>>
    for ConstraintSchema<TTypes, TValues>
{
    fn from(val: RConstraintSchema<TTypes, TValues>) -> Self {
        Self {
            template_library: val
                .template_library
                .get()
                .iter()
                .map(|(index, item)| (*index, item.clone().into()))
                .collect(),

            instance_library: val
                .instance_library
                .get()
                .iter()
                .map(|(index, item)| (*index, item.clone().into()))
                .collect(),

            operative_library: val
                .operative_library
                .get()
                .iter()
                .map(|(index, item)| (*index, item.clone().into()))
                .collect(),

            traits: val
                .traits
                .get()
                .iter()
                .map(|(index, item)| (*index, item.clone().into()))
                .collect(),
        }
    }
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
impl From<Tag> for RTag {
    fn from(value: Tag) -> Self {
        Self {
            name: RwSignal::new(value.name),
            id: RwSignal::new(value.id),
        }
    }
}
impl From<RTag> for Tag {
    fn from(value: RTag) -> Self {
        Self {
            name: value.name.get(),
            id: value.id.get(),
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
impl<TTypes: ConstraintTraits, TValues: ConstraintTraits> From<LibraryTemplate<TTypes, TValues>>
    for RLibraryTemplate<TTypes, TValues>
{
    fn from(value: LibraryTemplate<TTypes, TValues>) -> Self {
        Self {
            field_constraints: RwSignal::new(
                value
                    .field_constraints
                    .iter()
                    .map(|(index, item)| (*index, item.clone().into()))
                    .collect(),
            ),
            operative_slots: RwSignal::new(
                value
                    .operative_slots
                    .iter()
                    .map(|(index, item)| (*index, item.clone().into()))
                    .collect(),
            ),

            instances: RwSignal::new(value.instances),
            trait_impls: RwSignal::new(
                value
                    .trait_impls
                    .iter()
                    .map(|(index, item)| {
                        (
                            *index,
                            RwSignal::new(
                                item.iter()
                                    .map(|(index_inner, inner)| {
                                        (
                                            *index_inner,
                                            RwSignal::new(
                                                inner
                                                    .clone()
                                                    .iter()
                                                    .cloned()
                                                    .map(|vec_item| vec_item.into())
                                                    .collect(),
                                            ),
                                        )
                                    })
                                    .collect(),
                            ),
                        )
                    })
                    .collect(),
            ),

            tag: value.tag.into(),
            _phantom: PhantomData,
        }
    }
}
impl<TTypes: ConstraintTraits, TValues: ConstraintTraits> From<RLibraryTemplate<TTypes, TValues>>
    for LibraryTemplate<TTypes, TValues>
{
    fn from(value: RLibraryTemplate<TTypes, TValues>) -> Self {
        Self {
            field_constraints: value
                .field_constraints
                .get()
                .iter()
                .map(|(index, item)| (*index, item.clone().into()))
                .collect(),

            operative_slots: value
                .operative_slots
                .get()
                .iter()
                .map(|(index, item)| (*index, item.clone().into()))
                .collect(),

            instances: value.instances.get(),
            trait_impls: value
                .trait_impls
                .get()
                .iter()
                .map(|(index, item)| {
                    (
                        *index,
                        item.get()
                            .iter()
                            .map(|(index_inner, inner)| {
                                (
                                    *index_inner,
                                    inner
                                        .get()
                                        .clone()
                                        .iter()
                                        .cloned()
                                        .map(|vec_item| vec_item.into())
                                        .collect(),
                                )
                            })
                            .collect(),
                    )
                })
                .collect(),

            tag: value.tag.into(),
            _phantom: PhantomData,
        }
    }
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
impl From<SlotBounds> for RSlotBounds {
    fn from(value: SlotBounds) -> Self {
        match value {
            // SlotBounds::Unbounded => RSlotBounds::Unbounded,
            SlotBounds::Single => RSlotBounds::Single,
            SlotBounds::LowerBound(val) => RSlotBounds::LowerBound(RwSignal::new(val)),
            SlotBounds::UpperBound(val) => RSlotBounds::UpperBound(RwSignal::new(val)),
            SlotBounds::Range(lower, upper) => {
                RSlotBounds::Range(RwSignal::new(lower), RwSignal::new(upper))
            }
            SlotBounds::LowerBoundOrZero(val) => RSlotBounds::LowerBoundOrZero(RwSignal::new(val)),
            SlotBounds::RangeOrZero(lower, upper) => {
                RSlotBounds::RangeOrZero(RwSignal::new(lower), RwSignal::new(upper))
            }
        }
    }
}
impl From<RSlotBounds> for SlotBounds {
    fn from(value: RSlotBounds) -> Self {
        match value {
            // RSlotBounds::Unbounded => SlotBounds::Unbounded,
            RSlotBounds::Single => SlotBounds::Single,
            RSlotBounds::LowerBound(val) => SlotBounds::LowerBound(val.get()),
            RSlotBounds::UpperBound(val) => SlotBounds::UpperBound(val.get()),
            RSlotBounds::Range(lower, upper) => SlotBounds::Range(lower.get(), upper.get()),
            RSlotBounds::LowerBoundOrZero(val) => SlotBounds::LowerBoundOrZero(val.get()),
            RSlotBounds::RangeOrZero(lower, upper) => {
                SlotBounds::RangeOrZero(lower.get(), upper.get())
            }
        }
    }
}

impl From<OperativeSlot> for ROperativeSlot {
    fn from(value: OperativeSlot) -> Self {
        Self {
            tag: value.tag.into(),
            operative_descriptor: value.operative_descriptor.into(),
            bounds: RwSignal::new(value.bounds.into()),
        }
    }
}
impl From<ROperativeSlot> for OperativeSlot {
    fn from(value: ROperativeSlot) -> Self {
        Self {
            tag: value.tag.into(),
            operative_descriptor: value.operative_descriptor.into(),
            bounds: value.bounds.get().into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, EnumString, Hash)]
pub enum ROperativeVariants {
    LibraryOperative(RwSignal<Uid>),
    TraitOperative(RTraitOperative),
}
impl From<OperativeVariants> for ROperativeVariants {
    fn from(value: OperativeVariants) -> Self {
        match value {
            OperativeVariants::TraitOperative(val) => {
                ROperativeVariants::TraitOperative(val.into())
            }
            OperativeVariants::LibraryOperative(val) => {
                ROperativeVariants::LibraryOperative(RwSignal::new(val))
            }
        }
    }
}
impl From<ROperativeVariants> for OperativeVariants {
    fn from(value: ROperativeVariants) -> Self {
        match value {
            ROperativeVariants::TraitOperative(val) => {
                OperativeVariants::TraitOperative(val.into())
            }
            ROperativeVariants::LibraryOperative(val) => {
                OperativeVariants::LibraryOperative(val.get())
            }
        }
    }
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

impl<TTypes: ConstraintTraits> From<FieldConstraint<TTypes>> for RFieldConstraint<TTypes> {
    fn from(value: FieldConstraint<TTypes>) -> Self {
        Self {
            tag: value.tag.into(),
            value_type: RwSignal::new(value.value_type),
        }
    }
}
impl<TTypes: ConstraintTraits> From<RFieldConstraint<TTypes>> for FieldConstraint<TTypes> {
    fn from(value: RFieldConstraint<TTypes>) -> Self {
        Self {
            tag: value.tag.into(),
            value_type: value.value_type.get(),
        }
    }
}
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
impl From<TraitOperative> for RTraitOperative {
    fn from(value: TraitOperative) -> Self {
        Self {
            trait_ids: RwSignal::new(value.trait_ids),
            tag: value.tag.into(),
        }
    }
}
impl From<RTraitOperative> for TraitOperative {
    fn from(value: RTraitOperative) -> Self {
        Self {
            trait_ids: value.trait_ids.get(),
            tag: value.tag.into(),
        }
    }
}

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
impl From<TraitMethodImplPath> for RTraitMethodImplPath {
    fn from(value: TraitMethodImplPath) -> Self {
        match value {
            TraitMethodImplPath::Field(val) => RTraitMethodImplPath::Field(RwSignal::new(val)),
            TraitMethodImplPath::TraitMethod {
                trait_id,
                trait_method_id,
            } => RTraitMethodImplPath::TraitMethod {
                trait_id: RwSignal::new(trait_id),
                trait_method_id: RwSignal::new(trait_method_id),
            },
            TraitMethodImplPath::Constituent(val) => {
                RTraitMethodImplPath::Constituent(RwSignal::new(val))
            }
        }
    }
}
impl From<RTraitMethodImplPath> for TraitMethodImplPath {
    fn from(value: RTraitMethodImplPath) -> Self {
        match value {
            RTraitMethodImplPath::Field(val) => TraitMethodImplPath::Field(val.get()),
            RTraitMethodImplPath::TraitMethod {
                trait_id,
                trait_method_id,
            } => TraitMethodImplPath::TraitMethod {
                trait_id: trait_id.get(),
                trait_method_id: trait_method_id.get(),
            },
            RTraitMethodImplPath::Constituent(val) => TraitMethodImplPath::Constituent(val.get()),
        }
    }
}

// Instance --------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub struct RSlottedInstances {
    pub operative_id: RwSignal<Uid>,
    pub operative_slot_id: RwSignal<Uid>,
    pub fulfilling_instance_ids: RwSignal<Vec<Uid>>,
}
impl From<SlottedInstances> for RSlottedInstances {
    fn from(value: SlottedInstances) -> Self {
        Self {
            operative_slot_id: RwSignal::new(value.operative_slot_id),
            operative_id: RwSignal::new(value.operative_id),
            fulfilling_instance_ids: RwSignal::new(value.fulfilling_instance_ids),
        }
    }
}
impl From<RSlottedInstances> for SlottedInstances {
    fn from(value: RSlottedInstances) -> Self {
        Self {
            operative_slot_id: value.operative_slot_id.get(),
            operative_id: value.operative_id.get(),
            fulfilling_instance_ids: value.fulfilling_instance_ids.get(),
        }
    }
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
impl<TValues: ConstraintTraits> From<LockedFieldConstraint<TValues>>
    for RLockedFieldConstraint<TValues>
{
    fn from(value: LockedFieldConstraint<TValues>) -> Self {
        Self {
            field_constraint_name: RwSignal::new(value.field_constraint_name),
            field_constraint_id: value.field_constraint_id.into(),
            value: RwSignal::new(value.value),
        }
    }
}
impl<TValues: ConstraintTraits> From<RLockedFieldConstraint<TValues>>
    for LockedFieldConstraint<TValues>
{
    fn from(value: RLockedFieldConstraint<TValues>) -> Self {
        Self {
            field_constraint_name: value.field_constraint_name.get(),
            field_constraint_id: value.field_constraint_id.get(),
            value: value.value.get(),
        }
    }
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
impl<TTypes: ConstraintTraits, TValues: ConstraintTraits> From<LibraryOperative<TTypes, TValues>>
    for RLibraryOperative<TTypes, TValues>
{
    fn from(value: LibraryOperative<TTypes, TValues>) -> Self {
        Self {
            template_id: RwSignal::new(value.template_id),
            parent_operative_id: RwSignal::new(value.parent_operative_id),
            tag: value.tag.into(),
            slotted_instances: RwSignal::new(
                value
                    .slotted_instances
                    .iter()
                    .map(|(index, item)| (*index, item.clone().into()))
                    .collect(),
            ),
            locked_fields: RwSignal::new(
                value
                    .locked_fields
                    .iter()
                    .map(|(index, item)| (*index, item.clone().into()))
                    .collect(),
            ),
            trait_impls: RwSignal::new(
                value
                    .trait_impls
                    .iter()
                    .map(|(index, item)| {
                        (
                            *index,
                            RwSignal::new(
                                item.iter()
                                    .map(|(index_inner, inner)| {
                                        (
                                            *index_inner,
                                            RwSignal::new(
                                                inner
                                                    .clone()
                                                    .iter()
                                                    .cloned()
                                                    .map(|vec_item| vec_item.into())
                                                    .collect(),
                                            ),
                                        )
                                    })
                                    .collect(),
                            ),
                        )
                    })
                    .collect(),
            ),
            _phantom: value._phantom,
        }
    }
}
impl<TTypes: ConstraintTraits, TValues: ConstraintTraits> From<RLibraryOperative<TTypes, TValues>>
    for LibraryOperative<TTypes, TValues>
{
    fn from(value: RLibraryOperative<TTypes, TValues>) -> Self {
        Self {
            template_id: value.template_id.get(),
            parent_operative_id: value.parent_operative_id.get(),
            tag: value.tag.into(),
            slotted_instances: value
                .slotted_instances
                .get()
                .iter()
                .map(|(index, item)| (*index, item.clone().into()))
                .collect(),
            locked_fields: value
                .locked_fields
                .get()
                .iter()
                .map(|(index, item)| (*index, item.clone().into()))
                .collect(),
            trait_impls: value
                .trait_impls
                .get()
                .iter()
                .map(|(index, item)| {
                    (
                        *index,
                        item.get()
                            .iter()
                            .map(|(index_inner, inner)| {
                                (
                                    *index_inner,
                                    inner
                                        .get()
                                        .clone()
                                        .iter()
                                        .cloned()
                                        .map(|vec_item| vec_item.into())
                                        .collect(),
                                )
                            })
                            .collect(),
                    )
                })
                .collect(),
            _phantom: value._phantom,
        }
    }
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
impl<TTypes: ConstraintTraits> From<TraitDef<TTypes>> for RTraitDef<TTypes> {
    fn from(value: TraitDef<TTypes>) -> Self {
        Self {
            tag: value.tag.into(),
            methods: RwSignal::new(
                value
                    .methods
                    .iter()
                    .map(|(index, item)| (*index, item.clone().into()))
                    .collect(),
            ),
        }
    }
}
impl<TTypes: ConstraintTraits> From<RTraitDef<TTypes>> for TraitDef<TTypes> {
    fn from(value: RTraitDef<TTypes>) -> Self {
        Self {
            tag: value.tag.into(),
            methods: value
                .methods
                .get()
                .iter()
                .map(|(index, item)| (*index, item.clone().into()))
                .collect(),
        }
    }
}

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
impl<TTypes: ConstraintTraits> From<TraitMethodDef<TTypes>> for RTraitMethodDef<TTypes> {
    fn from(value: TraitMethodDef<TTypes>) -> Self {
        Self {
            tag: value.tag.into(),
            return_type: RwSignal::new(value.return_type),
        }
    }
}
impl<TTypes: ConstraintTraits> From<RTraitMethodDef<TTypes>> for TraitMethodDef<TTypes> {
    fn from(value: RTraitMethodDef<TTypes>) -> Self {
        Self {
            tag: value.tag.into(),
            return_type: value.return_type.get(),
        }
    }
}
