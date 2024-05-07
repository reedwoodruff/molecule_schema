use anyhow::{Error, Result};
use leptos::logging::log;
use std::collections::HashSet;
use std::fmt;

use std::{cell::RefCell, collections::HashMap, marker::PhantomData, rc::Rc};

use base_types::constraint_schema::{
    LibraryOperative, LibraryTemplate, OperativeSlot, OperativeVariants, SlotBounds,
};
use base_types::constraint_schema_item::ConstraintSchemaItem;
use base_types::{
    common::{ConstraintTraits, Uid},
    constraint_schema::{
        ConstraintSchema,
        // LibraryOperative, LibraryTemplate, OperativeSlot, SlotBounds,
    },
    primitives::{PrimitiveTypes, PrimitiveValues},
};
// use RGSOBuilderModule::RGSOBuilder;

use base_types::traits::{
    ConnectionAction, ElementCreationError, ElementDeletionError, FieldEdit, Finalizable,
    HistoryFieldEdit, SlotRef, TaggedAction, Verifiable,
};
use base_types::utils::IntoPrimitiveValue;
use leptos::{
    batch, RwSignal, SignalGet, SignalSet, SignalUpdate, SignalWith, SignalWithUntracked,
};

pub trait RProducable<T> {
    type Schema: RGSO<Schema = Self::Schema>;
    fn produce(&self) -> T;
}

trait RFieldEditable {
    fn apply_field_edit(&self, field_edit: FieldEdit);
}
#[derive(Clone, Debug)]
pub struct RHistoryContainer<TSchema: EditRGSO<Schema = TSchema>> {
    pub undo: Vec<Blueprint<TSchema>>,
    pub redo: Vec<Blueprint<TSchema>>,
}

#[derive(Debug, Clone)]
pub struct RBaseGraphEnvironment<TSchema: EditRGSO<Schema = TSchema> + 'static> {
    pub created_instances: RwSignal<HashMap<Uid, TSchema>>,
    pub constraint_schema: &'static ConstraintSchema<PrimitiveTypes, PrimitiveValues>,
    pub history: Rc<RefCell<RHistoryContainer<TSchema>>>,
}
impl<TSchema: EditRGSO<Schema = TSchema> + 'static> RBaseGraphEnvironment<TSchema> {
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

