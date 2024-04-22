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

#[cfg(feature = "reactive")]
mod reactive;

mod tests;

type LibOp = LibraryOperative<PrimitiveTypes, PrimitiveValues>;
type LibTemplate = LibraryTemplate<PrimitiveTypes, PrimitiveValues>;

#[derive(Debug, Display)]
enum ElementCreationError {
    BoundCheckOutOfRange,
    ChildElementIsWrongType,
    ChildElementDoesntExist,
}
impl std::error::Error for ElementCreationError {}

#[derive(Debug, Display)]
enum ElementDeletionError {
    RequiredByParentSlot,
}
impl std::error::Error for ElementDeletionError {}

#[derive(Debug)]
pub struct BaseGraphEnvironment<TSchema: GSO> {
    pub created_instances: HashMap<Uid, TSchema>,
    pub constraint_schema: ConstraintSchema<PrimitiveTypes, PrimitiveValues>,
}
impl<TSchema: GSO> BaseGraphEnvironment<TSchema> {
    pub fn new(constraint_schema: ConstraintSchema<PrimitiveTypes, PrimitiveValues>) -> Self {
        Self {
            created_instances: HashMap::new(),
            constraint_schema,
        }
    }
    pub fn new_without_schema() -> Self {
        Self {
            created_instances: HashMap::new(),
            constraint_schema: ConstraintSchema {
                template_library: HashMap::new(),
                instance_library: HashMap::new(),
                operative_library: HashMap::new(),
                traits: HashMap::new(),
            },
        }
    }
}

impl<TSchema: GSO + 'static> BaseGraphEnvironment<TSchema> {
    fn check_and_delete_children(&mut self, id: &Uid, parent_id: Option<&Uid>) {
        let mut should_delete = if parent_id.is_some() { false } else { true };
        let item = self.get_mut(&id).unwrap();

        if let Some(parent_id) = parent_id {
            let child_parent_slots = item.get_parent_slots();
            let remaining_parents = child_parent_slots
                .iter()
                .filter(|slot_ref| slot_ref.host_instance_id != *parent_id)
                .collect::<Vec<_>>();
            if remaining_parents.is_empty() {
                should_delete = true;
            }
        }

        if !should_delete && parent_id.is_some() {
            item.remove_from_parent_slot(&parent_id.unwrap(), None);
        }

        if should_delete {
            item.get_slots().clone().values().for_each(|slot| {
                slot.slotted_instances
                    .iter()
                    .for_each(|slotted_instance_id| {
                        self.check_and_delete_children(slotted_instance_id, Some(id));
                    });
            });
        }
        self.created_instances.remove(&id);
    }
}
impl<TSchema: GSO + 'static> GraphEnvironment for BaseGraphEnvironment<TSchema> {
    type Schema = TSchema;
    type Types = PrimitiveTypes;
    type Values = PrimitiveValues;

    fn get_constraint_schema(&self) -> &ConstraintSchema<Self::Types, Self::Values> {
        &self.constraint_schema
    }

    fn get(&self, id: &Uid) -> Option<&Self::Schema> {
        self.created_instances.get(id)
    }
    fn get_mut(&mut self, id: &Uid) -> Option<&mut Self::Schema> {
        self.created_instances.get_mut(id)
    }
    fn instantiate_element<T: std::fmt::Debug + Clone + 'static>(
        &mut self,
        element: InstantiableWrapper<GSOWrapper<T>, Self::Schema>,
    ) -> Uid
    where
        Self: Sized,
        GSOWrapper<T>: Instantiable<Schema = Self::Schema>,
    {
        let id = *element.get_instantiable_instance().get_id();
        // self.created_instances.insert(id, element);
        element.child_updates.iter().for_each(|child_update| {
            let child = self.created_instances.get_mut(&child_update.0).unwrap();
            child.add_parent_slot(child_update.1.clone());
        });
        element.flatten().into_iter().for_each(|instantiable| {
            let instantiated = instantiable.instantiate();
            self.created_instances
                .insert(*instantiable.get_instance_id(), instantiated);
        });
        id
    }

    fn delete(&mut self, id: &Uid) -> Result<(), Error> {
        let parent_slots = self.get(id).unwrap().get_parent_slots().clone();

        let can_delete = parent_slots.iter().all(|parent_slot| {
            self.get(&parent_slot.host_instance_id)
                .unwrap()
                .get_slot_by_id(&parent_slot.slot_id)
                .unwrap()
                .can_remove_one()
        });
        if !can_delete {
            return Err(Error::new(ElementDeletionError::RequiredByParentSlot));
        }
        parent_slots.iter().for_each(|parent_slot| {
            self.get_mut(&parent_slot.host_instance_id)
                .unwrap()
                .remove_from_child_slot(parent_slot);
        });

        self.check_and_delete_children(id, None);

        Ok(())
    }
    // fn instantiate_elements()
}

