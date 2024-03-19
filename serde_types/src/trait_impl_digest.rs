use std::collections::HashMap;

use crate::{common::Uid, constraint_schema::TraitImpl};

pub struct TraitImplDigest<'a>(pub HashMap<Uid, RelatedTraitImpl<'a>>);
pub struct RelatedTraitImpl<'a> {
    pub trait_impl: &'a TraitImpl,
    pub hosting_element_id: Uid,
}
