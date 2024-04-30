use anyhow::{Error, Result};
use std::fmt;

use std::{any::Any, cell::RefCell, collections::HashMap, marker::PhantomData, rc::Rc};

use crate::constraint_schema::{
    LibraryOperative, LibraryTemplate, OperativeSlot, OperativeVariants, SlotBounds,
};
use crate::constraint_schema_item::ConstraintSchemaItem;
use crate::{
    common::{ConstraintTraits, Uid},
    constraint_schema::{
        ConstraintSchema,
        // LibraryOperative, LibraryTemplate, OperativeSlot, SlotBounds,
    },
    primitives::{PrimitiveTypes, PrimitiveValues},
};

use super::{
    ConnectionAction, ElementCreationError, ElementDeletionError, FieldEdit, Finalizable,
    HistoryFieldEdit, SlotRef, TaggedAction, Verifiable,
};
use leptos::{RwSignal, SignalGet, SignalSet, SignalUpdate, SignalWith};

pub trait RProducable<T> {
    type Schema: RGSO;
    fn produce(&self, graph: Rc<RBaseGraphEnvironment<Self::Schema>>) -> T;
}

pub trait RFieldEditable {
    fn apply_field_edit(&self, field_edit: FieldEdit);
}
pub type RHistoryStack<TSchema> = Vec<Vec<RHistoryItem<TSchema>>>;
pub type RHistoryRef<TSchema> = Rc<RefCell<RHistoryContainer<TSchema>>>;
#[derive(Clone, Debug)]
pub struct RHistoryContainer<TSchema: RGSO> {
    pub undo: RHistoryStack<TSchema>,
    pub redo: RHistoryStack<TSchema>,
}

#[derive(Debug, Clone)]
pub struct RBaseGraphEnvironment<TSchema: RGSO + 'static> {
    pub created_instances: RwSignal<HashMap<Uid, TSchema>>,
    pub constraint_schema: &'static ConstraintSchema<PrimitiveTypes, PrimitiveValues>,
    pub history: RHistoryRef<TSchema>,
}
impl<TSchema: RGSO + 'static> RBaseGraphEnvironment<TSchema> {
    pub fn new(
        constraint_schema: &'static ConstraintSchema<PrimitiveTypes, PrimitiveValues>,
    ) -> Self {
        Self {
            created_instances: RwSignal::new(HashMap::new()),
            constraint_schema,
            history: Rc::new(RefCell::new(RHistoryContainer {
                undo: Vec::new(),
                redo: Vec::new(),
            })),
        }
    }
}

