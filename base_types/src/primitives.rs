use std::{collections::HashMap, fmt::Display, ops::Deref};

use crate::common::*;

use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};
// pub struct TypeContainer {
//     types: Vec<TypeDef>,
// }
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Hash, Eq, EnumString, EnumIter, Default, Display)]
pub enum PrimitiveTypes {
    #[default]
    EmptyTuple,
    Bool,
    Char,
    Int,
    // Float,
    String,
    Option(Box<PrimitiveTypes>),
    List(Box<PrimitiveTypes>),
}
#[cfg(feature = "to_tokens")]
impl quote::ToTokens for PrimitiveTypes {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ts = match *self {
            PrimitiveTypes::EmptyTuple => {
                quote::quote! {base_types::primitives::PrimitiveTypes::EmptyTuple}
            }
            PrimitiveTypes::Bool => quote::quote! { base_types::primitives::PrimitiveTypes::Bool },
            PrimitiveTypes::Char => quote::quote! { base_types::primitives::PrimitiveTypes::Char },
            PrimitiveTypes::Int => quote::quote! { base_types::primitives::PrimitiveTypes::Int },
            // PrimitiveTypes::Float => {
            //     quote::quote! { base_types::primitives::PrimitiveTypes::Float }
            // }
            PrimitiveTypes::String => {
                quote::quote! { base_types::primitives::PrimitiveTypes::String }
            }
            PrimitiveTypes::Option(ref inner) => {
                quote::quote! { base_types::primitives::PrimitiveTypes::Option(Box::new(#inner)) }
            }
            PrimitiveTypes::List(ref inner) => {
                quote::quote! { base_types::primitives::PrimitiveTypes::List(Box::new(#inner)) }
            }
        };
        ts.to_tokens(tokens);
    }
}

impl PrimitiveTypes {
    pub fn get_type_options() -> HashMap<PrimitiveTypes, String> {
        let mut map = HashMap::new();
        map.insert(PrimitiveTypes::Int, "Int".to_string());
        // map.insert(PrimitiveTypes::Float, "Float".to_string());
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

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PrimitiveValues {
    Int(u32),
    // Float(f32),
    String(String),
    Bool(bool),
    Char(char),
    Option(Box<Option<PrimitiveValues>>),
    List(Vec<PrimitiveValues>),
}
impl Default for PrimitiveValues {
    fn default() -> Self {
        Self::String("DefaultString".to_string())
    }
}
impl Display for PrimitiveValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrimitiveValues::Char(val) => write!(f, "{}", val),
            PrimitiveValues::Int(val) => write!(f, "{}", val,),
            PrimitiveValues::String(val) => write!(f, "{}", val),
            // PrimitiveValues::Float(val) => write!(f, "{}", val),
            PrimitiveValues::Bool(val) => write!(f, "{}", val),
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
impl PrimitiveValues {
    pub fn get_primitive_type(&self) -> PrimitiveTypes {
        match self {
            PrimitiveValues::Int(_) => PrimitiveTypes::Int,
            // PrimitiveValues::Float(_) => PrimitiveTypes::Float,
            PrimitiveValues::String(_) => PrimitiveTypes::String,
            PrimitiveValues::Bool(_) => PrimitiveTypes::Bool,
            PrimitiveValues::Char(_) => PrimitiveTypes::Char,
            PrimitiveValues::Option(val) => match val.deref() {
                Some(val) => val.get_primitive_type(),
                None => PrimitiveTypes::Option(Box::new(PrimitiveTypes::EmptyTuple)),
            },
            PrimitiveValues::List(val) => {
                PrimitiveTypes::List(Box::new(match val.deref().iter().next() {
                    Some(val) => val.get_primitive_type(),
                    None => PrimitiveTypes::EmptyTuple,
                }))
            }
        }
    }
}
