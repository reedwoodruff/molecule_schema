use anyhow::{Error, Result};
use leptos::logging::log;
use std::borrow::BorrowMut;
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BlueprintId {
    Existing(Uid),
    Temporary(String),
}
impl Into<BlueprintId> for Uid {
    fn into(self) -> BlueprintId {
        BlueprintId::Existing(self)
    }
}
impl Into<BlueprintId> for &Uid {
    fn into(self) -> BlueprintId {
        BlueprintId::Existing(self.clone())
    }
}

impl BlueprintId {
    fn new_temporary(name: &str) -> Self {
        BlueprintId::Temporary(name.to_string())
    }
}

trait RProducable<T> {
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
        log!("starting processing of blueprint");
        // let blueprint_clone = blueprint.clone();
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
                .add_outgoing_updates
                .into_iter()
                .for_each(|add_outgoing| {
                    self.created_instances.with(|created_instances| {
                        created_instances
                            .get(&add_outgoing.0)
                            .unwrap()
                            .add_outgoing(add_outgoing.1);
                    });
                });
            blueprint
                .add_incoming_updates
                .into_iter()
                .for_each(|add_incoming| {
                    self.created_instances.with(|created_instances| {
                        created_instances
                            .get(&add_incoming.0)
                            .unwrap()
                            .add_incoming(add_incoming.1);
                    });
                });
            blueprint
                .remove_outgoing_updates
                .into_iter()
                .for_each(|remove_outgoing| {
                    self.created_instances.with(|created_instances| {
                        created_instances
                            .get(&remove_outgoing.0)
                            .unwrap()
                            .remove_outgoing(&remove_outgoing.1);
                    });
                });
            blueprint
                .remove_incoming_updates
                .into_iter()
                .for_each(|remove_incoming| {
                    self.created_instances.with(|created_instances| {
                        created_instances
                            .get(&remove_incoming.0)
                            .unwrap()
                            .remove_incoming(
                                &remove_incoming.1.host_instance_id,
                                Some(&remove_incoming.1.slot_id),
                            );
                    });
                });
        });
        log!("finished processing of blueprint");
    }
    fn push_undo(&self, blueprint: Blueprint<TSchema>) {
        self.history.as_ref().borrow_mut().undo.push(blueprint);
    }
    fn push_redo(&self, blueprint: Blueprint<TSchema>) {
        self.history.as_ref().borrow_mut().redo.push(blueprint);
    }
    fn clear_redo(&self) {
        self.history.as_ref().borrow_mut().redo.clear();
    }
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

    fn undo(&self) {
        let undo_item = self.history.as_ref().borrow_mut().undo.pop();
        if undo_item.is_none() {
            return;
        }
        let undo_item = undo_item.unwrap().reverse();

        self.process_blueprint(undo_item.clone());
        self.push_redo(undo_item)
    }

    fn redo(&self) {
        let redo_item = self.history.as_ref().borrow_mut().redo.pop();
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
    fn get_constraint_schema(&self) -> &ConstraintSchema<Self::Types, Self::Values>;
    fn undo(&self);
    fn redo(&self);
}

pub trait RGSO: std::fmt::Debug + Clone {
    type Schema: EditRGSO<Schema = Self::Schema>;
    /// Instance ID
    fn get_id(&self) -> &Uid;
    fn operative(&self) -> &'static LibraryOperative<PrimitiveTypes, PrimitiveValues>;
    fn template(&self) -> &'static LibraryTemplate<PrimitiveTypes, PrimitiveValues>;
    fn slot_by_id<E: Into<Uid>>(&self, slot_id: E) -> Option<&RActiveSlot> {
        self.outgoing_slots().get(&slot_id.into())
    }
    fn outgoing_slots(&self) -> &HashMap<Uid, RActiveSlot>;
    fn incoming_slots(&self) -> RwSignal<Vec<SlotRef>>;
    fn incoming_slot_ids_by_id<E: Into<Uid>>(&self, slot_variant: E) -> Vec<SlotRef> {
        let slot_variant = &slot_variant.into();
        self.incoming_slots().with(|incoming_slots| {
            incoming_slots
                .iter()
                .filter(|slot| &slot.slot_id == slot_variant)
                .cloned()
                .collect::<Vec<_>>()
        })
    }
}
impl From<RActiveSlot> for Uid {
    fn from(value: RActiveSlot) -> Self {
        todo!()
    }
}
trait EditRGSO: RGSO + RFieldEditable {
    fn add_incoming(&self, slot_ref: SlotRef) -> &Self;
    fn add_outgoing(&self, slot_ref: SlotRef) -> &Self;
    fn remove_outgoing(&self, slot_ref: &SlotRef) -> &Self;
    fn remove_incoming(&self, parent_id: &Uid, slot_id: Option<&Uid>) -> Vec<SlotRef>;
    fn get_graph(&self) -> &Rc<RBaseGraphEnvironment<Self::Schema>>;
}

