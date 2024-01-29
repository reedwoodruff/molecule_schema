pub type Uid = u128;

pub trait GraphEnvironment {
    type Schema;
    fn get_element(&self, id: Uid) -> Option<Self::Schema>;
    fn instantiate_element(&self, element: &Self::Schema) -> Uid;
}
pub trait GSO {
    type Builder;

    fn get_constraint_schema_id(&self) -> Uid;
    fn initiate_build() -> Self::Builder;
    fn get_operative_by_id(&self, operative_id: &Uid) -> Option<Uid>;
}
