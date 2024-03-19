use std::collections::HashMap;

use serde_types::common::Uid;

use super::reactive_types::RTraitImpl;

pub struct RTraitImplDigest(pub HashMap<Uid, RRelatedTraitImpl>);
pub struct RRelatedTraitImpl {
    pub trait_impl: RTraitImpl,
    pub hosting_element_id: Uid,
}
