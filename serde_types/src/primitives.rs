use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};

use crate::common::*;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;
// pub struct TypeContainer {
//     types: Vec<TypeDef>,
// }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Eq, EnumIter, Default)]
pub enum PrimitiveTypes {
    #[default]
    Bool,
    Char,
    Int,
    Float,
    String,
    Option(Box<PrimitiveTypes>),
    List(Box<PrimitiveTypes>),
}

impl PrimitiveTypes {
    pub fn get_type_options() -> HashMap<PrimitiveTypes, String> {
        let mut map = HashMap::new();
        map.insert(PrimitiveTypes::Int, "Int".to_string());
        map.insert(PrimitiveTypes::Float, "Float".to_string());
        map.insert(PrimitiveTypes::String, "String".to_string());
        map.insert(PrimitiveTypes::Bool, "Bool".to_string());
        map.insert(PrimitiveTypes::Char, "Char".to_string());
        for variant in PrimitiveTypes::iter() {
            match variant {
                PrimitiveTypes::Option(_) => {}
                PrimitiveTypes::List(_) => {}
                _ => {
                    map.insert(
                        PrimitiveTypes::Option(Box::new(variant.clone())),
                        format!("Option({})", variant),
                    );
                    map.insert(
                        PrimitiveTypes::List(Box::new(variant.clone())),
                        format!("List({})", variant),
                    );
                }
            }
        }
        map
    }
}
impl ConstraintTraits for PrimitiveTypes {}

impl Display for PrimitiveTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrimitiveTypes::Char => write!(f, "char"),
            PrimitiveTypes::Int => write!(f, "int"),
            PrimitiveTypes::String => write!(f, "string"),
            PrimitiveTypes::Float => write!(f, "float"),
            PrimitiveTypes::Bool => write!(f, "bool"),
            PrimitiveTypes::Option(val) => write!(f, "option({})", val),
            PrimitiveTypes::List(val) => write!(f, "list({})", val),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PrimitiveValues {
    Int(u32),
    Float(f32),
    String(String),
    Bool(bool),
    Char(char),
    Option(Box<Option<PrimitiveValues>>),
    List(Box<Vec<PrimitiveValues>>),
}
impl Display for PrimitiveValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrimitiveValues::Char(val) => write!(f, "{}", val.to_string()),
            PrimitiveValues::Int(val) => write!(f, "{}", val.to_string(),),
            PrimitiveValues::String(val) => write!(f, "{}", val),
            PrimitiveValues::Float(val) => write!(f, "{}", val.to_string()),
            PrimitiveValues::Bool(val) => write!(f, "{}", val.to_string()),
            PrimitiveValues::Option(val) => {
                if let Some(val) = val.as_ref() {
                    write!(f, "Some({})", val)
                } else {
                    write!(f, "None")
                }
            }
            PrimitiveValues::List(val) => {
                let string_combined = val
                    .iter()
                    .map(|val| val.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "{}", string_combined)
            }
        }
    }
}
impl ConstraintTraits for PrimitiveValues {}