impl<TSchema: RGSO<Schema = TSchema> + 'static> RBaseGraphEnvironment<TSchema> {
    fn push_history_item(&self, history_item: Vec<RHistoryItem<TSchema>>, tag: &TaggedAction) {
        match tag {
            TaggedAction::Normal => self.history.borrow_mut().undo.push(history_item),
            TaggedAction::Undo => self.history.borrow_mut().redo.push(history_item),
            TaggedAction::Redo => self.history.borrow_mut().undo.push(history_item),
        }
    }
    fn append_history_item(&self, history_item: RHistoryItem<TSchema>, tag: &TaggedAction) {
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
            child_parent_slots.with(|child_parent_slots| {
                let remaining_parents = child_parent_slots
                    .iter()
                    .filter(|slot_ref| slot_ref.host_instance_id != *parent_id)
                    .collect::<Vec<_>>();
                if remaining_parents.is_empty() {
                    should_delete = true;
                }
            });
        }

        if !should_delete && parent_id.is_some() {
            let removed_slot_refs = self
                .get(id)
                .unwrap()
                .remove_parent(parent_id.unwrap(), None);
            removed_slot_refs.into_iter().for_each(|slot_ref| {
                self.append_history_item(RHistoryItem::RemoveParent(slot_ref), tag);
            })
        }

        if should_delete {
            self.get(id).unwrap().get_slots().values().for_each(|slot| {
                slot.slotted_instances.with(|slotted_instances| {
                    slotted_instances.iter().for_each(|slotted_instance_id| {
                        self.check_and_delete_children_tagged(slotted_instance_id, Some(id), tag);
                    });
                })
            });
        }
        let mut removed_value = None;
        self.created_instances
            .update(|prev| removed_value = prev.remove(id));
        let mut removed_value = removed_value.unwrap();
        // removed_value.set_history(None);
        self.append_history_item(RHistoryItem::Delete(removed_value), tag);
    }
    fn instantiate_element_tagged<T: std::fmt::Debug + Clone + 'static>(
        &self,
        element: RInstantiableWrapper<RGSOWrapperBuilder<T>, TSchema>,
        tag: &TaggedAction,
    ) -> Result<Uid, Error>
    where
        Self: Sized,
        // RGSOWrapper<T, TSchema>: RInstantiable<Schema = TSchema> + RFieldEditable,
        T: RIntoSchema<Schema = TSchema>,
    {
        for child_update in element.child_updates.iter() {
            let parent = element
                .prereq_instantiables
                .iter()
                .find(|prereq_inst| *prereq_inst.get_id() == child_update.1.host_instance_id);
            let operative_descriptor = if parent.is_none() {
                &element
                    .instantiable_instance
                    .get_template()
                    .operative_slots
                    .get(&child_update.1.slot_id)
                    .unwrap()
                    .operative_descriptor
            } else {
                &parent
                    .unwrap()
                    .get_template()
                    .operative_slots
                    .get(&child_update.1.slot_id)
                    .unwrap()
                    .operative_descriptor
            };
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
        let id = *element.get_instantiable_instance().get_id();
        self.push_history_item(vec![RHistoryItem::BlockActionMarker], &tag);

        element.child_updates.iter().for_each(|child_update| {
            let child = self.get(&child_update.0).unwrap();
            child.add_parent_slot(child_update.1.clone());
            self.append_history_item(RHistoryItem::AddParent(child_update.1.clone()), &tag);
        });
        element.parent_updates.iter().for_each(|parent_update| {
            let parent = self.get(&parent_update.0).unwrap();
            parent.add_child_to_slot(parent_update.1.clone());
            self.append_history_item(RHistoryItem::AddChild(parent_update.1.clone()), &tag);
        });
        element.flatten().into_iter().for_each(|instantiable| {
            let instantiated = instantiable.instantiate(Rc::new(self.clone()));
            self.append_history_item(RHistoryItem::Create(*instantiable.get_id()), &tag);
            self.created_instances.update(|prev| {
                prev.insert(*instantiable.get_id(), instantiated);
            });
        });
        Ok(id)
    }
    fn create_connection_tagged(
        &self,
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
                        .get(&connection.slot_ref.child_instance_id)
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
                    .get_trait_impl_digest(&self.constraint_schema);
                let matches_trait_bounds = trait_op
                    .trait_ids
                    .iter()
                    .all(|trait_id| child_digest.trait_impls.contains_key(trait_id));
                if !matches_trait_bounds {
                    return Err(Error::new(ElementCreationError::ChildElementIsWrongType));
                }
            }
        }
        let parent = self.get(&connection.slot_ref.host_instance_id).unwrap();
        if parent
            .get_slot_by_id(&connection.slot_ref.slot_id)
            .unwrap()
            .can_add_one()
        {
            parent.add_child_to_slot(connection.slot_ref.clone());
        } else {
            return Err(Error::new(ElementCreationError::BoundCheckOutOfRange));
        }
        self.get(&connection.slot_ref.child_instance_id)
            .unwrap()
            .add_parent_slot(connection.slot_ref.clone());
        let history_item = vec![
            RHistoryItem::<TSchema>::BlockActionMarker,
            RHistoryItem::AddChild(connection.slot_ref.clone()),
            RHistoryItem::AddParent(connection.slot_ref.clone()),
        ];
        self.push_history_item(history_item, &tag);
        Ok(())
    }
    fn delete_tagged(&mut self, id: &Uid, tag: &TaggedAction) -> Result<(), Error> {
        let parent_slots = self.get(id).unwrap().get_parent_slots();

        let can_delete = parent_slots.with(|parent_slots| {
            parent_slots.iter().all(|parent_slot| {
                self.get(&parent_slot.host_instance_id)
                    .unwrap()
                    .get_slot_by_id(&parent_slot.slot_id)
                    .unwrap()
                    .can_remove_one()
            })
        });
        if !can_delete {
            return Err(Error::new(ElementDeletionError::RequiredByParentSlot));
        }
        self.push_history_item(vec![RHistoryItem::BlockActionMarker], &tag);
        parent_slots.with(|parent_slots| {
            parent_slots.iter().for_each(|parent_slot| {
                self.get(&parent_slot.host_instance_id)
                    .unwrap()
                    .remove_child_from_slot(parent_slot);
            })
        });

        self.check_and_delete_children_tagged(id, None, tag);

        Ok(())
    }
    fn process_history_tagged(&self, history: Vec<RHistoryItem<TSchema>>, tag: &TaggedAction) {
        if !history.is_empty() {
            self.push_history_item(vec![RHistoryItem::BlockActionMarker], tag);
        }
        history.into_iter().for_each(|action| match action {
            RHistoryItem::RemoveChildFromSlot(slot_ref) => {
                let host = self.get(&slot_ref.host_instance_id).unwrap();
                host.add_child_to_slot(slot_ref.clone());
                self.append_history_item(RHistoryItem::AddChild(slot_ref), tag);
            }
            RHistoryItem::RemoveParent(slot_ref) => {
                let child = self.get(&slot_ref.child_instance_id).unwrap();
                child.add_parent_slot(slot_ref.clone());
                self.append_history_item(RHistoryItem::AddParent(slot_ref), tag);
            }
            RHistoryItem::AddParent(slot_ref) => {
                let child = self.get(&slot_ref.child_instance_id).unwrap();
                child.remove_parent(&slot_ref.host_instance_id, Some(&slot_ref.slot_id));
                self.append_history_item(RHistoryItem::RemoveParent(slot_ref), tag);
            }
            RHistoryItem::AddChild(slot_ref) => {
                let host = self.get(&slot_ref.host_instance_id).unwrap();
                host.remove_child_from_slot(&slot_ref);
                self.append_history_item(RHistoryItem::RemoveChildFromSlot(slot_ref), tag);
            }
            RHistoryItem::Delete(mut deleted_node) => {
                let deleted_node_id = *deleted_node.get_id();
                // deleted_node.set_history(Some(self.history.clone()));
                self.created_instances.update(|prev| {
                    prev.insert(deleted_node_id.clone(), deleted_node);
                });
                self.append_history_item(RHistoryItem::Create(deleted_node_id), tag);
            }
            RHistoryItem::Create(node_id) => {
                let mut created_node = None;
                self.created_instances.update(|prev| {
                    created_node = prev.remove(&node_id);
                });
                let mut created_node = created_node.unwrap();
                // created_node.set_history(None);
                self.append_history_item(RHistoryItem::Delete(created_node), tag);
            }
            RHistoryItem::EditField(field_edit) => {
                self.created_instances.update(|prev| {
                    prev.get_mut(&field_edit.instance_id)
                        .unwrap()
                        .apply_field_edit(FieldEdit {
                            field_id: field_edit.field_id.clone(),
                            value: field_edit.prev_value.clone(),
                        });
                });
                self.append_history_item(RHistoryItem::EditField(field_edit.reverse()), tag);
            }
            RHistoryItem::BlockActionMarker => {}
        })
    }
}
impl<TSchema: RGSO<Schema = TSchema> + 'static> RGraphEnvironment
    for RBaseGraphEnvironment<TSchema>
{
    type Schema = TSchema;
    type Types = PrimitiveTypes;
    type Values = PrimitiveValues;

    fn get_constraint_schema(&self) -> &ConstraintSchema<Self::Types, Self::Values> {
        &self.constraint_schema
    }

    fn get(&self, id: &Uid) -> Option<Self::Schema> {
        let test = self
            .created_instances
            .with(|created_instances| created_instances.get(id).cloned());
        test
    }
    fn create_connection(&self, connection: ConnectionAction) -> Result<(), Error> {
        self.history.borrow_mut().redo.clear();
        self.create_connection_tagged(connection, &TaggedAction::Normal)
    }
    fn instantiate_element<T>(
        &self,
        element: RInstantiableWrapper<RGSOWrapperBuilder<T>, Self::Schema>,
    ) -> Result<Uid, Error>
    where
        // RGSOWrapper<T, Self::Schema>: RInstantiable<Schema = Self::Schema>,
        Self: Sized,
        T: std::fmt::Debug + Clone + RIntoSchema<Schema = Self::Schema> + 'static,
    {
        self.history.borrow_mut().redo.clear();
        self.instantiate_element_tagged(element, &TaggedAction::Normal)
    }

    fn delete(&mut self, id: &Uid) -> Result<(), Error> {
        self.history.borrow_mut().redo.clear();
        self.delete_tagged(id, &TaggedAction::Normal)
    }

    fn undo(&self) {
        let undo_item = self.history.borrow_mut().undo.pop();
        if undo_item.is_none() {
            return;
        }
        let undo_item = undo_item.unwrap();
        self.process_history_tagged(undo_item, &TaggedAction::Undo);
    }

    fn redo(&self) {
        let redo_item = self.history.borrow_mut().redo.pop();
        if redo_item.is_none() {
            return;
        }
        let redo_item = redo_item.unwrap();
        self.process_history_tagged(redo_item, &TaggedAction::Redo);
    }
}

