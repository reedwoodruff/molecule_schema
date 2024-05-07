use anyhow::{Error, Result};
use std::fmt;

use std::ops::Deref;
use std::{any::Any, cell::RefCell, collections::HashMap, marker::PhantomData, rc::Rc};
use strum_macros::Display;

use crate::constraint_schema::OperativeVariants;
use crate::constraint_schema_item::ConstraintSchemaItem;
use crate::{
    common::{ConstraintTraits, Tag, Uid},
    constraint_schema::{
        ConstraintSchema, LibraryOperative, LibraryTemplate, OperativeSlot, SlotBounds,
    },
    primitives::{PrimitiveTypes, PrimitiveValues},
};

pub mod blueprint;
#[cfg(feature = "reactive")]
pub mod reactive;

mod tests;

pub type LibOp = LibraryOperative<PrimitiveTypes, PrimitiveValues>;
pub type LibTemplate = LibraryTemplate<PrimitiveTypes, PrimitiveValues>;

#[derive(Debug, Display)]
pub enum ElementCreationError {
    RequiredFieldIsEmpty,
    BoundCheckOutOfRange,
    ChildElementIsWrongType,
    ChildElementDoesntExist,
}
impl std::error::Error for ElementCreationError {}

#[derive(Debug, Display)]
pub enum ElementDeletionError {
    RequiredByParentSlot,
}
impl std::error::Error for ElementDeletionError {}

#[derive(Debug, Clone)]
pub enum TaggedAction {
    Normal,
    Undo,
    Redo,
}

pub type HistoryStack<TSchema> = Vec<Vec<HistoryItem<TSchema>>>;
pub type HistoryRef<TSchema> = Rc<RefCell<HistoryContainer<TSchema>>>;
#[derive(Debug)]
pub struct BaseGraphEnvironment<TSchema: GSO> {
    pub created_instances: HashMap<Uid, TSchema>,
    pub constraint_schema: &'static ConstraintSchema<PrimitiveTypes, PrimitiveValues>,
    pub history: HistoryRef<TSchema>,
}
impl<TSchema: GSO + 'static> BaseGraphEnvironment<TSchema> {
    pub fn new(
        constraint_schema: &'static ConstraintSchema<PrimitiveTypes, PrimitiveValues>,
    ) -> Self {
        Self {
            created_instances: HashMap::new(),
            constraint_schema,
            history: Rc::new(RefCell::new(HistoryContainer {
                undo: Vec::new(),
                redo: Vec::new(),
            })),
        }
    }
    // pub fn new_without_schema() -> Self {
    //     Self {
    //         history: Rc::new(RefCell::new(HistoryContainer {
    //             undo: Vec::new(),
    //             redo: Vec::new(),
    //         })),
    //         created_instances: HashMap::new(),
    //         constraint_schema: &ConstraintSchema {
    //             template_library: HashMap::new(),
    //             instance_library: HashMap::new(),
    //             operative_library: HashMap::new(),
    //             traits: HashMap::new(),
    //         },
    //     }
    // }
}

