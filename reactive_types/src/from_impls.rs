use std::{marker::PhantomData};

use crate::{
    reactive_types::{
        RConstraintSchema, RFieldConstraint, RLibraryOperative, RLibraryTemplate,
        RLockedFieldConstraint, ROperativeSlot, ROperativeVariants, RSlotBounds, RSlottedInstances,
        RTag, RTraitDef, RTraitMethodDef, RTraitMethodImplPath, RTraitOperative,
    },
    ConstraintTraits,
};
use leptos::*;

use base_types::{
    common::Tag,
    constraint_schema::{
        ConstraintSchema, FieldConstraint, LibraryOperative, LibraryTemplate,
        LockedFieldConstraint, OperativeSlot, OperativeVariants, SlotBounds, SlottedInstances,
        TraitDef, TraitMethodDef, TraitMethodImplPath, TraitOperative,
    },
};

impl<TTypes: ConstraintTraits, TValues: ConstraintTraits> From<ConstraintSchema<TTypes, TValues>>
    for RConstraintSchema<TTypes, TValues>
{
    fn from(value: ConstraintSchema<TTypes, TValues>) -> Self {
        Self {
            template_library: RwSignal::new(
                value
                    .template_library
                    .iter()
                    // .map(|(index, item)| (*index, (*item).as_ref().clone().into()))
                    .map(|(index, item)| (*index, item.clone().into()))
                    .collect(),
            ),
            instance_library: RwSignal::new(
                value
                    .instance_library
                    .iter()
                    .map(|(index, item)| (*index, item.clone().into()))
                    .collect(),
            ),
            operative_library: RwSignal::new(
                value
                    .operative_library
                    .iter()
                    .map(|(index, item)| {
                        let reactive = item.clone();
                        // let reactive = (*item).as_ref().clone();
                        (*index, reactive.into())
                    })
                    .collect(),
            ),
            traits: RwSignal::new(
                value
                    .traits
                    .iter()
                    .map(|(index, item)| (*index, item.clone().into()))
                    .collect(),
            ),
        }
    }
}
impl<TTypes: ConstraintTraits, TValues: ConstraintTraits> From<RConstraintSchema<TTypes, TValues>>
    for ConstraintSchema<TTypes, TValues>
{
    fn from(val: RConstraintSchema<TTypes, TValues>) -> Self {
        Self {
            template_library: val
                .template_library
                .get()
                .iter()
                // .map(|(index, item)| (*index, Rc::new(item.clone().into())))
                .map(|(index, item)| (*index, item.clone().into()))
                .collect(),

            instance_library: val
                .instance_library
                .get()
                .iter()
                .map(|(index, item)| (*index, item.clone().into()))
                .collect(),

            operative_library: val
                .operative_library
                .get()
                .iter()
                // .map(|(index, item)| (*index, Rc::new(item.clone().into())))
                .map(|(index, item)| (*index, item.clone().into()))
                .collect(),

            traits: val
                .traits
                .get()
                .iter()
                .map(|(index, item)| (*index, item.clone().into()))
                .collect(),
        }
    }
}

impl From<Tag> for RTag {
    fn from(value: Tag) -> Self {
        Self {
            name: RwSignal::new(value.name),
            id: RwSignal::new(value.id),
        }
    }
}
impl From<RTag> for Tag {
    fn from(value: RTag) -> Self {
        Self {
            name: value.name.get(),
            id: value.id.get(),
        }
    }
}

