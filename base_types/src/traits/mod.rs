use anyhow::{Error, Result};

use std::{
    any::{Any, TypeId},
    collections::HashMap,
    marker::PhantomData,
    rc::Rc,
};
use strum_macros::Display;

use validator::Validate;

use crate::{
    common::{ConstraintTraits, Tag, Uid},
    constraint_schema::{
        ConstraintSchema, LibraryOperative, LibraryTemplate, OperativeSlot, SlotBounds,
    },
    constraint_schema_item::ConstraintSchemaItem,
    primitives::{PrimitiveTypes, PrimitiveValues},
};
mod tests;

type LibOp = LibraryOperative<PrimitiveTypes, PrimitiveValues>;
type LibTemplate = LibraryTemplate<PrimitiveTypes, PrimitiveValues>;

#[derive(Debug)]
pub struct BaseGraphEnvironment<TSchema: GSO> {
    pub created_instances: HashMap<Uid, TSchema>,
    pub constraint_schema: ConstraintSchema<PrimitiveTypes, PrimitiveValues>,
}
// impl<TSchema: GSO> BaseGraphEnvironment {
//     fn new() -> Self {
//         Self {
//             created_instances: HashMap::new(),
//         }
//     }
// }

impl<TSchema: GSO> GraphEnvironment for BaseGraphEnvironment<TSchema> {
    type Schema = TSchema;
    type Types = PrimitiveTypes;
    type Values = PrimitiveValues;

    fn get_constraint_schema(&self) -> &ConstraintSchema<Self::Types, Self::Values> {
        &self.constraint_schema
    }

    fn get_element(&self, id: &Uid) -> Option<&Self::Schema> {
        self.created_instances.get(id)
    }
    fn instantiate_element(&mut self, element: Self::Schema) -> Uid {
        let id = *element.get_id();
        self.created_instances.insert(id, element);
        id
    }
}

pub trait GraphEnvironment {
    type Types: ConstraintTraits;
    type Values: ConstraintTraits;
    type Schema: GSO;

    fn get_element(&self, id: &Uid) -> Option<&Self::Schema>;
    fn instantiate_element(&mut self, element: Self::Schema) -> Uid;
    fn get_constraint_schema(&self) -> &ConstraintSchema<Self::Types, Self::Values>;
}

pub trait GSO: std::fmt::Debug {
    /// Instance ID
    fn get_id(&self) -> &Uid;
    // fn get_constraint_schema_operative_tag(&self) -> Rc<LibOp>;
    // fn get_constraint_schema_template_tag(&self) -> Rc<LibTemplate>;
    fn get_constraint_schema_operative_tag(&self) -> Rc<Tag>;
    fn get_constraint_schema_template_tag(&self) -> Rc<Tag>;
    fn get_slot_by_id(&self, slot_id: &Uid) -> Option<&ActiveSlot> {
        self.get_slots().get(slot_id)
    }
    fn get_slots(&self) -> &HashMap<Uid, ActiveSlot>;
    fn get_parent_slots(&self) -> &Vec<ParentSlotRef>;
}

#[derive(Clone, Debug)]
pub struct ParentSlotRef {
    pub host_instance_id: Uid,
    pub slot_id: Uid,
}
#[derive(Clone, Debug)]
pub struct ChildSlotRef {
    pub child_instance_id: Uid,
    pub slot_id: Uid,
}

pub trait Slotted {}

