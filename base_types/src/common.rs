use uuid::Uuid;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub enum Dir {
    Emit,
    Recv,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub enum EdgeType {
    Normal,
    // Hole(Uid),
    Slot(Uid),
}
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct EdgeInstance {
    pub dir: Dir,
    pub host: Uid,
    pub target: Uid,
    pub edge_type: EdgeType,
}
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct Tag {
    pub name: String,
    pub id: Uid,
}

#[cfg(feature = "to_tokens")]
impl quote::ToTokens for Tag {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let id = self.id;
        let name = self.name.clone();
        tokens.extend(quote::quote! {base_types::common::Tag {
            id: #id,
            name: #name.to_string(),
        }});
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

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
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

pub trait ConstraintTraits: Clone + std::fmt::Debug + PartialEq + Default + 'static {}

pub type Uid = u128;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StrUid(String);
pub fn u128_to_string(id: Uid) -> String {
    uuid::Uuid::from_u128(id).to_string().into()
}
pub fn string_to_u128(id: String) -> Uid {
    id.parse::<Uuid>().unwrap().as_u128()
}

impl From<String> for StrUid {
    fn from(value: String) -> Self {
        StrUid(value)
    }
}
impl From<Uid> for StrUid {
    fn from(value: Uid) -> Self {
        uuid::Uuid::from_u128(value).to_string().into()
    }
}
impl From<StrUid> for Uid {
    fn from(value: StrUid) -> Self {
        value.0.parse::<Uuid>().unwrap().as_u128()
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
