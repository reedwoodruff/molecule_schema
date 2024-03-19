use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Dir {
    Emit,
    Recv,
}
pub type Uid = u128;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum EdgeType {
    Normal,
    // Hole(Uid),
    Slot(Uid),
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EdgeInstance {
    pub dir: Dir,
    pub host: Uid,
    pub target: Uid,
    pub edge_type: EdgeType,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Tag {
    pub name: String,
    pub id: Uid,
}
impl From<Tag> for output_types::Tag {
    fn from(value: Tag) -> Self {
        output_types::Tag {
            id: value.id,
            name: value.name,
        }
    }
}
impl Tag {
    pub fn new<T>(name: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            name: name.into(),
            id: Uuid::new_v4().as_u128(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FuzzyEdgeDescriptor {
    dir: Option<Dir>,
    host: Option<Uid>,
    host_schema_traits: Option<Vec<Uid>>,
    target: Option<Uid>,
    target_schema_traits: Option<Vec<Uid>>,
    edge_type: Option<EdgeType>,
}

impl Default for FuzzyEdgeDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl FuzzyEdgeDescriptor {
    pub fn new() -> Self {
        Self {
            dir: None,
            host: None,
            host_schema_traits: None,
            target: None,
            target_schema_traits: None,
            edge_type: None,
        }
    }
    pub fn dir(mut self, dir: Dir) -> Self {
        self.dir = Some(dir);
        self
    }
    pub fn host(mut self, host: Uid) -> Self {
        self.host = Some(host);
        self
    }
    pub fn add_host_schema_trait(mut self, host_schema_trait: Uid) -> Self {
        if let Some(ref mut tags) = self.host_schema_traits {
            tags.push(host_schema_trait);
        } else {
            self.host_schema_traits = Some(vec![host_schema_trait]);
        }
        self
    }
    pub fn target(mut self, target: Uid) -> Self {
        self.target = Some(target);
        self
    }
    pub fn add_target_schema_trait(mut self, target_schema_trait: Uid) -> Self {
        if let Some(ref mut tags) = self.target_schema_traits {
            tags.push(target_schema_trait);
        } else {
            self.target_schema_traits = Some(vec![target_schema_trait]);
        }
        self
    }

    pub fn edge_type(mut self, edge_type: EdgeType) -> Self {
        self.edge_type = Some(edge_type);
        self
    }
}

pub trait ConstraintTraits: Clone + std::fmt::Debug + PartialEq + 'static {}
