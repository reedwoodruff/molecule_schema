use std::collections::HashMap;

use crate::{common::Uid, constraint_schema::TraitImpl};

pub struct TraitImplDigest<'a> {
    pub trait_impls: HashMap<Uid, RelatedTraitImpl<'a>>,
    pub object_id: Uid,
}

pub struct RelatedTraitImpl<'a> {
    pub trait_impl: &'a TraitImpl,
    pub hosting_element_id: Uid,
}

impl<'a> TraitImplDigest<'a> {
    pub fn new(object_id: Uid) -> Self {
        Self {
            trait_impls: HashMap::new(),
            object_id,
        }
    }
    pub fn get_local_trait_impls(&self) -> HashMap<Uid, &'a RelatedTraitImpl> {
        self.trait_impls
            .iter()
            .filter(|(trait_id, trait_impl_digest)| {
                trait_impl_digest.hosting_element_id != self.object_id
            })
            .map(|(trait_id, trait_impl_digest)| (*trait_id, trait_impl_digest.clone()))
            .collect()
    }
    pub fn get_ancestors_trait_impls(&self) -> HashMap<Uid, &'a RelatedTraitImpl> {
        self.trait_impls
            .iter()
            .filter(|(trait_id, trait_impl_digest)| {
                trait_impl_digest.hosting_element_id != self.object_id
            })
            .map(|(trait_id, trait_impl_digest)| (*trait_id, trait_impl_digest.clone()))
            .collect()
    }
}
