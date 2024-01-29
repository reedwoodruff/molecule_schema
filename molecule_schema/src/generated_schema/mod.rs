// // use crate::commonConstraintTraits;
// use molecule_types::*;
// use uuid::Uuid;
//
// mod test;
// mod utils;
//
// #[derive(Clone)]
// /// Generated Schema Object
// pub struct InstanceGSO<T>
// where
//     T: Clone,
// {
//     pub id: Uid,
//     pub structure_ids: Option<Vec<Uid>>,
//     pub schema_tag: Uid,
//     pub required_edges: Vec<EdgeInstance>,
//     pub optional_edges: Vec<EdgeInstance>,
//     pub data: T,
// }
//
// #[derive(Clone)]
// pub struct OperativeGSO<T, VT>
// where
//     T: Clone,
//     VT: ConstraintTraits,
// {
//     pub id: Uid,
//     pub structure_ids: Option<Vec<Uid>>,
//     pub schema_tag: Uid,
//     pub parent_template_tag: Uid,
//     pub locked_edges: Option<Vec<FuzzyEdgeDescriptor>>,
//     pub operative_edges: Vec<EdgeInstance>,
//     pub locked_data: T,
//     pub operative_fields: Vec<FieldDescriptor<VT>>,
// }
//
// #[derive(Clone)]
// pub struct FieldDef<T> {
//     name: String,
//     id: Uid,
//     value: T,
// }
// impl<T> FieldDef<T> {
//     fn new<N>(name: N, value: T) -> Self
//     where
//         N: Into<String>,
//     {
//         Self {
//             name: name.into(),
//             id: Uuid::new_v4().as_u128(),
//             value,
//         }
//     }
// }
