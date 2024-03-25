use std::collections::HashMap;

use serde_types::common::Uid;

use super::reactive_types::RTraitImpl;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RTraitImplDigest {
    pub object_id: Uid,
    pub trait_impls: HashMap<Uid, RRelatedTraitImpl>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RRelatedTraitImpl {
    pub trait_impl: RTraitImpl,
    pub hosting_element_id: Uid,
}

impl RTraitImplDigest {
    pub fn new(object_id: Uid) -> Self {
        Self {
            trait_impls: HashMap::new(),
            object_id,
        }
    }
    pub fn get_local_trait_impls(&self) -> HashMap<Uid, RRelatedTraitImpl> {
        self.trait_impls
            .iter()
            .filter(|(trait_id, trait_impl_digest)| {
                trait_impl_digest.hosting_element_id != self.object_id
            })
            .map(|(trait_id, trait_impl_digest)| (trait_id.clone(), trait_impl_digest.clone()))
            .collect()
    }
    pub fn get_ancestors_trait_impls(&self) -> HashMap<Uid, RRelatedTraitImpl> {
        self.trait_impls
            .iter()
            .filter(|(trait_id, trait_impl_digest)| {
                trait_impl_digest.hosting_element_id != self.object_id
            })
            .map(|(trait_id, trait_impl_digest)| (trait_id.clone(), trait_impl_digest.clone()))
            .collect()
    }
}
