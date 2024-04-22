use std::collections::HashMap;

use leptos::RwSignal;

use crate::{
    common::Uid,
    constraint_schema::ConstraintSchema,
    primitives::{PrimitiveTypes, PrimitiveValues},
};

// use super::GSO;

pub trait RGSO {}

#[derive(Debug)]
pub struct RBaseGraphEnvironment<TSchema: RGSO + 'static> {
    pub created_instances: RwSignal<HashMap<Uid, TSchema>>,
    pub constraint_schema: ConstraintSchema<PrimitiveTypes, PrimitiveValues>,
}