impl<TTypes: ConstraintTraits, TValues: ConstraintTraits> From<LibraryTemplate<TTypes, TValues>>
    for RLibraryTemplate<TTypes, TValues>
{
    fn from(value: LibraryTemplate<TTypes, TValues>) -> Self {
        Self {
            field_constraints: RwSignal::new(
                value
                    .field_constraints
                    .iter()
                    .map(|(index, item)| (*index, item.clone().into()))
                    .collect(),
            ),
            operative_slots: RwSignal::new(
                value
                    .operative_slots
                    .iter()
                    .map(|(index, item)| (*index, item.clone().into()))
                    .collect(),
            ),

            instances: RwSignal::new(value.instances),
            trait_impls: RwSignal::new(
                value
                    .trait_impls
                    .iter()
                    .map(|(index, item)| {
                        (
                            *index,
                            RwSignal::new(
                                item.iter()
                                    .map(|(index_inner, inner)| {
                                        (
                                            *index_inner,
                                            RwSignal::new(
                                                inner
                                                    .clone()
                                                    .iter()
                                                    .cloned()
                                                    .map(|vec_item| vec_item.into())
                                                    .collect(),
                                            ),
                                        )
                                    })
                                    .collect(),
                            ),
                        )
                    })
                    .collect(),
            ),

            tag: value.tag.into(),
            _phantom: PhantomData,
        }
    }
}
impl<TTypes: ConstraintTraits, TValues: ConstraintTraits> From<RLibraryTemplate<TTypes, TValues>>
    for LibraryTemplate<TTypes, TValues>
{
    fn from(value: RLibraryTemplate<TTypes, TValues>) -> Self {
        Self {
            field_constraints: value
                .field_constraints
                .get()
                .iter()
                .map(|(index, item)| (*index, item.clone().into()))
                .collect(),

            operative_slots: value
                .operative_slots
                .get()
                .iter()
                .map(|(index, item)| (*index, item.clone().into()))
                .collect(),

            instances: value.instances.get(),
            trait_impls: value
                .trait_impls
                .get()
                .iter()
                .map(|(index, item)| {
                    (
                        *index,
                        item.get()
                            .iter()
                            .map(|(index_inner, inner)| {
                                (
                                    *index_inner,
                                    inner
                                        .get()
                                        .clone()
                                        .iter()
                                        .cloned()
                                        .map(|vec_item| vec_item.into())
                                        .collect(),
                                )
                            })
                            .collect(),
                    )
                })
                .collect(),

            tag: value.tag.into(),
            _phantom: PhantomData,
        }
    }
}

impl From<SlotBounds> for RSlotBounds {
    fn from(value: SlotBounds) -> Self {
        match value {
            // SlotBounds::Unbounded => RSlotBounds::Unbounded,
            SlotBounds::Single => RSlotBounds::Single,
            SlotBounds::LowerBound(val) => RSlotBounds::LowerBound(RwSignal::new(val)),
            SlotBounds::UpperBound(val) => RSlotBounds::UpperBound(RwSignal::new(val)),
            SlotBounds::Range(lower, upper) => {
                RSlotBounds::Range(RwSignal::new(lower), RwSignal::new(upper))
            }
            SlotBounds::LowerBoundOrZero(val) => RSlotBounds::LowerBoundOrZero(RwSignal::new(val)),
            SlotBounds::RangeOrZero(lower, upper) => {
                RSlotBounds::RangeOrZero(RwSignal::new(lower), RwSignal::new(upper))
            }
        }
    }
}
impl From<RSlotBounds> for SlotBounds {
    fn from(value: RSlotBounds) -> Self {
        match value {
            // RSlotBounds::Unbounded => SlotBounds::Unbounded,
            RSlotBounds::Single => SlotBounds::Single,
            RSlotBounds::LowerBound(val) => SlotBounds::LowerBound(val.get()),
            RSlotBounds::UpperBound(val) => SlotBounds::UpperBound(val.get()),
            RSlotBounds::Range(lower, upper) => SlotBounds::Range(lower.get(), upper.get()),
            RSlotBounds::LowerBoundOrZero(val) => SlotBounds::LowerBoundOrZero(val.get()),
            RSlotBounds::RangeOrZero(lower, upper) => {
                SlotBounds::RangeOrZero(lower.get(), upper.get())
            }
        }
    }
}

impl From<OperativeSlot> for ROperativeSlot {
    fn from(value: OperativeSlot) -> Self {
        Self {
            tag: value.tag.into(),
            operative_descriptor: value.operative_descriptor.into(),
            bounds: RwSignal::new(value.bounds.into()),
        }
    }
}
impl From<ROperativeSlot> for OperativeSlot {
    fn from(value: ROperativeSlot) -> Self {
        Self {
            tag: value.tag.into(),
            operative_descriptor: value.operative_descriptor.into(),
            bounds: value.bounds.get().into(),
        }
    }
}