impl<TSchema: EditRGSO<Schema = TSchema> + 'static> RBaseGraphEnvironment<TSchema> {
    fn process_blueprint(&self, blueprint: Blueprint<TSchema>) {
        let blueprint_clone = blueprint.clone();
        batch(|| {
            blueprint.added_instances.into_iter().for_each(|instance| {
                self.created_instances.update(|prev| {
                    prev.insert(*instance.get_id(), instance);
                });
            });
            blueprint
                .deleted_instances
                .into_iter()
                .for_each(|instance| {
                    self.created_instances.update(|prev| {
                        prev.remove(&instance.get_id());
                    });
                });
            blueprint
                .add_child_updates
                .into_iter()
                .for_each(|add_child| {
                    self.created_instances.with(|created_instances| {
                        created_instances
                            .get(&add_child.0)
                            .unwrap()
                            .add_child_to_slot(add_child.1);
                    });
                });
            blueprint
                .add_parent_updates
                .into_iter()
                .for_each(|add_parent| {
                    self.created_instances.with(|created_instances| {
                        created_instances
                            .get(&add_parent.0)
                            .unwrap()
                            .add_parent_slot(add_parent.1);
                    });
                });
            blueprint
                .remove_child_updates
                .into_iter()
                .for_each(|remove_child| {
                    self.created_instances.with(|created_instances| {
                        created_instances
                            .get(&remove_child.0)
                            .unwrap()
                            .remove_child_from_slot(&remove_child.1);
                    });
                });
            blueprint
                .remove_parent_updates
                .into_iter()
                .for_each(|remove_parent| {
                    self.created_instances.with(|created_instances| {
                        created_instances
                            .get(&remove_parent.0)
                            .unwrap()
                            .remove_parent(
                                &remove_parent.1.host_instance_id,
                                Some(&remove_parent.1.slot_id),
                            );
                    });
                });
        });
    }
    fn push_undo(&self, blueprint: Blueprint<TSchema>) {
        self.history.borrow_mut().undo.push(blueprint);
    }
    fn push_redo(&self, blueprint: Blueprint<TSchema>) {
        self.history.borrow_mut().redo.push(blueprint);
    }
    fn clear_redo(&self) {
        self.history.borrow_mut().redo.clear();
    }
    // fn push_history_item(&self, history_item: Vec<RHistoryItem<TSchema>>, tag: &TaggedAction) {
    //     match tag {
    //         TaggedAction::Normal => self.history.borrow_mut().undo.push(history_item),
    //         TaggedAction::Undo => self.history.borrow_mut().redo.push(history_item),
    //         TaggedAction::Redo => self.history.borrow_mut().undo.push(history_item),
    //     }
    // }
    // fn append_history_item(&self, history_item: RHistoryItem<TSchema>, tag: &TaggedAction) {
    //     match tag {
    //         TaggedAction::Normal => self
    //             .history
    //             .borrow_mut()
    //             .undo
    //             .last_mut()
    //             .unwrap()
    //             .push(history_item),
    //         TaggedAction::Undo => self
    //             .history
    //             .borrow_mut()
    //             .redo
    //             .last_mut()
    //             .unwrap()
    //             .push(history_item),
    //         TaggedAction::Redo => self
    //             .history
    //             .borrow_mut()
    //             .undo
    //             .last_mut()
    //             .unwrap()
    //             .push(history_item),
    //     }
    // }
    // fn check_and_delete_children_tagged(
    //     &mut self,
    //     id: &Uid,
    //     parent_id: Option<&Uid>,
    //     tag: &TaggedAction,
    // ) {
    //     let mut should_delete = parent_id.is_none();

    //     if let Some(parent_id) = parent_id {
    //         let child_parent_slots = self.get(id).unwrap().get_parent_slots();
    //         child_parent_slots.with_untracked(|child_parent_slots| {
    //             let remaining_parents = child_parent_slots
    //                 .iter()
    //                 .filter(|slot_ref| slot_ref.host_instance_id != *parent_id)
    //                 .collect::<Vec<_>>();
    //             if remaining_parents.is_empty() {
    //                 should_delete = true;
    //             }
    //         });
    //     }

    //     if !should_delete && parent_id.is_some() {
    //         let removed_slot_refs = self
    //             .get(id)
    //             .unwrap()
    //             .remove_parent(parent_id.unwrap(), None);
    //         removed_slot_refs.into_iter().for_each(|slot_ref| {
    //             self.append_history_item(RHistoryItem::RemoveParent(slot_ref), tag);
    //         })
    //     }

    //     if should_delete {
    //         self.get(id).unwrap().get_slots().values().for_each(|slot| {
    //             slot.slotted_instances.with_untracked(|slotted_instances| {
    //                 slotted_instances.iter().for_each(|slotted_instance_id| {
    //                     self.check_and_delete_children_tagged(slotted_instance_id, Some(id), tag);
    //                 });
    //             })
    //         });
    //     }
    //     let mut removed_value = None;
    //     self.created_instances
    //         .update(|prev| removed_value = prev.remove(id));
    //     let mut removed_value = removed_value.unwrap();
    //     self.append_history_item(RHistoryItem::Delete(removed_value), tag);
    // }
    // fn instantiate_element_tagged<T: std::fmt::Debug + Clone + 'static>(
    //     &self,
    //     element: RGSOBuilder<RGSOWrapperBuilder<T, TSchema>>,
    //     tag: &TaggedAction,
    // ) -> Result<Uid, Error>
    // where
    //     Self: Sized,
    //     T: RIntoSchema<Schema = TSchema>,
    // {
    //     for child_update in element.child_updates.iter() {
    //         let parent = element
    //             .prereq_instantiables
    //             .iter()
    //             .find(|prereq_inst| *prereq_inst.get_id() == child_update.1.host_instance_id);
    //         let operative_descriptor = if parent.is_none() {
    //             &element
    //                 .instantiable_instance
    //                 .get_template()
    //                 .operative_slots
    //                 .get(&child_update.1.slot_id)
    //                 .unwrap()
    //                 .operative_descriptor
    //         } else {
    //             &parent
    //                 .unwrap()
    //                 .get_template()
    //                 .operative_slots
    //                 .get(&child_update.1.slot_id)
    //                 .unwrap()
    //                 .operative_descriptor
    //         };
    //         match operative_descriptor {
    //             OperativeVariants::LibraryOperative(lib_op_id) => {
    //                 if *lib_op_id
    //                     != self
    //                         .get(&child_update.0)
    //                         .unwrap()
    //                         .get_operative()
    //                         .get_tag()
    //                         .id
    //                 {
    //                     return Err(Error::new(ElementCreationError::ChildElementIsWrongType));
    //                 };
    //             }
    //             OperativeVariants::TraitOperative(trait_op) => {
    //                 let child_digest = self
    //                     .get(&child_update.0)
    //                     .unwrap()
    //                     .get_operative()
    //                     .get_trait_impl_digest(self.constraint_schema);
    //                 let matches_trait_bounds = trait_op
    //                     .trait_ids
    //                     .iter()
    //                     .all(|trait_id| child_digest.trait_impls.contains_key(trait_id));
    //                 if !matches_trait_bounds {
    //                     return Err(Error::new(ElementCreationError::ChildElementIsWrongType));
    //                 }
    //             }
    //         }
    //     }
    //     let id = *element.get_instantiable_instance().get_id();
    //     self.push_history_item(vec![RHistoryItem::BlockActionMarker], &tag);

    //     element.child_updates.iter().for_each(|child_update| {
    //         let child = self.get(&child_update.0).unwrap();
    //         child.add_parent_slot(child_update.1.clone());
    //         self.append_history_item(RHistoryItem::AddParent(child_update.1.clone()), &tag);
    //     });
    //     element.parent_updates.iter().for_each(|parent_update| {
    //         let parent = self.get(&parent_update.0).unwrap();
    //         parent.add_child_to_slot(parent_update.1.clone());
    //         self.append_history_item(RHistoryItem::AddChild(parent_update.1.clone()), &tag);
    //     });
    //     element.flatten().into_iter().for_each(|instantiable| {
    //         let instantiated = instantiable.instantiate();
    //         self.append_history_item(RHistoryItem::Create(*instantiable.get_id()), &tag);
    //         self.created_instances.update(|prev| {
    //             prev.insert(*instantiable.get_id(), instantiated);
    //         });
    //     });
    //     Ok(id)
    // }
    // fn create_connection_tagged(
    //     &self,
    //     connection: ConnectionAction,
    //     tag: &TaggedAction,
    // ) -> Result<(), Error> {
    //     match &self
    //         .get(&connection.slot_ref.host_instance_id)
    //         .unwrap()
    //         .get_slot_by_id(&connection.slot_ref.slot_id)
    //         .unwrap()
    //         .slot
    //         .operative_descriptor
    //     {
    //         OperativeVariants::LibraryOperative(expected_id) => {
    //             if *expected_id
    //                 != self
    //                     .get(&connection.slot_ref.child_instance_id)
    //                     .unwrap()
    //                     .get_operative()
    //                     .get_tag()
    //                     .id
    //             {
    //                 return Err(Error::new(ElementCreationError::ChildElementIsWrongType));
    //             }
    //         }
    //         OperativeVariants::TraitOperative(trait_op) => {
    //             let child_digest = self
    //                 .get(&connection.slot_ref.child_instance_id)
    //                 .unwrap()
    //                 .get_operative()
    //                 .get_trait_impl_digest(&self.constraint_schema);
    //             let matches_trait_bounds = trait_op
    //                 .trait_ids
    //                 .iter()
    //                 .all(|trait_id| child_digest.trait_impls.contains_key(trait_id));
    //             if !matches_trait_bounds {
    //                 return Err(Error::new(ElementCreationError::ChildElementIsWrongType));
    //             }
    //         }
    //     }
    //     let parent = self.get(&connection.slot_ref.host_instance_id).unwrap();
    //     if parent
    //         .get_slot_by_id(&connection.slot_ref.slot_id)
    //         .unwrap()
    //         .can_add_one()
    //     {
    //         parent.add_child_to_slot(connection.slot_ref.clone());
    //     } else {
    //         return Err(Error::new(ElementCreationError::BoundCheckOutOfRange));
    //     }
    //     self.get(&connection.slot_ref.child_instance_id)
    //         .unwrap()
    //         .add_parent_slot(connection.slot_ref.clone());
    //     let history_item = vec![
    //         RHistoryItem::<TSchema>::BlockActionMarker,
    //         RHistoryItem::AddChild(connection.slot_ref.clone()),
    //         RHistoryItem::AddParent(connection.slot_ref.clone()),
    //     ];
    //     self.push_history_item(history_item, &tag);
    //     Ok(())
    // }
    // fn delete_tagged(&mut self, id: &Uid, tag: &TaggedAction) -> Result<(), Error> {
    //     let parent_slots = self.get(id).unwrap().get_parent_slots();

    //     let can_delete = parent_slots.with_untracked(|parent_slots| {
    //         parent_slots.iter().all(|parent_slot| {
    //             self.get(&parent_slot.host_instance_id)
    //                 .unwrap()
    //                 .get_slot_by_id(&parent_slot.slot_id)
    //                 .unwrap()
    //                 .can_remove_one()
    //         })
    //     });
    //     if !can_delete {
    //         return Err(Error::new(ElementDeletionError::RequiredByParentSlot));
    //     }
    //     self.push_history_item(vec![RHistoryItem::BlockActionMarker], &tag);
    //     parent_slots.with_untracked(|parent_slots| {
    //         parent_slots.iter().for_each(|parent_slot| {
    //             self.get(&parent_slot.host_instance_id)
    //                 .unwrap()
    //                 .remove_child_from_slot(parent_slot);
    //         })
    //     });

    //     self.check_and_delete_children_tagged(id, None, tag);

    //     Ok(())
    // }
    // fn process_history_tagged(&self, history: Vec<RHistoryItem<TSchema>>, tag: &TaggedAction) {
    //     if !history.is_empty() {
    //         self.push_history_item(vec![RHistoryItem::BlockActionMarker], tag);
    //     }
    //     history.into_iter().for_each(|action| match action {
    //         RHistoryItem::RemoveChildFromSlot(slot_ref) => {
    //             let host = self.get(&slot_ref.host_instance_id).unwrap();
    //             host.add_child_to_slot(slot_ref.clone());
    //             self.append_history_item(RHistoryItem::AddChild(slot_ref), tag);
    //         }
    //         RHistoryItem::RemoveParent(slot_ref) => {
    //             let child = self.get(&slot_ref.child_instance_id).unwrap();
    //             child.add_parent_slot(slot_ref.clone());
    //             self.append_history_item(RHistoryItem::AddParent(slot_ref), tag);
    //         }
    //         RHistoryItem::AddParent(slot_ref) => {
    //             let child = self.get(&slot_ref.child_instance_id).unwrap();
    //             child.remove_parent(&slot_ref.host_instance_id, Some(&slot_ref.slot_id));
    //             self.append_history_item(RHistoryItem::RemoveParent(slot_ref), tag);
    //         }
    //         RHistoryItem::AddChild(slot_ref) => {
    //             let host = self.get(&slot_ref.host_instance_id).unwrap();
    //             host.remove_child_from_slot(&slot_ref);
    //             self.append_history_item(RHistoryItem::RemoveChildFromSlot(slot_ref), tag);
    //         }
    //         RHistoryItem::Delete(mut deleted_node) => {
    //             let deleted_node_id = *deleted_node.get_id();
    //             self.created_instances.update(|prev| {
    //                 prev.insert(deleted_node_id.clone(), deleted_node);
    //             });
    //             self.append_history_item(RHistoryItem::Create(deleted_node_id), tag);
    //         }
    //         RHistoryItem::Create(node_id) => {
    //             let mut created_node = None;
    //             self.created_instances.update(|prev| {
    //                 created_node = prev.remove(&node_id);
    //             });
    //             let mut created_node = created_node.unwrap();
    //             self.append_history_item(RHistoryItem::Delete(created_node), tag);
    //         }
    //         RHistoryItem::EditField(field_edit) => {
    //             self.created_instances.update(|prev| {
    //                 prev.get_mut(&field_edit.instance_id)
    //                     .unwrap()
    //                     .apply_field_edit(FieldEdit {
    //                         field_id: field_edit.field_id.clone(),
    //                         value: field_edit.prev_value.clone(),
    //                     });
    //             });
    //             self.append_history_item(RHistoryItem::EditField(field_edit.reverse()), tag);
    //         }
    //         RHistoryItem::BlockActionMarker => {}
    //     })
    // }
}
impl<TSchema: EditRGSO<Schema = TSchema> + 'static> RGraphEnvironment
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
            .with_untracked(|created_instances| created_instances.get(id).cloned());
        test
    }
    // fn create_connection(&self, connection: ConnectionAction) -> Result<(), Error> {
    //     self.history.borrow_mut().redo.clear();
    //     self.create_connection_tagged(connection, &TaggedAction::Normal)
    // }
    // fn instantiate_element<T>(
    //     &self,
    //     element: RInstantiableWrapper<RGSOWrapperBuilder<T, TSchema>>,
    // ) -> Result<Uid, Error>
    // where
    //     Self: Sized,
    //     T: std::fmt::Debug + Clone + RIntoSchema<Schema = Self::Schema> + 'static,
    // {
    //     self.history.borrow_mut().redo.clear();
    //     self.instantiate_element_tagged(element, &TaggedAction::Normal)
    // }

    // fn delete(&mut self, id: &Uid) -> Result<(), Error> {
    //     self.history.borrow_mut().redo.clear();
    //     self.delete_tagged(id, &TaggedAction::Normal)
    // }

    fn undo(&self) {
        let undo_item = self.history.borrow_mut().undo.pop();
        if undo_item.is_none() {
            return;
        }
        let undo_item = undo_item.unwrap().reverse();

        self.process_blueprint(undo_item.clone());
        self.push_redo(undo_item)
    }

    fn redo(&self) {
        let redo_item = self.history.borrow_mut().redo.pop();
        if redo_item.is_none() {
            return;
        }
        let redo_item = redo_item.unwrap().reverse();
        self.process_blueprint(redo_item.clone());
        self.push_undo(redo_item);
    }
}