impl<TSchema: GSO<Schema = TSchema> + 'static> BaseGraphEnvironment<TSchema> {
    fn push_history_item(&mut self, history_item: Vec<HistoryItem<TSchema>>, tag: &TaggedAction) {
        match tag {
            TaggedAction::Normal => self.history.borrow_mut().undo.push(history_item),
            TaggedAction::Undo => self.history.borrow_mut().redo.push(history_item),
            TaggedAction::Redo => self.history.borrow_mut().undo.push(history_item),
        }
    }
    fn append_history_item(&mut self, history_item: HistoryItem<TSchema>, tag: &TaggedAction) {
        match tag {
            TaggedAction::Normal => self
                .history
                .borrow_mut()
                .undo
                .last_mut()
                .unwrap()
                .push(history_item),
            TaggedAction::Undo => self
                .history
                .borrow_mut()
                .redo
                .last_mut()
                .unwrap()
                .push(history_item),
            TaggedAction::Redo => self
                .history
                .borrow_mut()
                .undo
                .last_mut()
                .unwrap()
                .push(history_item),
        }
    }
    fn check_and_delete_children_tagged(
        &mut self,
        id: &Uid,
        parent_id: Option<&Uid>,
        tag: &TaggedAction,
    ) {
        let mut should_delete = parent_id.is_none();

        if let Some(parent_id) = parent_id {
            let child_parent_slots = self.get(id).unwrap().get_parent_slots();
            let remaining_parents = child_parent_slots
                .iter()
                .filter(|slot_ref| slot_ref.host_instance_id != *parent_id)
                .collect::<Vec<_>>();
            if remaining_parents.is_empty() {
                should_delete = true;
            }
        }

        if !should_delete && parent_id.is_some() {
            let removed_slot_refs = self
                .get_mut(id)
                .unwrap()
                .remove_parent(parent_id.unwrap(), None);
            removed_slot_refs.into_iter().for_each(|slot_ref| {
                self.append_history_item(HistoryItem::RemoveParent(slot_ref), tag);
            })
        }

        if should_delete {
            self.get(id)
                .unwrap()
                .get_slots()
                .clone()
                .values()
                .for_each(|slot| {
                    slot.slotted_instances
                        .iter()
                        .for_each(|slotted_instance_id| {
                            self.check_and_delete_children_tagged(
                                slotted_instance_id,
                                Some(id),
                                tag,
                            );
                        });
                });
        }
        let mut removed_value = self.created_instances.remove(id).unwrap();
        removed_value.set_history(None);
        self.append_history_item(HistoryItem::Delete(removed_value), tag);
    }
    fn instantiate_element_tagged<T: std::fmt::Debug + Clone + 'static>(
        &mut self,
        element: InstantiableWrapper<GSOWrapper<T, TSchema>, TSchema>,
        tag: &TaggedAction,
    ) -> Result<Uid, Error>
    where
        Self: Sized,
        GSOWrapper<T, TSchema>: Instantiable<Schema = TSchema>,
    {
        // Assumption here that when instantiating with this method,
        // the only child updates will be with regards to parents which are now being created
        for child_update in element.child_updates.iter() {
            let mut parent = element.prereq_instantiables.iter().find(|prereq_inst| {
                *prereq_inst.get_instance_id() == child_update.1.host_instance_id
            });
            let mut operative_descriptor;
            if parent.is_none() {
                operative_descriptor = &element
                    .instantiable_instance
                    .get_template()
                    .operative_slots
                    .get(&child_update.1.slot_id)
                    .unwrap()
                    .operative_descriptor;
            } else {
                operative_descriptor = &parent
                    .unwrap()
                    .get_template()
                    .operative_slots
                    .get(&child_update.1.slot_id)
                    .unwrap()
                    .operative_descriptor;
            }
            match operative_descriptor {
                OperativeVariants::LibraryOperative(lib_op_id) => {
                    if *lib_op_id
                        != self
                            .get(&child_update.0)
                            .unwrap()
                            .get_operative()
                            .get_tag()
                            .id
                    {
                        return Err(Error::new(ElementCreationError::ChildElementIsWrongType));
                    };
                }
                OperativeVariants::TraitOperative(trait_op) => {
                    let child_digest = self
                        .get(&child_update.0)
                        .unwrap()
                        .get_operative()
                        .get_trait_impl_digest(self.constraint_schema);
                    let matches_trait_bounds = trait_op
                        .trait_ids
                        .iter()
                        .all(|trait_id| child_digest.trait_impls.contains_key(trait_id));
                    if !matches_trait_bounds {
                        return Err(Error::new(ElementCreationError::ChildElementIsWrongType));
                    }
                }
            }
        }
        let id = *element.get_instantiable_instance().get_instance_id();
        self.push_history_item(vec![HistoryItem::BlockActionMarker], &tag);

        element.child_updates.iter().for_each(|child_update| {
            let child = self.get_mut(&child_update.0).unwrap();
            child.add_parent_slot(&child_update.1.clone());
            self.append_history_item(HistoryItem::AddParent(child_update.1.clone()), &tag);
        });
        element.parent_updates.iter().for_each(|parent_update| {
            let parent = self.get_mut(&parent_update.0).unwrap();
            parent.add_child_to_slot(&parent_update.1);
            self.append_history_item(HistoryItem::AddChild(parent_update.1.clone()), &tag);
        });
        element.flatten().into_iter().for_each(|instantiable| {
            let instantiated = instantiable.instantiate(self.history.clone());
            self.append_history_item(HistoryItem::Create(*instantiable.get_instance_id()), &tag);
            self.created_instances
                .insert(*instantiable.get_instance_id(), instantiated);
        });
        Ok(id)
    }
    fn create_connection_tagged(
        &mut self,
        connection: ConnectionAction,
        tag: &TaggedAction,
    ) -> Result<(), Error> {
        match &self
            .get(&connection.slot_ref.host_instance_id)
            .unwrap()
            .get_slot_by_id(&connection.slot_ref.slot_id)
            .unwrap()
            .slot
            .operative_descriptor
        {
            OperativeVariants::LibraryOperative(expected_id) => {
                if *expected_id
                    != self
                        .get_mut(&connection.slot_ref.child_instance_id)
                        .unwrap()
                        .get_operative()
                        .get_tag()
                        .id
                {
                    return Err(Error::new(ElementCreationError::ChildElementIsWrongType));
                }
            }
            OperativeVariants::TraitOperative(trait_op) => {
                let child_digest = self
                    .get(&connection.slot_ref.child_instance_id)
                    .unwrap()
                    .get_operative()
                    .get_trait_impl_digest(self.constraint_schema);
                let matches_trait_bounds = trait_op
                    .trait_ids
                    .iter()
                    .all(|trait_id| child_digest.trait_impls.contains_key(trait_id));
                if !matches_trait_bounds {
                    return Err(Error::new(ElementCreationError::ChildElementIsWrongType));
                }
            }
        }
        let parent = self.get_mut(&connection.slot_ref.host_instance_id).unwrap();
        if parent
            .get_slot_by_id(&connection.slot_ref.slot_id)
            .unwrap()
            .can_add_one()
        {
            parent.add_child_to_slot(&connection.slot_ref);
        } else {
            return Err(Error::new(ElementCreationError::BoundCheckOutOfRange));
        }
        self.get_mut(&connection.slot_ref.child_instance_id)
            .unwrap()
            .add_parent_slot(&connection.slot_ref);
        let history_item = vec![
            HistoryItem::<TSchema>::BlockActionMarker,
            HistoryItem::AddChild(connection.slot_ref.clone()),
            HistoryItem::AddParent(connection.slot_ref.clone()),
        ];
        self.push_history_item(history_item, &tag);
        Ok(())
    }
    fn delete_tagged(&mut self, id: &Uid, tag: &TaggedAction) -> Result<(), Error> {
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
        self.push_history_item(vec![HistoryItem::BlockActionMarker], &tag);
        parent_slots.iter().for_each(|parent_slot| {
            self.get_mut(&parent_slot.host_instance_id)
                .unwrap()
                .remove_child_from_slot(parent_slot);
        });

        self.check_and_delete_children_tagged(id, None, tag);

        Ok(())
    }
    fn process_history_tagged(&mut self, history: Vec<HistoryItem<TSchema>>, tag: &TaggedAction) {
        if !history.is_empty() {
            self.push_history_item(vec![HistoryItem::BlockActionMarker], tag);
        }
        history.into_iter().for_each(|action| match action {
            HistoryItem::RemoveChildFromSlot(slot_ref) => {
                let host = self.get_mut(&slot_ref.host_instance_id).unwrap();
                host.add_child_to_slot(&slot_ref);
                self.append_history_item(HistoryItem::AddChild(slot_ref), tag);
            }
            HistoryItem::RemoveParent(slot_ref) => {
                let child = self.get_mut(&slot_ref.child_instance_id).unwrap();
                child.add_parent_slot(&slot_ref);
                self.append_history_item(HistoryItem::AddParent(slot_ref), tag);
            }
            HistoryItem::AddParent(slot_ref) => {
                let child = self.get_mut(&slot_ref.child_instance_id).unwrap();
                child.remove_parent(&slot_ref.host_instance_id, Some(&slot_ref.slot_id));
                self.append_history_item(HistoryItem::RemoveParent(slot_ref), tag);
            }
            HistoryItem::AddChild(slot_ref) => {
                let host = self.get_mut(&slot_ref.host_instance_id).unwrap();
                host.remove_child_from_slot(&slot_ref);
                self.append_history_item(HistoryItem::RemoveChildFromSlot(slot_ref), tag);
            }
            HistoryItem::Delete(mut deleted_node) => {
                let deleted_node_id = *deleted_node.get_id();
                deleted_node.set_history(Some(self.history.clone()));
                self.created_instances
                    .insert(deleted_node_id.clone(), deleted_node);
                self.append_history_item(HistoryItem::Create(deleted_node_id), tag);
            }
            HistoryItem::Create(node_id) => {
                let created_node = self.created_instances.remove(&node_id).unwrap();
                self.append_history_item(HistoryItem::Delete(created_node), tag);
            }
            HistoryItem::EditField(field_edit) => {
                self.created_instances
                    .get_mut(&field_edit.instance_id)
                    .unwrap()
                    .apply_field_edit(FieldEdit {
                        field_id: field_edit.field_id.clone(),
                        value: field_edit.prev_value.clone(),
                    });
                self.append_history_item(HistoryItem::EditField(field_edit.reverse()), tag);
            }
            HistoryItem::BlockActionMarker => {}
        })
    }
}
impl<TSchema: GSO<Schema = TSchema> + 'static> GraphEnvironment for BaseGraphEnvironment<TSchema> {
    type Schema = TSchema;
    type Types = PrimitiveTypes;
    type Values = PrimitiveValues;

    fn get_constraint_schema(&self) -> &ConstraintSchema<Self::Types, Self::Values> {
        &self.constraint_schema
    }

    fn get(&self, id: &Uid) -> Option<&Self::Schema> {
        self.created_instances.get(id)
    }
    fn create_connection(&mut self, connection: ConnectionAction) -> Result<(), Error> {
        self.history.borrow_mut().redo.clear();
        self.create_connection_tagged(connection, &TaggedAction::Normal)
    }
    fn instantiate_element<T: std::fmt::Debug + Clone + 'static>(
        &mut self,
        element: InstantiableWrapper<GSOWrapper<T, Self::Schema>, Self::Schema>,
    ) -> Result<Uid, Error>
    where
        Self: Sized,
        GSOWrapper<T, Self::Schema>: Instantiable<Schema = Self::Schema>,
    {
        self.history.borrow_mut().redo.clear();
        self.instantiate_element_tagged(element, &TaggedAction::Normal)
    }

    fn delete(&mut self, id: &Uid) -> Result<(), Error> {
        self.history.borrow_mut().redo.clear();
        self.delete_tagged(id, &TaggedAction::Normal)
    }

    fn get_mut(&mut self, id: &Uid) -> Option<&mut Self::Schema> {
        self.created_instances.get_mut(id)
    }

    fn undo(&mut self) {
        let undo_item = self.history.borrow_mut().undo.pop();
        if undo_item.is_none() {
            return;
        }
        let undo_item = undo_item.unwrap();
        self.process_history_tagged(undo_item, &TaggedAction::Undo);
    }

    fn redo(&mut self) {
        let redo_item = self.history.borrow_mut().redo.pop();
        if redo_item.is_none() {
            return;
        }
        let redo_item = redo_item.unwrap();
        self.process_history_tagged(redo_item, &TaggedAction::Redo);
    }
}