pub trait RGraphEnvironment {
    type Types: ConstraintTraits;
    type Values: ConstraintTraits;
    type Schema: RGSO + 'static;

    fn get(&self, id: &Uid) -> Option<Self::Schema>;
    fn create_connection(&self, connection: ConnectionAction) -> Result<(), Error>;
    fn instantiate_element<T>(
        &self,
        element: RInstantiableWrapper<RGSOWrapperBuilder<T>, Self::Schema>,
    ) -> Result<Uid, Error>
    where
        // RGSOWrapper<T, Self::Schema>: RInstantiable<Schema = Self::Schema>,
        Self: Sized,
        T: std::fmt::Debug + Clone + RIntoSchema<Schema = Self::Schema> + 'static;
    fn get_constraint_schema(&self) -> &ConstraintSchema<Self::Types, Self::Values>;
    fn delete(&mut self, id: &Uid) -> Result<(), Error>;
    fn undo(&self);
    fn redo(&self);
}

#[derive(Debug, Clone)]
pub enum RHistoryItem<TSchema: RGSO> {
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
pub trait RGSO: std::fmt::Debug + Clone + RFieldEditable {
    type Schema: RGSO;
    /// Instance ID
    fn get_id(&self) -> &Uid;
    fn get_operative(&self) -> &'static LibraryOperative<PrimitiveTypes, PrimitiveValues>;
    fn get_template(&self) -> &'static LibraryTemplate<PrimitiveTypes, PrimitiveValues>;
    fn get_slot_by_id(&self, slot_id: &Uid) -> Option<&RActiveSlot> {
        self.get_slots().get(slot_id)
    }
    fn get_slots(&self) -> &HashMap<Uid, RActiveSlot>;
    fn get_parent_slots(&self) -> RwSignal<Vec<SlotRef>>;
    fn add_parent_slot(&self, slot_ref: SlotRef) -> &Self;
    fn add_child_to_slot(&self, slot_ref: SlotRef) -> &Self;
    fn remove_child_from_slot(&self, slot_ref: &SlotRef) -> &Self;
    fn remove_parent(&self, parent_id: &Uid, slot_id: Option<&Uid>) -> Vec<SlotRef>;
    // fn set_history(&mut self, history: Option<RHistoryRef<Self::Schema>>);
    fn get_graph(&self) -> Rc<RBaseGraphEnvironment<Self::Schema>>;
}

pub trait Slotted {}

#[derive(Clone, Debug)]
pub struct RActiveSlot {
    pub slot: &'static OperativeSlot,
    pub slotted_instances: RwSignal<Vec<Uid>>,
}

impl RActiveSlot {
    pub fn check_current_conformity(&self) -> bool {
        let len = self
            .slotted_instances
            .with(|slotted_instances| slotted_instances.len());
        self.check_bound_conformity(len)
    }
    pub fn can_remove_one(&self) -> bool {
        let len = self
            .slotted_instances
            .with(|slotted_instances| slotted_instances.len())
            - 1;
        self.check_bound_conformity(len)
    }
    pub fn can_add_one(&self) -> bool {
        let len = self
            .slotted_instances
            .with(|slotted_instances| slotted_instances.len())
            + 1;
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

#[derive(Clone)]
pub struct RGSOWrapper<T, TSchema: RGSO + 'static> {
    id: Uid,
    pub data: HashMap<Uid, RwSignal<PrimitiveValues>>,
    // pub history: Option<RHistoryRef<TSchema>>,
    pub graph: Rc<RBaseGraphEnvironment<TSchema>>,
    slots: HashMap<Uid, RActiveSlot>,
    parent_slots: RwSignal<Vec<SlotRef>>,
    operative: &'static LibraryOperative<PrimitiveTypes, PrimitiveValues>,
    template: &'static LibraryTemplate<PrimitiveTypes, PrimitiveValues>,
    _phantom: PhantomData<T>,
}
impl<T: std::fmt::Debug, TSchema: RGSO> std::fmt::Debug for RGSOWrapper<T, TSchema> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GSOWrapper")
            .field("id", &self.id)
            .field("slots", &self.slots)
            .field("parent_slots", &self.parent_slots)
            .field("data", &self.data)
            .finish()
    }
}

