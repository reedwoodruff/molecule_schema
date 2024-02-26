use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};

use crate::common::*;

// pub struct TypeContainer {
//     types: Vec<TypeDef>,
// }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Eq)]
pub enum PrimitiveTypes {
    I32,
    U32,
    F32,
    String,
    Bool,
    Char,
    Option(Box<PrimitiveTypes>),
}
impl PrimitiveTypes {
    pub fn get_type_options() -> HashMap<PrimitiveTypes, String> {
        let mut map = HashMap::new();
        map.insert(PrimitiveTypes::I32, "I32".to_string());
        map.insert(PrimitiveTypes::U32, "U32".to_string());
        map.insert(PrimitiveTypes::F32, "F32".to_string());
        map.insert(PrimitiveTypes::String, "String".to_string());
        map.insert(PrimitiveTypes::Bool, "Bool".to_string());
        map.insert(PrimitiveTypes::Char, "Char".to_string());
        map.insert(
            PrimitiveTypes::Option(Box::new(PrimitiveTypes::U32)),
            "Option".to_string(),
        );
        map
    }
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
impl Display for PrimitiveTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrimitiveTypes::Char => write!(f, "char"),
            PrimitiveTypes::I32 => write!(f, "i32"),
            PrimitiveTypes::String => write!(f, "string"),
            PrimitiveTypes::U32 => write!(f, "u32"),
            PrimitiveTypes::F32 => write!(f, "f32"),
            PrimitiveTypes::Bool => write!(f, "bool"),
            PrimitiveTypes::Option(val) => write!(f, "option({})", val),
        }
    }
}
impl Display for PrimitiveValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrimitiveValues::Char(val) => write!(f, "{}", val.to_string()),
            PrimitiveValues::I32(val) => write!(f, "{}", val.to_string(),),
            PrimitiveValues::String(val) => write!(f, "{}", val),
            PrimitiveValues::U32(val) => write!(f, "{}", val.to_string()),
            PrimitiveValues::F32(val) => write!(f, "{}", val.to_string()),
            PrimitiveValues::Bool(val) => write!(f, "{}", val.to_string()),
            PrimitiveValues::Option(val) => write!(f, "option({})", val.to_string()),
        }
    }
}
impl ConstraintTraits for PrimitiveValues {}
// pub struct TypeDef {
//     name: String,
//     data_type: ,
// }
