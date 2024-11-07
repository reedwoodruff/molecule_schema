use std::{collections::HashMap, sync::Arc};

use crate::{
    constraint_schema::ConstraintSchema,
    post_generation::{
        reactive::{from_reactive::FromStandalone, RBaseGraphEnvironment, SharedGraph},
        StandaloneRGSOConcrete,
    },
    primitives::{PrimitiveTypes, PrimitiveValues},
};

#[cfg(feature = "to_tokens")]
pub fn get_primitive_value(ty: &PrimitiveValues) -> proc_macro2::TokenStream {
    match ty {
        PrimitiveValues::Int(val) => quote::quote! {#val},
        // PrimitiveValues::Float(val) => quote::quote! {#val},
        PrimitiveValues::String(val) => quote::quote! {#val},
        PrimitiveValues::Bool(val) => quote::quote! {#val},
        // PrimitiveValues::Char(val) => quote::quote! {#val},
        PrimitiveValues::Option(val) => {
            if let Some(present_value) = val.as_ref() {
                let inner = get_primitive_value(present_value);
                quote::quote! {#inner}
            } else {
                quote::quote! {None}
            }
        }
        PrimitiveValues::List(val) => {
            let inner = val.iter().map(get_primitive_value);
            quote::quote! {vec![#(#inner)*]}
        }
    }
}

#[cfg(feature = "serde")]
pub fn print_schema(schema: &ConstraintSchema<PrimitiveTypes, PrimitiveValues>) {
    let _path = std::path::Path::new("../../../constraint_schema/resources/schema.json");

    // let converted: ConstraintSchema<PrimitiveTypes, PrimitiveValues> = schema.clone().into();
    let json = serde_json::to_string_pretty(&schema).unwrap();
    // std::fs::write(path, json).expect("Unable to write file");
    println!("{}", json);
}

pub fn map_to_reactive_types(_schema: ConstraintSchema<PrimitiveTypes, PrimitiveValues>) {
    // let reactive_
}

pub trait IntoPrimitiveValue {
    fn into_primitive_value(self) -> PrimitiveValues;
}

impl IntoPrimitiveValue for u32 {
    fn into_primitive_value(self) -> PrimitiveValues {
        PrimitiveValues::Int(self)
    }
}

// impl IntoPrimitiveValue for f32 {
//     fn into_primitive_value(self) -> PrimitiveValues {
//         PrimitiveValues::Float(self)
//     }
// }

impl IntoPrimitiveValue for String {
    fn into_primitive_value(self) -> PrimitiveValues {
        PrimitiveValues::String(self)
    }
}

impl IntoPrimitiveValue for bool {
    fn into_primitive_value(self) -> PrimitiveValues {
        PrimitiveValues::Bool(self)
    }
}

// impl IntoPrimitiveValue for char {
//     fn into_primitive_value(self) -> PrimitiveValues {
//         PrimitiveValues::Char(self)
//     }
// }
impl<T: IntoPrimitiveValue> IntoPrimitiveValue for Option<T> {
    fn into_primitive_value(self) -> PrimitiveValues {
        match self {
            Some(value) => PrimitiveValues::Option(Box::new(Some(value.into_primitive_value()))),
            None => PrimitiveValues::Option(Box::new(None)), // Assume you have a None variant
        }
    }
}
impl<T: IntoPrimitiveValue> IntoPrimitiveValue for Vec<T> {
    fn into_primitive_value(self) -> PrimitiveValues {
        PrimitiveValues::List(
            self.into_iter()
                .map(|item| item.into_primitive_value())
                .collect(),
        )
    }
}

// pub trait FromPrimitiveValue {
//     type Output;
//     fn from_primitive_value(value: &PrimitiveValues) -> Self::Output;
// }

// impl FromPrimitiveValue for u32 {
//     type Output = u32;
//     fn from_primitive_value(value: &PrimitiveValues) -> Self::Output {
//         if let PrimitiveValues::Int(val) = value {
//             *val
//         } else {
//             unreachable!()
//         }
//     }
// }

// impl FromPrimitiveValue for f32 {
//     type Output = f32;
//     fn from_primitive_value(value: &PrimitiveValues) -> Self::Output {
//         if let PrimitiveValues::Float(val) = value {
//             *val
//         } else {
//             unreachable!()
//         }
//     }
// }

// impl FromPrimitiveValue for String {
//     type Output = String;
//     fn from_primitive_value(value: &PrimitiveValues) -> Self::Output {
//         if let PrimitiveValues::String(val) = value {
//             val.clone()
//         } else {
//             unreachable!()
//         }
//     }
// }

// impl FromPrimitiveValue for bool {
//     type Output = bool;
//     fn from_primitive_value(value: &PrimitiveValues) -> Self::Output {
//         if let PrimitiveValues::Bool(val) = value {
//             *val
//         } else {
//             unreachable!()
//         }
//     }
// }
// impl FromPrimitiveValue for Box<Option<PrimitiveValues>> {
//     type Output = Option<PrimitiveValues>;
//     fn from_primitive_value(value: &PrimitiveValues) -> Self::Output {
//         if let PrimitiveValues::Option(val) = value {
//             val.deref().clone()
//         } else {
//             unreachable!()
//         }
//     }
// }

// impl FromPrimitiveValue for Vec<PrimitiveValues> {
//     type Output = Vec<PrimitiveValues>;
//     fn from_primitive_value(value: &PrimitiveValues) -> Self::Output {
//         if let PrimitiveValues::List(val) = value {
//             val.clone()
//         } else {
//             unreachable!()
//         }
//     }
// }

pub fn initialize_graph_unpopulated<TSchema: Sync + Send + 'static>(
    constraint_schema: &'static ConstraintSchema<PrimitiveTypes, PrimitiveValues>,
) -> SharedGraph<TSchema> {
    Arc::new(RBaseGraphEnvironment::new(&constraint_schema)).into()
}
#[cfg(feature = "serde")]
pub fn initialize_graph_populated<
    TSchema: Sync + Send + FromStandalone<Schema = TSchema> + 'static,
>(
    constraint_schema: &'static ConstraintSchema<PrimitiveTypes, PrimitiveValues>,
    json_initial_population: &str,
) -> SharedGraph<TSchema> {
    let initial_population: Vec<StandaloneRGSOConcrete> =
        serde_json::from_str(json_initial_population)
            .expect("initial population data formatted incorrectly");
    let graph = Arc::new(RBaseGraphEnvironment::new(&constraint_schema));
    let formatted_initial_population = initial_population
        .into_iter()
        .map(|standalone| {
            (
                standalone.id.clone(),
                TSchema::from_standalone(standalone, graph.clone().into()),
            )
        })
        .collect::<HashMap<crate::common::Uid, TSchema>>();
    graph.initialize(formatted_initial_population);
    graph.into()
}
