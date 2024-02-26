

pub type Uid = u128;

pub trait GraphEnvironment {
    type Schema: GSO;
    fn get_element(&self, id: &Uid) -> Option<&Self::Schema>;
    fn instantiate_element(&mut self, element: Self::Schema) -> Uid;
}
pub trait GSO {
    type Builder;

    fn get_id(&self) -> Uid;
    fn get_constraint_schema_tag(&self) -> &ConstraintSchemaTag;
    // fn get_constraint_schema_id(&self) -> ConstraintSchemaId;
    fn get_template_id(&self) -> Uid;
    fn initiate_build() -> Self::Builder;
    fn get_operative_by_id(&self, operative_id: &Uid) -> Option<Uid>;
}

#[derive(Debug, Clone)]
pub struct Tag {
    pub id: Uid,
    pub name: String,
}

#[derive(Debug, Clone)]
pub enum ConstraintSchemaTag {
    Template(Tag),
    Instance(Tag),
    Operative(Tag),
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
