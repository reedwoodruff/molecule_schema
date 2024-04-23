use anyhow::{Error, Result};

use std::{
    any::{Any, TypeId},
    cell::RefCell,
    collections::HashMap,
    io::Write,
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

pub type HistoryStack<TSchema> = Rc<RefCell<Vec<Vec<HistoryItem<TSchema>>>>>;
#[derive(Debug)]
pub struct BaseGraphEnvironment<TSchema: GSO> {
    pub created_instances: HashMap<Uid, TSchema>,
    pub constraint_schema: ConstraintSchema<PrimitiveTypes, PrimitiveValues>,
    pub undo_stack: HistoryStack<TSchema>,
}
impl<TSchema: GSO> BaseGraphEnvironment<TSchema> {
    pub fn new(constraint_schema: ConstraintSchema<PrimitiveTypes, PrimitiveValues>) -> Self {
        Self {
            created_instances: HashMap::new(),
            constraint_schema,
            undo_stack: Rc::new(RefCell::new(Vec::new())),
        }
    }
    pub fn new_without_schema() -> Self {
        Self {
            undo_stack: Rc::new(RefCell::new(Vec::new())),
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
        // let item_slots = self.get(&id).unwrap();
        // let mut item_mut = self.get_mut(&id).unwrap();

        if let Some(parent_id) = parent_id {
            let child_parent_slots = self.get(&id).unwrap().get_parent_slots();
            let remaining_parents = child_parent_slots
                .iter()
                .filter(|slot_ref| slot_ref.host_instance_id != *parent_id)
                .collect::<Vec<_>>();
            if remaining_parents.is_empty() {
                should_delete = true;
            }
        }

        if !should_delete && parent_id.is_some() {
            self.get_mut(&id)
                .unwrap()
                .remove_parent(&parent_id.unwrap(), None);
        }

        if should_delete {
            self.get(&id)
                .unwrap()
                .get_slots()
                .clone()
                .values()
                .for_each(|slot| {
                    slot.slotted_instances
                        .iter()
                        .for_each(|slotted_instance_id| {
                            self.check_and_delete_children(slotted_instance_id, Some(id));
                        });
                });
        }
        let mut removed_value = self.created_instances.remove(&id).unwrap();
        removed_value.set_history(None);
        self.undo_stack
            .borrow_mut()
            .last_mut()
            .unwrap()
            .push(HistoryItem::Delete(removed_value));
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
    fn instantiate_element<T: std::fmt::Debug + Clone + 'static>(
        &mut self,
        element: InstantiableWrapper<GSOWrapper<T, Self::Schema>, Self::Schema>,
    ) -> Uid
    where
        Self: Sized,
        GSOWrapper<T, Self::Schema>: Instantiable<Schema = Self::Schema>,
    {
        let id = *element.get_instantiable_instance().get_id();
        self.undo_stack
            .borrow_mut()
            .push(vec![HistoryItem::BlockActionMarker]);
        element.child_updates.iter().for_each(|child_update| {
            let mut child = self.get_mut(&child_update.0).unwrap();
            child.add_parent_slot(&child_update.1.clone());
        });
        element.flatten().into_iter().for_each(|mut instantiable| {
            let instantiated = instantiable.instantiate(self.undo_stack.clone());
            self.undo_stack
                .borrow_mut()
                .last_mut()
                .unwrap()
                .push(HistoryItem::Create(*instantiable.get_instance_id()));
            self.created_instances
                .insert(*instantiable.get_instance_id(), instantiated);
        });
        id
    }

    fn delete(&mut self, id: &Uid) -> Result<(), Error> {
        println!("entering delete");
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
        self.undo_stack
            .borrow_mut()
            .push(vec![HistoryItem::BlockActionMarker]);
        parent_slots.iter().for_each(|parent_slot| {
            self.get_mut(&parent_slot.host_instance_id)
                .unwrap()
                .remove_child_from_slot(parent_slot);
        });

        self.check_and_delete_children(id, None);

        Ok(())
    }

    fn get_mut(&mut self, id: &Uid) -> Option<&mut Self::Schema> {
        self.created_instances.get_mut(id)
    }
}

pub trait GraphEnvironment {
    type Types: ConstraintTraits;
    type Values: ConstraintTraits;
    type Schema: GSO + 'static;

    fn get(&self, id: &Uid) -> Option<&Self::Schema>;
    fn instantiate_element<T: std::fmt::Debug + Clone + 'static>(
        &mut self,
        element: InstantiableWrapper<GSOWrapper<T, Self::Schema>, Self::Schema>,
    ) -> Uid
    where
        GSOWrapper<T, Self::Schema>: Instantiable<Schema = Self::Schema>,
        Self: Sized;
    fn get_mut(&mut self, id: &Uid) -> Option<&mut Self::Schema>;
    fn get_constraint_schema(&self) -> &ConstraintSchema<Self::Types, Self::Values>;
    fn delete(&mut self, id: &Uid) -> Result<(), Error>;
}

#[derive(Debug, Clone)]
pub enum HistoryItem<TSchema: GSO> {
    RemoveChildFromSlot(Vec<SlotRef>),
    RemoveParent(Vec<SlotRef>),
    // AddChildToSlot(),
    AddParent(SlotRef),
    Delete(TSchema),
    Create(Uid),
    BlockActionMarker,
}

pub trait GSO: std::fmt::Debug + Clone {
    type Schema: GSO;
    /// Instance ID
    fn get_id(&self) -> &Uid;
    fn get_constraint_schema_operative_tag(&self) -> Rc<Tag>;
    fn get_constraint_schema_template_tag(&self) -> Rc<Tag>;
    fn get_slot_by_id(&self, slot_id: &Uid) -> Option<&ActiveSlot> {
        self.get_slots().get(slot_id)
    }
    fn get_slots(&self) -> &HashMap<Uid, ActiveSlot>;
    fn get_parent_slots(&self) -> &Vec<SlotRef>;
    fn add_parent_slot(&mut self, slot_ref: &SlotRef) -> &mut Self;
    fn remove_child_from_slot(&mut self, slot_ref: &SlotRef) -> &mut Self;
    fn remove_parent(&mut self, parent_id: &Uid, slot_id: Option<&Uid>) -> &mut Self;
    fn set_history(&mut self, history: Option<HistoryStack<Self::Schema>>);
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
pub struct GSOWrapper<T, TSchema: GSO> {
    id: Uid,
    slots: HashMap<Uid, ActiveSlot>,
    parent_slots: Vec<SlotRef>,
    pub data: T,
    operative_tag: Rc<Tag>,
    template_tag: Rc<Tag>,
    history_handle: Option<HistoryStack<TSchema>>,
}
impl<T: Clone + std::fmt::Debug, TSchema: GSO> GSOWrapper<T, TSchema> {}

impl<T: Clone + std::fmt::Debug, TSchema: GSO> GSO for GSOWrapper<T, TSchema> {
    type Schema = TSchema;
    fn get_id(&self) -> &Uid {
        &self.id
    }

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

    fn add_parent_slot(&mut self, slot_ref: &SlotRef) -> &mut Self {
        self.parent_slots.push(slot_ref.clone());
        self.history_handle
            .as_mut()
            .unwrap()
            .borrow_mut()
            .last_mut()
            .unwrap()
            .push(HistoryItem::AddParent(slot_ref.clone()));
        self
    }

    fn remove_child_from_slot(&mut self, slot_ref: &SlotRef) -> &mut Self {
        self.slots
            .get_mut(&slot_ref.slot_id)
            .unwrap()
            .slotted_instances
            .retain(|slotted_instance_id| *slotted_instance_id != slot_ref.child_instance_id);
        self.history_handle
            .as_mut()
            .unwrap()
            .borrow_mut()
            .last_mut()
            .unwrap()
            .push(HistoryItem::RemoveChildFromSlot(vec![slot_ref.clone()]));
        self
    }

    fn remove_parent(&mut self, parent_id: &Uid, slot_id: Option<&Uid>) -> &mut Self {
        let mut removed = Vec::new();
        self.parent_slots.retain(|slot_ref| {
            if slot_ref.host_instance_id != *parent_id {
                removed.push(slot_ref.clone());
                return true;
            }
            if let Some(slot_id) = slot_id {
                if *slot_id != slot_ref.slot_id {
                    return false;
                } else {
                    removed.push(slot_ref.clone());
                    return true;
                };
            }
            false
        });
        self.history_handle
            .as_mut()
            .unwrap()
            .borrow_mut()
            .last_mut()
            .unwrap()
            .push(HistoryItem::RemoveParent(removed));
        self
    }

    fn set_history(&mut self, history: Option<HistoryStack<Self::Schema>>) {
        self.history_handle = history;
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
}

impl<T: Clone + std::fmt::Debug> GSOWrapperBuilder<T> {
    pub fn new(
        data: T,
        slots: Option<HashMap<Uid, ActiveSlot>>,
        operative_tag: Rc<Tag>,
        template_tag: Rc<Tag>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().as_u128(),
            slots: slots.unwrap_or(HashMap::new()),
            parent_slots: Vec::new(),
            data,
            operative_tag,
            template_tag,
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
impl<F, T, TSchema: GSO> Producable<GSOWrapper<T, TSchema>> for GSOWrapperBuilder<F>
where
    F: Producable<T>,
{
    fn produce(&self) -> GSOWrapper<T, TSchema> {
        GSOWrapper::<T, TSchema> {
            history_handle: None,
            id: self.id.clone(),
            slots: self.slots.clone(),
            parent_slots: self.parent_slots.clone(),
            data: self.data.produce(),
            operative_tag: self.operative_tag.clone(),
            template_tag: self.template_tag.clone(),
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

impl<F, T, TSchema: GSO> Finalizable<GSOWrapper<T, TSchema>> for GSOWrapperBuilder<F> where
    F: Verifiable + Producable<T>
{
}

pub trait Buildable
where
    Self: Sized + 'static,
    GSOWrapper<Self, Self::Schema>: Instantiable<Schema = Self::Schema>,
{
    type Builder: Finalizable<GSOWrapper<Self, Self::Schema>>;
    type Schema: GSO;

    fn initiate_build() -> GSOBuilder<Self::Builder, GSOWrapper<Self, Self::Schema>, Self::Schema>;
    fn get_operative_id() -> Uid;
}

pub trait Verifiable {
    fn verify(&self) -> Result<(), Error>;
}
pub trait Instantiable: std::fmt::Debug + Any {
    type Schema: GSO;

    fn instantiate(&self, history: HistoryStack<Self::Schema>) -> Self::Schema;
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
impl<T, TSchema: GSO> InstantiableWrapper<GSOWrapper<T, TSchema>, TSchema>
where
    GSOWrapper<T, TSchema>: Instantiable<Schema = TSchema>,
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

pub fn integrate_child<F, T, C, TSchema: GSO>(
    builder: &mut GSOBuilder<GSOWrapperBuilder<F>, GSOWrapper<T, TSchema>, TSchema>,
    mut child: InstantiableWrapper<GSOWrapper<C, TSchema>, TSchema>,
    slot_id: Uid,
) -> &mut GSOBuilder<GSOWrapperBuilder<F>, GSOWrapper<T, TSchema>, TSchema>
where
    F: Verifiable + Producable<T> + Clone + std::fmt::Debug,
    T: Clone + std::fmt::Debug,
    GSOWrapper<C, TSchema>: Instantiable<Schema = TSchema> + 'static,
{
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

pub fn integrate_child_id<'a, F, T, TSchema: GSO>(
    builder: &'a mut GSOBuilder<GSOWrapperBuilder<F>, GSOWrapper<T, TSchema>, TSchema>,
    child_id: &Uid,
    slot_id: Uid,
) -> &'a mut GSOBuilder<GSOWrapperBuilder<F>, GSOWrapper<T, TSchema>, TSchema>
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

impl<T, TSchema: GSO + 'static> Instantiable for GSOWrapper<T, TSchema>
where
    T: Clone + std::fmt::Debug + IntoSchema<Schema = TSchema> + 'static,
{
    type Schema = TSchema;

    fn instantiate(&self, history: HistoryStack<TSchema>) -> Self::Schema {
        let mut new_self = self.clone();
        new_self.set_history(Some(history));
        T::into_schema(new_self)
    }

    fn get_instance_id(&self) -> &Uid {
        self.get_id()
    }
}

pub trait IntoSchema
where
    Self: Sized,
{
    type Schema: GSO;
    fn into_schema(instantiable: GSOWrapper<Self, Self::Schema>) -> Self::Schema;
}
