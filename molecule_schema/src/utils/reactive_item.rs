use std::collections::HashMap;

use leptos::*;
use serde_types::common::{ConstraintTraits, Uid};

use super::reactive_types::{
    RConstraintSchema, RFieldConstraint, RFulfilledFieldConstraint, RFulfilledOperative,
    RLibraryInstance, RLibraryOperative, RLibraryTemplate, RTag, RTraitImpl, RTraitOperative,
    Tagged,
};

pub trait RConstraintSchemaItem: Tagged + PartialEq {
    type TTypes: ConstraintTraits;

    type TValues: ConstraintTraits;

    fn get_template_id(&self) -> Uid;
    fn get_parent_operative_id(&self) -> Option<Uid>;
    fn get_local_trait_impls(&self) -> HashMap<Uid, RTraitImpl>;
    fn get_ancestors_trait_impls(
        &self,
        schema: &RConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> HashMap<Uid, RTraitImpl>;
    fn get_local_fulfilled_library_operatives(&self) -> Vec<RFulfilledOperative>;
    fn get_ancestors_fulfilled_library_operatives(
        &self,
        schema: &RConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Vec<RFulfilledOperative>;
    fn get_all_unfulfilled_library_operatives_ids(
        &self,
        schema: &RConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Vec<Uid>;
    fn get_all_unfulfilled_library_operatives(
        &self,
        schema: &RConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Vec<RLibraryOperative<Self::TTypes, Self::TValues>> {
        self.get_all_unfulfilled_library_operatives_ids(schema)
            .iter()
            .map(|op_id| {
                schema
                    .operative_library
                    .with(|ops| ops.get(op_id).unwrap().clone())
            })
            .collect()
    }
    fn get_local_fulfilled_trait_operatives(&self) -> Vec<RFulfilledOperative>;
    fn get_ancestors_fulfilled_trait_operatives(
        &self,
        schema: &RConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Vec<RFulfilledOperative>;
    fn get_all_unfulfilled_trait_operatives(
        &self,
        schema: &RConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Vec<RTraitOperative>;
    fn get_local_fulfilled_fields(
        &self,
    ) -> Vec<RFulfilledFieldConstraint<Self::TTypes, Self::TValues>>;
    fn get_ancestors_fulfilled_fields(
        &self,
        schema: &RConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Vec<RFulfilledFieldConstraint<Self::TTypes, Self::TValues>>;
    fn get_all_unfulfilled_fields(
        &self,
        schema: &RConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Vec<RFieldConstraint<Self::TTypes>>;
    fn get_all_constituent_instance_ids(
        &self,
        schema: &RConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Vec<Uid>;
    fn get_all_constituent_instances(
        &self,
        schema: &RConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Vec<RLibraryInstance<Self::TTypes, Self::TValues>> {
        self.get_all_constituent_instance_ids(schema)
            .iter()
            .map(|instance_id| {
                schema
                    .instance_library
                    .with(|instances| instances.get(instance_id).unwrap().clone())
            })
            .collect()
    }
}

impl<TTypes: ConstraintTraits, TValues: ConstraintTraits> RConstraintSchemaItem
    for RLibraryTemplate<TTypes, TValues>
{
    type TTypes = TTypes;
    type TValues = TValues;
    fn get_template_id(&self) -> Uid {
        <Self as Tagged>::get_tag(self).id.get()
    }

    fn get_parent_operative_id(&self) -> Option<Uid> {
        None
    }

    fn get_local_fulfilled_library_operatives(&self) -> Vec<RFulfilledOperative> {
        Vec::new()
    }

    fn get_ancestors_fulfilled_library_operatives(
        &self,
        _schema: &RConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Vec<RFulfilledOperative> {
        Vec::new()
    }

    fn get_all_unfulfilled_library_operatives_ids(
        &self,
        _schema: &RConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Vec<Uid> {
        self.library_operatives.get()
    }

    fn get_local_fulfilled_trait_operatives(&self) -> Vec<RFulfilledOperative> {
        Vec::new()
    }

    fn get_ancestors_fulfilled_trait_operatives(
        &self,
        _schema: &RConstraintSchema<TTypes, TValues>,
    ) -> Vec<RFulfilledOperative> {
        Vec::new()
    }

    fn get_all_unfulfilled_trait_operatives(
        &self,
        _schema: &RConstraintSchema<TTypes, TValues>,
    ) -> Vec<RTraitOperative> {
        self.trait_operatives.get()
    }

    fn get_local_fulfilled_fields(&self) -> Vec<RFulfilledFieldConstraint<TTypes, TValues>> {
        Vec::new()
    }

    fn get_ancestors_fulfilled_fields(
        &self,
        _schema: &RConstraintSchema<TTypes, TValues>,
    ) -> Vec<RFulfilledFieldConstraint<TTypes, TValues>> {
        Vec::new()
    }

    fn get_all_unfulfilled_fields(
        &self,
        _schema: &RConstraintSchema<TTypes, TValues>,
    ) -> Vec<RFieldConstraint<TTypes>> {
        self.field_constraints.get()
    }

    fn get_all_constituent_instance_ids(
        &self,
        _schema: &RConstraintSchema<TTypes, TValues>,
    ) -> Vec<Uid> {
        self.instances.get()
    }

    fn get_local_trait_impls(&self) -> HashMap<Uid, RTraitImpl> {
        self.trait_impls.get()
    }

    fn get_ancestors_trait_impls(
        &self,
        _schema: &RConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> HashMap<Uid, RTraitImpl> {
        HashMap::new()
    }
}

impl<TTypes: ConstraintTraits, TValues: ConstraintTraits> RConstraintSchemaItem
    for RLibraryOperative<TTypes, TValues>
{
    type TTypes = TTypes;
    type TValues = TValues;
    fn get_template_id(&self) -> Uid {
        self.template_id.get()
    }

    fn get_parent_operative_id(&self) -> Option<Uid> {
        self.parent_operative_id.get()
    }

    fn get_local_trait_impls(&self) -> HashMap<Uid, RTraitImpl> {
        self.trait_impls.get()
    }

    fn get_ancestors_trait_impls(
        &self,
        schema: &RConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> HashMap<Uid, RTraitImpl> {
        let mut maybe_next_parent = self.parent_operative_id.get();
        let mut ancestor_trait_impls = HashMap::new();
        while let Some(next_parent) = maybe_next_parent {
            schema.operative_library.with(|ops| {
                let parent = ops.get(&next_parent).unwrap();
                ancestor_trait_impls.extend(parent.get_local_trait_impls());
                maybe_next_parent = parent.parent_operative_id.get();
            });
        }
        let template_traits_impled = schema
            .template_library
            .with(|templates| {
                templates
                    .get(&self.template_id.get())
                    .unwrap()
                    .trait_impls
                    .get()
            })
            .clone();
        ancestor_trait_impls.extend(template_traits_impled);
        ancestor_trait_impls
    }

    fn get_local_fulfilled_library_operatives(&self) -> Vec<RFulfilledOperative> {
        self.fulfilled_library_operatives.get()
    }

    fn get_ancestors_fulfilled_library_operatives(
        &self,
        schema: &RConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Vec<RFulfilledOperative> {
        let mut maybe_next_parent = self.parent_operative_id.get();
        let mut ancestor_fulfilled = Vec::new();
        while let Some(next_parent) = maybe_next_parent {
            schema.operative_library.with(|ops| {
                let parent = ops.get(&next_parent).unwrap();
                ancestor_fulfilled.extend(parent.get_local_fulfilled_library_operatives());
                maybe_next_parent = parent.parent_operative_id.get();
            });
        }
        ancestor_fulfilled
    }

    fn get_all_unfulfilled_library_operatives_ids(
        &self,
        schema: &RConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Vec<Uid> {
        let parent_template = schema
            .template_library
            .with(|templates| templates.get(&self.template_id.get()).unwrap().clone());
        let potentially_unfulfilled_op_ids = parent_template.library_operatives.get();
        let fulfilled_op_ids = self
            .get_local_fulfilled_library_operatives()
            .iter()
            .map(|op| op.operative_id.get())
            .collect::<Vec<_>>();
        let ancestor_fulfilled = self
            .get_ancestors_fulfilled_library_operatives(schema)
            .iter()
            .map(|op| op.operative_id.get())
            .collect::<Vec<_>>();

        potentially_unfulfilled_op_ids
            .into_iter()
            .filter(|op_id| {
                !fulfilled_op_ids.contains(op_id) && !ancestor_fulfilled.contains(op_id)
            })
            .collect::<Vec<_>>()
    }

    fn get_local_fulfilled_trait_operatives(&self) -> Vec<RFulfilledOperative> {
        self.fulfilled_trait_operatives.get()
    }

    fn get_ancestors_fulfilled_trait_operatives(
        &self,
        schema: &RConstraintSchema<TTypes, TValues>,
    ) -> Vec<RFulfilledOperative> {
        let mut maybe_next_parent = self.parent_operative_id.get();
        let mut ancestor_fulfilled = Vec::new();
        while let Some(next_parent) = maybe_next_parent {
            schema.operative_library.with(|ops| {
                let parent = ops.get(&next_parent).unwrap();
                ancestor_fulfilled.extend(parent.get_local_fulfilled_trait_operatives());
                maybe_next_parent = parent.parent_operative_id.get();
            });
        }
        ancestor_fulfilled
    }

    fn get_all_unfulfilled_trait_operatives(
        &self,
        schema: &RConstraintSchema<TTypes, TValues>,
    ) -> Vec<RTraitOperative> {
        let parent_template = schema
            .template_library
            .with(|templates| templates.get(&self.template_id.get()).unwrap().clone());
        let potentially_unfulfilled_trait_ops = parent_template.trait_operatives.get();
        let fulfilled_op_ids = self
            .get_local_fulfilled_trait_operatives()
            .iter()
            .map(|op| op.operative_id.get())
            .collect::<Vec<_>>();
        let ancestor_fulfilled = self
            .get_ancestors_fulfilled_trait_operatives(schema)
            .iter()
            .map(|op| op.operative_id.get())
            .collect::<Vec<_>>();

        potentially_unfulfilled_trait_ops
            .into_iter()
            .filter(|trait_op| {
                let trait_op_id = &trait_op.tag.id.get();
                !fulfilled_op_ids.contains(trait_op_id) && !ancestor_fulfilled.contains(trait_op_id)
            })
            .collect::<Vec<_>>()
    }

    fn get_local_fulfilled_fields(&self) -> Vec<RFulfilledFieldConstraint<TTypes, TValues>> {
        self.locked_fields.get()
    }

    fn get_ancestors_fulfilled_fields(
        &self,
        schema: &RConstraintSchema<TTypes, TValues>,
    ) -> Vec<RFulfilledFieldConstraint<TTypes, TValues>> {
        let mut maybe_next_parent = self.parent_operative_id.get();
        let mut ancestor_fulfilled = Vec::new();
        while let Some(next_parent) = maybe_next_parent {
            schema.operative_library.with(|ops| {
                let parent = ops.get(&next_parent).unwrap();
                ancestor_fulfilled.extend(parent.get_local_fulfilled_fields());
                maybe_next_parent = parent.parent_operative_id.get();
            });
        }
        ancestor_fulfilled
    }

    fn get_all_unfulfilled_fields(
        &self,
        schema: &RConstraintSchema<TTypes, TValues>,
    ) -> Vec<RFieldConstraint<TTypes>> {
        let parent_template = schema
            .template_library
            .with(|templates| templates.get(&self.template_id.get()).unwrap().clone());
        let potentially_unfulfilled_fields = parent_template.field_constraints.get();
        let fulfilled_field_ids = self
            .get_local_fulfilled_fields()
            .iter()
            .map(|field| field.tag.id.get())
            .collect::<Vec<_>>();
        let ancestor_fulfilled = self
            .get_ancestors_fulfilled_fields(schema)
            .iter()
            .map(|field| field.tag.id.get())
            .collect::<Vec<_>>();

        potentially_unfulfilled_fields
            .into_iter()
            .filter(|field| {
                let field_id = &field.tag.id.get();
                !fulfilled_field_ids.contains(field_id) && !ancestor_fulfilled.contains(field_id)
            })
            .collect::<Vec<_>>()
    }

    fn get_all_constituent_instance_ids(
        &self,
        schema: &RConstraintSchema<TTypes, TValues>,
    ) -> Vec<Uid> {
        let parent_template = schema
            .template_library
            .with(|templates| templates.get(&self.template_id.get()).unwrap().clone());
        let mut template_instance_ids = parent_template.instances.get();
        let lib_op_instance_ids = self
            .get_local_fulfilled_library_operatives()
            .into_iter()
            .chain(self.get_ancestors_fulfilled_library_operatives(schema))
            .map(|op| op.operative_id.get())
            .collect::<Vec<_>>();
        let trait_op_instance_ids = self
            .get_local_fulfilled_trait_operatives()
            .iter()
            .chain(self.get_ancestors_fulfilled_trait_operatives(schema).iter())
            .map(|op| op.operative_id.get())
            .collect::<Vec<_>>();

        template_instance_ids.extend(lib_op_instance_ids);
        template_instance_ids.extend(trait_op_instance_ids);
        template_instance_ids
    }
}

impl<TTypes: ConstraintTraits, TValues: ConstraintTraits> RConstraintSchemaItem
    for RLibraryInstance<TTypes, TValues>
{
    type TTypes = TTypes;
    type TValues = TValues;
    fn get_template_id(&self) -> Uid {
        self.template_id.get()
    }

    fn get_parent_operative_id(&self) -> Option<Uid> {
        self.parent_operative_id.get()
    }

    fn get_local_fulfilled_library_operatives(&self) -> Vec<RFulfilledOperative> {
        self.fulfilled_library_operatives.get()
    }

    fn get_ancestors_fulfilled_library_operatives(
        &self,
        schema: &RConstraintSchema<TTypes, TValues>,
    ) -> Vec<RFulfilledOperative> {
        let mut maybe_next_parent = self.parent_operative_id.get();
        let mut ancestor_fulfilled = Vec::new();
        while let Some(next_parent) = maybe_next_parent {
            schema.operative_library.with(|ops| {
                let parent = ops.get(&next_parent).unwrap();
                ancestor_fulfilled.extend(parent.get_local_fulfilled_library_operatives());
                maybe_next_parent = parent.parent_operative_id.get();
            });
        }
        ancestor_fulfilled
    }

    fn get_all_unfulfilled_library_operatives_ids(
        &self,
        schema: &RConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Vec<Uid> {
        let parent_template = schema
            .template_library
            .with(|templates| templates.get(&self.template_id.get()).unwrap().clone());
        let potentially_unfulfilled_op_ids = parent_template.library_operatives.get();
        let fulfilled_op_ids = self
            .get_local_fulfilled_library_operatives()
            .iter()
            .map(|op| op.operative_id.get())
            .collect::<Vec<_>>();
        let ancestor_fulfilled = self
            .get_ancestors_fulfilled_library_operatives(schema)
            .iter()
            .map(|op| op.operative_id.get())
            .collect::<Vec<_>>();

        potentially_unfulfilled_op_ids
            .into_iter()
            .filter(|op_id| {
                !fulfilled_op_ids.contains(op_id) && !ancestor_fulfilled.contains(op_id)
            })
            .collect::<Vec<_>>()
    }

    fn get_local_fulfilled_trait_operatives(&self) -> Vec<RFulfilledOperative> {
        Vec::new()
    }

    fn get_ancestors_fulfilled_trait_operatives(
        &self,
        schema: &RConstraintSchema<TTypes, TValues>,
    ) -> Vec<RFulfilledOperative> {
        let mut maybe_next_parent = self.parent_operative_id.get();
        let mut ancestor_fulfilled = Vec::new();
        while let Some(next_parent) = maybe_next_parent {
            schema.operative_library.with(|ops| {
                let parent = ops.get(&next_parent).unwrap();
                ancestor_fulfilled.extend(parent.get_local_fulfilled_trait_operatives());
                maybe_next_parent = parent.parent_operative_id.get();
            });
        }
        ancestor_fulfilled
    }

    fn get_all_unfulfilled_trait_operatives(
        &self,
        schema: &RConstraintSchema<TTypes, TValues>,
    ) -> Vec<RTraitOperative> {
        let parent_template = schema
            .template_library
            .with(|templates| templates.get(&self.template_id.get()).unwrap().clone());
        let potentially_unfulfilled_trait_ops = parent_template.trait_operatives.get();
        let fulfilled_op_ids = self
            .get_local_fulfilled_trait_operatives()
            .iter()
            .map(|op| op.operative_id.get())
            .collect::<Vec<_>>();
        let ancestor_fulfilled = self
            .get_ancestors_fulfilled_trait_operatives(schema)
            .iter()
            .map(|op| op.operative_id.get())
            .collect::<Vec<_>>();

        potentially_unfulfilled_trait_ops
            .into_iter()
            .filter(|trait_op| {
                let trait_op_id = &trait_op.tag.id.get();
                !fulfilled_op_ids.contains(trait_op_id) && !ancestor_fulfilled.contains(trait_op_id)
            })
            .collect::<Vec<_>>()
    }

    fn get_local_fulfilled_fields(&self) -> Vec<RFulfilledFieldConstraint<TTypes, TValues>> {
        self.data.get()
    }

    fn get_ancestors_fulfilled_fields(
        &self,
        schema: &RConstraintSchema<TTypes, TValues>,
    ) -> Vec<RFulfilledFieldConstraint<TTypes, TValues>> {
        let mut maybe_next_parent = self.parent_operative_id.get();
        let mut ancestor_fulfilled = Vec::new();
        while let Some(next_parent) = maybe_next_parent {
            schema.operative_library.with(|ops| {
                let parent = ops.get(&next_parent).unwrap();
                ancestor_fulfilled.extend(parent.get_local_fulfilled_fields());
                maybe_next_parent = parent.parent_operative_id.get();
            });
        }
        ancestor_fulfilled
    }

    fn get_all_unfulfilled_fields(
        &self,
        schema: &RConstraintSchema<TTypes, TValues>,
    ) -> Vec<RFieldConstraint<TTypes>> {
        let parent_template = schema
            .template_library
            .with(|templates| templates.get(&self.template_id.get()).unwrap().clone());
        let potentially_unfulfilled_fields = parent_template.field_constraints.get();
        let fulfilled_field_ids = self
            .get_local_fulfilled_fields()
            .iter()
            .map(|field| field.tag.id.get())
            .collect::<Vec<_>>();
        let ancestor_fulfilled = self
            .get_ancestors_fulfilled_fields(schema)
            .iter()
            .map(|field| field.tag.id.get())
            .collect::<Vec<_>>();

        potentially_unfulfilled_fields
            .into_iter()
            .filter(|field| {
                let field_id = &field.tag.id.get();
                !fulfilled_field_ids.contains(field_id) && !ancestor_fulfilled.contains(field_id)
            })
            .collect::<Vec<_>>()
    }

    fn get_all_constituent_instance_ids(
        &self,
        schema: &RConstraintSchema<TTypes, TValues>,
    ) -> Vec<Uid> {
        let parent_template = schema
            .template_library
            .with(|templates| templates.get(&self.template_id.get()).unwrap().clone());
        let mut template_instance_ids = parent_template.instances.get();
        let lib_op_instance_ids = self
            .get_local_fulfilled_library_operatives()
            .into_iter()
            .chain(self.get_ancestors_fulfilled_library_operatives(schema))
            .map(|op| op.operative_id.get())
            .collect::<Vec<_>>();
        let trait_op_instance_ids = self
            .get_local_fulfilled_trait_operatives()
            .iter()
            .chain(self.get_ancestors_fulfilled_trait_operatives(schema).iter())
            .map(|op| op.operative_id.get())
            .collect::<Vec<_>>();

        template_instance_ids.extend(lib_op_instance_ids);
        template_instance_ids.extend(trait_op_instance_ids);
        template_instance_ids
    }

    fn get_local_trait_impls(&self) -> HashMap<Uid, RTraitImpl> {
        self.trait_impls.get()
    }

    fn get_ancestors_trait_impls(
        &self,
        schema: &RConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> HashMap<Uid, RTraitImpl> {
        let mut maybe_next_parent = self.parent_operative_id.get();
        let mut ancestor_trait_impls = HashMap::new();
        while let Some(next_parent) = maybe_next_parent {
            schema.operative_library.with(|ops| {
                let parent = ops.get(&next_parent).unwrap();
                ancestor_trait_impls.extend(parent.get_local_trait_impls());
                maybe_next_parent = parent.parent_operative_id.get();
            });
        }
        let template_traits_impled = schema
            .template_library
            .with(|templates| {
                templates
                    .get(&self.template_id.get())
                    .unwrap()
                    .trait_impls
                    .get()
            })
            .clone();
        ancestor_trait_impls.extend(template_traits_impled);
        ancestor_trait_impls
    }
}

#[derive(PartialEq)]
pub enum RItem<TTypes: ConstraintTraits, TValues: ConstraintTraits> {
    Template(RLibraryTemplate<TTypes, TValues>),
    LibraryOperative(RLibraryOperative<TTypes, TValues>),
    Instance(RLibraryInstance<TTypes, TValues>),
}

impl<TTypes: ConstraintTraits, TValues: ConstraintTraits> Tagged for RItem<TTypes, TValues> {
    fn get_tag(&self) -> &RTag {
        match self {
            Self::Template(_item) => <Self as Tagged>::get_tag(self),
            Self::LibraryOperative(_item) => <Self as Tagged>::get_tag(self),
            Self::Instance(_item) => <Self as Tagged>::get_tag(self),
        }
    }
}

impl<TTypes: ConstraintTraits, TValues: ConstraintTraits> RConstraintSchemaItem
    for RItem<TTypes, TValues>
{
    type TTypes = TTypes;

    type TValues = TValues;

    fn get_template_id(&self) -> Uid {
        match self {
            Self::Template(item) => item.get_template_id(),
            Self::LibraryOperative(item) => item.get_template_id(),
            Self::Instance(item) => item.get_template_id(),
        }
    }

    fn get_parent_operative_id(&self) -> Option<Uid> {
        match self {
            Self::Template(item) => item.get_parent_operative_id(),
            Self::LibraryOperative(item) => item.get_parent_operative_id(),
            Self::Instance(item) => item.get_parent_operative_id(),
        }
    }

    fn get_local_trait_impls(&self) -> HashMap<Uid, RTraitImpl> {
        match self {
            Self::Template(item) => item.get_local_trait_impls(),
            Self::LibraryOperative(item) => item.get_local_trait_impls(),
            Self::Instance(item) => item.get_local_trait_impls(),
        }
    }

    fn get_ancestors_trait_impls(
        &self,
        schema: &RConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> HashMap<Uid, RTraitImpl> {
        match self {
            Self::Template(item) => item.get_ancestors_trait_impls(schema),
            Self::LibraryOperative(item) => item.get_ancestors_trait_impls(schema),
            Self::Instance(item) => item.get_ancestors_trait_impls(schema),
        }
    }

    fn get_local_fulfilled_library_operatives(&self) -> Vec<RFulfilledOperative> {
        match self {
            Self::Template(item) => item.get_local_fulfilled_library_operatives(),
            Self::LibraryOperative(item) => item.get_local_fulfilled_library_operatives(),
            Self::Instance(item) => item.get_local_fulfilled_library_operatives(),
        }
    }

    fn get_ancestors_fulfilled_library_operatives(
        &self,
        schema: &RConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Vec<RFulfilledOperative> {
        match self {
            Self::Template(item) => item.get_ancestors_fulfilled_library_operatives(schema),
            Self::LibraryOperative(item) => item.get_ancestors_fulfilled_library_operatives(schema),
            Self::Instance(item) => item.get_ancestors_fulfilled_library_operatives(schema),
        }
    }

    fn get_all_unfulfilled_library_operatives_ids(
        &self,
        schema: &RConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Vec<Uid> {
        match self {
            Self::Template(item) => item.get_all_unfulfilled_library_operatives_ids(schema),
            Self::LibraryOperative(item) => item.get_all_unfulfilled_library_operatives_ids(schema),
            Self::Instance(item) => item.get_all_unfulfilled_library_operatives_ids(schema),
        }
    }

    fn get_local_fulfilled_trait_operatives(&self) -> Vec<RFulfilledOperative> {
        todo!()
    }

    fn get_ancestors_fulfilled_trait_operatives(
        &self,
        schema: &RConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Vec<RFulfilledOperative> {
        match self {
            Self::Template(item) => item.get_ancestors_fulfilled_trait_operatives(schema),
            Self::LibraryOperative(item) => item.get_ancestors_fulfilled_trait_operatives(schema),
            Self::Instance(item) => item.get_ancestors_fulfilled_trait_operatives(schema),
        }
    }

    fn get_all_unfulfilled_trait_operatives(
        &self,
        schema: &RConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Vec<RTraitOperative> {
        match self {
            Self::Template(item) => item.get_all_unfulfilled_trait_operatives(schema),
            Self::LibraryOperative(item) => item.get_all_unfulfilled_trait_operatives(schema),
            Self::Instance(item) => item.get_all_unfulfilled_trait_operatives(schema),
        }
    }

    fn get_local_fulfilled_fields(
        &self,
    ) -> Vec<RFulfilledFieldConstraint<Self::TTypes, Self::TValues>> {
        todo!()
    }

    fn get_ancestors_fulfilled_fields(
        &self,
        schema: &RConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Vec<RFulfilledFieldConstraint<Self::TTypes, Self::TValues>> {
        match self {
            Self::Template(item) => item.get_ancestors_fulfilled_fields(schema),
            Self::LibraryOperative(item) => item.get_ancestors_fulfilled_fields(schema),
            Self::Instance(item) => item.get_ancestors_fulfilled_fields(schema),
        }
    }

    fn get_all_unfulfilled_fields(
        &self,
        schema: &RConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Vec<RFieldConstraint<Self::TTypes>> {
        match self {
            Self::Template(item) => item.get_all_unfulfilled_fields(schema),
            Self::LibraryOperative(item) => item.get_all_unfulfilled_fields(schema),
            Self::Instance(item) => item.get_all_unfulfilled_fields(schema),
        }
    }

    fn get_all_constituent_instance_ids(
        &self,
        schema: &RConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Vec<Uid> {
        match self {
            Self::Template(item) => item.get_all_constituent_instance_ids(schema),
            Self::LibraryOperative(item) => item.get_all_constituent_instance_ids(schema),
            Self::Instance(item) => item.get_all_constituent_instance_ids(schema),
        }
    }
}
