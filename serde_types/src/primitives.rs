use serde::{Deserialize, Serialize};

use crate::common::*;

// pub struct TypeContainer {
//     types: Vec<TypeDef>,
// }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PrimitiveTypes {
    I32,
    U32,
    F32,
    String,
    Bool,
    Char,
    Option(Box<PrimitiveTypes>),
}
impl ConstraintTraits for PrimitiveTypes {}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PrimitiveValues {
    I32(i32),
    U32(u32),
    F32(f32),
    String(String),
    Bool(bool),
    Char(char),
    Option(Box<PrimitiveValues>),
    // Vec(Vec<Primitives>),
    // UDef(Uid),
}
impl ConstraintTraits for PrimitiveValues {}
// pub struct TypeDef {
//     name: String,
//     data_type: ,
// }