pub trait GraphEnvironment {
    type Types: ConstraintTraits;
    type Values: ConstraintTraits;
    type Schema: GSO + 'static;

    fn get(&self, id: &Uid) -> Option<&Self::Schema>;
    fn create_connection(&mut self, connection: ConnectionAction) -> Result<(), Error>;
    fn instantiate_element<T>(
        &mut self,
        element: InstantiableWrapper<GSOWrapper<T, Self::Schema>, Self::Schema>,
    ) -> Result<Uid, Error>
    where
        GSOWrapper<T, Self::Schema>: Instantiable<Schema = Self::Schema>,
        Self: Sized,
        T: std::fmt::Debug + Clone + 'static;
    fn get_mut(&mut self, id: &Uid) -> Option<&mut Self::Schema>;
    fn get_constraint_schema(&self) -> &ConstraintSchema<Self::Types, Self::Values>;
    fn delete(&mut self, id: &Uid) -> Result<(), Error>;
    fn undo(&mut self);
    fn redo(&mut self);
}

#[derive(Debug, Clone)]
pub enum HistoryItem<TSchema: GSO> {
    RemoveChildFromSlot(SlotRef),
    RemoveParent(SlotRef),
    // AddChildToSlot(),
    AddParent(SlotRef),
    AddChild(SlotRef),
    Delete(TSchema),
    Create(Uid),
    EditField(HistoryFieldEdit),
    BlockActionMarker,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HistoryFieldEdit {
    pub instance_id: Uid,
    pub field_id: Uid,
    pub new_value: PrimitiveValues,
    pub prev_value: PrimitiveValues,
}
impl HistoryFieldEdit {
    pub fn reverse(self) -> Self {
        Self {
            instance_id: self.instance_id,
            field_id: self.field_id,
            new_value: self.prev_value,
            prev_value: self.new_value,
        }
    }
}
pub struct FieldEdit {
    pub field_id: Uid,
    pub value: PrimitiveValues,
}

pub trait FieldEditable {
    fn apply_field_edit(&mut self, field_edit: FieldEdit);
}
pub trait GSO: std::fmt::Debug + Clone + FieldEditable {
    type Schema: GSO;
    /// Instance ID
    fn get_id(&self) -> &Uid;
    fn get_operative(&self) -> &'static LibraryOperative<PrimitiveTypes, PrimitiveValues>;
    fn get_template(&self) -> &'static LibraryTemplate<PrimitiveTypes, PrimitiveValues>;
    fn get_slot_by_id(&self, slot_id: &Uid) -> Option<&ActiveSlot> {
        self.get_slots().get(slot_id)
    }
    fn get_slots(&self) -> &HashMap<Uid, ActiveSlot>;
    fn get_parent_slots(&self) -> &Vec<SlotRef>;
    fn add_parent_slot(&mut self, slot_ref: &SlotRef) -> &mut Self;
    fn add_child_to_slot(&mut self, slot_ref: &SlotRef) -> &mut Self;
    fn remove_child_from_slot(&mut self, slot_ref: &SlotRef) -> &mut Self;
    fn remove_parent(&mut self, parent_id: &Uid, slot_id: Option<&Uid>) -> Vec<SlotRef>;
    fn set_history(&mut self, history: Option<HistoryRef<Self::Schema>>);
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SlotRef {
    pub host_instance_id: Uid,
    pub child_instance_id: Uid,
    pub slot_id: Uid,
}

pub trait Slotted {}

#[derive(Clone, Debug)]
pub struct ActiveSlot {
    pub slot: &'static OperativeSlot,
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
    pub fn can_add_one(&self) -> bool {
        let len = self.slotted_instances.len() + 1;
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
pub struct HistoryContainer<TSchema: GSO> {
    pub undo: HistoryStack<TSchema>,
    pub redo: HistoryStack<TSchema>,
}

#[derive(Clone)]
pub struct GSOWrapper<T, TSchema: GSO> {
    id: Uid,
    slots: HashMap<Uid, ActiveSlot>,
    parent_slots: Vec<SlotRef>,
    pub data: HashMap<Uid, PrimitiveValues>,
    operative: &'static LibraryOperative<PrimitiveTypes, PrimitiveValues>,
    template: &'static LibraryTemplate<PrimitiveTypes, PrimitiveValues>,
    pub history: Option<HistoryRef<TSchema>>,
    _phantom: PhantomData<T>,
}
impl<T: std::fmt::Debug, TSchema: GSO> std::fmt::Debug for GSOWrapper<T, TSchema> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GSOWrapper")
            .field("id", &self.id)
            .field(
                "slots",
                &self
                    .slots
                    .values()
                    .map(|slot| (&slot.slot.tag.name, &slot.slotted_instances))
                    .collect::<HashMap<_, _>>(),
            )
            .field(
                "parent_slots",
                &self
                    .parent_slots
                    .iter()
                    .map(|parent_slot| parent_slot.host_instance_id)
                    .collect::<Vec<_>>(),
            )
            // .field("parent_slots", &self.parent_slots)
            .field("data", &self.data)
            .finish()
    }
}

impl<T: Clone + std::fmt::Debug, TSchema: GSO> GSOWrapper<T, TSchema> {}
impl<T: Clone + std::fmt::Debug + FieldEditable, TSchema: GSO> FieldEditable
    for GSOWrapper<T, TSchema>
{
    fn apply_field_edit(&mut self, field_edit: FieldEdit) {
        // self.data.apply_field_edit(field_edit);
        self.data.insert(field_edit.field_id, field_edit.value);
    }
}

impl<T: Clone + std::fmt::Debug + FieldEditable, TSchema: GSO> GSO for GSOWrapper<T, TSchema> {
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

