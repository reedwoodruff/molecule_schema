use crate::common::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, marker::PhantomData};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConstraintSchema<TTypes: ConstraintTraits, TValues: ConstraintTraits> {
    pub constraint_objects: HashMap<Uid, ConstraintObject<TTypes, TValues>>,
    pub instance_library: HashMap<Uid, LibraryInstance<TTypes, TValues>>,
    pub operative_library: HashMap<Uid, LibraryOperative<TTypes, TValues>>,
    pub traits: HashMap<Uid, TraitDef<TTypes>>,
}

pub enum ConstraintSchemaInstantiableType {
    ConstraintObject,
    Instance,
    Operative,
}

pub trait ConstraintSchemaInstantiable {
    type TTypes: ConstraintTraits;
    type TValues: ConstraintTraits;

    fn get_constraint_schema_instantiable_type(&self) -> ConstraintSchemaInstantiableType;
    fn get_constraint_object_id(&self) -> Option<&Uid> {
        None
    }
    fn get_operative_library_id(&self) -> Option<&Uid> {
        None
    }
    fn get_tag(&self) -> &Tag;
    fn get_trait_impls(&self) -> &HashMap<Uid, TraitImpl>;
    fn get_fulfilled_operatives(&self) -> Option<&Vec<FulfilledOperative>> {
        None
    }
    fn get_fulfilled_fields(
        &self,
    ) -> Option<&Vec<FulfilledFieldConstraint<Self::TTypes, Self::TValues>>> {
        None
    }
}

// macro_rules! apply_instantiable {
//     ( $($x:expr),* ) => {
//         $(impl ConstraintSchemaInstantiable for $x {
//             fn get_constraint_object_id
//         })*
//     };
// }

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LibraryInstance<TTypes: ConstraintTraits, TValues: ConstraintTraits> {
    pub constraint_object_id: Uid,
    // If the instance is of a particular operative
    pub operative_library_id: Uid,
    pub tag: Tag,
    // pub other_edges: Vec<LibraryEdgeInstance>,
    pub other_edges: Vec<FulfilledOperative>,
    pub fulfilled_operatives: Vec<FulfilledOperative>,
    pub data: Vec<FulfilledFieldConstraint<TTypes, TValues>>,
    pub trait_impls: HashMap<Uid, TraitImpl>,
}

impl<TTypes: ConstraintTraits, TValues: ConstraintTraits> ConstraintSchemaInstantiable
    for LibraryInstance<TTypes, TValues>
{
    type TTypes = TTypes;
    type TValues = TValues;

    fn get_constraint_schema_instantiable_type(&self) -> ConstraintSchemaInstantiableType {
        ConstraintSchemaInstantiableType::Instance
    }
    fn get_constraint_object_id(&self) -> Option<&Uid> {
        Some(&self.constraint_object_id)
    }
    fn get_operative_library_id(&self) -> Option<&Uid> {
        Some(&self.operative_library_id)
    }
    fn get_tag(&self) -> &Tag {
        &self.tag
    }
    fn get_trait_impls(&self) -> &HashMap<Uid, TraitImpl> {
        &self.trait_impls
    }
    fn get_fulfilled_operatives(&self) -> Option<&Vec<FulfilledOperative>> {
        Some(&self.fulfilled_operatives)
    }
    fn get_fulfilled_fields(
        &self,
    ) -> Option<&Vec<FulfilledFieldConstraint<Self::TTypes, Self::TValues>>> {
        Some(&self.data)
    }
}
// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub enum LibraryOperative<TTypes: ConstraintTraits, TValues: ConstraintTraits> {
//     TemplateOperative {
//         constraint_object_id: Uid,
//         tag: Tag,
//         // other_edges: Vec<LibraryEdgeInstance>,
//         // fulfilled_edges: Vec<FulfilledEdgeConstraint>,
//         fulfilled_operatives: Vec<FulfilledOperative>,
//         locked_fields: Vec<FulfilledFieldConstraint<TTypes, TValues>>,
//         // operative_edges: Vec<FuzzyEdgeDescriptor>,
//         // operative_fields: Vec<FieldConstraint<TTypes>>,
//     },
//     TraitOperative {
//         tag: Tag,
//         trait_id: Uid,
//     },
// }

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LibraryOperative<TTypes: ConstraintTraits, TValues: ConstraintTraits> {
    pub constraint_object_id: Uid,
    pub tag: Tag,
    pub fulfilled_operatives: Vec<FulfilledOperative>,
    pub locked_fields: Vec<FulfilledFieldConstraint<TTypes, TValues>>,
    pub trait_impls: HashMap<Uid, TraitImpl>,
}

