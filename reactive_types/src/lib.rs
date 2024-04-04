use base_types::{
    common::{ConstraintTraits, Tag},
    constraint_schema::ConstraintSchema,
    primitives::{PrimitiveTypes, PrimitiveValues},
    utils::print_schema,
};
use reactive_types::RConstraintSchema;

#[derive(Debug, Clone)]
pub enum ConstraintSchemaTag {
    Template(Tag),
    Instance(Tag),
    Operative(Tag),
}

pub fn print_schema_reactive(reactive_schema: &RConstraintSchema<PrimitiveTypes, PrimitiveValues>) {
    let base_schema: ConstraintSchema<PrimitiveTypes, PrimitiveValues> =
        reactive_schema.clone().into();
    print_schema(&base_schema);
}

pub mod from_impls;
pub mod locked_field_digest;
pub mod operative_digest;
pub mod reactive_item;
pub mod reactive_types;
pub mod trait_impl_digest;

// pub trait Schema;

// struct SchemaElementBuilder<TSchema> {
//     identifier: Option<syn::Ident>,
//     name_map: HashMap<String, syn::Ident>,
//     _phantom: std::marker::PhantomData<TSchema>,
// }
//
// impl<TSchema> SchemaElementBuilder<TSchema> {
//     pub fn new(name_map: HashMap<String, syn::Ident>) -> Self {
//         Self {
//             identifier: None,
//             name_map,
//             _phantom: std::marker::PhantomData,
//         }
//     }
//     pub fn name<'a, T>(&mut self, name: T) -> &mut Self
//     where
//         T: Into<&'a String>,
//     {
//         self.identifier = self.name_map.get(name.into());
//         self
//     }
//
//     pub fn build(&self) -> TSchema {}
// }