    fn get_operative(&self) -> &'static LibraryOperative<PrimitiveTypes, PrimitiveValues> {
        &self.operative
    }

    fn get_template(&self) -> &'static LibraryTemplate<PrimitiveTypes, PrimitiveValues> {
        &self.template
    }

    fn add_parent_slot(&mut self, slot_ref: &SlotRef) -> &mut Self {
        self.parent_slots.push(slot_ref.clone());
        // self.history
        //     .as_mut()
        //     .unwrap()
        //     .borrow_mut()
        //     .last_mut()
        //     .unwrap()
        //     .push(HistoryItem::AddParent(slot_ref.clone()));
        self
    }

    fn remove_child_from_slot(&mut self, slot_ref: &SlotRef) -> &mut Self {
        self.slots
            .get_mut(&slot_ref.slot_id)
            .unwrap()
            .slotted_instances
            .retain(|slotted_instance_id| *slotted_instance_id != slot_ref.child_instance_id);
        // self.history
        //     .as_mut()
        //     .unwrap()
        //     .borrow_mut()
        //     .last_mut()
        //     .unwrap()
        //     .push(HistoryItem::RemoveChildFromSlot(vec![slot_ref.clone()]));
        self
    }

    fn remove_parent(&mut self, parent_id: &Uid, slot_id: Option<&Uid>) -> Vec<SlotRef> {
        let mut removed = Vec::new();
        self.parent_slots.retain(|slot_ref| {
            let matches_parent = slot_ref.host_instance_id == *parent_id;
            let matches_slot_id = if let Some(given_slot_id) = slot_id {
                slot_ref.slot_id == *given_slot_id
            } else {
                true
            };
            if matches_parent && matches_slot_id {
                removed.push(slot_ref.clone());
                return false;
            } else {
                return true;
            }
        });
        // self.history
        //     .as_mut()
        //     .unwrap()
        //     .borrow_mut()
        //     .last_mut()
        //     .unwrap()
        //     .push(HistoryItem::RemoveParent(removed));
        removed
    }

