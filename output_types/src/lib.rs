pub type Uid = u128;

pub trait GraphEnvironment {
    type Schema;
    fn get_element(&self, id: Uid) -> Option<Self::Schema>;
    fn instantiate_element(&self, element: &Self::Schema) -> Uid;
}