impl<T: Clone + std::fmt::Debug, TSchema: RGSO> RGSOWrapper<T, TSchema> {}
impl<T: Clone + std::fmt::Debug, TSchema: RGSO> RFieldEditable for RGSOWrapper<T, TSchema> {
    fn apply_field_edit(&self, field_edit: FieldEdit) {
        self.data
            .get(&field_edit.field_id)
            .unwrap()
            .set(field_edit.value);
    }
}

impl<T: Clone + std::fmt::Debug, TSchema: RGSO> RGSO for RGSOWrapper<T, TSchema>
where
    RGSOWrapper<T, TSchema>: RFieldEditable,
{
    type Schema = TSchema;
    fn get_id(&self) -> &Uid {
        &self.id
    }

    fn get_slots(&self) -> &HashMap<Uid, RActiveSlot> {
        &self.slots
    }

    fn get_parent_slots(&self) -> RwSignal<Vec<SlotRef>> {
        self.parent_slots
    }

    fn get_operative(&self) -> &'static LibraryOperative<PrimitiveTypes, PrimitiveValues> {
        self.operative
    }

    fn get_template(&self) -> &'static LibraryTemplate<PrimitiveTypes, PrimitiveValues> {
        self.template
    }

    fn add_parent_slot(&self, slot_ref: SlotRef) -> &Self {
        self.parent_slots.update(|parent_slots| {
            parent_slots.push(slot_ref.clone());
        });
        self
    }

    fn remove_child_from_slot(&self, slot_ref: &SlotRef) -> &Self {
        self.slots
            .get(&slot_ref.slot_id)
            .unwrap()
            .slotted_instances
            .update(|slotted_instances| {
                slotted_instances.retain(|slotted_instance_id| {
                    *slotted_instance_id != slot_ref.child_instance_id
                });
            });
        self
    }

    fn remove_parent(&self, parent_id: &Uid, slot_id: Option<&Uid>) -> Vec<SlotRef> {
        let mut removed = Vec::new();
        self.parent_slots.update(|parent_slots| {
            parent_slots.retain(|slot_ref| {
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
        });
        removed
    }

    fn add_child_to_slot(&self, slot_ref: SlotRef) -> &Self {
        self.slots
            .get(&slot_ref.slot_id)
            .unwrap()
            .slotted_instances
            .update(|slotted_instances| {
                slotted_instances.push(slot_ref.child_instance_id);
            });
        self
    }
    fn get_graph(&self) -> Rc<RBaseGraphEnvironment<Self::Schema>> {
        self.graph.clone()
    }
}
#[derive(Clone, Debug)]
pub struct RGSOWrapperBuilder<T> {
    id: Uid,
    slots: HashMap<Uid, RActiveSlot>,
    parent_slots: RwSignal<Vec<SlotRef>>,
    pub data: HashMap<Uid, RwSignal<Option<RwSignal<PrimitiveValues>>>>,
    operative: &'static LibraryOperative<PrimitiveTypes, PrimitiveValues>,
    template: &'static LibraryTemplate<PrimitiveTypes, PrimitiveValues>,
    _phantom: PhantomData<T>,
}

impl<T: Clone + std::fmt::Debug> RGSOWrapperBuilder<T> {
    pub fn new(
        data: HashMap<Uid, RwSignal<Option<RwSignal<PrimitiveValues>>>>,
        slots: Option<HashMap<Uid, RActiveSlot>>,
        operative: &'static LibraryOperative<PrimitiveTypes, PrimitiveValues>,
        template: &'static LibraryTemplate<PrimitiveTypes, PrimitiveValues>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().as_u128(),
            slots: slots.unwrap_or_default(),
            parent_slots: RwSignal::new(Vec::new()),
            data,
            operative,
            template,
            _phantom: PhantomData,
        }
    }
    fn add_instance_to_slot(&self, slot_id: &Uid, instance_id: Uid) -> &Self {
        self.slots
            .get(slot_id)
            .unwrap()
            .slotted_instances
            .update(|slotted_instances| {
                slotted_instances.push(instance_id);
            });
        self
    }
    pub fn get_id(&self) -> &Uid {
        &self.id
    }
}
impl<T, TSchema: RGSO> RProducable<RGSOWrapper<T, TSchema>> for RGSOWrapperBuilder<T> {
    type Schema = TSchema;
    fn produce(&self, graph: Rc<RBaseGraphEnvironment<TSchema>>) -> RGSOWrapper<T, TSchema> {
        RGSOWrapper::<T, TSchema> {
            // history: None,
            id: self.id,
            slots: self.slots.clone(),
            parent_slots: self.parent_slots,
            graph,
            data: self
                .data
                .iter()
                .map(|(id, build_data)| (*id, build_data.get().unwrap()))
                .collect::<HashMap<Uid, RwSignal<PrimitiveValues>>>(),
            operative: self.operative,
            template: self.template,
            _phantom: PhantomData,
        }
    }
}