#[derive(Clone, Debug)]
pub struct ActiveSlot {
    pub slot: OperativeSlot,
    pub slotted_instances: Vec<Uid>,
}
#[cfg(feature = "to_tokens")]
impl quote::ToTokens for ActiveSlot {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let slotted_instances = self.slotted_instances.clone();
        let slot = self.slot.clone();
        tokens.extend(quote::quote! {
            base_types::traits::ActiveSlot {
                slotted_instances: vec![#(#slotted_instances,)*],
                slot: #slot,
            }

        })
    }
}
impl ActiveSlot {
    fn check_bound_conformity(&self) -> bool {
        let len = self.slotted_instances.len();
        match &self.slot.bounds {
            SlotBounds::Single => len == 1,
            SlotBounds::LowerBound(lower_bound) => lower_bound <= &len,
            SlotBounds::UpperBound(upper_bound) => upper_bound >= &len,
            SlotBounds::Range(lower_range, upper_range) => {
                lower_range <= &len && &len <= upper_range
            }
            SlotBounds::LowerBoundOrZero(lower_bound) => len == 0 || lower_bound <= &len,
            SlotBounds::RangeOrZero(lower_range, upper_range) => {
                len == 0 || (lower_range <= &len && &len <= upper_range)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct GSOWrapper<T> {
    id: Uid,
    slots: HashMap<Uid, ActiveSlot>,
    parent_slots: Vec<ParentSlotRef>,
    pub data: T,
    operative_tag: Rc<Tag>,
    template_tag: Rc<Tag>,
    // operative: Rc<LibOp>,
    // template: Rc<LibTemplate>,
}
impl<T: Clone + std::fmt::Debug> GSOWrapper<T> {}

impl<T: Clone + std::fmt::Debug> GSO for GSOWrapper<T> {
    fn get_id(&self) -> &Uid {
        &self.id
    }

    // fn get_constraint_schema_operative_tag(&self) -> Rc<LibOp> {
    //     self.operative
    // }

    // fn get_constraint_schema_template_tag(&self) -> Rc<LibTemplate> {
    //     self.template
    // }

    fn get_slots(&self) -> &HashMap<Uid, ActiveSlot> {
        &self.slots
    }

    fn get_parent_slots(&self) -> &Vec<ParentSlotRef> {
        &self.parent_slots
    }

    fn get_constraint_schema_operative_tag(&self) -> Rc<Tag> {
        self.operative_tag.clone()
    }

    fn get_constraint_schema_template_tag(&self) -> Rc<Tag> {
        self.template_tag.clone()
    }
}
// pub trait InitializeWrapperBuilder<T> {
//     fn initialize(
//         data: T,
//         slots: Option<HashMap<Uid, ActiveSlot>>,
//         operative_tag: Rc<Tag>,
//         template_tag: Rc<Tag>,
//     ) -> Self;
// }
#[derive(Clone, Debug)]
pub struct GSOWrapperBuilder<T> {
    id: Uid,
    slots: HashMap<Uid, ActiveSlot>,
    parent_slots: Vec<ParentSlotRef>,
    pub data: T,
    operative_tag: Rc<Tag>,
    template_tag: Rc<Tag>,
    // operative: Rc<LibOp>,
    // template: Rc<LibTemplate>,
}
// impl<T: Clone + std::fmt::Debug> InitializeWrapperBuilder<T> for GSOWrapperBuilder<T> {
//     fn initialize(
//         data: T,
//         slots: Option<HashMap<Uid, ActiveSlot>>,
//         operative_tag: Rc<Tag>,
//         template_tag: Rc<Tag>,
//     ) -> Self {
//         Self {
//             id: uuid::Uuid::new_v4().as_u128(),
//             slots: slots.unwrap_or(HashMap::new()),
//             parent_slots: Vec::new(),
//             data,
//             operative_tag,
//             template_tag,
//         }
//     }
// }

impl<T: Clone + std::fmt::Debug> GSOWrapperBuilder<T> {
    pub fn new(
        data: T,
        slots: Option<HashMap<Uid, ActiveSlot>>,
        operative_tag: Rc<Tag>,
        template_tag: Rc<Tag>,
        // , operative: Rc<LibOp>, template: Rc<LibTemplate>
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().as_u128(),
            slots: slots.unwrap_or(HashMap::new()),
            parent_slots: Vec::new(),
            data,
            operative_tag,
            template_tag, // operative,
                          // template,
        }
    }
    fn replace_slots(&mut self, new_slots: HashMap<Uid, ActiveSlot>) -> &mut Self {
        self.slots = new_slots;
        self
    }
    fn add_instance_to_slot(&mut self, slot_id: &Uid, instance_id: Uid) -> &mut Self {
        self.slots
            .get_mut(slot_id)
            .unwrap()
            .slotted_instances
            .push(instance_id);
        self
    }
    fn add_instance_to_parent_slot(&mut self, slot_ref: ParentSlotRef) -> &mut Self {
        self.parent_slots.push(slot_ref);
        self
    }
}
impl<F, T> Producable<GSOWrapper<T>> for GSOWrapperBuilder<F>
where
    F: Producable<T>,
{
    fn produce(&self) -> GSOWrapper<T> {
        GSOWrapper::<T> {
            id: self.id.clone(),
            slots: self.slots.clone(),
            parent_slots: self.parent_slots.clone(),
            data: self.data.produce(),
            operative_tag: self.operative_tag.clone(),
            template_tag: self.template_tag.clone(),
            // operative: self.operative.clone(),
            // template: self.template.clone(),
        }
    }
}
#[derive(Debug, Display)]
enum ElementCreationError {
    BoundCheckOutOfRange,
    ChildElementIsWrongType,
    ChildElementDoesntExist,
}
impl std::error::Error for ElementCreationError {}
impl<F> Verifiable for GSOWrapperBuilder<F>
where
    F: Verifiable,
{
    fn verify(&self) -> Result<(), Error> {
        self.data.verify()?;
        let slot_errors = self
            .slots
            .values()
            .filter_map(|active_slot| {
                if !active_slot.check_bound_conformity() {
                    Some(Error::new(ElementCreationError::BoundCheckOutOfRange))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        if slot_errors.is_empty() {
            return Ok(());
        }
        // TODO make this return all of the errors
        Err(Error::new(ElementCreationError::BoundCheckOutOfRange))
    }
}
// impl<F, T, G: GraphEnvironment> Finalizable<GSOWrapper<T>, G> for GSOWrapperBuilder<F> where
//     F: Finalizable<T, G>
// {
// }
impl<F, T> Finalizable<T> for F
where
    F: Verifiable + Producable<T>,
    T: Instantiable,
{
}

pub trait Buildable
where
    Self: Sized + 'static,
    GSOWrapper<Self>: Instantiable,
{
    type Builder: Finalizable<GSOWrapper<Self>>;

    fn initiate_build() -> GSOBuilder<Self::Builder, GSOWrapper<Self>>;
    // {
    //     GSOBuilder::<Self::Builder, GSOWrapper<Self>>::new()
    // }
    fn get_operative_id() -> Uid;
}

pub trait Verifiable {
    fn verify(&self) -> Result<(), Error>;
}
pub trait Instantiable: GSO {
    // type Graph: GraphEnvironment;

    fn instantiate(&self) -> Result<(), Error>;
    fn get_id(&self) -> &Uid;
}
type InstantiableElements = Vec<Rc<dyn Instantiable>>;

#[derive(Debug, Clone)]
pub struct InstantiableWrapper<T>
where
    T: Instantiable,
{
    prereq_instantiables: InstantiableElements,
    instantiable_instance: T,
}

impl<T> InstantiableWrapper<T>
where
    T: Instantiable + 'static,
{
    pub fn flatten(mut self) -> InstantiableElements {
        self.prereq_instantiables
            .push(Rc::new(self.instantiable_instance));
        self.prereq_instantiables
    }
    pub fn get_prereq_instantiables(&self) -> &InstantiableElements {
        &self.prereq_instantiables
    }
    pub fn get_instantiable_instance(&self) -> &T {
        &self.instantiable_instance
    }
}
impl<T> InstantiableWrapper<GSOWrapper<T>>
where
    GSOWrapper<T>: Instantiable,
{
    pub fn add_parent_slot(&mut self, parent_slot: ParentSlotRef) {
        self.instantiable_instance.parent_slots.push(parent_slot);
    }
}

pub trait Producable<T> {
    fn produce(&self) -> T;
}

pub trait Finalizable<T>: Verifiable + Producable<T> {
    fn finalize(&self) -> Result<T, Error> {
        self.verify()?;
        Ok(self.produce())
    }
}

#[derive(Default, Debug)]
pub struct GSOBuilder<F, T>
where
    F: Finalizable<T>,
    T: Instantiable,
{
    instantiables: Vec<Rc<dyn Instantiable>>,
    child_updates: Vec<(Uid, ParentSlotRef)>,
    parent_updates: Vec<(Uid, ChildSlotRef)>,
    pub wip_instance: F,
    _phantom: PhantomData<T>,
}

impl<F, T> GSOBuilder<F, T>
where
    F: Finalizable<T>,
    T: Instantiable + 'static,
{
    pub fn build(&mut self) -> Result<InstantiableWrapper<T>, Error> {
        Ok(InstantiableWrapper {
            instantiable_instance: self.wip_instance.finalize()?,
            prereq_instantiables: self.instantiables.clone(),
        })
    }
    pub fn new(builder_wrapper_instance: F) -> Self {
        Self {
            instantiables: vec![],
            wip_instance: builder_wrapper_instance,
            child_updates: Vec::new(),
            parent_updates: Vec::new(),
            _phantom: PhantomData,
        }
    }
}

// pub trait Integrable<C> {
//     fn get_slot_id() -> Uid;
//     // fn integrate(&mut self, child_id: &Uid) -> ParentSlotRef;
// }

// impl<F, T> Integrable<T> for GSOWrapperBuilder<F> {
//     fn integrate(&mut self, child: &T) -> &mut Self {}
// }
pub fn integrate_child<F, T, C>(
    builder: &mut GSOBuilder<GSOWrapperBuilder<F>, GSOWrapper<T>>,
    mut child: InstantiableWrapper<GSOWrapper<C>>,
    slot_id: Uid,
) -> &mut GSOBuilder<GSOWrapperBuilder<F>, GSOWrapper<T>>
where
    F: Verifiable + Producable<T> + Clone + std::fmt::Debug,
    T: Clone + std::fmt::Debug,
    GSOWrapper<C>: Instantiable + 'static,
{
    // let slot_id = <F as Integrable<C>>::get_slot_id().clone();
    builder
        .wip_instance
        .add_instance_to_slot(&slot_id, child.get_instantiable_instance().id);
    let slot_ref = ParentSlotRef {
        slot_id,
        host_instance_id: builder.wip_instance.id,
    };
    child.add_parent_slot(slot_ref);
    builder.instantiables.extend(child.flatten());
    builder
}

pub fn integrate_child_id<'a, F, T>(
    builder: &'a mut GSOBuilder<GSOWrapperBuilder<F>, GSOWrapper<T>>,
    mut child_id: &Uid,
    slot_id: Uid,
) -> &'a mut GSOBuilder<GSOWrapperBuilder<F>, GSOWrapper<T>>
where
    F: Verifiable + Producable<T> + Clone + std::fmt::Debug,
    T: Clone + std::fmt::Debug,
{
    // let slot_id = <F as Integrable<C>>::get_slot_id().clone();
    builder
        .wip_instance
        .add_instance_to_slot(&slot_id, *child_id);
    let slot_ref = ParentSlotRef {
        slot_id,
        host_instance_id: builder.wip_instance.id,
    };
    // child.add_parent_slot(slot_ref);
    builder.child_updates.push((*child_id, slot_ref));
    builder
}
// pub fn integrate_child_id<'a, F, T, C>(
//     builder: &'a mut GSOBuilder<F, T>,
//     child_id: &Uid,
// ) -> &'a mut GSOBuilder<F, T>
// where
//     F: Integrable<GSOWrapper<C>> + Finalizable<T>,
//     T: Instantiable,
//     GSOWrapper<C>: Instantiable + 'static,
// {
//     let slot_ref = builder.wip_instance.integrate(child_id);
//     builder.child_updates.push((*child_id, slot_ref));
//     builder
// }

impl<T: Clone + std::fmt::Debug> Instantiable for GSOWrapper<T> {
    // type Graph = G;

    fn instantiate(&self) -> Result<(), Error> {
        todo!()
    }

    fn get_id(&self) -> &Uid {
        todo!()
    }
}
