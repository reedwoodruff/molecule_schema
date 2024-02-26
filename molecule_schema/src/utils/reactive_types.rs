use std::{collections::HashMap, marker::PhantomData};

use leptos::signal_prelude::*;
use leptos::*;
use serde_types::{
    common::{ConstraintTraits, Tag, Uid},
    constraint_schema::{
        ConstraintSchema, FieldConstraint, FulfilledFieldConstraint, FulfilledOperative,
        LibraryInstance, LibraryOperative, LibraryTemplate, OperativeVariants, TraitDef,
        TraitMethodDef, TraitMethodImplPath, TraitOperative,
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
apply_tagged!(RLibraryInstance<TTypes,TValues>);
apply_tagged!(RLibraryOperative<TTypes,TValues>);

#[derive(Clone, Debug, PartialEq)]
pub struct RConstraintSchema<TTypes: ConstraintTraits, TValues: ConstraintTraits> {
    pub template_library: RwSignal<HashMap<Uid, RLibraryTemplate<TTypes, TValues>>>,
    pub instance_library: RwSignal<HashMap<Uid, RLibraryInstance<TTypes, TValues>>>,
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
    pub field_constraints: RwSignal<Vec<RFieldConstraint<TTypes>>>,
    pub library_operatives: RwSignal<Vec<Uid>>,
    pub trait_operatives: RwSignal<Vec<RTraitOperative>>,
    pub instances: RwSignal<Vec<Uid>>,
    pub trait_impls: RwSignal<HashMap<Uid, RTraitImpl>>,
    pub tag: RTag,
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
                    .cloned()
                    .map(|item| item.into())
                    .collect(),
            ),
            library_operatives: RwSignal::new(value.library_operatives),
            trait_operatives: RwSignal::new(
                value
                    .trait_operatives
                    .iter()
                    .cloned()
                    .map(|item| item.into())
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
                .cloned()
                .map(|item| item.into())
                .collect(),

            library_operatives: value.library_operatives.get(),
            trait_operatives: value
                .trait_operatives
                .get()
                .iter()
                .cloned()
                .map(|item| item.into())
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
impl<TTypes: ConstraintTraits, TValues: ConstraintTraits> RCSO<TTypes, TValues>
    for RLibraryTemplate<TTypes, TValues>
{
    fn get_fields(&self) -> Vec<impl Tagged + FieldInfo<TTypes, TValues>> {
        self.field_constraints.get()
    }
}
impl<TTypes: ConstraintTraits, TValues: ConstraintTraits> RLibraryTemplate<TTypes, TValues> {
    pub fn new() -> Self {
        Self {
            field_constraints: RwSignal::new(vec![]),
            library_operatives: RwSignal::new(vec![]),
            trait_operatives: RwSignal::new(vec![]),
            instances: RwSignal::new(vec![]),
            trait_impls: RwSignal::new(HashMap::new()),
            tag: RTag::new("NewConstraintObject"),
            _phantom: PhantomData,
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RTraitOperative {
    pub trait_id: RwSignal<Uid>,
    pub tag: RTag,
}
apply_tagged!(RTraitOperative);
impl From<TraitOperative> for RTraitOperative {
    fn from(value: TraitOperative) -> Self {
        Self {
            trait_id: RwSignal::new(value.trait_id),
            tag: value.tag.into(),
        }
    }
}
impl From<RTraitOperative> for TraitOperative {
    fn from(value: RTraitOperative) -> Self {
        Self {
            trait_id: value.trait_id.get(),
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
    // Denotes jumping to a constituent element in the structure
    InstanceConstituent(RwSignal<Uid>),
    LibraryOperativeConstituent(RwSignal<Uid>),
    TraitOperativeConstituent {
        trait_operative_id: RwSignal<Uid>,
        trait_id: RwSignal<Uid>,
        trait_method_id: RwSignal<Uid>,
    },
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
            TraitMethodImplPath::InstanceConstituent(val) => {
                RTraitMethodImplPath::InstanceConstituent(RwSignal::new(val))
            }
            TraitMethodImplPath::LibraryOperativeConstituent(val) => {
                RTraitMethodImplPath::LibraryOperativeConstituent(RwSignal::new(val))
            }
            TraitMethodImplPath::TraitOperativeConstituent {
                trait_operative_id,
                trait_id,
                trait_method_id,
            } => RTraitMethodImplPath::TraitOperativeConstituent {
                trait_operative_id: RwSignal::new(trait_operative_id),
                trait_id: RwSignal::new(trait_id),
                trait_method_id: RwSignal::new(trait_method_id),
            },
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
            RTraitMethodImplPath::InstanceConstituent(val) => {
                TraitMethodImplPath::InstanceConstituent(val.get())
            }
            RTraitMethodImplPath::LibraryOperativeConstituent(val) => {
                TraitMethodImplPath::LibraryOperativeConstituent(val.get())
            }
            RTraitMethodImplPath::TraitOperativeConstituent {
                trait_operative_id,
                trait_id,
                trait_method_id,
            } => TraitMethodImplPath::TraitOperativeConstituent {
                trait_operative_id: trait_operative_id.get(),
                trait_id: trait_id.get(),
                trait_method_id: trait_method_id.get(),
            },
        }
    }
}

// Instance --------------------------------------------------------
#[derive(Clone, Debug, PartialEq)]
pub struct RLibraryInstance<TTypes: ConstraintTraits, TValues: ConstraintTraits> {
    pub template_id: RwSignal<Uid>,
    // If the instance is of a particular operative
    pub parent_operative_id: RwSignal<Option<Uid>>,
    pub tag: RTag,
    pub other_edges: RwSignal<Vec<RFulfilledOperative>>,
    pub fulfilled_library_operatives: RwSignal<Vec<RFulfilledOperative>>,
    pub fulfilled_trait_operatives: RwSignal<Vec<RFulfilledOperative>>,
    pub data: RwSignal<Vec<RFulfilledFieldConstraint<TTypes, TValues>>>,
    pub trait_impls: RwSignal<HashMap<Uid, RTraitImpl>>,
}
impl<TTypes: ConstraintTraits, TValues: ConstraintTraits> From<LibraryInstance<TTypes, TValues>>
    for RLibraryInstance<TTypes, TValues>
{
    fn from(value: LibraryInstance<TTypes, TValues>) -> Self {
        Self {
            template_id: RwSignal::new(value.template_id),
            parent_operative_id: RwSignal::new(value.parent_operative_id),
            tag: value.tag.into(),
            other_edges: RwSignal::new(
                value
                    .other_edges
                    .iter()
                    .cloned()
                    .map(|edge| edge.into())
                    .collect(),
            ),
            fulfilled_library_operatives: RwSignal::new(
                value
                    .fulfilled_library_operatives
                    .iter()
                    .cloned()
                    .map(|operative| operative.into())
                    .collect(),
            ),
            fulfilled_trait_operatives: RwSignal::new(
                value
                    .fulfilled_trait_operatives
                    .iter()
                    .cloned()
                    .map(|operative| operative.into())
                    .collect(),
            ),
            data: RwSignal::new(
                value
                    .data
                    .iter()
                    .cloned()
                    .map(|field| field.into())
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
        }
    }
}
impl<TTypes: ConstraintTraits, TValues: ConstraintTraits> From<RLibraryInstance<TTypes, TValues>>
    for LibraryInstance<TTypes, TValues>
{
    fn from(value: RLibraryInstance<TTypes, TValues>) -> Self {
        Self {
            template_id: value.template_id.get(),
            parent_operative_id: value.parent_operative_id.get(),
            tag: value.tag.into(),
            other_edges: value
                .other_edges
                .get()
                .iter()
                .cloned()
                .map(|edge| edge.into())
                .collect(),
            fulfilled_library_operatives: value
                .fulfilled_library_operatives
                .get()
                .iter()
                .cloned()
                .map(|operative| operative.into())
                .collect(),
            fulfilled_trait_operatives: value
                .fulfilled_library_operatives
                .get()
                .iter()
                .cloned()
                .map(|operative| operative.into())
                .collect(),
            data: value
                .data
                .get()
                .iter()
                .cloned()
                .map(|field| field.into())
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
        }
    }
}
impl<TTypes: ConstraintTraits, TValues: ConstraintTraits> RCSO<TTypes, TValues>
    for RLibraryInstance<TTypes, TValues>
{
    fn get_fields(&self) -> Vec<impl Tagged + FieldInfo<TTypes, TValues>> {
        self.data.get()
    }
}
impl<TTypes: ConstraintTraits, TValues: ConstraintTraits> RLibraryInstance<TTypes, TValues> {
    pub fn new<T>(template_id: Uid, operative_library_id: Option<Uid>, name: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            template_id: RwSignal::new(template_id),
            parent_operative_id: RwSignal::new(operative_library_id),
            tag: RTag::new(name),
            other_edges: RwSignal::new(vec![]),
            fulfilled_library_operatives: RwSignal::new(vec![]),
            fulfilled_trait_operatives: RwSignal::new(vec![]),
            data: RwSignal::new(vec![]),
            trait_impls: RwSignal::new(HashMap::new()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RFulfilledOperative {
    pub operative_id: RwSignal<Uid>,
    pub fulfilling_instance_id: RwSignal<Uid>,
}
impl From<FulfilledOperative> for RFulfilledOperative {
    fn from(value: FulfilledOperative) -> Self {
        Self {
            operative_id: RwSignal::new(value.operative_id.into()),
            fulfilling_instance_id: RwSignal::new(value.fulfilling_instance_id),
        }
    }
}
impl From<RFulfilledOperative> for FulfilledOperative {
    fn from(value: RFulfilledOperative) -> Self {
        Self {
            operative_id: value.operative_id.get().into(),
            fulfilling_instance_id: value.fulfilling_instance_id.get(),
        }
    }
}

// #[derive(Clone, Debug, PartialEq)]
// pub enum ROperativeVariants {
//     LibraryOperative(RwSignal<Uid>),
//     TraitOperative(RwSignal<Uid>),
// }
// impl From<OperativeVariants> for ROperativeVariants {
//     fn from(value: OperativeVariants) -> Self {
//         match value {
//             OperativeVariants::TraitOperative(val) => {
//                 ROperativeVariants::TraitOperative(RwSignal::new(val))
//             }
//             OperativeVariants::LibraryOperative(val) => {
//                 ROperativeVariants::LibraryOperative(RwSignal::new(val))
//             }
//         }
//     }
// }
// impl From<ROperativeVariants> for OperativeVariants {
//     fn from(value: ROperativeVariants) -> Self {
//         match value {
//             ROperativeVariants::TraitOperative(val) => OperativeVariants::TraitOperative(val.get()),
//             ROperativeVariants::LibraryOperative(val) => {
//                 OperativeVariants::LibraryOperative(val.get())
//             }
//         }
//     }
// }

#[derive(Clone, Debug, PartialEq)]
pub struct RFulfilledFieldConstraint<TTypes: ConstraintTraits, TValues: ConstraintTraits> {
    pub tag: RTag,
    pub value_type: RwSignal<TTypes>,
    pub value: RwSignal<TValues>,
}
impl<TTypes: ConstraintTraits, TValues: ConstraintTraits> FieldInfo<TTypes, TValues>
    for RFulfilledFieldConstraint<TTypes, TValues>
{
    fn get_value_type(&self) -> TTypes {
        self.value_type.get()
    }

    fn get_value(&self) -> Option<TValues> {
        Some(self.value.get())
    }
}
apply_tagged!(RFulfilledFieldConstraint<TTypes,TValues>);
impl<TTypes: ConstraintTraits, TValues: ConstraintTraits>
    From<FulfilledFieldConstraint<TTypes, TValues>> for RFulfilledFieldConstraint<TTypes, TValues>
{
    fn from(value: FulfilledFieldConstraint<TTypes, TValues>) -> Self {
        Self {
            tag: value.tag.into(),
            value_type: RwSignal::new(value.value_type),
            value: RwSignal::new(value.value),
        }
    }
}
impl<TTypes: ConstraintTraits, TValues: ConstraintTraits>
    From<RFulfilledFieldConstraint<TTypes, TValues>> for FulfilledFieldConstraint<TTypes, TValues>
{
    fn from(value: RFulfilledFieldConstraint<TTypes, TValues>) -> Self {
        Self {
            tag: value.tag.into(),
            value_type: value.value_type.get(),
            value: value.value.get(),
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
    pub fulfilled_library_operatives: RwSignal<Vec<RFulfilledOperative>>,
    pub fulfilled_trait_operatives: RwSignal<Vec<RFulfilledOperative>>,
    pub locked_fields: RwSignal<Vec<RFulfilledFieldConstraint<TTypes, TValues>>>,
    pub trait_impls: RwSignal<HashMap<Uid, RTraitImpl>>,
}
impl<TTypes: ConstraintTraits, TValues: ConstraintTraits> From<LibraryOperative<TTypes, TValues>>
    for RLibraryOperative<TTypes, TValues>
{
    fn from(value: LibraryOperative<TTypes, TValues>) -> Self {
        Self {
            template_id: RwSignal::new(value.template_id),
            parent_operative_id: RwSignal::new(value.parent_operative_id),
            tag: value.tag.into(),
            fulfilled_library_operatives: RwSignal::new(
                value
                    .fulfilled_library_operatives
                    .iter()
                    .cloned()
                    .map(|item| item.into())
                    .collect(),
            ),
            fulfilled_trait_operatives: RwSignal::new(
                value
                    .fulfilled_trait_operatives
                    .iter()
                    .cloned()
                    .map(|item| item.into())
                    .collect(),
            ),
            locked_fields: RwSignal::new(
                value
                    .locked_fields
                    .iter()
                    .cloned()
                    .map(|item| item.into())
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
            fulfilled_library_operatives: value
                .fulfilled_library_operatives
                .get()
                .iter()
                .cloned()
                .map(|item| item.into())
                .collect(),
            fulfilled_trait_operatives: value
                .fulfilled_trait_operatives
                .get()
                .iter()
                .cloned()
                .map(|item| item.into())
                .collect(),
            locked_fields: value
                .locked_fields
                .get()
                .iter()
                .cloned()
                .map(|item| item.into())
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
        }
    }
}
impl<TTypes: ConstraintTraits, TValues: ConstraintTraits> RCSO<TTypes, TValues>
    for RLibraryOperative<TTypes, TValues>
{
    fn get_fields(&self) -> Vec<impl Tagged + FieldInfo<TTypes, TValues>> {
        Vec::<RFieldConstraint<TTypes>>::new()
    }
}
impl<TTypes: ConstraintTraits, TValues: ConstraintTraits> RLibraryOperative<TTypes, TValues> {
    pub fn new<T>(template_id: Uid, operative_library_id: Option<Uid>, name: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            template_id: RwSignal::new(template_id),
            parent_operative_id: RwSignal::new(operative_library_id),
            tag: RTag::new(name),
            fulfilled_library_operatives: RwSignal::new(vec![]),
            fulfilled_trait_operatives: RwSignal::new(vec![]),
            locked_fields: RwSignal::new(vec![]),
            trait_impls: RwSignal::new(HashMap::new()),
        }
    }
}
// Traits --------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq)]
pub struct RTraitDef<TTypes: ConstraintTraits> {
    pub tag: RTag,
    pub methods: RwSignal<Vec<RTraitMethodDef<TTypes>>>,
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
                    .cloned()
                    .map(|item| item.into())
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
                .cloned()
                .map(|item| item.into())
                .collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RTraitMethodDef<TTypes: ConstraintTraits> {
    pub tag: RTag,
    pub return_type: RwSignal<TTypes>,
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