pub trait RGraphEnvironment {
    type Types: ConstraintTraits;
    type Values: ConstraintTraits;
    type Schema: RGSO<Schema = Self::Schema> + 'static;

    fn get(&self, id: &Uid) -> Option<Self::Schema>;
    // fn create_connection(&self, connection: ConnectionAction) -> Result<(), Error>;
    // fn instantiate_element<T>(
    //     &self,
    //     element: RInstantiableWrapper<RGSOWrapperBuilder<T, Self::Schema>>,
    // ) -> Result<Uid, Error>
    // where
    //     Self: Sized,
    //     T: std::fmt::Debug + Clone + RIntoSchema<Schema = Self::Schema> + 'static;
    fn get_constraint_schema(&self) -> &ConstraintSchema<Self::Types, Self::Values>;
    // fn delete(&mut self, id: &Uid) -> Result<(), Error>;
    fn undo(&self);
    fn redo(&self);
}

#[derive(Debug, Clone)]
pub enum RHistoryItem<TSchema: RGSO<Schema = TSchema>> {
    RemoveChildFromSlot(SlotRef),
    RemoveParent(SlotRef),
    AddParent(SlotRef),
    AddChild(SlotRef),
    Delete(TSchema),
    Create(Uid),
    EditField(HistoryFieldEdit),
    BlockActionMarker,
}
pub trait RGSO: std::fmt::Debug + Clone + RFieldEditable {
    type Schema: EditRGSO<Schema = Self::Schema>;
    /// Instance ID
    fn get_id(&self) -> &Uid;
    fn get_operative(&self) -> &'static LibraryOperative<PrimitiveTypes, PrimitiveValues>;
    fn get_template(&self) -> &'static LibraryTemplate<PrimitiveTypes, PrimitiveValues>;
    fn get_slot_by_id(&self, slot_id: &Uid) -> Option<&RActiveSlot> {
        self.get_slots().get(slot_id)
    }
    fn get_slots(&self) -> &HashMap<Uid, RActiveSlot>;
    fn get_parent_slots(&self) -> RwSignal<Vec<SlotRef>>;
}
trait EditRGSO: RGSO {
    fn add_parent_slot(&self, slot_ref: SlotRef) -> &Self;
    fn add_child_to_slot(&self, slot_ref: SlotRef) -> &Self;
    fn remove_child_from_slot(&self, slot_ref: &SlotRef) -> &Self;
    fn remove_parent(&self, parent_id: &Uid, slot_id: Option<&Uid>) -> Vec<SlotRef>;
    fn get_graph(&self) -> &Rc<RBaseGraphEnvironment<Self::Schema>>;
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
pub struct RGSOWrapper<T, TSchema: EditRGSO<Schema = TSchema> + 'static> {
    id: Uid,
    data: HashMap<Uid, RwSignal<PrimitiveValues>>,
    graph: Rc<RBaseGraphEnvironment<TSchema>>,
    slots: HashMap<Uid, RActiveSlot>,
    parent_slots: RwSignal<Vec<SlotRef>>,
    operative: &'static LibraryOperative<PrimitiveTypes, PrimitiveValues>,
    template: &'static LibraryTemplate<PrimitiveTypes, PrimitiveValues>,
    _phantom: PhantomData<T>,
}
impl<T: std::fmt::Debug, TSchema: EditRGSO<Schema = TSchema>> std::fmt::Debug
    for RGSOWrapper<T, TSchema>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GSOWrapper")
            .field("id", &self.id)
            .field("slots", &self.slots)
            .field("parent_slots", &self.parent_slots)
            .field("data", &self.data)
            .finish()
    }
}

