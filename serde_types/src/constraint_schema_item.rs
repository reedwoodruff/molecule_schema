use std::collections::HashMap;

use crate::{
    common::{ConstraintTraits, Tag, Uid},
    constraint_schema::{
        ConstraintSchema, ConstraintSchemaInstantiableType, FieldConstraint,
        FulfilledFieldConstraint, FulfilledOperative, LibraryInstance, LibraryOperative,
        LibraryTemplate, TraitImpl, TraitOperative,
    },
};

pub trait ConstraintSchemaInstantiable {
    type TTypes: ConstraintTraits;

    type TValues: ConstraintTraits;

    fn get_template_id(&self) -> &Uid;
    fn get_constraint_schema_instantiable_type(&self) -> ConstraintSchemaInstantiableType;
    fn get_parent_operative_id(&self) -> Option<&Uid>;
    fn get_tag(&self) -> &Tag;
    fn get_local_trait_impls(&self) -> &HashMap<Uid, TraitImpl>;
    fn get_ancestors_trait_impls(
        &self,
        schema: &ConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> HashMap<Uid, TraitImpl>;
    fn get_local_fulfilled_library_operatives(&self) -> Option<&Vec<FulfilledOperative>>;
    fn get_ancestors_fulfilled_library_operatives(
        &self,
        schema: &ConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Vec<FulfilledOperative>;
    fn get_all_unfulfilled_library_operatives_ids(
        &self,
        schema: &ConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Vec<Uid>;
    fn get_all_unfulfilled_library_operatives<'a>(
        &'a self,
        schema: &'a ConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Vec<&LibraryOperative<Self::TTypes, Self::TValues>> {
        self.get_all_unfulfilled_library_operatives_ids(schema)
            .into_iter()
            .map(|id| {
                let operative = schema
                    .operative_library
                    .get(&id)
                    .expect("operative should be present");
                operative
            })
            .collect()
    }
    fn get_local_fulfilled_trait_operatives(&self) -> Option<&Vec<FulfilledOperative>>;
    fn get_ancestors_fulfilled_trait_operatives(
        &self,
        schema: &ConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Vec<FulfilledOperative>;
    fn get_all_unfulfilled_trait_operatives(
        &self,
        schema: &ConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Vec<TraitOperative>;
    fn get_local_fulfilled_fields(
        &self,
    ) -> Option<&Vec<FulfilledFieldConstraint<Self::TTypes, Self::TValues>>>;
    fn get_ancestors_fulfilled_fields(
        &self,
        schema: &ConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Vec<FulfilledFieldConstraint<Self::TTypes, Self::TValues>>;
    fn get_all_unfulfilled_fields(
        &self,
        schema: &ConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Vec<FieldConstraint<Self::TTypes>>;
    fn get_all_constituent_instance_ids(
        &self,
        schema: &ConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Vec<Uid> {
        let parent_template = schema
            .template_library
            .get(&self.get_template_id())
            .unwrap();
        let mut template_instance_ids = parent_template.instances.clone();
        let lib_op_instance_ids = self
            .get_local_fulfilled_library_operatives()
            .cloned()
            .unwrap_or_else(Vec::new)
            .into_iter()
            .chain(self.get_ancestors_fulfilled_library_operatives(schema))
            .map(|op| op.fulfilling_instance_id)
            .collect::<Vec<_>>();
        let trait_op_instance_ids = self
            .get_local_fulfilled_trait_operatives()
            .cloned()
            .unwrap_or_else(Vec::new)
            .iter()
            .chain(self.get_ancestors_fulfilled_trait_operatives(schema).iter())
            .map(|op| op.fulfilling_instance_id)
            .collect::<Vec<_>>();

        template_instance_ids.extend(lib_op_instance_ids);
        template_instance_ids.extend(trait_op_instance_ids);
        template_instance_ids
    }
    fn get_all_constituent_instances<'a>(
        &'a self,
        schema: &'a ConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Vec<&LibraryInstance<Self::TTypes, Self::TValues>> {
        self.get_all_constituent_instance_ids(schema)
            .iter()
            .map(|instance_id| schema.instance_library.get(instance_id).unwrap())
            .collect()
    }
}

impl<TTypes: ConstraintTraits, TValues: ConstraintTraits> ConstraintSchemaInstantiable
    for LibraryTemplate<TTypes, TValues>
{
    type TTypes = TTypes;
    type TValues = TValues;

    fn get_template_id(&self) -> &Uid {
        &self.tag.id
    }
    fn get_constraint_schema_instantiable_type(&self) -> ConstraintSchemaInstantiableType {
        ConstraintSchemaInstantiableType::Template
    }
    fn get_parent_operative_id(&self) -> Option<&Uid> {
        None
    }
    fn get_tag(&self) -> &Tag {
        &self.tag
    }
    fn get_local_trait_impls(&self) -> &HashMap<Uid, TraitImpl> {
        &self.trait_impls
    }
    fn get_ancestors_trait_impls(
        &self,
        _schema: &ConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> HashMap<Uid, TraitImpl> {
        HashMap::new()
    }
    fn get_local_fulfilled_library_operatives(&self) -> Option<&Vec<FulfilledOperative>> {
        None
    }

    fn get_ancestors_fulfilled_library_operatives(
        &self,
        _schema: &ConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Vec<FulfilledOperative> {
        Vec::new()
    }

    fn get_all_unfulfilled_library_operatives_ids(
        &self,
        _schema: &ConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Vec<Uid> {
        self.library_operatives.clone()
    }

    fn get_local_fulfilled_trait_operatives(&self) -> Option<&Vec<FulfilledOperative>> {
        None
    }

    fn get_ancestors_fulfilled_trait_operatives(
        &self,
        _schema: &ConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Vec<FulfilledOperative> {
        Vec::new()
    }

    fn get_all_unfulfilled_trait_operatives(
        &self,
        _schema: &ConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Vec<TraitOperative> {
        self.trait_operatives.clone()
    }

    fn get_local_fulfilled_fields(
        &self,
    ) -> Option<&Vec<FulfilledFieldConstraint<Self::TTypes, Self::TValues>>> {
        None
    }

    fn get_ancestors_fulfilled_fields(
        &self,
        _schema: &ConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Vec<FulfilledFieldConstraint<Self::TTypes, Self::TValues>> {
        Vec::new()
    }

    fn get_all_unfulfilled_fields(
        &self,
        _schema: &ConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Vec<FieldConstraint<Self::TTypes>> {
        self.field_constraints.clone()
    }
}

impl<TTypes: ConstraintTraits, TValues: ConstraintTraits> ConstraintSchemaInstantiable
    for LibraryOperative<TTypes, TValues>
{
    type TTypes = TTypes;
    type TValues = TValues;

    fn get_template_id(&self) -> &Uid {
        &self.template_id
    }
    fn get_constraint_schema_instantiable_type(&self) -> ConstraintSchemaInstantiableType {
        ConstraintSchemaInstantiableType::Operative
    }
    fn get_parent_operative_id(&self) -> Option<&Uid> {
        self.parent_operative_id.as_ref()
    }
    fn get_tag(&self) -> &Tag {
        &self.tag
    }
    fn get_local_trait_impls(&self) -> &HashMap<Uid, TraitImpl> {
        &self.trait_impls
    }
    fn get_ancestors_trait_impls(
        &self,
        schema: &ConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> HashMap<Uid, TraitImpl> {
        let mut maybe_next_parent = self.parent_operative_id;
        let mut ancestor_trait_impls = HashMap::new();
        while let Some(next_parent) = maybe_next_parent {
            let parent = schema.operative_library.get(&next_parent).unwrap();
            ancestor_trait_impls.extend(parent.get_local_trait_impls().to_owned());
            maybe_next_parent = parent.parent_operative_id;
        }
        let template_traits_impled = schema
            .template_library
            .get(&self.template_id)
            .unwrap()
            .trait_impls
            .clone();
        ancestor_trait_impls.extend(template_traits_impled);
        ancestor_trait_impls
    }
    fn get_local_fulfilled_library_operatives(&self) -> Option<&Vec<FulfilledOperative>> {
        Some(&self.fulfilled_library_operatives)
    }
    fn get_ancestors_fulfilled_library_operatives(
        &self,
        schema: &ConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Vec<FulfilledOperative> {
        let mut maybe_next_parent = self.parent_operative_id;
        let mut ancestor_fulfilled = Vec::new();
        while let Some(next_parent) = maybe_next_parent {
            let parent = schema.operative_library.get(&next_parent).unwrap();
            let parent_ops = parent.get_local_fulfilled_library_operatives();
            if let Some(parent_ops) = parent_ops {
                ancestor_fulfilled.extend(parent_ops.clone());
            }
            maybe_next_parent = parent.parent_operative_id;
        }
        ancestor_fulfilled
    }

    fn get_all_unfulfilled_library_operatives_ids(
        &self,
        schema: &ConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Vec<Uid> {
        let parent_template = schema.template_library.get(&self.template_id).unwrap();
        let potentially_unfulfilled_op_ids = parent_template.library_operatives.clone();
        let fulfilled_op_ids = self
            .get_local_fulfilled_library_operatives()
            .cloned()
            .unwrap_or_else(Vec::new)
            .into_iter()
            .map(|op| op.operative_id)
            .collect::<Vec<_>>();
        let ancestor_fulfilled = self
            .get_ancestors_fulfilled_library_operatives(schema)
            .into_iter()
            .map(|op| op.operative_id)
            .collect::<Vec<_>>();

        potentially_unfulfilled_op_ids
            .into_iter()
            .filter(|op_id| {
                !fulfilled_op_ids.contains(op_id) && !ancestor_fulfilled.contains(op_id)
            })
            .collect::<Vec<_>>()
    }

    fn get_local_fulfilled_trait_operatives(&self) -> Option<&Vec<FulfilledOperative>> {
        Some(&self.fulfilled_trait_operatives)
    }

    fn get_ancestors_fulfilled_trait_operatives(
        &self,
        schema: &ConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Vec<FulfilledOperative> {
        let mut maybe_next_parent = self.parent_operative_id;
        let mut ancestor_fulfilled = Vec::new();
        while let Some(next_parent) = maybe_next_parent {
            let parent = schema.operative_library.get(&next_parent).unwrap();
            if let Some(parent_ops) = parent.get_local_fulfilled_trait_operatives() {
                ancestor_fulfilled.extend(parent_ops.clone());
            }
            maybe_next_parent = parent.parent_operative_id;
        }
        ancestor_fulfilled
    }

    fn get_all_unfulfilled_trait_operatives(
        &self,
        schema: &ConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Vec<TraitOperative> {
        let parent_template = schema.template_library.get(&self.template_id).unwrap();
        let potentially_unfulfilled_trait_ops = parent_template.trait_operatives.clone();
        let fulfilled_op_ids = self
            .get_local_fulfilled_trait_operatives()
            .cloned()
            .unwrap_or_else(Vec::new)
            .into_iter()
            .map(|op| op.operative_id)
            .collect::<Vec<_>>();
        let ancestor_fulfilled = self
            .get_ancestors_fulfilled_trait_operatives(schema)
            .into_iter()
            .map(|op| op.operative_id)
            .collect::<Vec<_>>();

        potentially_unfulfilled_trait_ops
            .into_iter()
            .filter(|trait_op| {
                let trait_op_id = &trait_op.tag.id;
                !fulfilled_op_ids.contains(trait_op_id) && !ancestor_fulfilled.contains(trait_op_id)
            })
            .collect::<Vec<_>>()
    }

    fn get_local_fulfilled_fields(
        &self,
    ) -> Option<&Vec<FulfilledFieldConstraint<Self::TTypes, Self::TValues>>> {
        Some(&self.locked_fields)
    }

    fn get_ancestors_fulfilled_fields(
        &self,
        schema: &ConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Vec<FulfilledFieldConstraint<Self::TTypes, Self::TValues>> {
        let mut maybe_next_parent = self.parent_operative_id;
        let mut ancestor_fulfilled = Vec::new();
        while let Some(next_parent) = maybe_next_parent {
            let parent = schema.operative_library.get(&next_parent).unwrap();
            if let Some(parent_fields) = parent.get_local_fulfilled_fields() {
                ancestor_fulfilled.extend(parent_fields.clone());
            }
            maybe_next_parent = parent.parent_operative_id;
        }
        ancestor_fulfilled
    }

    fn get_all_unfulfilled_fields(
        &self,
        schema: &ConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Vec<FieldConstraint<Self::TTypes>> {
        let parent_template = schema.template_library.get(&self.template_id).unwrap();
        let potentially_unfulfilled_fields = parent_template.field_constraints.clone();
        let fulfilled_field_ids = self
            .get_local_fulfilled_fields()
            .cloned()
            .unwrap_or_else(Vec::new)
            .into_iter()
            .map(|field| field.tag.id)
            .collect::<Vec<_>>();
        let ancestor_fulfilled = self
            .get_ancestors_fulfilled_fields(schema)
            .into_iter()
            .map(|field| field.tag.id)
            .collect::<Vec<_>>();

        potentially_unfulfilled_fields
            .into_iter()
            .filter(|field| {
                let field_id = &field.tag.id;
                !fulfilled_field_ids.contains(field_id) && !ancestor_fulfilled.contains(field_id)
            })
            .collect::<Vec<_>>()
    }
}

impl<TTypes: ConstraintTraits, TValues: ConstraintTraits> ConstraintSchemaInstantiable
    for LibraryInstance<TTypes, TValues>
{
    type TTypes = TTypes;
    type TValues = TValues;

    fn get_template_id(&self) -> &Uid {
        &self.template_id
    }
    fn get_constraint_schema_instantiable_type(&self) -> ConstraintSchemaInstantiableType {
        ConstraintSchemaInstantiableType::Operative
    }
    fn get_parent_operative_id(&self) -> Option<&Uid> {
        self.parent_operative_id.as_ref()
    }
    fn get_tag(&self) -> &Tag {
        &self.tag
    }
    fn get_local_trait_impls(&self) -> &HashMap<Uid, TraitImpl> {
        &self.trait_impls
    }
    fn get_ancestors_trait_impls(
        &self,
        schema: &ConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> HashMap<Uid, TraitImpl> {
        let mut maybe_next_parent = self.parent_operative_id;
        let mut ancestor_trait_impls = HashMap::new();
        while let Some(next_parent) = maybe_next_parent {
            let parent = schema.operative_library.get(&next_parent).unwrap();
            ancestor_trait_impls.extend(parent.get_local_trait_impls().to_owned());
            maybe_next_parent = parent.parent_operative_id;
        }
        let template_traits_impled = schema
            .template_library
            .get(&self.template_id)
            .unwrap()
            .trait_impls
            .clone();
        ancestor_trait_impls.extend(template_traits_impled);
        ancestor_trait_impls
    }
    fn get_local_fulfilled_library_operatives(&self) -> Option<&Vec<FulfilledOperative>> {
        Some(&self.fulfilled_library_operatives)
    }
    fn get_ancestors_fulfilled_library_operatives(
        &self,
        schema: &ConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Vec<FulfilledOperative> {
        let mut maybe_next_parent = self.parent_operative_id;
        let mut ancestor_fulfilled = Vec::new();
        while let Some(next_parent) = maybe_next_parent {
            let parent = schema.operative_library.get(&next_parent).unwrap();
            let parent_ops = parent.get_local_fulfilled_library_operatives();
            if let Some(parent_ops) = parent_ops {
                ancestor_fulfilled.extend(parent_ops.clone());
            }
            maybe_next_parent = parent.parent_operative_id;
        }
        ancestor_fulfilled
    }

    fn get_all_unfulfilled_library_operatives_ids(
        &self,
        schema: &ConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Vec<Uid> {
        let parent_template = schema.template_library.get(&self.template_id).unwrap();
        let potentially_unfulfilled_op_ids = parent_template.library_operatives.clone();
        let fulfilled_op_ids = self
            .get_local_fulfilled_library_operatives()
            .cloned()
            .unwrap_or_else(Vec::new)
            .into_iter()
            .map(|op| op.operative_id)
            .collect::<Vec<_>>();
        let ancestor_fulfilled = self
            .get_ancestors_fulfilled_library_operatives(schema)
            .into_iter()
            .map(|op| op.operative_id)
            .collect::<Vec<_>>();

        potentially_unfulfilled_op_ids
            .into_iter()
            .filter(|op_id| {
                !fulfilled_op_ids.contains(op_id) && !ancestor_fulfilled.contains(op_id)
            })
            .collect::<Vec<_>>()
    }

    fn get_local_fulfilled_trait_operatives(&self) -> Option<&Vec<FulfilledOperative>> {
        Some(&self.fulfilled_trait_operatives)
    }

    fn get_ancestors_fulfilled_trait_operatives(
        &self,
        schema: &ConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Vec<FulfilledOperative> {
        let mut maybe_next_parent = self.parent_operative_id;
        let mut ancestor_fulfilled = Vec::new();
        while let Some(next_parent) = maybe_next_parent {
            let parent = schema.operative_library.get(&next_parent).unwrap();
            if let Some(parent_ops) = parent.get_local_fulfilled_trait_operatives() {
                ancestor_fulfilled.extend(parent_ops.clone());
            }
            maybe_next_parent = parent.parent_operative_id;
        }
        ancestor_fulfilled
    }

    fn get_all_unfulfilled_trait_operatives(
        &self,
        schema: &ConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Vec<TraitOperative> {
        let parent_template = schema.template_library.get(&self.template_id).unwrap();
        let potentially_unfulfilled_trait_ops = parent_template.trait_operatives.clone();
        let fulfilled_op_ids = self
            .get_local_fulfilled_trait_operatives()
            .cloned()
            .unwrap_or_else(Vec::new)
            .into_iter()
            .map(|op| op.operative_id)
            .collect::<Vec<_>>();
        let ancestor_fulfilled = self
            .get_ancestors_fulfilled_trait_operatives(schema)
            .into_iter()
            .map(|op| op.operative_id)
            .collect::<Vec<_>>();

        potentially_unfulfilled_trait_ops
            .into_iter()
            .filter(|trait_op| {
                let trait_op_id = &trait_op.tag.id;
                !fulfilled_op_ids.contains(trait_op_id) && !ancestor_fulfilled.contains(trait_op_id)
            })
            .collect::<Vec<_>>()
    }

    fn get_local_fulfilled_fields(
        &self,
    ) -> Option<&Vec<FulfilledFieldConstraint<Self::TTypes, Self::TValues>>> {
        Some(&self.data)
    }

    fn get_ancestors_fulfilled_fields(
        &self,
        schema: &ConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Vec<FulfilledFieldConstraint<Self::TTypes, Self::TValues>> {
        let mut maybe_next_parent = self.parent_operative_id;
        let mut ancestor_fulfilled = Vec::new();
        while let Some(next_parent) = maybe_next_parent {
            let parent = schema.operative_library.get(&next_parent).unwrap();
            if let Some(parent_fields) = parent.get_local_fulfilled_fields() {
                ancestor_fulfilled.extend(parent_fields.clone());
            }
            maybe_next_parent = parent.parent_operative_id;
        }
        ancestor_fulfilled
    }

    fn get_all_unfulfilled_fields(
        &self,
        schema: &ConstraintSchema<Self::TTypes, Self::TValues>,
    ) -> Vec<FieldConstraint<Self::TTypes>> {
        let parent_template = schema.template_library.get(&self.template_id).unwrap();
        let potentially_unfulfilled_fields = parent_template.field_constraints.clone();
        let fulfilled_field_ids = self
            .get_local_fulfilled_fields()
            .cloned()
            .unwrap_or_else(Vec::new)
            .into_iter()
            .map(|field| field.tag.id)
            .collect::<Vec<_>>();
        let ancestor_fulfilled = self
            .get_ancestors_fulfilled_fields(schema)
            .into_iter()
            .map(|field| field.tag.id)
            .collect::<Vec<_>>();

        potentially_unfulfilled_fields
            .into_iter()
            .filter(|field| {
                let field_id = &field.tag.id;
                !fulfilled_field_ids.contains(field_id) && !ancestor_fulfilled.contains(field_id)
            })
            .collect::<Vec<_>>()
    }
}