impl From<OperativeVariants> for ROperativeVariants {
    fn from(value: OperativeVariants) -> Self {
        match value {
            OperativeVariants::TraitOperative(val) => {
                ROperativeVariants::TraitOperative(val.into())
            }
            OperativeVariants::LibraryOperative(val) => {
                ROperativeVariants::LibraryOperative(RwSignal::new(val))
            }
        }
    }
}
impl From<ROperativeVariants> for OperativeVariants {
    fn from(value: ROperativeVariants) -> Self {
        match value {
            ROperativeVariants::TraitOperative(val) => {
                OperativeVariants::TraitOperative(val.into())
            }
            ROperativeVariants::LibraryOperative(val) => {
                OperativeVariants::LibraryOperative(val.get())
            }
        }
    }
}

impl<TTypes: ConstraintTraits> From<FieldConstraint<TTypes>> for RFieldConstraint<TTypes> {
    fn from(value: FieldConstraint<TTypes>) -> Self {
        Self {
            tag: value.tag.into(),
            value_type: RwSignal::new(value.value_type),
        }
    }
}
impl<TTypes: ConstraintTraits> From<RFieldConstraint<TTypes>> for FieldConstraint<TTypes> {
    fn from(value: RFieldConstraint<TTypes>) -> Self {
        Self {
            tag: value.tag.into(),
            value_type: value.value_type.get(),
        }
    }
}

impl From<TraitOperative> for RTraitOperative {
    fn from(value: TraitOperative) -> Self {
        Self {
            trait_ids: RwSignal::new(value.trait_ids),
            tag: value.tag.into(),
        }
    }
}
impl From<RTraitOperative> for TraitOperative {
    fn from(value: RTraitOperative) -> Self {
        Self {
            trait_ids: value.trait_ids.get(),
            tag: value.tag.into(),
        }
    }
}

impl From<TraitMethodImplPath> for RTraitMethodImplPath {
    fn from(value: TraitMethodImplPath) -> Self {
        match value {
            TraitMethodImplPath::Field(val) => RTraitMethodImplPath::Field(RwSignal::new(val)),
            TraitMethodImplPath::TraitMethod {
                trait_id,
                trait_method_id,
            } => RTraitMethodImplPath::TraitMethod {
                trait_id: RwSignal::new(trait_id),
                trait_method_id: RwSignal::new(trait_method_id),
            },
            TraitMethodImplPath::Constituent(val) => {
                RTraitMethodImplPath::Constituent(RwSignal::new(val))
            }
        }
    }
}
impl From<RTraitMethodImplPath> for TraitMethodImplPath {
    fn from(value: RTraitMethodImplPath) -> Self {
        match value {
            RTraitMethodImplPath::Field(val) => TraitMethodImplPath::Field(val.get()),
            RTraitMethodImplPath::TraitMethod {
                trait_id,
                trait_method_id,
            } => TraitMethodImplPath::TraitMethod {
                trait_id: trait_id.get(),
                trait_method_id: trait_method_id.get(),
            },
            RTraitMethodImplPath::Constituent(val) => TraitMethodImplPath::Constituent(val.get()),
        }
    }
}

impl From<SlottedInstances> for RSlottedInstances {
    fn from(value: SlottedInstances) -> Self {
        Self {
            operative_slot_id: RwSignal::new(value.operative_slot_id),
            operative_id: RwSignal::new(value.operative_id),
            fulfilling_instance_ids: RwSignal::new(value.fulfilling_instance_ids),
        }
    }
}
impl From<RSlottedInstances> for SlottedInstances {
    fn from(value: RSlottedInstances) -> Self {
        Self {
            operative_slot_id: value.operative_slot_id.get(),
            operative_id: value.operative_id.get(),
            fulfilling_instance_ids: value.fulfilling_instance_ids.get(),
        }
    }
}

impl<TValues: ConstraintTraits> From<LockedFieldConstraint<TValues>>
    for RLockedFieldConstraint<TValues>
{
    fn from(value: LockedFieldConstraint<TValues>) -> Self {
        Self {
            field_constraint_name: RwSignal::new(value.field_constraint_name),
            field_constraint_id: value.field_constraint_id.into(),
            value: RwSignal::new(value.value),
        }
    }
}
impl<TValues: ConstraintTraits> From<RLockedFieldConstraint<TValues>>
    for LockedFieldConstraint<TValues>
{
    fn from(value: RLockedFieldConstraint<TValues>) -> Self {
        Self {
            field_constraint_name: value.field_constraint_name.get(),
            field_constraint_id: value.field_constraint_id.get(),
            value: value.value.get(),
        }
    }
}