impl<TTypes: ConstraintTraits, TValues: ConstraintTraits> ConstraintSchemaInstantiable
    for LibraryOperative<TTypes, TValues>
{
    type TTypes = TTypes;
    type TValues = TValues;

    fn get_constraint_schema_instantiable_type(&self) -> ConstraintSchemaInstantiableType {
        ConstraintSchemaInstantiableType::Operative
    }
    fn get_constraint_object_id(&self) -> Option<&Uid> {
        Some(&self.constraint_object_id)
    }
    fn get_tag(&self) -> &Tag {
        &self.tag
    }
    fn get_trait_impls(&self) -> &HashMap<Uid, TraitImpl> {
        &self.trait_impls
    }
    fn get_fulfilled_operatives(&self) -> Option<&Vec<FulfilledOperative>> {
        Some(&self.fulfilled_operatives)
    }
    fn get_fulfilled_fields(
        &self,
    ) -> Option<&Vec<FulfilledFieldConstraint<Self::TTypes, Self::TValues>>> {
        Some(&self.locked_fields)
    }
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

// impl<TTypes: ConstraintTraits, TValues: ConstraintTraits> LibraryOperative<TTypes, TValues> {
//     pub fn get_tag(&self) -> &Tag {
//         match self {
//             LibraryOperative::TraitOperative { tag, .. } => tag,
//             LibraryOperative::TemplateOperative { tag, .. } => tag,
//         }
//     }
// }
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
pub struct ConstraintObject<TTypes: ConstraintTraits, TValues: ConstraintTraits> {
    pub field_constraints: Vec<FieldConstraint<TTypes>>,
    // pub edge_constraints: Vec<FuzzyEdgeDescriptor>,
    // pub constituents: Vec<LibraryReference>,
    pub library_operatives: Vec<Uid>,
    pub trait_operatives: Vec<TraitOperative>,
    pub instances: Vec<Uid>,
    pub trait_impls: HashMap<Uid, TraitImpl>,
    pub tag: Tag,
    pub _phantom: PhantomData<TValues>,
}

impl<TTypes: ConstraintTraits, TValues: ConstraintTraits> ConstraintSchemaInstantiable
    for ConstraintObject<TTypes, TValues>
{
    type TTypes = TTypes;
    type TValues = TValues;

    fn get_constraint_schema_instantiable_type(&self) -> ConstraintSchemaInstantiableType {
        ConstraintSchemaInstantiableType::ConstraintObject
    }
    fn get_tag(&self) -> &Tag {
        &self.tag
    }
    fn get_trait_impls(&self) -> &HashMap<Uid, TraitImpl> {
        &self.trait_impls
    }
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

// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub enum TraitImplPathType {
//     ChildFulfills(),
// }
// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub struct TraitImpl {
//     // Not unique
//     pub trait_id: Uid,
//     // Uid here representing the id of the method in the trait
//     pub method_impl_paths: HashMap<Uid, Vec<TraitMethodImplPath>>,
// }
pub type TraitImpl = HashMap<Uid, Vec<TraitMethodImplPath>>;
// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub struct TraitMethodImpl {
//     pub trait_method_id: Uid,
//     // Some way to describe where in either its structure or its native fields it will get the
//     // information to return the required data type. Seems like this will likely be some kind of
//     // DSL to describe structural locations, which will then be used upon instantiation to create
//     // an actual method which will use those locations to generate a value when called.
//     // The difficult thing I'm running into is how you might handle this for a single return type
//     // which you may want to build from multiple locations.
//     pub fulfillment_path: Vec<TraitPath>,
// }

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TraitMethodImplPath {
    // Denotes that the current path element has a field with id [Uid] which holds the
    // required information.
    Field(Uid),
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
    pub operative_id: OperativeVariants,
    pub fulfilling_instance_id: Uid,
}
// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub enum LibraryReference {
//     Instance(Uid),
//     Operative(Uid),
// }
// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub struct LibraryEdgeInstance {
//     pub dir: Dir,
//     pub host: LibraryReference,
//     pub target: LibraryReference,
//     pub edge_type: EdgeType,
// }