pub trait Slotted {}

#[derive(Clone)]
pub struct RActiveSlot {
    pub slot: &'static OperativeSlot,
    pub slotted_instances: RwSignal<Vec<Uid>>,
}

impl std::fmt::Debug for RActiveSlot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RActiveSlot")
            .field("slot", &self.slot.tag.name)
            .field("instances", &self.slotted_instances.get())
            .finish()
    }
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
    outgoing_slots: HashMap<Uid, RActiveSlot>,
    incoming_slots: RwSignal<Vec<SlotRef>>,
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
            .field(
                "slots",
                &self
                    .outgoing_slots
                    .iter()
                    .map(|slot| (slot.1.slot.tag.name.clone(), slot.1.slotted_instances.get()))
                    .collect::<Vec<_>>(),
            )
            .field("incoming_slots", &self.incoming_slots.get())
            .field(
                "data",
                &self
                    .data
                    .iter()
                    .map(|(field_id, data)| {
                        (
                            self.template
                                .field_constraints
                                .get(&field_id)
                                .unwrap()
                                .tag
                                .name
                                .clone(),
                            data.get(),
                        )
                    })
                    .collect::<Vec<_>>(),
            )
            .finish()
    }
}

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

    fn outgoing_slots(&self) -> &HashMap<Uid, RActiveSlot> {
        &self.outgoing_slots
    }

    fn incoming_slots(&self) -> RwSignal<Vec<SlotRef>> {
        self.incoming_slots
    }

    fn operative(&self) -> &'static LibraryOperative<PrimitiveTypes, PrimitiveValues> {
        self.operative
    }

    fn template(&self) -> &'static LibraryTemplate<PrimitiveTypes, PrimitiveValues> {
        self.template
    }
}

impl<T: Clone + std::fmt::Debug, TSchema: EditRGSO<Schema = TSchema>> EditRGSO
    for RGSOWrapper<T, TSchema>