    fn set_history(&mut self, history: Option<HistoryRef<Self::Schema>>) {
        self.history = history;
    }

    fn add_child_to_slot(&mut self, slot_ref: &SlotRef) -> &mut Self {
        self.slots
            .get_mut(&slot_ref.slot_id)
            .unwrap()
            .slotted_instances
            .push(slot_ref.child_instance_id);
        self
    }
}
#[derive(Clone, Debug)]
pub struct GSOWrapperBuilder<T> {
    id: Uid,
    slots: HashMap<Uid, ActiveSlot>,
    parent_slots: Vec<SlotRef>,
    pub data: HashMap<Uid, Option<PrimitiveValues>>,
    operative: &'static LibraryOperative<PrimitiveTypes, PrimitiveValues>,
    template: &'static LibraryTemplate<PrimitiveTypes, PrimitiveValues>,
    _phantom: PhantomData<T>,
}

impl<T: Clone + std::fmt::Debug> GSOWrapperBuilder<T> {
    pub fn new(
        data: Option<HashMap<Uid, Option<PrimitiveValues>>>,
        slots: Option<HashMap<Uid, ActiveSlot>>,
        operative: &'static LibraryOperative<PrimitiveTypes, PrimitiveValues>,
        template: &'static LibraryTemplate<PrimitiveTypes, PrimitiveValues>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().as_u128(),
            slots: slots.unwrap_or_default(),
            parent_slots: Vec::new(),
            data: data.unwrap_or_default(),
            operative,
            template,
            _phantom: PhantomData,
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
            history: None,
            id: self.id,
            slots: self.slots.clone(),
            parent_slots: self.parent_slots.clone(),
            data: self
                .data
                .iter()
                .map(|(id, value)| (*id, value.clone().unwrap()))
                .collect::<HashMap<Uid, PrimitiveValues>>(),
            operative: &self.operative,
            template: &self.template,
            _phantom: PhantomData,
        }
    }
}