// impl<T: Clone + std::fmt::Debug, TSchema: EditRGSO<Schema = TSchema>> RGSOWrapper<T, TSchema> {}
impl<T: Clone + std::fmt::Debug, TSchema: EditRGSO<Schema = TSchema>> RFieldEditable
    for RGSOWrapper<T, TSchema>
{
    fn apply_field_edit(&self, field_edit: FieldEdit) {
        self.data
            .get(&field_edit.field_id)
            .unwrap()
            .set(field_edit.value);
    }
}

impl<T: Clone + std::fmt::Debug, TSchema: EditRGSO<Schema = TSchema>> RGSO
    for RGSOWrapper<T, TSchema>
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
}

impl<T: Clone + std::fmt::Debug, TSchema: EditRGSO<Schema = TSchema>> EditRGSO
    for RGSOWrapper<T, TSchema>
where
    RGSOWrapper<T, TSchema>: RFieldEditable,
{
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
    fn get_graph(&self) -> &Rc<RBaseGraphEnvironment<Self::Schema>> {
        &self.graph
    }
}
#[derive(Clone, Debug)]
pub struct RGSOWrapperBuilder<T, TSchema: EditRGSO<Schema = TSchema> + 'static> {
    id: Uid,
    slots: HashMap<Uid, RActiveSlot>,
    parent_slots: RwSignal<Vec<SlotRef>>,
    pub data: HashMap<Uid, RwSignal<Option<RwSignal<PrimitiveValues>>>>,
    operative: &'static LibraryOperative<PrimitiveTypes, PrimitiveValues>,
    template: &'static LibraryTemplate<PrimitiveTypes, PrimitiveValues>,
    graph: Rc<RBaseGraphEnvironment<TSchema>>,
    _phantom: PhantomData<T>,
}

