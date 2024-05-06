// use std::collections::HashMap;

// use crate::common::Uid;

// use super::{
//     reactive::{RInstantiable, RGSO},
//     HistoryFieldEdit, SlotRef,
// };

// pub struct BlueprintBuilder<S: RGSO<Schema = S>> {
//     changes: HashMap<Uid, Change>,
//     new: HashMap<Uid, NewItem<S>>,
// }

// pub struct NewItem<S: RGSO<Schema = S>> {
//     instantiable: Box<dyn RInstantiable<Schema = S>>,
// }
// pub struct Change {
//     remove_parents: Vec<SlotRef>,
//     remove_children: Vec<SlotRef>,
//     add_parents: Vec<SlotRef>,
//     add_children: Vec<SlotRef>,
//     change_contents: Vec<HistoryFieldEdit>,
// }