impl<F> Verifiable for RGSOWrapperBuilder<F> {
    fn verify(&self) -> Result<(), Error> {
        // self.data.verify()?;
        let field_errors = self
            .data
            .values()
            .filter_map(|field_val| {
                if field_val.with(|field_val| field_val.is_none()) {
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
        if !slot_errors.is_empty() {
            return Err(Error::new(ElementCreationError::BoundCheckOutOfRange));
        }
        if !field_errors.is_empty() {
            return Err(Error::new(ElementCreationError::RequiredFieldIsEmpty));
        }
        Ok(())
    }
}

// impl<T, TSchema: RGSO> Finalizable<RGSOWrapper<T, TSchema>> for RGSOWrapperBuilder<T> {}

pub trait RBuildable
where
    Self: Sized + 'static,
    RGSOWrapperBuilder<Self>: RInstantiable<Schema = Self::Schema>,
{
    type Schema: RGSO;

    fn initiate_build(
    ) -> RGSOBuilder<RGSOWrapperBuilder<Self>, RGSOWrapper<Self, Self::Schema>, Self::Schema>;
    fn get_operative_id() -> Uid;
}

pub trait RInstantiable: std::fmt::Debug + Any {
    type Schema: RGSO;

    fn instantiate(&self, graph: Rc<RBaseGraphEnvironment<Self::Schema>>) -> Self::Schema;
    fn get_id(&self) -> &Uid;
    fn get_template(&self) -> &'static LibraryTemplate<PrimitiveTypes, PrimitiveValues>;
}
type RInstantiableElements<TSchema> = Vec<Rc<dyn RInstantiable<Schema = TSchema>>>;

#[derive(Debug, Clone)]
pub struct RInstantiableWrapper<T, TSchema>
where
    T: RInstantiable<Schema = TSchema>,
{
    prereq_instantiables: RInstantiableElements<TSchema>,
    instantiable_instance: T,
    pub parent_updates: Vec<(Uid, SlotRef)>,
    child_updates: Vec<(Uid, SlotRef)>,
}

impl<T, TSchema> RInstantiableWrapper<T, TSchema>
where
    T: RInstantiable<Schema = TSchema> + 'static,
{
    pub fn flatten(mut self) -> RInstantiableElements<TSchema> {
        self.prereq_instantiables
            .push(Rc::new(self.instantiable_instance));
        self.prereq_instantiables
    }
    pub fn get_prereq_instantiables(&self) -> &RInstantiableElements<TSchema> {
        &self.prereq_instantiables
    }
    pub fn get_instantiable_instance(&self) -> &T {
        &self.instantiable_instance
    }
}
impl<T, TSchema: RGSO> RInstantiableWrapper<RGSOWrapperBuilder<T>, TSchema>
where
    RGSOWrapperBuilder<T>: RInstantiable<Schema = TSchema>,
{
    pub fn add_parent_slot(&mut self, parent_slot: SlotRef) {
        self.instantiable_instance
            .parent_slots
            .update(|parent_slots| {
                parent_slots.push(parent_slot);
            });
    }
}

#[derive(Default, Debug)]
pub struct RGSOBuilder<F, T, TSchema: 'static>
where
    F: RProducable<T>,
{
    instantiables: RwSignal<Vec<Rc<dyn RInstantiable<Schema = TSchema>>>>,
    child_updates: RwSignal<Vec<(Uid, SlotRef)>>,
    parent_updates: RwSignal<Vec<(Uid, SlotRef)>>,
    pub wip_instance: F,
    _phantom: PhantomData<T>,
}

impl<F, T, TSchema: RGSO> RGSOBuilder<F, T, TSchema>
where
    F: RProducable<T> + Verifiable + RInstantiable<Schema = TSchema> + Clone,
    // T: RInstantiable<Schema = TSchema> + 'static,
{
    pub fn build(
        &mut self,
        graph: &impl RGraphEnvironment<
            Types = PrimitiveTypes,
            Values = PrimitiveValues,
            Schema = TSchema,
        >,
    ) -> Result<RInstantiableWrapper<F, TSchema>, Error> {
        let mut error = None;
        self.parent_updates.with(|parent_updates| {
            parent_updates.iter().for_each(|parent_update| {
                let can_add_one = graph
                    .get(&parent_update.0)
                    .unwrap()
                    .get_slot_by_id(&parent_update.1.slot_id)
                    .unwrap()
                    .can_add_one();
                if !can_add_one {
                    error = Some(Err(Error::new(ElementCreationError::BoundCheckOutOfRange)));
                }
            })
        });
        if let Some(err) = error {
            return err;
        }
        self.wip_instance.verify()?;
        Ok(RInstantiableWrapper {
            child_updates: self.child_updates.get(),
            parent_updates: self.parent_updates.clone().get(),
            instantiable_instance: self.wip_instance.clone(),
            prereq_instantiables: self.instantiables.get(),
        })
    }
    pub fn new(builder_wrapper_instance: F) -> Self {
        Self {
            instantiables: RwSignal::new(vec![]),
            wip_instance: builder_wrapper_instance,
            child_updates: RwSignal::new(Vec::new()),
            parent_updates: RwSignal::new(Vec::new()),
            _phantom: PhantomData,
        }
    }
}

pub fn r_integrate_child<T, C, TSchema: RGSO>(
    builder: &mut RGSOBuilder<RGSOWrapperBuilder<T>, RGSOWrapper<T, TSchema>, TSchema>,
    mut child: RInstantiableWrapper<RGSOWrapperBuilder<C>, TSchema>,
    slot_id: Uid,
) -> &mut RGSOBuilder<RGSOWrapperBuilder<T>, RGSOWrapper<T, TSchema>, TSchema>
where
    T: Clone + std::fmt::Debug,
    RGSOWrapperBuilder<C>: RInstantiable<Schema = TSchema> + 'static,
{
    builder
        .wip_instance
        .add_instance_to_slot(&slot_id, child.get_instantiable_instance().id);
    let slot_ref = SlotRef {
        slot_id,
        child_instance_id: *child.get_instantiable_instance().get_id(),
        host_instance_id: builder.wip_instance.id,
    };
    child.add_parent_slot(slot_ref);
    builder.instantiables.update(|instantiables| {
        instantiables.extend(child.flatten());
    });
    builder
}

pub fn r_integrate_child_id<'a, T, TSchema: RGSO>(
    builder: &'a mut RGSOBuilder<RGSOWrapperBuilder<T>, RGSOWrapper<T, TSchema>, TSchema>,
    child_id: &Uid,
    slot_id: Uid,
) -> &'a mut RGSOBuilder<RGSOWrapperBuilder<T>, RGSOWrapper<T, TSchema>, TSchema>
where
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
    builder.child_updates.update(|child_updates| {
        child_updates.push((*child_id, slot_ref));
    });
    builder
}

// impl<T, TSchema: RGSO + 'static> RInstantiable for RGSOWrapper<T, TSchema>
// where
//     T: Clone + std::fmt::Debug + RIntoSchema<Schema = TSchema> + 'static,
//     RGSOWrapper<T, TSchema>: RFieldEditable,
// {
//     type Schema = TSchema;

//     fn instantiate(&self, history: RHistoryRef<TSchema>) -> Self::Schema {
//         let mut new_self = self.clone();
//         new_self.set_history(Some(history));
//         T::into_schema(self.clone())
//     }

//     fn get_instance_id(&self) -> &Uid {
//         self.get_id()
//     }
//     fn get_template(&self) -> &'static LibraryTemplate<PrimitiveTypes, PrimitiveValues> {
//         self.template
//     }
// }
impl<T, TSchema: RGSO + 'static> RInstantiable for RGSOWrapperBuilder<T>
where
    T: Clone + std::fmt::Debug + RIntoSchema<Schema = TSchema> + 'static,
    RGSOWrapper<T, TSchema>: RFieldEditable,
{
    type Schema = TSchema;

    fn instantiate(&self, graph: Rc<RBaseGraphEnvironment<Self::Schema>>) -> Self::Schema {
        // self.finalize();
        // let mut new_self = self.clone();
        // new_self.set_history(Some(history));
        T::into_schema(self.produce(graph))
    }

    fn get_id(&self) -> &Uid {
        &self.id
    }
    fn get_template(&self) -> &'static LibraryTemplate<PrimitiveTypes, PrimitiveValues> {
        self.template
    }
}

pub trait RIntoSchema
where
    Self: Sized,
{
    type Schema: RGSO;
    fn into_schema(instantiable: RGSOWrapper<Self, Self::Schema>) -> Self::Schema;
}
