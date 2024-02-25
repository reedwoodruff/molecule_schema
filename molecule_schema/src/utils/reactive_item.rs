use leptos::*;
use serde_types::common::{ConstraintTraits, Uid};

use super::reactive_types::{
    RFieldConstraint, RFulfilledFieldConstraint, RFulfilledOperative, RLibraryInstance,
    RLibraryOperative, RLibraryTemplate, RTag, Tagged,
};

pub trait RConstraintSchemaItem<TTypes: ConstraintTraits, TValues: ConstraintTraits> {
    fn get_template_id(&self) -> Uid;
    fn get_parent_operative_id(&self) -> Uid;
    fn get_tag(&self) -> RTag;
    fn get_parent_fulfilled_operatives(&self) -> Vec<RFulfilledOperative>;
    fn get_fulfilled_operatives(&self) -> Vec<RFulfilledOperative>;
    fn get_parent_fulfilled_fields(&self) -> Vec<RFulfilledFieldConstraint<TTypes, TValues>>;
    fn get_fulfilled_fields(&self) -> Vec<RFulfilledFieldConstraint<TTypes, TValues>>;
    fn get_unfulfilled_operatives(&self) -> Vec<RLibraryOperative<TTypes, TValues>>;
    fn get_unfulfilled_fields(&self) -> Vec<RFieldConstraint<TTypes>>;
    fn get_constituent_instances(&self) -> Vec<RLibraryInstance<TTypes, TValues>>;
}

impl<TTypes: ConstraintTraits, TValues: ConstraintTraits> RConstraintSchemaItem<TTypes, TValues>
    for RLibraryTemplate<TTypes, TValues>
{
    fn get_template_id(&self) -> Uid {
        self.tag.id.get()
    }

    fn get_parent_operative_id(&self) -> Uid {
        self.tag.id.get()
    }

    fn get_tag(&self) -> RTag {
        self.tag.clone()
    }

    fn get_parent_fulfilled_operatives(&self) -> Vec<RFulfilledOperative> {
        Vec::new()
    }

    fn get_fulfilled_operatives(&self) -> Vec<RFulfilledOperative> {
        Vec::new()
    }

    fn get_parent_fulfilled_fields(&self) -> Vec<RFulfilledFieldConstraint<TTypes, TValues>> {
        Vec::new()
    }

    fn get_fulfilled_fields(&self) -> Vec<RFulfilledFieldConstraint<TTypes, TValues>> {
        Vec::new()
    }

    fn get_unfulfilled_operatives(&self) -> Vec<RLibraryOperative<TTypes, TValues>> {
        // self.library_operatives
        todo!()
    }

    fn get_unfulfilled_fields(&self) -> Vec<RFieldConstraint<TTypes>> {
        todo!()
    }

    fn get_constituent_instances(&self) -> Vec<RLibraryInstance<TTypes, TValues>> {
        todo!()
    }
}