impl<F> Verifiable for GSOWrapperBuilder<F>
where
    F: Verifiable,
{
    fn verify(&self) -> Result<(), Error> {
        // self.data.verify()?;
        let field_errors = self
            .data
            .values()
            .filter_map(|field_val| {
                if field_val.is_none() {
                    return Some(Error::new(ElementCreationError::RequiredFieldIsEmpty));
                }
                None
            })
            .collect::<Vec<_>>();
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
        if slot_errors.is_empty() && field_errors.is_empty() {
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

    fn instantiate(&self, history: HistoryRef<Self::Schema>) -> Self::Schema;
    fn get_instance_id(&self) -> &Uid;
    fn get_template(&self) -> &'static LibraryTemplate<PrimitiveTypes, PrimitiveValues>;
}
pub type InstantiableElements<TSchema> = Vec<Rc<dyn Instantiable<Schema = TSchema>>>;

#[derive(Debug, Clone)]
pub struct InstantiableWrapper<T, TSchema>
where
    T: Instantiable<Schema = TSchema>,
{
    prereq_instantiables: InstantiableElements<TSchema>,
    instantiable_instance: T,
    pub parent_updates: Vec<(Uid, SlotRef)>,
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

pub struct ConnectionAction {
    pub slot_ref: SlotRef,
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

impl<F, T, TSchema: GSO> GSOBuilder<F, T, TSchema>
where
    F: Finalizable<T>,
    T: Instantiable<Schema = TSchema> + 'static,
{
    pub fn build(
        &mut self,
        graph: &impl GraphEnvironment<
            Types = PrimitiveTypes,
            Values = PrimitiveValues,
            Schema = TSchema,
        >,
    ) -> Result<InstantiableWrapper<T, TSchema>, Error> {
        for parent_update in self.parent_updates.iter() {
            let can_add_one = graph
                .get(&parent_update.0)
                .unwrap()
                .get_slot_by_id(&parent_update.1.slot_id)
                .unwrap()
                .can_add_one();
            if !can_add_one {
                return Err(Error::new(ElementCreationError::BoundCheckOutOfRange));
            }
        }

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
    T: Clone + std::fmt::Debug + IntoSchema<Schema = TSchema> + FieldEditable + 'static,
{
    type Schema = TSchema;

    fn instantiate(&self, history: HistoryRef<TSchema>) -> Self::Schema {
        let mut new_self = self.clone();
        new_self.set_history(Some(history));
        T::into_schema(new_self)
    }

    fn get_instance_id(&self) -> &Uid {
        self.get_id()
    }
    fn get_template(&self) -> &'static LibraryTemplate<PrimitiveTypes, PrimitiveValues> {
        self.template
    }
}

pub trait IntoSchema
where
    Self: Sized,
{
    type Schema: GSO;
    fn into_schema(instantiable: GSOWrapper<Self, Self::Schema>) -> Self::Schema;
}