where
    RGSOWrapper<T, TSchema>: RFieldEditable,
{
    fn add_incoming(&self, slot_ref: SlotRef) -> &Self {
        self.incoming_slots.update(|incoming_slots| {
            incoming_slots.push(slot_ref.clone());
        });
        self
    }

    fn remove_outgoing(&self, slot_ref: &SlotRef) -> &Self {
        self.outgoing_slots
            .get(&slot_ref.slot_id)
            .unwrap()
            .slotted_instances
            .update(|slotted_instances| {
                slotted_instances.retain(|slotted_instance_id| {
                    *slotted_instance_id != slot_ref.target_instance_id
                });
            });
        self
    }

    fn remove_incoming(&self, host_id: &Uid, slot_id: Option<&Uid>) -> Vec<SlotRef> {
        let mut removed = Vec::new();
        self.incoming_slots.update(|incoming_slots| {
            incoming_slots.retain(|slot_ref| {
                let matches_host = slot_ref.host_instance_id == *host_id;
                let matches_slot_id = if let Some(given_slot_id) = slot_id {
                    slot_ref.slot_id == *given_slot_id
                } else {
                    true
                };
                if matches_host && matches_slot_id {
                    removed.push(slot_ref.clone());
                    return false;
                } else {
                    return true;
                }
            });
        });
        removed
    }

    fn add_outgoing(&self, slot_ref: SlotRef) -> &Self {
        self.outgoing_slots
            .get(&slot_ref.slot_id)
            .unwrap()
            .slotted_instances
            .update(|slotted_instances| {
                slotted_instances.push(slot_ref.target_instance_id);
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
    incoming_slots: RwSignal<Vec<SlotRef>>,
    pub data: HashMap<Uid, RwSignal<Option<RwSignal<PrimitiveValues>>>>,
    operative: &'static LibraryOperative<PrimitiveTypes, PrimitiveValues>,
    template: &'static LibraryTemplate<PrimitiveTypes, PrimitiveValues>,
    graph: Rc<RBaseGraphEnvironment<TSchema>>,
    temp_id: String,
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
            incoming_slots: RwSignal::new(Vec::new()),
            data,
            operative,
            template,
            graph,
            _phantom: PhantomData,
            temp_id: uuid::Uuid::new_v4().to_string(),
        }
    }
    pub fn get_id(&self) -> &Uid {
        &self.id
    }
    pub fn set_temp_id(&mut self, id: &str) -> &mut Self {
        self.temp_id = id.to_string();
        self
    }
    fn get_temp_id(&self) -> &String {
        &self.temp_id
    }
}
impl<T, TSchema: EditRGSO<Schema = TSchema>> RProducable<RGSOWrapper<T, TSchema>>
    for RGSOWrapperBuilder<T, TSchema>
{
    type Schema = TSchema;
    fn produce(&self) -> RGSOWrapper<T, TSchema> {
        RGSOWrapper::<T, TSchema> {
            id: self.id,
            outgoing_slots: self.slots.clone(),
            incoming_slots: self.incoming_slots,
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
    fn get_temp_id(&self) -> &String;
    fn get_template(&self) -> &'static LibraryTemplate<PrimitiveTypes, PrimitiveValues>;
    fn add_incoming(&mut self, host_id: &Uid, slot_id: &Uid);
    fn add_outgoing(&mut self, target_id: &Uid, slot_id: &Uid);
}
type RInstantiableElements<TSchema> = Vec<Rc<dyn RInstantiable<Schema = TSchema>>>;

#[derive(Clone, Debug)]
pub struct Blueprint<TSchema: EditRGSO<Schema = TSchema>> {
    added_instances: Vec<TSchema>,
    deleted_instances: Vec<TSchema>,
    add_outgoing_updates: HashSet<(Uid, SlotRef)>,
    remove_outgoing_updates: HashSet<(Uid, SlotRef)>,
    add_incoming_updates: HashSet<(Uid, SlotRef)>,
    remove_incoming_updates: HashSet<(Uid, SlotRef)>,
    field_updates: HashSet<(Uid, HistoryFieldEdit)>,
    action_tag: Option<TaggedAction>,
}
impl<TSchema: EditRGSO<Schema = TSchema>> Blueprint<TSchema> {
    fn reverse(self) -> Self {
        Self {
            added_instances: self.deleted_instances,
            deleted_instances: self.added_instances,
            add_outgoing_updates: self.remove_outgoing_updates,
            remove_outgoing_updates: self.add_outgoing_updates,
            add_incoming_updates: self.remove_incoming_updates,
            remove_incoming_updates: self.add_incoming_updates,
            field_updates: self
                .field_updates
                .into_iter()
                .map(|(id, field_update)| (id, field_update.reverse()))
                .collect(),
            action_tag: self.action_tag,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct TempAddIncomingSlotRef {
    pub host_instance_id: BlueprintId,
    pub slot_id: Uid,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct TempAddOutgoingSlotRef {
    pub target_instance_id: BlueprintId,
    pub slot_id: Uid,
}

pub struct ExecutionResult {
    temp_id_map: HashMap<String, Uid>,
}
impl ExecutionResult {
    fn get_final_id(&self, temp_id: &str) -> Option<&Uid> {
        self.temp_id_map.get(temp_id)
    }
}

#[derive(Debug, Clone)]
pub struct RGSOBuilder<T, TSchema: EditRGSO<Schema = TSchema> + 'static> {
    instantiables: RwSignal<Vec<Rc<RefCell<dyn RInstantiable<Schema = TSchema>>>>>,
    add_outgoing_updates: RwSignal<HashSet<(Uid, SlotRef)>>,
    add_incoming_updates: RwSignal<HashSet<(Uid, SlotRef)>>,
    remove_outgoing_updates: RwSignal<HashSet<(Uid, SlotRef)>>,
    remove_incoming_updates: RwSignal<HashSet<(Uid, SlotRef)>>,
    deleted_instances: RwSignal<HashSet<Uid>>,
    to_delete_recursive: RwSignal<HashSet<Uid>>,
    field_updates: RwSignal<HashSet<(Uid, HistoryFieldEdit)>>,
    temp_add_incoming_updates: RwSignal<HashSet<(BlueprintId, TempAddIncomingSlotRef)>>,
    temp_add_outgoing_updates: RwSignal<HashSet<(BlueprintId, TempAddOutgoingSlotRef)>>,
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
    pub fn execute(&self) -> Result<ExecutionResult, Error> {
        let graph = self.graph.clone();
        let (blueprint, execution_result) = self.clone().get_blueprint()?;
        graph.clear_redo();
        graph.push_undo(blueprint.clone());
        graph.process_blueprint(blueprint);
        Ok(execution_result)
    }
    pub fn incorporate<C: std::fmt::Debug + Clone + RIntoSchema<Schema = TSchema> + 'static>(
        &mut self,
        other_builder: &RGSOBuilder<C, TSchema>,
    ) {
        self.add_outgoing_updates.update(|outgoing_updates| {
            outgoing_updates.extend(other_builder.add_outgoing_updates.get())
        });
        self.add_incoming_updates.update(|incoming_updates| {
            incoming_updates.extend(other_builder.add_incoming_updates.get())
        });
        self.remove_outgoing_updates.update(|outgoing_updates| {
            outgoing_updates.extend(other_builder.remove_outgoing_updates.get())
        });
        self.remove_incoming_updates.update(|incoming_updates| {
            incoming_updates.extend(other_builder.remove_incoming_updates.get())
        });
        self.instantiables.update(|prev| {
            prev.extend(other_builder.instantiables.get());
            if let Some(inner) = other_builder.wip_instance.clone() {
                prev.push(Rc::new(RefCell::new(inner)));
            }
        });
        self.deleted_instances.update(|prev| {
            prev.extend(other_builder.deleted_instances.get());
        });
        self.temp_add_incoming_updates.update(|prev| {
            prev.extend(other_builder.temp_add_incoming_updates.get());
        });
        self.temp_add_outgoing_updates.update(|prev| {
            prev.extend(other_builder.temp_add_outgoing_updates.get());
        });
        self.to_delete_recursive.update(|prev| {
            prev.extend(other_builder.to_delete_recursive.get());
        });
        self.field_updates.update(|prev| {
            prev.extend(other_builder.field_updates.get());
        });
    }
    pub fn set_temp_id(&mut self, temp_id: &str) -> &mut Self {
        if let Some(wip_instance) = &mut self.wip_instance {
            wip_instance.set_temp_id(temp_id);
        }
        self
    }
    fn delete_recursive_handler(&self, id: &Uid) {
        let item = self.graph.get(id).unwrap();
        let pending_incoming_removals = self.remove_incoming_updates.with(|remove_updates| {
            remove_updates
                .iter()
                .filter(|update| update.0 == *id)
                .collect::<HashSet<_>>()
                .len()
        });
        let pending_incoming_additions = self.add_incoming_updates.with(|add_updates| {
            add_updates
                .iter()
                .filter(|update| update.0 == *id)
                .collect::<HashSet<_>>()
                .len()
        });
        if item.incoming_slots().with(|incoming_slots| {
            incoming_slots.len() + pending_incoming_additions - pending_incoming_removals == 0
        }) {
            let slotted_instances = item
                .outgoing_slots()
                .values()
                .flat_map(|slot| slot.slotted_instances.get());
            slotted_instances.for_each(|instance_id| self.delete_recursive_handler(&instance_id));
        }
    }
    // Perform final calculations to gather all changes
    fn get_blueprint(mut self) -> Result<(Blueprint<TSchema>, ExecutionResult), Error> {
        let mut new_instantiables = self.instantiables.get();
        if let Some(instance) = &self.wip_instance {
            new_instantiables.push(Rc::new(RefCell::new(instance.clone())));
        }

        let temp_id_map = new_instantiables
            .iter()
            .map(|instantiable| {
                (
                    instantiable.borrow().get_temp_id().clone(),
                    instantiable.borrow().get_id().clone(),
                )
            })
            .collect::<HashMap<_, _>>();

        // Perform any incoming or outgoing updates for temporary ids
        self.temp_add_incoming_updates.with(|updates| {
            updates.iter().for_each(|update| {
                let final_host_id = match &update.1.host_instance_id {
                    BlueprintId::Existing(existing_id) => *existing_id,
                    BlueprintId::Temporary(temp_id) => temp_id_map.get(temp_id).unwrap().clone(),
                };
                match &update.0 {
                    BlueprintId::Existing(existing_id) => {
                        self.add_incoming_updates.update(|prev| {
                            prev.insert((
                                existing_id.clone(),
                                SlotRef {
                                    target_instance_id: existing_id.clone(),
                                    host_instance_id: final_host_id,
                                    slot_id: update.1.slot_id,
                                },
                            ));
                        });
                    }
                    BlueprintId::Temporary(temp_id) => {
                        if let Some(instantiable) = new_instantiables
                            .iter_mut()
                            .find(|instantiable| instantiable.borrow().get_temp_id() == temp_id)
                        {
                            instantiable
                                .as_ref()
                                .borrow_mut()
                                .add_incoming(&final_host_id, &update.1.slot_id);
                        }
                    }
                };
            });
        });
        self.temp_add_outgoing_updates.with(|updates| {
            updates.iter().for_each(|update| {
                let final_target_id = match &update.1.target_instance_id {
                    BlueprintId::Existing(existing_id) => *existing_id,
                    BlueprintId::Temporary(temp_id) => temp_id_map.get(temp_id).unwrap().clone(),
                };
                match &update.0 {
                    BlueprintId::Existing(existing_id) => {
                        self.add_outgoing_updates.update(|prev| {
                            prev.insert((
                                existing_id.clone(),
                                SlotRef {
                                    target_instance_id: final_target_id,
                                    host_instance_id: existing_id.clone(),
                                    slot_id: update.1.slot_id,
                                },
                            ));
                        });
                    }
                    BlueprintId::Temporary(temp_id) => {
                        if let Some(instantiable) = new_instantiables
                            .iter_mut()
                            .find(|instantiable| instantiable.borrow().get_temp_id() == temp_id)
                        {
                            instantiable
                                .as_ref()
                                .borrow_mut()
                                .add_outgoing(&final_target_id, &update.1.slot_id);
                        }
                    }
                };
            });
        });

        let to_delete = self.to_delete_recursive.get();
        to_delete.iter().for_each(|to_delete_id| {
            self.delete(to_delete_id);
            let item = self.graph.get(to_delete_id).unwrap();
            let slotted_instances = item
                .outgoing_slots()
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
                    self.add_outgoing_updates
                        .update(|prev| prev.retain(|change| change.0 != *deleted_instance_id));
                    self.remove_outgoing_updates
                        .update(|prev| prev.retain(|change| change.0 != *deleted_instance_id));
                    self.add_incoming_updates
                        .update(|prev| prev.retain(|change| change.0 != *deleted_instance_id));
                    self.remove_incoming_updates
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
        let bounds_checks = self
            .remove_outgoing_updates
            .with(|remove_outgoing_updates| {
                self.add_outgoing_updates.with(|add_outgoing_updates| {
                    let errors = remove_outgoing_updates
                        .iter()
                        .chain(add_outgoing_updates)
                        .filter_map(|update| {
                            if already_checked.contains(&update.0) {
                                return None;
                            }
                            already_checked.push(update.0);

                            let all_removals = remove_outgoing_updates
                                .iter()
                                .filter(|item| item.0 == update.0);
                            let all_additions = add_outgoing_updates
                                .iter()
                                .filter(|item| item.0 == update.0);
                            let errors = self
                                .graph
                                .get(&update.0)
                                .unwrap()
                                .outgoing_slots()
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
            .map(|el| el.borrow().instantiate())
            .collect::<Vec<_>>();

        Ok((
            Blueprint::<TSchema> {
                added_instances: instantiated_elements,
                add_outgoing_updates: self.add_outgoing_updates.get(),
                add_incoming_updates: self.add_incoming_updates.get(),
                remove_outgoing_updates: self.remove_outgoing_updates.get(),
                remove_incoming_updates: self.remove_incoming_updates.get(),
                deleted_instances: cloned_delete_instances,
                field_updates: self.field_updates.get(),
                action_tag: None,
            },
            ExecutionResult {
                temp_id_map: temp_id_map,
            },
        ))
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
            add_outgoing_updates: RwSignal::new(HashSet::new()),
            add_incoming_updates: RwSignal::new(HashSet::new()),
            remove_outgoing_updates: RwSignal::new(HashSet::new()),
            remove_incoming_updates: RwSignal::new(HashSet::new()),
            temp_add_incoming_updates: RwSignal::new(HashSet::new()),
            temp_add_outgoing_updates: RwSignal::new(HashSet::new()),
            _phantom: PhantomData,
            field_updates: RwSignal::new(HashSet::new()),
            deleted_instances: RwSignal::new(HashSet::new()),
            to_delete_recursive: RwSignal::new(HashSet::new()),
        }
    }
    fn add_outgoing<C: std::fmt::Debug + Clone + RIntoSchema<Schema = TSchema> + 'static>(
        &mut self,
        slot_id: &Uid,
        // BlueprintId in this case meaning that:
        //    Existing: The ID is known
        //    Temporary: The ID is non known, only the temp_id
        target_id: BlueprintId,
        instantiable: Option<RGSOBuilder<C, TSchema>>,
    ) {
        // If this is a newly created instance
        if let Some(instance) = &self.wip_instance {
            match &target_id {
                BlueprintId::Existing(existing_target_id) => {
                    let slot = instance.slots.get(&slot_id).unwrap();
                    slot.slotted_instances.update(|prev| {
                        prev.push(existing_target_id.clone());
                    });
                }
                BlueprintId::Temporary(temp_target_id) => {
                    self.temp_add_outgoing(
                        BlueprintId::Temporary(instance.get_temp_id().clone()),
                        TempAddOutgoingSlotRef {
                            target_instance_id: target_id.clone(),
                            slot_id: slot_id.clone(),
                        },
                    );
                }
            }
        // If this is an existing element being edited
        } else {
            match &target_id {
                BlueprintId::Existing(existing_target_id) => {
                    self.add_outgoing_updates.update(|prev| {
                        prev.insert((
                            self.id.clone(),
                            SlotRef {
                                host_instance_id: self.id.clone(),
                                target_instance_id: existing_target_id.clone(),
                                slot_id: slot_id.clone(),
                            },
                        ));
                    });
                }
                BlueprintId::Temporary(temp_target_id) => self.temp_add_outgoing(
                    BlueprintId::Existing(self.get_id().clone()),
                    TempAddOutgoingSlotRef {
                        target_instance_id: target_id,
                        slot_id: slot_id.clone(),
                    },
                ),
            }
        }
        if let Some(instantiable) = instantiable {
            self.incorporate(&instantiable);
        }
    }
    fn remove_outgoing(&mut self, slot_ref: SlotRef) {
        self.remove_outgoing_updates.update(|prev| {
            prev.insert((slot_ref.host_instance_id, slot_ref.clone()));
        });
        self.remove_incoming_updates.update(|prev| {
            prev.insert((slot_ref.target_instance_id, slot_ref));
        });
    }
    fn add_incoming<C: std::fmt::Debug + Clone + RIntoSchema<Schema = TSchema> + 'static>(
        &mut self,
        slot_ref: SlotRef,
        instantiable: Option<RGSOBuilder<C, TSchema>>,
    ) {
        if let Some(instance) = &self.wip_instance {
            instance
                .incoming_slots
                .update(|prev| prev.push(slot_ref.clone()))
        } else {
            self.add_incoming_updates.update(|prev| {
                prev.insert((slot_ref.target_instance_id.clone(), slot_ref));
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
        existing_instance.incoming_slots().with(|incoming_slots| {
            incoming_slots.iter().for_each(|incoming_slot| {
                self.remove_outgoing_updates.update(|removes| {
                    removes.insert((incoming_slot.host_instance_id, incoming_slot.clone()));
                })
            })
        });
        existing_instance
            .outgoing_slots()
            .values()
            .for_each(|slot| {
                slot.slotted_instances.with(|slotted_instances| {
                    slotted_instances.iter().for_each(|target_instance_id| {
                        self.remove_incoming_updates.update(|removes| {
                            removes.insert((
                                *target_instance_id,
                                SlotRef {
                                    host_instance_id: *to_delete_id,
                                    target_instance_id: *target_instance_id,
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
    fn temp_add_incoming(&mut self, host_id: BlueprintId, temp_slot_ref: TempAddIncomingSlotRef) {
        self.temp_add_incoming_updates
            .update(|temp_add_incoming_updates| {
                temp_add_incoming_updates.insert((host_id, temp_slot_ref));
            });
    }
    fn temp_add_outgoing(&mut self, target_id: BlueprintId, temp_slot_ref: TempAddOutgoingSlotRef) {
        self.temp_add_outgoing_updates
            .update(|temp_add_outgoing_updates| {
                temp_add_outgoing_updates.insert((target_id, temp_slot_ref));
            });
    }
}

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

    fn get_temp_id(&self) -> &String {
        &self.temp_id
    }

    fn add_incoming(&mut self, host_id: &Uid, slot_id: &Uid) {
        self.incoming_slots.update(|incoming_slots| {
            incoming_slots.push(SlotRef {
                host_instance_id: host_id.clone(),
                slot_id: slot_id.clone(),
                target_instance_id: self.id,
            })
        });
    }

    fn add_outgoing(&mut self, target_id: &Uid, slot_id: &Uid) {
        self.slots
            .get(slot_id)
            .unwrap()
            .slotted_instances
            .update(|slotted_instances| slotted_instances.push(target_id.clone()));
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