impl<TTypes: ConstraintTraits, TValues: ConstraintTraits> From<LibraryOperative<TTypes, TValues>>
    for RLibraryOperative<TTypes, TValues>
{
    fn from(value: LibraryOperative<TTypes, TValues>) -> Self {
        Self {
            template_id: RwSignal::new(value.template_id),
            parent_operative_id: RwSignal::new(value.parent_operative_id),
            tag: value.tag.into(),
            slotted_instances: RwSignal::new(
                value
                    .slotted_instances
                    .iter()
                    .map(|(index, item)| (*index, item.clone().into()))
                    .collect(),
            ),
            locked_fields: RwSignal::new(
                value
                    .locked_fields
                    .iter()
                    .map(|(index, item)| (*index, item.clone().into()))
                    .collect(),
            ),
            trait_impls: RwSignal::new(
                value
                    .trait_impls
                    .iter()
                    .map(|(index, item)| {
                        (
                            *index,
                            RwSignal::new(
                                item.iter()
                                    .map(|(index_inner, inner)| {
                                        (
                                            *index_inner,
                                            RwSignal::new(
                                                inner
                                                    .clone()
                                                    .iter()
                                                    .cloned()
                                                    .map(|vec_item| vec_item.into())
                                                    .collect(),
                                            ),
                                        )
                                    })
                                    .collect(),
                            ),
                        )
                    })
                    .collect(),
            ),
            _phantom: value._phantom,
        }
    }
}
impl<TTypes: ConstraintTraits, TValues: ConstraintTraits> From<RLibraryOperative<TTypes, TValues>>
    for LibraryOperative<TTypes, TValues>
{
    fn from(value: RLibraryOperative<TTypes, TValues>) -> Self {
        Self {
            template_id: value.template_id.get(),
            parent_operative_id: value.parent_operative_id.get(),
            tag: value.tag.into(),
            slotted_instances: value
                .slotted_instances
                .get()
                .iter()
                .map(|(index, item)| (*index, item.clone().into()))
                .collect(),
            locked_fields: value
                .locked_fields
                .get()
                .iter()
                .map(|(index, item)| (*index, item.clone().into()))
                .collect(),
            trait_impls: value
                .trait_impls
                .get()
                .iter()
                .map(|(index, item)| {
                    (
                        *index,
                        item.get()
                            .iter()
                            .map(|(index_inner, inner)| {
                                (
                                    *index_inner,
                                    inner
                                        .get()
                                        .clone()
                                        .iter()
                                        .cloned()
                                        .map(|vec_item| vec_item.into())
                                        .collect(),
                                )
                            })
                            .collect(),
                    )
                })
                .collect(),
            _phantom: value._phantom,
        }
    }
}

impl<TTypes: ConstraintTraits> From<TraitDef<TTypes>> for RTraitDef<TTypes> {
    fn from(value: TraitDef<TTypes>) -> Self {
        Self {
            tag: value.tag.into(),
            methods: RwSignal::new(
                value
                    .methods
                    .iter()
                    .map(|(index, item)| (*index, item.clone().into()))
                    .collect(),
            ),
        }
    }
}
impl<TTypes: ConstraintTraits> From<RTraitDef<TTypes>> for TraitDef<TTypes> {
    fn from(value: RTraitDef<TTypes>) -> Self {
        Self {
            tag: value.tag.into(),
            methods: value
                .methods
                .get()
                .iter()
                .map(|(index, item)| (*index, item.clone().into()))
                .collect(),
        }
    }
}

impl<TTypes: ConstraintTraits> From<TraitMethodDef<TTypes>> for RTraitMethodDef<TTypes> {
    fn from(value: TraitMethodDef<TTypes>) -> Self {
        Self {
            tag: value.tag.into(),
            return_type: RwSignal::new(value.return_type),
        }
    }
}
impl<TTypes: ConstraintTraits> From<RTraitMethodDef<TTypes>> for TraitMethodDef<TTypes> {
    fn from(value: RTraitMethodDef<TTypes>) -> Self {
        Self {
            tag: value.tag.into(),
            return_type: value.return_type.get(),
        }
    }
}
