use std::collections::HashMap;

pub type Uid = u128;

pub trait GraphEnvironment {
    type Schema: GSO;
    fn get_element(&self, id: &Uid) -> Option<&Self::Schema>;
    fn instantiate_element(&mut self, element: Self::Schema) -> Uid;
}
pub trait GSO {
    type Builder;

    fn get_id(&self) -> Uid;
    fn get_constraint_schema_id(&self) -> Uid;
    fn initiate_build() -> Self::Builder;
    fn get_operative_by_id(&self, operative_id: &Uid) -> Option<Uid>;
}

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