impl<T: Clone + std::fmt::Debug, TSchema: EditRGSO<Schema = TSchema>>
    RGSOWrapperBuilder<T, TSchema>
{
    pub fn new(
        data: HashMap<Uid, RwSignal<Option<RwSignal<PrimitiveValues>>>>,
        slots: Option<HashMap<Uid, RActiveSlot>>,
        operative: &'static LibraryOperative<PrimitiveTypes, PrimitiveValues>,
        template: &'static LibraryTemplate<PrimitiveTypes, PrimitiveValues>,
        graph: Rc<RBaseGraphEnvironment<TSchema>>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().as_u128(),
            slots: slots.unwrap_or_default(),
            parent_slots: RwSignal::new(Vec::new()),
            data,
            operative,
            template,
            graph,
            _phantom: PhantomData,
        }
    }
    // fn add_instance_to_slot(&self, slot_id: &Uid, instance_id: Uid) -> &Self {
    //     self.slots
    //         .get(slot_id)
    //         .unwrap()
    //         .slotted_instances
    //         .update(|slotted_instances| {
    //             slotted_instances.push(instance_id);
    //         });
    //     self
    // }
    pub fn get_id(&self) -> &Uid {
        &self.id
    }
}
impl<T, TSchema: EditRGSO<Schema = TSchema>> RProducable<RGSOWrapper<T, TSchema>>
    for RGSOWrapperBuilder<T, TSchema>
{
    type Schema = TSchema;
    fn produce(&self) -> RGSOWrapper<T, TSchema> {
        RGSOWrapper::<T, TSchema> {
            id: self.id,
            slots: self.slots.clone(),
            parent_slots: self.parent_slots,
            graph: self.graph.clone(),
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

impl<T, TSchema: EditRGSO<Schema = TSchema>> Verifiable for RGSOWrapperBuilder<T, TSchema> {
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

pub trait RBuildable: Clone + std::fmt::Debug
where
    Self: Sized + 'static,
    RGSOWrapperBuilder<Self, Self::Schema>: RInstantiable<Schema = Self::Schema>,
{
    type Schema: EditRGSO<Schema = Self::Schema>;

    fn initiate_build(
        graph: &Rc<RBaseGraphEnvironment<Self::Schema>>,
    ) -> RGSOBuilder<Self, Self::Schema>;
    fn initiate_edit(
        id: Uid,
        graph: &Rc<RBaseGraphEnvironment<Self::Schema>>,
    ) -> RGSOBuilder<Self, Self::Schema>;
    fn get_operative_id() -> Uid;
}

trait RInstantiable: std::fmt::Debug {
    type Schema: RGSO<Schema = Self::Schema>;

    fn instantiate(&self) -> Self::Schema;
    fn get_id(&self) -> &Uid;
    fn get_template(&self) -> &'static LibraryTemplate<PrimitiveTypes, PrimitiveValues>;
}
type RInstantiableElements<TSchema> = Vec<Rc<dyn RInstantiable<Schema = TSchema>>>;

// #[derive(Debug, Clone)]
// pub struct RInstantiableWrapper<T>
// where
//     T: RInstantiable,
// {
//     prereq_instantiables: RInstantiableElements<T::Schema>,
//     instantiable_instance: T,
//     pub parent_updates: Vec<(Uid, SlotRef)>,
//     child_updates: Vec<(Uid, SlotRef)>,
// }

// impl<T> RInstantiableWrapper<T>
// where
//     T: RInstantiable + 'static,
// {
//     pub fn flatten(mut self) -> RInstantiableElements<T::Schema> {
//         self.prereq_instantiables
//             .push(Rc::new(self.instantiable_instance));
//         self.prereq_instantiables
//     }
//     pub fn get_prereq_instantiables(&self) -> &RInstantiableElements<T::Schema> {
//         &self.prereq_instantiables
//     }
//     pub fn get_instantiable_instance(&self) -> &T {
//         &self.instantiable_instance
//     }
// }
// impl<T, TSchema: RGSO<Schema = TSchema>> RInstantiableWrapper<RGSOWrapperBuilder<T, TSchema>>
// where
//     RGSOWrapperBuilder<T, TSchema>: RInstantiable<Schema = TSchema>,
// {
//     pub fn add_parent_slot(&mut self, parent_slot: SlotRef) {
//         self.instantiable_instance
//             .parent_slots
//             .update(|parent_slots| {
//                 parent_slots.push(parent_slot);
//             });
//     }
// }

// mod RGSOBuilderModule {
//     use std::{marker::PhantomData, rc::Rc};

//     use anyhow::Error;
//     use base_types::{
//         common::Uid,
//         constraint_schema::LibraryTemplate,
//         primitives::{PrimitiveTypes, PrimitiveValues},
//         traits::{HistoryFieldEdit, SlotRef},
//     };
//     use leptos::{RwSignal, SignalGet, SignalSet, SignalUpdate, SignalWith, SignalWithUntracked};

//     use super::{
//         RBaseGraphEnvironment, RGSOWrapper, RGSOWrapperBuilder, RInstantiable, RIntoSchema,
//         RProducable, RGSO,
//     };
#[derive(Clone, Debug)]
pub struct Blueprint<TSchema: EditRGSO<Schema = TSchema>> {
    added_instances: Vec<TSchema>,
    deleted_instances: Vec<TSchema>,
    add_child_updates: HashSet<(Uid, SlotRef)>,
    remove_child_updates: HashSet<(Uid, SlotRef)>,
    add_parent_updates: HashSet<(Uid, SlotRef)>,
    remove_parent_updates: HashSet<(Uid, SlotRef)>,
    field_updates: HashSet<(Uid, HistoryFieldEdit)>,
    action_tag: Option<TaggedAction>,
}
impl<TSchema: EditRGSO<Schema = TSchema>> Blueprint<TSchema> {
    fn reverse(self) -> Self {
        Self {
            added_instances: self.deleted_instances,
            deleted_instances: self.added_instances,
            add_child_updates: self.remove_child_updates,
            remove_child_updates: self.add_child_updates,
            add_parent_updates: self.remove_parent_updates,
            remove_parent_updates: self.add_parent_updates,
            field_updates: self
                .field_updates
                .into_iter()
                .map(|(id, field_update)| (id, field_update.reverse()))
                .collect(),
            action_tag: self.action_tag,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RGSOBuilder<T, TSchema: EditRGSO<Schema = TSchema> + 'static> {
    instantiables: RwSignal<Vec<Rc<dyn RInstantiable<Schema = TSchema>>>>,
    add_child_updates: RwSignal<HashSet<(Uid, SlotRef)>>,
    add_parent_updates: RwSignal<HashSet<(Uid, SlotRef)>>,
    remove_child_updates: RwSignal<HashSet<(Uid, SlotRef)>>,
    remove_parent_updates: RwSignal<HashSet<(Uid, SlotRef)>>,
    deleted_instances: RwSignal<HashSet<Uid>>,
    to_delete_recursive: RwSignal<HashSet<Uid>>,
    field_updates: RwSignal<HashSet<(Uid, HistoryFieldEdit)>>,
    wip_instance: Option<RGSOWrapperBuilder<T, TSchema>>,
    id: Uid,
    graph: Rc<RBaseGraphEnvironment<TSchema>>,
    _phantom: PhantomData<T>,
}

impl<T, TSchema: EditRGSO<Schema = TSchema>> RGSOBuilder<T, TSchema>
where
    RGSOWrapperBuilder<T, TSchema>: RProducable<RGSOWrapper<T, TSchema>>,
    T: RIntoSchema<Schema = TSchema> + Clone + std::fmt::Debug + 'static,
{
    pub fn get_id(&self) -> &Uid {
        &self.id
    }
    pub fn execute(&self) -> Result<(), Error> {
        let graph = self.graph.clone();
        let blueprint = self.clone().get_blueprint()?;
        graph.clear_redo();
        graph.process_blueprint(blueprint);
        Ok(())
    }
    pub fn incorporate<C: std::fmt::Debug + Clone + RIntoSchema<Schema = TSchema> + 'static>(
        &mut self,
        other_builder: &RGSOBuilder<C, TSchema>,
    ) {
        self.add_child_updates
            .update(|child_updates| child_updates.extend(other_builder.add_child_updates.get()));
        self.add_parent_updates
            .update(|parent_updates| parent_updates.extend(other_builder.add_parent_updates.get()));
        self.remove_child_updates
            .update(|child_updates| child_updates.extend(other_builder.remove_child_updates.get()));
        self.remove_parent_updates.update(|parent_updates| {
            parent_updates.extend(other_builder.remove_parent_updates.get())
        });
        self.instantiables.update(|prev| {
            prev.extend(other_builder.instantiables.get());
            if let Some(inner) = other_builder.wip_instance.clone() {
                prev.push(Rc::new(inner));
            }
        });
    }
    fn delete_recursive_handler(&self, id: &Uid) {
        let item = self.graph.get(id).unwrap();
        let pending_parent_removals = self.remove_parent_updates.with(|remove_updates| {
            remove_updates
                .iter()
                .filter(|update| update.0 == *id)
                .collect::<HashSet<_>>()
                .len()
        });
        let pending_parent_additions = self.add_parent_updates.with(|add_updates| {
            add_updates
                .iter()
                .filter(|update| update.0 == *id)
                .collect::<HashSet<_>>()
                .len()
        });
        if item.get_parent_slots().with(|parent_slots| {
            parent_slots.len() + pending_parent_additions - pending_parent_removals == 0
        }) {
            let slotted_instances = item
                .get_slots()
                .values()
                .flat_map(|slot| slot.slotted_instances.get());
            slotted_instances.for_each(|instance_id| self.delete_recursive_handler(&instance_id));
        }
    }
    // Perform final calculations to gather all changes
    fn get_blueprint(mut self) -> Result<Blueprint<TSchema>, Error> {
        let mut new_instantiables = self.instantiables.get();
        if let Some(instance) = &self.wip_instance {
            new_instantiables.push(Rc::new(instance.clone()));
        }
        let to_delete = self.to_delete_recursive.get();
        to_delete.iter().for_each(|to_delete_id| {
            self.delete(to_delete_id);
            let item = self.graph.get(to_delete_id).unwrap();
            let slotted_instances = item
                .get_slots()
                .values()
                .flat_map(|slot| slot.slotted_instances.get());
            slotted_instances.for_each(|instance_id| self.delete_recursive_handler(&instance_id));
        });

        // Get rid of all changes on nodes that will be deleted
        // Also grab and clone the node about to be deleted to facilitate undoing
        let cloned_delete_instances = self.deleted_instances.with(|deleted_instances| {
            deleted_instances
                .iter()
                .map(|deleted_instance_id| {
                    self.add_child_updates
                        .update(|prev| prev.retain(|change| change.0 != *deleted_instance_id));
                    self.remove_child_updates
                        .update(|prev| prev.retain(|change| change.0 != *deleted_instance_id));
                    self.add_parent_updates
                        .update(|prev| prev.retain(|change| change.0 != *deleted_instance_id));
                    self.remove_parent_updates
                        .update(|prev| prev.retain(|change| change.0 != *deleted_instance_id));
                    self.field_updates
                        .update(|prev| prev.retain(|change| change.0 != *deleted_instance_id));
                    self.graph.created_instances.with(|created_instances| {
                        created_instances.get(&deleted_instance_id).unwrap().clone()
                    })
                })
                .collect::<Vec<_>>()
        });

        // Check slot bounds for conformity
        let mut already_checked = vec![];
        let bounds_checks = self.remove_child_updates.with(|remove_child_updates| {
            self.add_child_updates.with(|add_child_updates| {
                let errors = remove_child_updates
                    .iter()
                    .chain(add_child_updates)
                    .filter_map(|update| {
                        if already_checked.contains(&update.0) {
                            return None;
                        }
                        already_checked.push(update.0);
                        let all_removals = remove_child_updates
                            .iter()
                            .filter(|item| item.0 == update.0);
                        let all_additions =
                            add_child_updates.iter().filter(|item| item.0 == update.0);
                        let errors = self
                            .graph
                            .get(&update.0)
                            .unwrap()
                            .get_slots()
                            .iter()
                            .filter_map(|slot| {
                                let final_count = slot
                                    .1
                                    .slotted_instances
                                    .with(|slotted_instances| slotted_instances.len())
                                    + all_additions
                                        .clone()
                                        .filter(|addition| addition.1.slot_id == *slot.0)
                                        .collect::<Vec<_>>()
                                        .len()
                                    - all_removals
                                        .clone()
                                        .filter(|addition| addition.1.slot_id == *slot.0)
                                        .collect::<Vec<_>>()
                                        .len();
                                if slot.1.check_bound_conformity(final_count) == false {
                                    return Some(Error::new(
                                        ElementCreationError::BoundCheckOutOfRange,
                                    ));
                                } else {
                                    return None;
                                };
                            })
                            .collect::<Vec<_>>();
                        if errors.is_empty() {
                            return None;
                        } else {
                            return Some(errors);
                        }
                    })
                    .collect::<Vec<_>>();
                errors
            })
        });

        // TODO figure out how to return all errors
        if !bounds_checks.is_empty() {
            // return Err(Error::new(bounds_checks));
            return Err(Error::new(ElementCreationError::BoundCheckOutOfRange));
        }

        let mut instantiated_elements = new_instantiables
            .iter()
            .map(|el| el.instantiate())
            .collect::<Vec<_>>();
        // if instantiated_elements.any(|item| item.is_none()) {
        //     return Err(Error::new(ElementCreationError::BoundCheckOutOfRange));
        // }
        // let instantiated_elements = instantiated_elements
        //     .map(|item| item.unwrap())
        //     .collect::<Vec<_>>();

        Ok(Blueprint::<TSchema> {
            added_instances: instantiated_elements,
            add_child_updates: self.add_child_updates.get(),
            add_parent_updates: self.add_parent_updates.get(),
            remove_child_updates: self.remove_child_updates.get(),
            remove_parent_updates: self.remove_parent_updates.get(),
            deleted_instances: cloned_delete_instances,
            field_updates: self.field_updates.get(),
            action_tag: None,
        })
    }
    fn get_graph(&self) -> &Rc<RBaseGraphEnvironment<TSchema>> {
        &self.graph
    }
    fn new(
        builder_wrapper_instance: Option<RGSOWrapperBuilder<T, TSchema>>,
        id: Uid,
        graph: Rc<RBaseGraphEnvironment<TSchema>>,
    ) -> Self {
        Self {
            graph,
            instantiables: RwSignal::new(vec![]),
            wip_instance: builder_wrapper_instance,
            id,
            add_child_updates: RwSignal::new(HashSet::new()),
            add_parent_updates: RwSignal::new(HashSet::new()),
            remove_child_updates: RwSignal::new(HashSet::new()),
            remove_parent_updates: RwSignal::new(HashSet::new()),
            _phantom: PhantomData,
            field_updates: RwSignal::new(HashSet::new()),
            deleted_instances: RwSignal::new(HashSet::new()),
            to_delete_recursive: RwSignal::new(HashSet::new()),
        }
    }
    fn add_child_to_slot<C: std::fmt::Debug + Clone + RIntoSchema<Schema = TSchema> + 'static>(
        &mut self,
        slot_ref: SlotRef,
        instantiable: Option<RGSOBuilder<C, TSchema>>,
    ) {
        if let Some(instance) = &self.wip_instance {
            let slot = instance.slots.get(&slot_ref.slot_id).unwrap();
            slot.slotted_instances.update(|prev| {
                prev.push(slot_ref.child_instance_id);
            });
        } else {
            self.add_child_updates.update(|prev| {
                prev.insert((slot_ref.host_instance_id.clone(), slot_ref));
            });
        }
        if let Some(instantiable) = instantiable {
            self.incorporate(&instantiable);
        }
    }
    fn remove_child_from_slot(&mut self, slot_ref: SlotRef) {
        self.remove_child_updates.update(|prev| {
            prev.insert((slot_ref.host_instance_id, slot_ref.clone()));
        });
        self.remove_parent_updates.update(|prev| {
            prev.insert((slot_ref.child_instance_id, slot_ref));
        });
    }
    fn add_parent<C: std::fmt::Debug + Clone + RIntoSchema<Schema = TSchema> + 'static>(
        &mut self,
        slot_ref: SlotRef,
        instantiable: Option<RGSOBuilder<C, TSchema>>,
    ) {
        if let Some(instance) = &self.wip_instance {
            instance
                .parent_slots
                .update(|prev| prev.push(slot_ref.clone()))
        } else {
            self.add_parent_updates.update(|prev| {
                prev.insert((slot_ref.child_instance_id.clone(), slot_ref));
            });
        }
        if let Some(instantiable) = instantiable {
            self.incorporate(&instantiable);
        }
    }
    fn edit_field(&mut self, field_id: Uid, value: PrimitiveValues) {
        if let Some(instance) = &self.wip_instance {
            let signal = instance.data.get(&field_id).unwrap();
            let is_none = signal.with(|val| val.is_none());
            if is_none {
                signal.set(Some(RwSignal::new(value)));
            } else {
                signal.update(|prev| prev.unwrap().set(value))
            }
        } else {
            self.field_updates.update(|prev| {
                prev.insert((
                    *self.get_id(),
                    HistoryFieldEdit {
                        instance_id: *self.get_id(),
                        field_id: field_id,
                        new_value: value,
                        prev_value: PrimitiveValues::Bool(false),
                    },
                ));
            })
        }
    }
    fn delete(&mut self, to_delete_id: &Uid) {
        self.deleted_instances.update(|prev| {
            prev.insert(*to_delete_id);
        });
        let existing_instance = self.graph.get(&self.id).unwrap();
        existing_instance.get_parent_slots().with(|parent_slots| {
            parent_slots.iter().for_each(|parent_slot| {
                self.remove_child_updates.update(|removes| {
                    removes.insert((parent_slot.host_instance_id, parent_slot.clone()));
                })
            })
        });
        existing_instance.get_slots().values().for_each(|slot| {
            slot.slotted_instances.with(|slotted_instances| {
                slotted_instances.iter().for_each(|child_instance_id| {
                    self.remove_parent_updates.update(|removes| {
                        removes.insert((
                            *child_instance_id,
                            SlotRef {
                                host_instance_id: *to_delete_id,
                                child_instance_id: *child_instance_id,
                                slot_id: slot.slot.tag.id,
                            },
                        ));
                    })
                })
            })
        });
    }
    fn delete_recursive(&mut self) {
        self.to_delete_recursive.update(|prev| {
            prev.insert(self.id);
        });
    }
}
// impl<T, TSchema: RGSO<Schema = TSchema>> RInstantiable for RGSOBuilder<T, TSchema>
// where
//     T: std::fmt::Debug + Clone,
//     RGSOWrapperBuilder<T, TSchema>: RInstantiable<Schema = TSchema>,
// {
//     type Schema = TSchema;

//     fn instantiate(&self) -> Option<Self::Schema> {
//         self.wip_instance.clone()?.instantiate()
//     }

//     fn get_id(&self) -> &Uid {
//         &self.id
//     }

//     fn get_template(&self) -> &'static LibraryTemplate<PrimitiveTypes, PrimitiveValues> {
//         todo!()
//     }
// }

// }

// pub fn r_integrate_child<T, C, TSchema: RGSO<Schema = TSchema>>(
//     builder: &mut RGSOBuilder<T, TSchema>,
//     mut child: RInstantiableWrapper<RGSOWrapperBuilder<C, TSchema>>,
//     slot_id: Uid,
// ) -> &mut RGSOBuilder<T, TSchema>
// where
//     T: Clone + std::fmt::Debug,
//     RGSOWrapperBuilder<C, TSchema>: RInstantiable<Schema = TSchema> + 'static,
// {
//     builder
//         .wip_instance
//         .add_instance_to_slot(&slot_id, child.get_instantiable_instance().id);
//     let slot_ref = SlotRef {
//         slot_id,
//         child_instance_id: *child.get_instantiable_instance().get_id(),
//         host_instance_id: builder.wip_instance.id,
//     };
//     child.add_parent_slot(slot_ref);
//     builder.instantiables.update(|instantiables| {
//         instantiables.extend(child.flatten());
//     });
//     builder
// }

// pub fn r_integrate_child_id<'a, T, TSchema: RGSO<Schema = TSchema>>(
//     builder: &'a mut RGSOBuilder<T, TSchema>,
//     child_id: &Uid,
//     slot_id: Uid,
// ) -> &'a mut RGSOBuilder<T, TSchema>
// where
//     T: Clone + std::fmt::Debug,
// {
//     builder
//         .wip_instance
//         .add_instance_to_slot(&slot_id, *child_id);
//     let slot_ref = SlotRef {
//         slot_id,
//         child_instance_id: *child_id,
//         host_instance_id: builder.wip_instance.id,
//     };
//     builder.child_updates.update(|child_updates| {
//         child_updates.push((*child_id, slot_ref));
//     });
//     builder
// }

impl<T, TSchema: EditRGSO<Schema = TSchema> + 'static> RInstantiable
    for RGSOWrapperBuilder<T, TSchema>
where
    T: Clone + std::fmt::Debug + RIntoSchema<Schema = TSchema> + 'static,
    RGSOWrapper<T, TSchema>: RFieldEditable,
{
    type Schema = TSchema;

    fn instantiate(&self) -> Self::Schema {
        T::into_schema(self.produce())
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
    type Schema: EditRGSO<Schema = Self::Schema>;
    fn into_schema(instantiable: RGSOWrapper<Self, Self::Schema>) -> Self::Schema;
}

pub trait REditable<T>
where
    Self: Sized,
{
    type Schema: EditRGSO<Schema = Self::Schema>;
    fn initiate_edit(&self) -> RGSOBuilder<T, Self::Schema>;
}
impl<T, TSchema: EditRGSO<Schema = TSchema>> REditable<T> for RGSOWrapper<T, TSchema>
where
    T: Clone
        + std::fmt::Debug
        + RIntoSchema<Schema = TSchema>
        + RBuildable<Schema = TSchema>
        + 'static,
    RGSOWrapper<T, TSchema>: RFieldEditable,
{
    type Schema = TSchema;
    fn initiate_edit(&self) -> RGSOBuilder<T, Self::Schema> {
        T::initiate_edit(*self.get_id(), self.get_graph())
    }
}