pub trait GraphEnvironment {
    type Types: ConstraintTraits;
    type Values: ConstraintTraits;
    type Schema: GSO + 'static;

    fn get(&self, id: &Uid) -> Option<&Self::Schema>;
    fn instantiate_element<T: std::fmt::Debug + Clone + 'static>(
        &mut self,
        element: InstantiableWrapper<GSOWrapper<T>, Self::Schema>,
    ) -> Uid
    where
        GSOWrapper<T>: Instantiable<Schema = Self::Schema>,
        Self: Sized;
    fn get_mut(&mut self, id: &Uid) -> Option<&mut Self::Schema>;
    fn get_constraint_schema(&self) -> &ConstraintSchema<Self::Types, Self::Values>;
    fn delete(&mut self, id: &Uid) -> Result<(), Error>;
}

pub trait GSO: std::fmt::Debug + Clone {
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
    fn get_parent_slots(&self) -> &Vec<SlotRef>;
    fn add_parent_slot(&mut self, slot_ref: SlotRef) -> &mut Self;
    fn remove_from_child_slot(&mut self, slot_ref: &SlotRef) -> &mut Self;
    fn remove_from_parent_slot(&mut self, parent_id: &Uid, slot_id: Option<&Uid>) -> &mut Self;
}

#[derive(Clone, Debug)]
pub struct SlotRef {
    pub host_instance_id: Uid,
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
    pub fn check_current_conformity(&self) -> bool {
        let len = self.slotted_instances.len();
        self.check_bound_conformity(len)
    }
    pub fn can_remove_one(&self) -> bool {
        let len = self.slotted_instances.len() - 1;
        self.check_bound_conformity(len)
    }
    fn check_bound_conformity(&self, len: usize) -> bool {
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
    parent_slots: Vec<SlotRef>,
    pub data: T,
    operative_tag: Rc<Tag>,
    template_tag: Rc<Tag>,
    // _phantom: PhantomData<TSchema>, // operative: Rc<LibOp>,
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

    fn get_parent_slots(&self) -> &Vec<SlotRef> {
        &self.parent_slots
    }

    fn get_constraint_schema_operative_tag(&self) -> Rc<Tag> {
        self.operative_tag.clone()
    }

    fn get_constraint_schema_template_tag(&self) -> Rc<Tag> {
        self.template_tag.clone()
    }

    fn add_parent_slot(&mut self, slot_ref: SlotRef) -> &mut Self {
        self.parent_slots.push(slot_ref);
        self
    }

    fn remove_from_child_slot(&mut self, slot_ref: &SlotRef) -> &mut Self {
        self.slots
            .get_mut(&slot_ref.slot_id)
            .unwrap()
            .slotted_instances
            .retain(|slotted_instance_id| *slotted_instance_id != slot_ref.child_instance_id);
        self
    }

    fn remove_from_parent_slot(&mut self, parent_id: &Uid, slot_id: Option<&Uid>) -> &mut Self {
        self.parent_slots.retain(|slot_ref| {
            if slot_ref.host_instance_id != *parent_id {
                return true;
            }
            if let Some(slot_id) = slot_id {
                return *slot_id != slot_ref.slot_id;
            }
            false
        });
        self
    }
}
#[derive(Clone, Debug)]
pub struct GSOWrapperBuilder<T> {
    id: Uid,
    slots: HashMap<Uid, ActiveSlot>,
    parent_slots: Vec<SlotRef>,
    pub data: T,
    operative_tag: Rc<Tag>,
    template_tag: Rc<Tag>,
    // operative: Rc<LibOp>,
    // template: Rc<LibTemplate>,
}

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
    fn add_instance_to_parent_slot(&mut self, slot_ref: SlotRef) -> &mut Self {
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
                if !active_slot.check_current_conformity() {
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

impl<F, T> Finalizable<GSOWrapper<T>> for GSOWrapperBuilder<F> where F: Verifiable + Producable<T> {}

pub trait Buildable
where
    Self: Sized + 'static,
    GSOWrapper<Self>: Instantiable<Schema = Self::Schema>,
{
    type Builder: Finalizable<GSOWrapper<Self>>;
    type Schema;

    fn initiate_build() -> GSOBuilder<Self::Builder, GSOWrapper<Self>, Self::Schema>;
    fn get_operative_id() -> Uid;
}

pub trait Verifiable {
    fn verify(&self) -> Result<(), Error>;
}
pub trait Instantiable: std::fmt::Debug + Any {
    // type Graph: GraphEnvironment;
    type Schema;

    fn instantiate(&self) -> Self::Schema;
    fn get_instance_id(&self) -> &Uid;
}
type InstantiableElements<TSchema> = Vec<Rc<dyn Instantiable<Schema = TSchema>>>;

#[derive(Debug, Clone)]
pub struct InstantiableWrapper<T, TSchema>
where
    T: Instantiable<Schema = TSchema>,
{
    prereq_instantiables: InstantiableElements<TSchema>,
    instantiable_instance: T,
    parent_updates: Vec<(Uid, SlotRef)>,
    child_updates: Vec<(Uid, SlotRef)>,
}

impl<T, TSchema> InstantiableWrapper<T, TSchema>
where
    T: Instantiable<Schema = TSchema> + 'static,
{
    pub fn flatten(mut self) -> InstantiableElements<TSchema> {
        self.prereq_instantiables
            .push(Rc::new(self.instantiable_instance));
        self.prereq_instantiables
    }
    pub fn get_prereq_instantiables(&self) -> &InstantiableElements<TSchema> {
        &self.prereq_instantiables
    }
    pub fn get_instantiable_instance(&self) -> &T {
        &self.instantiable_instance
    }
}
impl<T, TSchema> InstantiableWrapper<GSOWrapper<T>, TSchema>
where
    GSOWrapper<T>: Instantiable<Schema = TSchema>,
{
    pub fn add_parent_slot(&mut self, parent_slot: SlotRef) {
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
pub struct GSOBuilder<F, T, TSchema>
where
    F: Finalizable<T>,
{
    instantiables: Vec<Rc<dyn Instantiable<Schema = TSchema>>>,
    child_updates: Vec<(Uid, SlotRef)>,
    parent_updates: Vec<(Uid, SlotRef)>,
    pub wip_instance: F,
    _phantom: PhantomData<T>,
}

impl<F, T, TSchema> GSOBuilder<F, T, TSchema>
where
    F: Finalizable<T>,
    T: Instantiable<Schema = TSchema> + 'static,
{
    pub fn build(&mut self) -> Result<InstantiableWrapper<T, TSchema>, Error> {
        Ok(InstantiableWrapper {
            child_updates: self.child_updates.clone(),
            parent_updates: self.parent_updates.clone(),
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

pub fn integrate_child<F, T, C, TSchema>(
    builder: &mut GSOBuilder<GSOWrapperBuilder<F>, GSOWrapper<T>, TSchema>,
    mut child: InstantiableWrapper<GSOWrapper<C>, TSchema>,
    slot_id: Uid,
) -> &mut GSOBuilder<GSOWrapperBuilder<F>, GSOWrapper<T>, TSchema>
where
    F: Verifiable + Producable<T> + Clone + std::fmt::Debug,
    T: Clone + std::fmt::Debug,
    GSOWrapper<C>: Instantiable<Schema = TSchema> + 'static,
{
    // let slot_id = <F as Integrable<C>>::get_slot_id().clone();
    builder
        .wip_instance
        .add_instance_to_slot(&slot_id, child.get_instantiable_instance().id);
    let slot_ref = SlotRef {
        slot_id,
        child_instance_id: *child.get_instantiable_instance().get_instance_id(),
        host_instance_id: builder.wip_instance.id,
    };
    child.add_parent_slot(slot_ref);
    builder.instantiables.extend(child.flatten());
    builder
}

pub fn integrate_child_id<'a, F, T, TSchema>(
    builder: &'a mut GSOBuilder<GSOWrapperBuilder<F>, GSOWrapper<T>, TSchema>,
    child_id: &Uid,
    slot_id: Uid,
) -> &'a mut GSOBuilder<GSOWrapperBuilder<F>, GSOWrapper<T>, TSchema>
where
    F: Verifiable + Producable<T> + Clone + std::fmt::Debug,
    T: Clone + std::fmt::Debug,
{
    builder
        .wip_instance
        .add_instance_to_slot(&slot_id, *child_id);
    let slot_ref = SlotRef {
        slot_id,
        child_instance_id: *child_id,
        host_instance_id: builder.wip_instance.id,
    };
    // child.add_parent_slot(slot_ref);
    builder.child_updates.push((*child_id, slot_ref));
    builder
}

impl<T, TSchema> Instantiable for GSOWrapper<T>
where
    T: Clone + std::fmt::Debug + IntoSchema<Schema = TSchema> + 'static,
{
    type Schema = TSchema;

    fn instantiate(&self) -> Self::Schema {
        T::into_schema(self.clone())
    }

    fn get_instance_id(&self) -> &Uid {
        self.get_id()
    }
}

pub trait IntoSchema
where
    Self: Sized,
{
    type Schema;
    fn into_schema(instantiable: GSOWrapper<Self>) -> Self::Schema;
}
