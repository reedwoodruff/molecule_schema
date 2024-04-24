

use crate::{
    constraint_schema::ConstraintSchema,
    primitives::{PrimitiveTypes, PrimitiveValues},
};

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

impl IntoPrimitiveValue for f32 {
    fn into_primitive_value(self) -> PrimitiveValues {
        PrimitiveValues::Float(self)
    }
}

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

impl IntoPrimitiveValue for char {
    fn into_primitive_value(self) -> PrimitiveValues {
        PrimitiveValues::Char(self)
    }
}
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
