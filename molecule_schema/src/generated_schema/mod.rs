use crate::common::ConstraintTraits;
use crate::constraint_schema::ConstraintSchema;
use molecule_types::*;

mod test;
mod utils;

// enum TemplateConstituent<T> {
//     Instance(Instance<T>),
//     Operative,
// }
// struct TemplateRepresentation<T> {
//     pub subgraph: Subgraph<T>,
// }
// struct Subgraph<T> {
//     pub elements: Instance<T>,
// }

// pub struct TemplateSpecs {}

// #[derive(Clone)]
// enum GSO<T, U> {
//     Instance(InstanceGSO<T, U>),
//     Operative(OperativeGSO<T, U>),
// }

#[derive(Clone)]
/// Generated Schema Object
pub struct InstanceGSO<T> {
    pub id: Uid,
    pub structure_ids: Option<Vec<Uid>>,
    pub schema_tag: SchemaTag,
    pub edge_constraints: Option<u32>,
    pub edges: Vec<EdgeInstance>,
    pub data: T,
}
#[derive(Clone)]
pub struct OperativeGSO<T> {
    pub id: Uid,
    pub structure_ids: Option<Vec<Uid>>,
    pub schema_tag: SchemaTag,
    pub edge_constraints: Option<u32>,
    pub edges: Vec<EdgeInstance>,
    pub operative_data: T,
}

impl<T> GSOCommon for InstanceGSO<T> {
    fn id(&self) -> &Uid {
        &self.id
    }
    fn structure_ids(&self) -> &Option<Vec<Uid>> {
        &self.structure_ids
    }
    fn schema_tag(&self) -> &SchemaTag {
        &self.schema_tag
    }
    fn edge_constraints(&self) -> &Option<u32> {
        &self.edge_constraints
    }
    fn edges(&self) -> &Vec<EdgeInstance> {
        &self.edges
    }
    // fn data(&self) -> &T {
    //     &self.data
    // }
}

pub trait GSOCommon {
    fn id(&self) -> &Uid;
    fn structure_ids(&self) -> &Option<Vec<Uid>>;
    fn schema_tag(&self) -> &SchemaTag;
    fn edge_constraints(&self) -> &Option<u32>;
    fn edges(&self) -> &Vec<EdgeInstance>;
    // fn data(&self) -> &Self;
}
