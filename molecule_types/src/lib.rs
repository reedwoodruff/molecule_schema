#[derive(Clone)]
pub enum Dir {
    Emit,
    Recv,
}

pub type Uid = u128;

#[derive(Clone)]
pub struct SchemaTag {
    pub id: Uid,
    pub name: String,
}
impl SchemaTag {
    pub fn new(name: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().as_u128(),
            name,
        }
    }
}

#[derive(Clone)]
pub enum EdgeType {
    Normal,
    Hole(String),
    Slot(String),
}

#[derive(Clone)]
pub struct EdgeInstance {
    pub dir: Dir,
    pub host: Uid,
    pub target: Uid,
    pub edge_type: EdgeType,
}
