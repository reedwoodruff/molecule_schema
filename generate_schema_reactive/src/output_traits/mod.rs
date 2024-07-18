// // pub mod from_reactive;

// pub use base_types::common::Uid;
// use base_types::constraint_schema::{LibraryOperative, LibraryTemplate, OperativeSlot, SlotBounds};

// use base_types::{
//     common::ConstraintTraits,
//     constraint_schema::ConstraintSchema,
//     primitives::{PrimitiveTypes, PrimitiveValues},
// };

// use base_types::post_generation::{
//     ElementCreationError, FieldEdit, HistoryFieldEdit, SlotRef, TaggedAction, Verifiable,
// };
// use base_types::utils::IntoPrimitiveValue;
// use leptos::{
//     batch, RwSignal, SignalGet, SignalSet, SignalUpdate, SignalWith, SignalWithUntracked,
// };

// pub trait FromNonReactive<NTSchema>
// where
//     Self: EditRGSO<Schema = Self>,
// {
//     fn from_non_reactive(value: NTSchema, graph: std::rc::Rc<RBaseGraphEnvironment<Self>>) -> Self;
// }
// fn saturate_wrapper<T: Clone + std::fmt::Debug, RTSchema: EditRGSO<Schema = RTSchema>>(
//     non_reactive: base_types::post_generation::GSOWrapper<T>,
//     graph: std::rc::Rc<RBaseGraphEnvironment<RTSchema>>,
// ) -> RGSOWrapper<T, RTSchema> {
//     RGSOWrapper::<T, RTSchema> {
//         id: non_reactive.id,
//         graph,
//         fields: non_reactive
//             .fields
//             .into_iter()
//             .map(|(id, val)| (id, RwSignal::new(val)))
//             .collect(),
//         outgoing_slots: non_reactive
//             .outgoing_slots
//             .into_iter()
//             .map(|(id, val)| (id, val.into()))
//             .collect(),
//         incoming_slots: RwSignal::new(non_reactive.incoming_slots.into_iter().collect()),
//         operative: non_reactive.operative,
//         template: non_reactive.template,
//         _phantom: std::marker::PhantomData,
//     }
// }

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub enum BlueprintId {
//     Existing(Uid),
//     Temporary(String),
// }
// // impl From<BlueprintId> for Uid {
// //     fn from(value: BlueprintId) -> Self {
// //         todo!()
// //     }
// // }
// impl From<Uid> for BlueprintId {
//     fn from(value: Uid) -> Self {
//         BlueprintId::Existing(value)
//     }
// }
// impl From<&Uid> for BlueprintId {
//     fn from(value: &Uid) -> Self {
//         BlueprintId::Existing(*value)
//     }
// }
// impl From<String> for BlueprintId {
//     fn from(value: String) -> Self {
//         BlueprintId::Temporary(value)
//     }
// }
// impl From<&str> for BlueprintId {
//     fn from(value: &str) -> Self {
//         BlueprintId::Temporary(value.to_string())
//     }
// }
// // impl Into<BlueprintId> for Uid {
// //     fn into(self) -> BlueprintId {
// //         BlueprintId::Existing(self)
// //     }
// // }
// // impl Into<BlueprintId> for &Uid {
// //     fn into(self) -> BlueprintId {
// //         BlueprintId::Existing(self.clone())
// //     }
// // }
// // impl Into<BlueprintId> for &str {
// //     fn into(self) -> BlueprintId {
// //         BlueprintId::Temporary(self.to_string())
// //     }
// // }
// // impl Into<BlueprintId> for String {
// //     fn into(self) -> BlueprintId {
// //         BlueprintId::Temporary(self)
// //     }
// // }

// impl BlueprintId {
//     fn new_temporary(name: &str) -> Self {
//         BlueprintId::Temporary(name.to_string())
//     }
// }

// trait RProducable<T> {
//     type Schema: RGSO<Schema = Self::Schema>;
//     fn produce(&self) -> T;
// }

// trait RFieldEditable {
//     fn apply_field_edit(&self, field_edit: FieldEdit);
// }
// #[derive(Clone, Debug)]
// pub struct RHistoryContainer<TSchema: EditRGSO<Schema = TSchema>> {
//     pub undo: Vec<Blueprint<TSchema>>,
//     pub redo: Vec<Blueprint<TSchema>>,
// }

// #[derive(Debug, Clone)]
// pub struct RBaseGraphEnvironment<TSchema: EditRGSO<Schema = TSchema> + 'static> {
//     pub created_instances: RwSignal<std::collections::HashMap<Uid, TSchema>>,
//     pub constraint_schema: &'static ConstraintSchema<PrimitiveTypes, PrimitiveValues>,
//     pub history: std::rc::Rc<std::cell::RefCell<RHistoryContainer<TSchema>>>,
// }
// impl<TSchema: EditRGSO<Schema = TSchema> + 'static> RBaseGraphEnvironment<TSchema> {
//     pub fn new(
//         constraint_schema: &'static ConstraintSchema<PrimitiveTypes, PrimitiveValues>,
//     ) -> Self {
//         Self {
//             created_instances: RwSignal::new(std::collections::HashMap::new()),
//             constraint_schema,
//             history: std::rc::Rc::new(std::cell::RefCell::new(RHistoryContainer {
//                 undo: Vec::new(),
//                 redo: Vec::new(),
//             })),
//         }
//     }
//     fn initialize(&self, created_instances: std::collections::HashMap<Uid, TSchema>) {
//         self.created_instances.set(created_instances);
//     }
// }

// impl<TSchema: EditRGSO<Schema = TSchema> + 'static> RBaseGraphEnvironment<TSchema> {
//     fn process_blueprint(&self, blueprint: Blueprint<TSchema>) {
//         leptos::logging::log!("starting processing of blueprint");
//         batch(|| {
//             blueprint.added_instances.into_iter().for_each(|instance| {
//                 self.created_instances.update(|prev| {
//                     prev.insert(*instance.get_id(), instance);
//                 });
//             });
//             blueprint
//                 .deleted_instances
//                 .into_iter()
//                 .for_each(|instance| {
//                     self.created_instances.update(|prev| {
//                         prev.remove(instance.get_id());
//                     });
//                 });
//             blueprint
//                 .add_outgoing_updates
//                 .into_iter()
//                 .for_each(|add_outgoing| {
//                     self.created_instances.with(|created_instances| {
//                         created_instances
//                             .get(&add_outgoing.0)
//                             .unwrap()
//                             .add_outgoing(add_outgoing.1);
//                     });
//                 });
//             blueprint
//                 .add_incoming_updates
//                 .into_iter()
//                 .for_each(|add_incoming| {
//                     self.created_instances.with(|created_instances| {
//                         created_instances
//                             .get(&add_incoming.0)
//                             .unwrap()
//                             .add_incoming(add_incoming.1);
//                     });
//                 });
//             blueprint
//                 .remove_outgoing_updates
//                 .into_iter()
//                 .for_each(|remove_outgoing| {
//                     self.created_instances.with(|created_instances| {
//                         created_instances
//                             .get(&remove_outgoing.0)
//                             .unwrap()
//                             .remove_outgoing(&remove_outgoing.1);
//                     });
//                 });
//             blueprint
//                 .remove_incoming_updates
//                 .into_iter()
//                 .for_each(|remove_incoming| {
//                     self.created_instances.with(|created_instances| {
//                         created_instances
//                             .get(&remove_incoming.0)
//                             .unwrap()
//                             .remove_incoming(
//                                 &remove_incoming.1.host_instance_id,
//                                 Some(&remove_incoming.1.slot_id),
//                             );
//                     });
//                 });
//         });
//         leptos::logging::log!("finished processing of blueprint");
//     }
//     fn push_undo(&self, blueprint: Blueprint<TSchema>) {
//         self.history.as_ref().borrow_mut().undo.push(blueprint);
//     }
//     fn push_redo(&self, blueprint: Blueprint<TSchema>) {
//         self.history.as_ref().borrow_mut().redo.push(blueprint);
//     }
//     fn clear_redo(&self) {
//         self.history.as_ref().borrow_mut().redo.clear();
//     }
// }
// impl<TSchema: EditRGSO<Schema = TSchema> + 'static> RGraphEnvironment
//     for RBaseGraphEnvironment<TSchema>
// {
//     type Schema = TSchema;
//     type Types = PrimitiveTypes;
//     type Values = PrimitiveValues;

//     fn get_constraint_schema(&self) -> &ConstraintSchema<Self::Types, Self::Values> {
//         self.constraint_schema
//     }

//     fn get(&self, id: &Uid) -> Option<Self::Schema> {
//         let test = self
//             .created_instances
//             .with_untracked(|created_instances| created_instances.get(id).cloned());
//         test
//     }

//     fn undo(&self) {
//         let undo_item = self.history.as_ref().borrow_mut().undo.pop();
//         if undo_item.is_none() {
//             return;
//         }
//         let undo_item = undo_item.unwrap().reverse();

//         self.process_blueprint(undo_item.clone());
//         self.push_redo(undo_item)
//     }

//     fn redo(&self) {
//         let redo_item = self.history.as_ref().borrow_mut().redo.pop();
//         if redo_item.is_none() {
//             return;
//         }
//         let redo_item = redo_item.unwrap().reverse();
//         self.process_blueprint(redo_item.clone());
//         self.push_undo(redo_item);
//     }
// }

// pub trait RGraphEnvironment {
//     type Types: ConstraintTraits;
//     type Values: ConstraintTraits;
//     type Schema: RGSO<Schema = Self::Schema> + 'static;

//     fn get(&self, id: &Uid) -> Option<Self::Schema>;
//     fn get_constraint_schema(&self) -> &ConstraintSchema<Self::Types, Self::Values>;
//     fn undo(&self);
//     fn redo(&self);
// }

// /// Reactive Generated Schema Object
// pub trait RGSO: std::fmt::Debug + Clone {
//     type Schema: EditRGSO<Schema = Self::Schema>;
//     /// Instance ID
//     fn get_id(&self) -> &Uid;
//     fn operative(&self) -> &'static LibraryOperative<PrimitiveTypes, PrimitiveValues>;
//     fn template(&self) -> &'static LibraryTemplate<PrimitiveTypes, PrimitiveValues>;
//     fn slot_by_id<E: Into<Uid>>(&self, slot_id: E) -> Option<&RActiveSlot> {
//         self.outgoing_slots().get(&slot_id.into())
//     }
//     fn outgoing_slots(&self) -> &std::collections::HashMap<Uid, RActiveSlot>;
//     fn incoming_slots(&self) -> RwSignal<Vec<SlotRef>>;
//     fn incoming_slot_ids_by_id<E: Into<Uid>>(&self, slot_variant: E) -> Vec<SlotRef> {
//         let slot_variant = &slot_variant.into();
//         self.incoming_slots().with(|incoming_slots| {
//             incoming_slots
//                 .iter()
//                 .filter(|slot| &slot.slot_id == slot_variant)
//                 .cloned()
//                 .collect::<Vec<_>>()
//         })
//     }
//     fn fields(&self) -> &std::collections::HashMap<Uid, RwSignal<PrimitiveValues>>;
// }
// impl From<RActiveSlot> for Uid {
//     fn from(value: RActiveSlot) -> Self {
//         todo!()
//     }
// }
// trait EditRGSO: RGSO + RFieldEditable {
//     fn add_incoming(&self, slot_ref: SlotRef) -> &Self;
//     fn add_outgoing(&self, slot_ref: SlotRef) -> &Self;
//     fn remove_outgoing(&self, slot_ref: &SlotRef) -> &Self;
//     fn remove_incoming(&self, parent_id: &Uid, slot_id: Option<&Uid>) -> Vec<SlotRef>;
//     fn get_graph(&self) -> &std::rc::Rc<RBaseGraphEnvironment<Self::Schema>>;
// }

// pub trait Slotted {}

// #[derive(Clone)]
// pub struct RActiveSlot {
//     pub slot: &'static OperativeSlot,
//     pub slotted_instances: RwSignal<Vec<Uid>>,
// }

// impl std::fmt::Debug for RActiveSlot {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("RActiveSlot")
//             .field("slot", &self.slot.tag.name)
//             .field("instances", &self.slotted_instances.get())
//             .finish()
//     }
// }

// impl RActiveSlot {
//     pub fn check_current_conformity(&self) -> bool {
//         let len = self
//             .slotted_instances
//             .with(|slotted_instances| slotted_instances.len());
//         self.check_bound_conformity(len)
//     }
//     pub fn can_remove_one(&self) -> bool {
//         let len = self
//             .slotted_instances
//             .with(|slotted_instances| slotted_instances.len())
//             - 1;
//         self.check_bound_conformity(len)
//     }
//     pub fn can_add_one(&self) -> bool {
//         let len = self
//             .slotted_instances
//             .with(|slotted_instances| slotted_instances.len())
//             + 1;
//         self.check_bound_conformity(len)
//     }
//     fn check_bound_conformity(&self, len: usize) -> bool {
//         match &self.slot.bounds {
//             SlotBounds::Single => len == 1,
//             SlotBounds::LowerBound(lower_bound) => lower_bound <= &len,
//             SlotBounds::UpperBound(upper_bound) => upper_bound >= &len,
//             SlotBounds::Range(lower_range, upper_range) => {
//                 lower_range <= &len && &len <= upper_range
//             }
//             SlotBounds::LowerBoundOrZero(lower_bound) => len == 0 || lower_bound <= &len,
//             SlotBounds::RangeOrZero(lower_range, upper_range) => {
//                 len == 0 || (lower_range <= &len && &len <= upper_range)
//             }
//         }
//     }
// }

// #[derive(Clone)]
// pub struct RGSOWrapper<T, TSchema: EditRGSO<Schema = TSchema> + 'static> {
//     id: Uid,
//     fields: std::collections::HashMap<Uid, RwSignal<PrimitiveValues>>,
//     graph: std::rc::Rc<RBaseGraphEnvironment<TSchema>>,
//     outgoing_slots: std::collections::HashMap<Uid, RActiveSlot>,
//     incoming_slots: RwSignal<Vec<SlotRef>>,
//     operative: &'static LibraryOperative<PrimitiveTypes, PrimitiveValues>,
//     template: &'static LibraryTemplate<PrimitiveTypes, PrimitiveValues>,
//     _phantom: std::marker::PhantomData<T>,
// }
// impl<T, TSchema: EditRGSO<Schema = TSchema>> PartialEq for RGSOWrapper<T, TSchema> {
//     fn eq(&self, other: &Self) -> bool {
//         self.id == other.id
//     }
// }
// impl<T: std::fmt::Debug, TSchema: EditRGSO<Schema = TSchema>> std::fmt::Debug
//     for RGSOWrapper<T, TSchema>
// {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("GSOWrapper")
//             .field("id", &self.id)
//             .field(
//                 "slots",
//                 &self
//                     .outgoing_slots
//                     .iter()
//                     .map(|slot| (slot.1.slot.tag.name.clone(), slot.1.slotted_instances.get()))
//                     .collect::<Vec<_>>(),
//             )
//             .field("incoming_slots", &self.incoming_slots.get())
//             .field(
//                 "data",
//                 &self
//                     .fields
//                     .iter()
//                     .map(|(field_id, data)| {
//                         (
//                             self.template
//                                 .field_constraints
//                                 .get(field_id)
//                                 .unwrap()
//                                 .tag
//                                 .name
//                                 .clone(),
//                             data.get(),
//                         )
//                     })
//                     .collect::<Vec<_>>(),
//             )
//             .finish()
//     }
// }

// impl<T: Clone + std::fmt::Debug, TSchema: EditRGSO<Schema = TSchema>> RFieldEditable
//     for RGSOWrapper<T, TSchema>
// {
//     fn apply_field_edit(&self, field_edit: FieldEdit) {
//         self.fields
//             .get(&field_edit.field_id)
//             .unwrap()
//             .set(field_edit.value);
//     }
// }

// impl<T: Clone + std::fmt::Debug, TSchema: EditRGSO<Schema = TSchema>> RGSO
//     for RGSOWrapper<T, TSchema>
// where
//     RGSOWrapper<T, TSchema>: RFieldEditable,
// {
//     type Schema = TSchema;
//     fn get_id(&self) -> &Uid {
//         &self.id
//     }

//     fn outgoing_slots(&self) -> &std::collections::HashMap<Uid, RActiveSlot> {
//         &self.outgoing_slots
//     }

//     fn incoming_slots(&self) -> RwSignal<Vec<SlotRef>> {
//         self.incoming_slots
//     }

//     fn operative(&self) -> &'static LibraryOperative<PrimitiveTypes, PrimitiveValues> {
//         self.operative
//     }

//     fn template(&self) -> &'static LibraryTemplate<PrimitiveTypes, PrimitiveValues> {
//         self.template
//     }
//     fn fields(&self) -> &std::collections::HashMap<Uid, RwSignal<PrimitiveValues>> {
//         &self.fields
//     }
// }

// impl<T: Clone + std::fmt::Debug, TSchema: EditRGSO<Schema = TSchema>> EditRGSO
//     for RGSOWrapper<T, TSchema>
// where
//     RGSOWrapper<T, TSchema>: RFieldEditable,
// {
//     fn add_incoming(&self, slot_ref: SlotRef) -> &Self {
//         self.incoming_slots.update(|incoming_slots| {
//             incoming_slots.push(slot_ref.clone());
//         });
//         self
//     }

//     fn remove_outgoing(&self, slot_ref: &SlotRef) -> &Self {
//         self.outgoing_slots
//             .get(&slot_ref.slot_id)
//             .unwrap()
//             .slotted_instances
//             .update(|slotted_instances| {
//                 slotted_instances.retain(|slotted_instance_id| {
//                     *slotted_instance_id != slot_ref.target_instance_id
//                 });
//             });
//         self
//     }

//     fn remove_incoming(&self, host_id: &Uid, slot_id: Option<&Uid>) -> Vec<SlotRef> {
//         let mut removed = Vec::new();
//         self.incoming_slots.update(|incoming_slots| {
//             incoming_slots.retain(|slot_ref| {
//                 let matches_host = slot_ref.host_instance_id == *host_id;
//                 let matches_slot_id = if let Some(given_slot_id) = slot_id {
//                     slot_ref.slot_id == *given_slot_id
//                 } else {
//                     true
//                 };
//                 if matches_host && matches_slot_id {
//                     removed.push(slot_ref.clone());
//                     false
//                 } else {
//                     true
//                 }
//             });
//         });
//         removed
//     }

//     fn add_outgoing(&self, slot_ref: SlotRef) -> &Self {
//         self.outgoing_slots
//             .get(&slot_ref.slot_id)
//             .unwrap()
//             .slotted_instances
//             .update(|slotted_instances| {
//                 slotted_instances.push(slot_ref.target_instance_id);
//             });
//         self
//     }
//     fn get_graph(&self) -> &std::rc::Rc<RBaseGraphEnvironment<Self::Schema>> {
//         &self.graph
//     }
// }
// #[derive(Clone, Debug)]
// pub struct RGSOWrapperBuilder<T, TSchema: EditRGSO<Schema = TSchema> + 'static> {
//     id: Uid,
//     slots: std::collections::HashMap<Uid, RActiveSlot>,
//     incoming_slots: RwSignal<Vec<SlotRef>>,
//     pub data: std::collections::HashMap<Uid, RwSignal<Option<RwSignal<PrimitiveValues>>>>,
//     operative: &'static LibraryOperative<PrimitiveTypes, PrimitiveValues>,
//     template: &'static LibraryTemplate<PrimitiveTypes, PrimitiveValues>,
//     graph: std::rc::Rc<RBaseGraphEnvironment<TSchema>>,
//     temp_id: String,
//     _phantom: std::marker::PhantomData<T>,
// }

// impl<T: Clone + std::fmt::Debug, TSchema: EditRGSO<Schema = TSchema>>
//     RGSOWrapperBuilder<T, TSchema>
// {
//     pub fn new(
//         data: std::collections::HashMap<Uid, RwSignal<Option<RwSignal<PrimitiveValues>>>>,
//         slots: Option<std::collections::HashMap<Uid, RActiveSlot>>,
//         operative: &'static LibraryOperative<PrimitiveTypes, PrimitiveValues>,
//         template: &'static LibraryTemplate<PrimitiveTypes, PrimitiveValues>,
//         graph: std::rc::Rc<RBaseGraphEnvironment<TSchema>>,
//     ) -> Self {
//         Self {
//             id: uuid::Uuid::new_v4().as_u128(),
//             slots: slots.unwrap_or_default(),
//             incoming_slots: RwSignal::new(Vec::new()),
//             data,
//             operative,
//             template,
//             graph,
//             _phantom: std::marker::PhantomData,
//             temp_id: uuid::Uuid::new_v4().to_string(),
//         }
//     }
//     pub fn get_id(&self) -> &Uid {
//         &self.id
//     }
//     pub fn set_temp_id(&mut self, id: &str) -> &mut Self {
//         self.temp_id = id.to_string();
//         self
//     }
//     fn get_temp_id(&self) -> &String {
//         &self.temp_id
//     }
// }
// impl<T, TSchema: EditRGSO<Schema = TSchema>> RProducable<RGSOWrapper<T, TSchema>>
//     for RGSOWrapperBuilder<T, TSchema>
// {
//     type Schema = TSchema;
//     fn produce(&self) -> RGSOWrapper<T, TSchema> {
//         RGSOWrapper::<T, TSchema> {
//             id: self.id,
//             outgoing_slots: self.slots.clone(),
//             incoming_slots: self.incoming_slots,
//             graph: self.graph.clone(),
//             fields: self
//                 .data
//                 .iter()
//                 .map(|(id, build_data)| (*id, build_data.get().unwrap()))
//                 .collect::<std::collections::HashMap<Uid, RwSignal<PrimitiveValues>>>(),
//             operative: self.operative,
//             template: self.template,
//             _phantom: std::marker::PhantomData,
//         }
//     }
// }

// impl<T, TSchema: EditRGSO<Schema = TSchema>> Verifiable for RGSOWrapperBuilder<T, TSchema> {
//     fn verify(&self) -> Result<(), base_types::post_generation::ElementCreationError> {
//         let field_errors = self
//             .data
//             .values()
//             .filter_map(|field_val| {
//                 if field_val.with(|field_val| field_val.is_none()) {
//                     return Some(ElementCreationError::RequiredFieldIsEmpty);
//                 }
//                 None
//             })
//             .collect::<Vec<_>>();
//         let slot_errors = self
//             .slots
//             .values()
//             .filter_map(|active_slot| {
//                 if !active_slot.check_current_conformity() {
//                     Some(ElementCreationError::BoundCheckOutOfRange(format!(
//                         "{}: {}\nBounds: {:?}, Attempted: {}",
//                         self.operative.tag.name,
//                         active_slot.slot.tag.name,
//                         active_slot.slot.bounds,
//                         active_slot.slotted_instances.get().len()
//                     )))
//                 } else {
//                     None
//                 }
//             })
//             .collect::<Vec<_>>();
//         let mut all_errors = Vec::new();
//         all_errors.extend(field_errors);
//         all_errors.extend(slot_errors);
//         if !all_errors.is_empty() {
//             leptos::logging::log!("{:#?}", all_errors);
//             return Err(ElementCreationError::Stack(all_errors));
//         }
//         Ok(())
//     }
// }

// pub trait RBuildable: Clone + std::fmt::Debug
// where
//     Self: Sized + 'static,
//     RGSOWrapperBuilder<Self, Self::Schema>: RInstantiable<Schema = Self::Schema>,
// {
//     type Schema: EditRGSO<Schema = Self::Schema>;

//     fn initiate_build(
//         graph: impl Into<std::rc::Rc<RBaseGraphEnvironment<Self::Schema>>>,
//     ) -> RGSOBuilder<Self, Self::Schema>;
//     fn initiate_edit(
//         id: Uid,
//         graph: impl Into<std::rc::Rc<RBaseGraphEnvironment<Self::Schema>>>,
//     ) -> RGSOBuilder<Self, Self::Schema>;
//     fn get_operative_id() -> Uid;
// }

// trait RInstantiable: std::fmt::Debug {
//     type Schema: RGSO<Schema = Self::Schema>;

//     fn instantiate(
//         &self,
//     ) -> Result<Self::Schema, base_types::post_generation::ElementCreationError>;
//     fn get_id(&self) -> &Uid;
//     fn get_temp_id(&self) -> &String;
//     fn get_template(&self) -> &'static LibraryTemplate<PrimitiveTypes, PrimitiveValues>;
//     fn add_incoming(&mut self, host_id: &Uid, slot_id: &Uid);
//     fn add_outgoing(&mut self, target_id: &Uid, slot_id: &Uid);
// }
// type RInstantiableElements<TSchema> = Vec<std::rc::Rc<dyn RInstantiable<Schema = TSchema>>>;

// #[derive(Clone, Debug)]
// pub struct Blueprint<TSchema: EditRGSO<Schema = TSchema>> {
//     added_instances: Vec<TSchema>,
//     deleted_instances: Vec<TSchema>,
//     add_outgoing_updates: std::collections::HashSet<(Uid, SlotRef)>,
//     remove_outgoing_updates: std::collections::HashSet<(Uid, SlotRef)>,
//     add_incoming_updates: std::collections::HashSet<(Uid, SlotRef)>,
//     remove_incoming_updates: std::collections::HashSet<(Uid, SlotRef)>,
//     field_updates: std::collections::HashSet<(Uid, HistoryFieldEdit)>,
//     action_tag: Option<TaggedAction>,
// }
// impl<TSchema: EditRGSO<Schema = TSchema>> Blueprint<TSchema> {
//     fn reverse(self) -> Self {
//         Self {
//             added_instances: self.deleted_instances,
//             deleted_instances: self.added_instances,
//             add_outgoing_updates: self.remove_outgoing_updates,
//             remove_outgoing_updates: self.add_outgoing_updates,
//             add_incoming_updates: self.remove_incoming_updates,
//             remove_incoming_updates: self.add_incoming_updates,
//             field_updates: self
//                 .field_updates
//                 .into_iter()
//                 .map(|(id, field_update)| (id, field_update.reverse()))
//                 .collect(),
//             action_tag: self.action_tag,
//         }
//     }
// }

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// struct TempAddIncomingSlotRef {
//     pub host_instance_id: BlueprintId,
//     pub slot_id: Uid,
// }
// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// struct TempAddOutgoingSlotRef {
//     pub target_instance_id: BlueprintId,
//     pub slot_id: Uid,
// }

// pub struct ExecutionResult {
//     pub temp_id_map: std::collections::HashMap<String, Uid>,
// }
// impl ExecutionResult {
//     pub fn get_final_id(&self, temp_id: &str) -> Option<&Uid> {
//         self.temp_id_map.get(temp_id)
//     }
// }

// #[derive(Debug, Clone)]
// pub struct RGSOBuilder<T, TSchema: EditRGSO<Schema = TSchema> + 'static> {
//     instantiables:
//         RwSignal<Vec<std::rc::Rc<std::cell::RefCell<dyn RInstantiable<Schema = TSchema>>>>>,
//     cumulative_errors: RwSignal<std::vec::Vec<ElementCreationError>>,
//     add_outgoing_updates: RwSignal<std::collections::HashSet<(Uid, SlotRef)>>,
//     add_incoming_updates: RwSignal<std::collections::HashSet<(Uid, SlotRef)>>,
//     remove_outgoing_updates: RwSignal<std::collections::HashSet<(Uid, SlotRef)>>,
//     remove_incoming_updates: RwSignal<std::collections::HashSet<(Uid, SlotRef)>>,
//     deleted_instances: RwSignal<std::collections::HashSet<Uid>>,
//     to_delete_recursive: RwSignal<std::collections::HashSet<Uid>>,
//     field_updates: RwSignal<std::collections::HashSet<(Uid, HistoryFieldEdit)>>,
//     temp_add_incoming_updates:
//         RwSignal<std::collections::HashSet<(BlueprintId, TempAddIncomingSlotRef)>>,
//     temp_add_outgoing_updates:
//         RwSignal<std::collections::HashSet<(BlueprintId, TempAddOutgoingSlotRef)>>,
//     wip_instance: Option<RGSOWrapperBuilder<T, TSchema>>,
//     id: Uid,
//     graph: std::rc::Rc<RBaseGraphEnvironment<TSchema>>,
//     _phantom: std::marker::PhantomData<T>,
// }

// impl<T, TSchema: EditRGSO<Schema = TSchema>> RGSOBuilder<T, TSchema>
// where
//     RGSOWrapperBuilder<T, TSchema>: RProducable<RGSOWrapper<T, TSchema>>,
//     T: RIntoSchema<Schema = TSchema> + Clone + std::fmt::Debug + 'static,
// {
//     pub fn get_id(&self) -> &Uid {
//         &self.id
//     }
//     pub fn execute(&self) -> Result<ExecutionResult, ElementCreationError> {
//         let graph = self.graph.clone();
//         let (blueprint, execution_result) = self.clone().get_blueprint()?;
//         graph.clear_redo();
//         graph.push_undo(blueprint.clone());
//         graph.process_blueprint(blueprint);
//         Ok(execution_result)
//     }
//     pub fn incorporate<C: std::fmt::Debug + Clone + RIntoSchema<Schema = TSchema> + 'static>(
//         &mut self,
//         other_builder: &RGSOBuilder<C, TSchema>,
//     ) {
//         self.add_outgoing_updates.update(|outgoing_updates| {
//             outgoing_updates.extend(other_builder.add_outgoing_updates.get())
//         });
//         self.add_incoming_updates.update(|incoming_updates| {
//             incoming_updates.extend(other_builder.add_incoming_updates.get())
//         });
//         self.remove_outgoing_updates.update(|outgoing_updates| {
//             outgoing_updates.extend(other_builder.remove_outgoing_updates.get())
//         });
//         self.remove_incoming_updates.update(|incoming_updates| {
//             incoming_updates.extend(other_builder.remove_incoming_updates.get())
//         });
//         self.instantiables.update(|prev| {
//             prev.extend(other_builder.instantiables.get());
//             if let Some(inner) = other_builder.wip_instance.clone() {
//                 prev.push(std::rc::Rc::new(std::cell::RefCell::new(inner)));
//             }
//         });
//         self.deleted_instances.update(|prev| {
//             prev.extend(other_builder.deleted_instances.get());
//         });
//         self.temp_add_incoming_updates.update(|prev| {
//             prev.extend(other_builder.temp_add_incoming_updates.get());
//         });
//         self.temp_add_outgoing_updates.update(|prev| {
//             prev.extend(other_builder.temp_add_outgoing_updates.get());
//         });
//         self.to_delete_recursive.update(|prev| {
//             prev.extend(other_builder.to_delete_recursive.get());
//         });
//         self.field_updates.update(|prev| {
//             prev.extend(other_builder.field_updates.get());
//         });
//         self.cumulative_errors
//             .update(|prev| prev.extend(other_builder.cumulative_errors.get()));
//     }
//     pub fn set_temp_id(&mut self, temp_id: &str) -> &mut Self {
//         if let Some(wip_instance) = &mut self.wip_instance {
//             wip_instance.set_temp_id(temp_id);
//         }
//         self
//     }
//     fn delete_recursive_handler(&self, id: &Uid) {
//         let item = self.graph.get(id).unwrap();
//         let pending_incoming_removals = self.remove_incoming_updates.with(|remove_updates| {
//             remove_updates
//                 .iter()
//                 .filter(|update| update.0 == *id)
//                 .collect::<std::collections::HashSet<_>>()
//                 .len()
//         });
//         let pending_incoming_additions = self.add_incoming_updates.with(|add_updates| {
//             add_updates
//                 .iter()
//                 .filter(|update| update.0 == *id)
//                 .collect::<std::collections::HashSet<_>>()
//                 .len()
//         });
//         if item.incoming_slots().with(|incoming_slots| {
//             incoming_slots.len() + pending_incoming_additions - pending_incoming_removals == 0
//         }) {
//             let slotted_instances = item
//                 .outgoing_slots()
//                 .values()
//                 .flat_map(|slot| slot.slotted_instances.get());
//             slotted_instances.for_each(|instance_id| self.delete_recursive_handler(&instance_id));
//         }
//     }
//     // Perform final calculations to gather all changes
//     fn get_blueprint(
//         mut self,
//     ) -> Result<(Blueprint<TSchema>, ExecutionResult), ElementCreationError> {
//         let mut all_errors = self.cumulative_errors.get();
//         let mut new_instantiables = self.instantiables.get();
//         if let Some(instance) = &self.wip_instance {
//             new_instantiables.push(std::rc::Rc::new(std::cell::RefCell::new(instance.clone())));
//         }

//         let temp_id_map = new_instantiables
//             .iter()
//             .map(|instantiable| {
//                 (
//                     instantiable.borrow().get_temp_id().clone(),
//                     *instantiable.borrow().get_id(),
//                 )
//             })
//             .collect::<std::collections::HashMap<_, _>>();

//         // Perform any incoming or outgoing updates for temporary ids
//         let temp_incoming_execution_errors = self.temp_add_incoming_updates.with(|updates| {
//             updates
//                 .iter()
//                 .filter_map(|update| {
//                     let final_host_id = match &update.1.host_instance_id {
//                         BlueprintId::Existing(existing_id) => Ok(*existing_id),
//                         BlueprintId::Temporary(temp_id) => temp_id_map.get(temp_id).cloned().ok_or(
//                             ElementCreationError::NonexistentTempId {
//                                 temp_id: temp_id.clone(),
//                             },
//                         ),
//                     };
//                     if let Some(error) = final_host_id.clone().err() {
//                         return Some(error);
//                     }
//                     let final_host_id = final_host_id.unwrap();
//                     match &update.0 {
//                         BlueprintId::Existing(existing_id) => {
//                             self.add_incoming_updates.update(|prev| {
//                                 prev.insert((
//                                     *existing_id,
//                                     SlotRef {
//                                         target_instance_id: *existing_id,
//                                         host_instance_id: final_host_id,
//                                         slot_id: update.1.slot_id,
//                                     },
//                                 ));
//                             });
//                         }
//                         BlueprintId::Temporary(temp_id) => {
//                             if let Some(instantiable) = new_instantiables
//                                 .iter_mut()
//                                 .find(|instantiable| instantiable.borrow().get_temp_id() == temp_id)
//                             {
//                                 instantiable
//                                     .as_ref()
//                                     .borrow_mut()
//                                     .add_incoming(&final_host_id, &update.1.slot_id);
//                             }
//                         }
//                     };
//                     None
//                 })
//                 .collect::<Vec<_>>()
//         });
//         all_errors.extend(temp_incoming_execution_errors);

//         let temp_outgoing_execution_errors = self.temp_add_outgoing_updates.with(|updates| {
//             updates
//                 .iter()
//                 .filter_map(|update| {
//                     let final_target_id = match &update.1.target_instance_id {
//                         BlueprintId::Existing(existing_id) => Ok(*existing_id),
//                         BlueprintId::Temporary(temp_id) => temp_id_map
//                             .get(temp_id)
//                             .ok_or(ElementCreationError::NonexistentTempId {
//                                 temp_id: temp_id.clone(),
//                             })
//                             .cloned(),
//                     };
//                     if let Some(error) = final_target_id.clone().err() {
//                         return Some(error);
//                     }
//                     let final_target_id = final_target_id.unwrap();

//                     match &update.0 {
//                         BlueprintId::Existing(existing_id) => {
//                             self.add_outgoing_updates.update(|prev| {
//                                 prev.insert((
//                                     *existing_id,
//                                     SlotRef {
//                                         target_instance_id: final_target_id,
//                                         host_instance_id: *existing_id,
//                                         slot_id: update.1.slot_id,
//                                     },
//                                 ));
//                             });
//                         }
//                         BlueprintId::Temporary(temp_id) => {
//                             if let Some(instantiable) = new_instantiables
//                                 .iter_mut()
//                                 .find(|instantiable| instantiable.borrow().get_temp_id() == temp_id)
//                             {
//                                 instantiable
//                                     .as_ref()
//                                     .borrow_mut()
//                                     .add_outgoing(&final_target_id, &update.1.slot_id);
//                             }
//                         }
//                     };
//                     None
//                 })
//                 .collect::<Vec<_>>()
//         });
//         all_errors.extend(temp_outgoing_execution_errors);

//         let to_delete = self.to_delete_recursive.get();
//         to_delete.iter().for_each(|to_delete_id| {
//             self.delete(to_delete_id);
//             let item = self.graph.get(to_delete_id).unwrap();
//             let slotted_instances = item
//                 .outgoing_slots()
//                 .values()
//                 .flat_map(|slot| slot.slotted_instances.get());
//             slotted_instances.for_each(|instance_id| self.delete_recursive_handler(&instance_id));
//         });

//         // Get rid of all changes on nodes that will be deleted
//         // Also grab and clone the node about to be deleted to facilitate undoing
//         let cloned_delete_instances = self.deleted_instances.with(|deleted_instances| {
//             deleted_instances
//                 .iter()
//                 .map(|deleted_instance_id| {
//                     self.add_outgoing_updates
//                         .update(|prev| prev.retain(|change| change.0 != *deleted_instance_id));
//                     self.remove_outgoing_updates
//                         .update(|prev| prev.retain(|change| change.0 != *deleted_instance_id));
//                     self.add_incoming_updates
//                         .update(|prev| prev.retain(|change| change.0 != *deleted_instance_id));
//                     self.remove_incoming_updates
//                         .update(|prev| prev.retain(|change| change.0 != *deleted_instance_id));
//                     self.field_updates
//                         .update(|prev| prev.retain(|change| change.0 != *deleted_instance_id));
//                     self.graph.created_instances.with(|created_instances| {
//                         created_instances.get(deleted_instance_id).unwrap().clone()
//                     })
//                 })
//                 .collect::<Vec<_>>()
//         });

//         // Check slot bounds for conformity
//         let mut already_checked = vec![];
//         let bounds_checks = self
//             .remove_outgoing_updates
//             .with(|remove_outgoing_updates| {
//                 self.add_outgoing_updates.with(|add_outgoing_updates| {
//                     let errors = remove_outgoing_updates
//                         .iter()
//                         .chain(add_outgoing_updates)
//                         .filter_map(|update| {
//                             if already_checked.contains(&update.0) {
//                                 return None;
//                             }
//                             already_checked.push(update.0);

//                             let all_removals = remove_outgoing_updates
//                                 .iter()
//                                 .filter(|item| item.0 == update.0);
//                             let all_additions = add_outgoing_updates
//                                 .iter()
//                                 .filter(|item| item.0 == update.0);
//                             let errors = self
//                                 .graph
//                                 .get(&update.0)
//                                 .unwrap()
//                                 .outgoing_slots()
//                                 .iter()
//                                 .filter_map(|slot| {
//                                     let final_count = slot
//                                         .1
//                                         .slotted_instances
//                                         .with(|slotted_instances| slotted_instances.len())
//                                         + all_additions
//                                             .clone()
//                                             .filter(|addition| addition.1.slot_id == *slot.0)
//                                             .collect::<Vec<_>>()
//                                             .len()
//                                         - all_removals
//                                             .clone()
//                                             .filter(|addition| addition.1.slot_id == *slot.0)
//                                             .collect::<Vec<_>>()
//                                             .len();
//                                     if !slot.1.check_bound_conformity(final_count) {
//                                         Some(ElementCreationError::BoundCheckOutOfRange(format!(
//                                             "{}: {}\nBounds: {:?}, Attempted: {}",
//                                             self.graph.get(&update.0).unwrap().operative().tag.name,
//                                             slot.1.slot.tag.name,
//                                             slot.1.slot.bounds,
//                                             slot.1.slotted_instances.get().len()
//                                         )))
//                                     } else {
//                                         None
//                                     }
//                                 })
//                                 .collect::<Vec<_>>();
//                             if errors.is_empty() {
//                                 None
//                             } else {
//                                 Some(errors)
//                             }
//                         })
//                         .flatten()
//                         .collect::<Vec<_>>();
//                     errors
//                 })
//             });

//         // TODO figure out how to return all errors
//         if !bounds_checks.is_empty() {
//             leptos::logging::log!("{:#?}", bounds_checks);
//             // return Err(Error::new(bounds_checks));
//             return Err(ElementCreationError::Stack(bounds_checks));
//         }

//         let (instantiated_elements, instantiation_errors) = new_instantiables.iter().fold(
//             (Vec::with_capacity(new_instantiables.len()), Vec::new()),
//             |mut agg, el| {
//                 match el.borrow().instantiate() {
//                     Ok(instance) => agg.0.push(instance),
//                     Err(error) => agg.1.push(error),
//                 }
//                 agg
//             },
//         );

//         all_errors.extend(instantiation_errors);
//         if !all_errors.is_empty() {
//             return Err(ElementCreationError::Stack(all_errors));
//         }

//         Ok((
//             Blueprint::<TSchema> {
//                 added_instances: instantiated_elements,
//                 add_outgoing_updates: self.add_outgoing_updates.get(),
//                 add_incoming_updates: self.add_incoming_updates.get(),
//                 remove_outgoing_updates: self.remove_outgoing_updates.get(),
//                 remove_incoming_updates: self.remove_incoming_updates.get(),
//                 deleted_instances: cloned_delete_instances,
//                 field_updates: self.field_updates.get(),
//                 action_tag: None,
//             },
//             ExecutionResult { temp_id_map },
//         ))
//     }
//     fn get_graph(&self) -> &std::rc::Rc<RBaseGraphEnvironment<TSchema>> {
//         &self.graph
//     }
//     fn new(
//         builder_wrapper_instance: Option<RGSOWrapperBuilder<T, TSchema>>,
//         id: Uid,
//         graph: std::rc::Rc<RBaseGraphEnvironment<TSchema>>,
//     ) -> Self {
//         Self {
//             graph,
//             instantiables: RwSignal::new(vec![]),
//             wip_instance: builder_wrapper_instance,
//             id,
//             cumulative_errors: RwSignal::new(std::vec::Vec::new()),
//             add_outgoing_updates: RwSignal::new(std::collections::HashSet::new()),
//             add_incoming_updates: RwSignal::new(std::collections::HashSet::new()),
//             remove_outgoing_updates: RwSignal::new(std::collections::HashSet::new()),
//             remove_incoming_updates: RwSignal::new(std::collections::HashSet::new()),
//             temp_add_incoming_updates: RwSignal::new(std::collections::HashSet::new()),
//             temp_add_outgoing_updates: RwSignal::new(std::collections::HashSet::new()),
//             _phantom: std::marker::PhantomData,
//             field_updates: RwSignal::new(std::collections::HashSet::new()),
//             deleted_instances: RwSignal::new(std::collections::HashSet::new()),
//             to_delete_recursive: RwSignal::new(std::collections::HashSet::new()),
//         }
//     }
//     fn raw_add_outgoing_to_updates(&mut self, slot_ref: SlotRef) {
//         self.add_outgoing_updates.update(|prev| {
//             prev.insert((slot_ref.host_instance_id, slot_ref));
//         });
//     }
//     fn raw_add_incoming_to_updates(&mut self, slot_ref: SlotRef) {
//         self.add_incoming_updates.update(|prev| {
//             prev.insert((slot_ref.target_instance_id, slot_ref));
//         });
//     }
//     fn add_outgoing<C: std::fmt::Debug + Clone + RIntoSchema<Schema = TSchema> + 'static>(
//         &mut self,
//         slot_id: &Uid,
//         // BlueprintId in this case meaning that:
//         //    Existing: The ID is known
//         //    Temporary: The ID is non known, only the temp_id
//         target_id: BlueprintId,
//         instantiable: Option<RGSOBuilder<C, TSchema>>,
//     ) {
//         // If this is a newly created instance
//         if let Some(instance) = &self.wip_instance {
//             match &target_id {
//                 BlueprintId::Existing(existing_target_id) => {
//                     let slot = instance.slots.get(slot_id).unwrap();
//                     slot.slotted_instances.update(|prev| {
//                         prev.push(*existing_target_id);
//                     });
//                 }
//                 BlueprintId::Temporary(_temp_target_id) => {
//                     self.temp_add_outgoing(
//                         BlueprintId::Temporary(instance.get_temp_id().clone()),
//                         TempAddOutgoingSlotRef {
//                             target_instance_id: target_id.clone(),
//                             slot_id: *slot_id,
//                         },
//                     );
//                 }
//             }
//         // If this is an existing element being edited
//         } else {
//             match &target_id {
//                 BlueprintId::Existing(existing_target_id) => {
//                     self.raw_add_outgoing_to_updates(SlotRef {
//                         host_instance_id: self.id,
//                         target_instance_id: *existing_target_id,
//                         slot_id: *slot_id,
//                     });
//                 }
//                 BlueprintId::Temporary(_temp_target_id) => self.temp_add_outgoing(
//                     BlueprintId::Existing(*self.get_id()),
//                     TempAddOutgoingSlotRef {
//                         target_instance_id: target_id,
//                         slot_id: *slot_id,
//                     },
//                 ),
//             }
//         }
//         if let Some(instantiable) = instantiable {
//             self.incorporate(&instantiable);
//         }
//     }
//     fn remove_outgoing(&mut self, slot_ref: SlotRef) {
//         self.remove_outgoing_updates.update(|prev| {
//             prev.insert((slot_ref.host_instance_id, slot_ref.clone()));
//         });
//         self.remove_incoming_updates.update(|prev| {
//             prev.insert((slot_ref.target_instance_id, slot_ref));
//         });
//     }
//     fn add_incoming<C: std::fmt::Debug + Clone + RIntoSchema<Schema = TSchema> + 'static>(
//         &mut self,
//         slot_ref: SlotRef,
//         instantiable: Option<RGSOBuilder<C, TSchema>>,
//     ) {
//         if let Some(instance) = &self.wip_instance {
//             instance
//                 .incoming_slots
//                 .update(|prev| prev.push(slot_ref.clone()))
//         } else {
//             self.add_incoming_updates.update(|prev| {
//                 prev.insert((slot_ref.target_instance_id, slot_ref));
//             });
//         }
//         if let Some(instantiable) = instantiable {
//             self.incorporate(&instantiable);
//         }
//     }
//     fn edit_field(&mut self, field_id: Uid, value: PrimitiveValues) {
//         if let Some(instance) = &self.wip_instance {
//             let signal = instance.data.get(&field_id).unwrap();
//             let is_none = signal.with(|val| val.is_none());
//             if is_none {
//                 signal.set(Some(RwSignal::new(value)));
//             } else {
//                 signal.update(|prev| prev.unwrap().set(value))
//             }
//         } else {
//             self.field_updates.update(|prev| {
//                 prev.insert((
//                     *self.get_id(),
//                     HistoryFieldEdit {
//                         instance_id: *self.get_id(),
//                         field_id,
//                         new_value: value,
//                         prev_value: PrimitiveValues::Bool(false),
//                     },
//                 ));
//             })
//         }
//     }
//     fn delete(&mut self, to_delete_id: &Uid) {
//         self.deleted_instances.update(|prev| {
//             prev.insert(*to_delete_id);
//         });
//         let existing_instance = self.graph.get(&self.id).unwrap();
//         existing_instance.incoming_slots().with(|incoming_slots| {
//             incoming_slots.iter().for_each(|incoming_slot| {
//                 self.remove_outgoing_updates.update(|removes| {
//                     removes.insert((incoming_slot.host_instance_id, incoming_slot.clone()));
//                 })
//             })
//         });
//         existing_instance
//             .outgoing_slots()
//             .values()
//             .for_each(|slot| {
//                 slot.slotted_instances.with(|slotted_instances| {
//                     slotted_instances.iter().for_each(|target_instance_id| {
//                         self.remove_incoming_updates.update(|removes| {
//                             removes.insert((
//                                 *target_instance_id,
//                                 SlotRef {
//                                     host_instance_id: *to_delete_id,
//                                     target_instance_id: *target_instance_id,
//                                     slot_id: slot.slot.tag.id,
//                                 },
//                             ));
//                         })
//                     })
//                 })
//             });
//     }
//     fn delete_recursive(&mut self) {
//         self.to_delete_recursive.update(|prev| {
//             prev.insert(self.id);
//         });
//     }
//     fn temp_add_incoming(&mut self, host_id: BlueprintId, temp_slot_ref: TempAddIncomingSlotRef) {
//         self.temp_add_incoming_updates
//             .update(|temp_add_incoming_updates| {
//                 temp_add_incoming_updates.insert((host_id, temp_slot_ref));
//             });
//     }
//     fn temp_add_outgoing(&mut self, target_id: BlueprintId, temp_slot_ref: TempAddOutgoingSlotRef) {
//         self.temp_add_outgoing_updates
//             .update(|temp_add_outgoing_updates| {
//                 temp_add_outgoing_updates.insert((target_id, temp_slot_ref));
//             });
//     }
//     fn add_error(&mut self, error: ElementCreationError) {
//         self.cumulative_errors.update(|prev| prev.push(error));
//     }
// }

// impl<T, TSchema: EditRGSO<Schema = TSchema> + 'static> RInstantiable
//     for RGSOWrapperBuilder<T, TSchema>
// where
//     T: Clone + std::fmt::Debug + RIntoSchema<Schema = TSchema> + 'static,
//     RGSOWrapper<T, TSchema>: RFieldEditable,
// {
//     type Schema = TSchema;

//     fn instantiate(
//         &self,
//     ) -> Result<Self::Schema, base_types::post_generation::ElementCreationError> {
//         self.verify()?;
//         Ok(T::into_schema(self.produce()))
//     }

//     fn get_id(&self) -> &Uid {
//         &self.id
//     }
//     fn get_template(&self) -> &'static LibraryTemplate<PrimitiveTypes, PrimitiveValues> {
//         self.template
//     }

//     fn get_temp_id(&self) -> &String {
//         &self.temp_id
//     }

//     fn add_incoming(&mut self, host_id: &Uid, slot_id: &Uid) {
//         self.incoming_slots.update(|incoming_slots| {
//             incoming_slots.push(SlotRef {
//                 host_instance_id: *host_id,
//                 slot_id: *slot_id,
//                 target_instance_id: self.id,
//             })
//         });
//     }

//     fn add_outgoing(&mut self, target_id: &Uid, slot_id: &Uid) {
//         self.slots
//             .get(slot_id)
//             .unwrap()
//             .slotted_instances
//             .update(|slotted_instances| slotted_instances.push(*target_id));
//     }
// }

// pub trait RIntoSchema
// where
//     Self: Sized,
// {
//     type Schema: EditRGSO<Schema = Self::Schema>;
//     fn into_schema(instantiable: RGSOWrapper<Self, Self::Schema>) -> Self::Schema;
// }

// pub trait REditable<T>
// where
//     Self: Sized,
// {
//     type Schema: EditRGSO<Schema = Self::Schema>;
//     fn initiate_edit(&self) -> RGSOBuilder<T, Self::Schema>;
// }
// impl<T, TSchema: EditRGSO<Schema = TSchema>> REditable<T> for RGSOWrapper<T, TSchema>
// where
//     T: Clone
//         + std::fmt::Debug
//         + RIntoSchema<Schema = TSchema>
//         + RBuildable<Schema = TSchema>
//         + 'static,
//     RGSOWrapper<T, TSchema>: RFieldEditable,
// {
//     type Schema = TSchema;
//     fn initiate_edit(&self) -> RGSOBuilder<T, Self::Schema> {
//         T::initiate_edit(*self.get_id(), self.get_graph().clone())
//     }
// }

// #[derive(Clone, Debug)]
// pub struct SharedGraph<TSchema: EditRGSO<Schema = TSchema> + 'static>(
//     std::rc::Rc<RBaseGraphEnvironment<TSchema>>,
// );
// impl<TSchema: EditRGSO<Schema = TSchema> + 'static>
//     From<std::rc::Rc<RBaseGraphEnvironment<TSchema>>> for SharedGraph<TSchema>
// {
//     fn from(value: std::rc::Rc<RBaseGraphEnvironment<TSchema>>) -> SharedGraph<TSchema> {
//         SharedGraph(value)
//     }
// }
// impl<TSchema: EditRGSO<Schema = TSchema> + 'static> From<SharedGraph<TSchema>>
//     for std::rc::Rc<RBaseGraphEnvironment<TSchema>>
// {
//     fn from(value: SharedGraph<TSchema>) -> std::rc::Rc<RBaseGraphEnvironment<TSchema>> {
//         value.0
//     }
// }
// impl<TSchema: EditRGSO<Schema = TSchema> + 'static> std::ops::Deref for SharedGraph<TSchema> {
//     type Target = std::rc::Rc<RBaseGraphEnvironment<TSchema>>;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
// mod from_reactive {
//     use std::{cell::RefCell, collections::HashMap, rc::Rc};

//     use super::SharedGraph;
//     use base_types::post_generation::{BaseGraphEnvironment, GSOWrapper};
//     use leptos::*;

//     use super::{
//         EditRGSO, FromNonReactive, RActiveSlot, RBaseGraphEnvironment, RGSOWrapper,
//         RHistoryContainer, RGSO,
//     };
//     impl<RTSchema: EditRGSO<Schema = RTSchema>, TSchema> From<SharedGraph<RTSchema>>
//         for BaseGraphEnvironment<TSchema>
//     where
//         RTSchema: Into<TSchema> + Clone,
//     {
//         fn from(value: SharedGraph<RTSchema>) -> Self {
//             Self {
//                 created_instances: value
//                     .0
//                     .created_instances
//                     .get()
//                     .into_iter()
//                     .map(|(id, val)| (id, val.into()))
//                     .collect(),
//                 constraint_schema: value.0.constraint_schema,
//             }
//         }
//     }
//     impl<RTSchema: EditRGSO<Schema = RTSchema>, TSchema> From<BaseGraphEnvironment<TSchema>>
//         for SharedGraph<RTSchema>
//     where
//         RTSchema: FromNonReactive<TSchema>,
//     {
//         fn from(value: BaseGraphEnvironment<TSchema>) -> Self {
//             let new_graph = RBaseGraphEnvironment::<RTSchema> {
//                 created_instances: RwSignal::new(HashMap::new()),
//                 constraint_schema: value.constraint_schema,
//                 history: Rc::new(RefCell::new(RHistoryContainer {
//                     undo: vec![],
//                     redo: vec![],
//                 })),
//                 // constraint_schema: todo!(),
//             };
//             let rc_graph = Rc::new(new_graph);
//             let members = value.created_instances.into_iter().map(|(id, val)| {
//                 (
//                     id,
//                     <RTSchema as FromNonReactive<TSchema>>::from_non_reactive(
//                         val,
//                         rc_graph.clone(),
//                     ),
//                 )
//             }); // .collect(),
//             rc_graph.initialize(members.collect());
//             rc_graph.into()

//             // The Rc's referencing this graph cannot have been used yet as they haven't been returned
//             // So it should be safe to replace the Rc's address with the updated value.
//             // unsafe {
//             //     let rc_graph_mut =
//             //         Rc::into_raw(rc_graph.clone()) as *mut RBaseGraphEnvironment<RTSchema>;
//             //     (*rc_graph_mut).created_instances = RwSignal::new(members.collect());
//             //     *(Box::from_raw(rc_graph_mut))
//             // }
//             // rc_graph
//             // Rc::get_mut(&mut Rc::clone(&rc_graph))
//             //     .expect("Rc should only have one strong reference during initialization")
//             //     .created_instances = RwSignal::new(members.collect());
//             // Rc::try_unwrap(rc_graph).expect("Rc should only have one strong reference")
//         }
//     }
//     impl From<RActiveSlot> for base_types::post_generation::ActiveSlot {
//         fn from(value: RActiveSlot) -> Self {
//             Self {
//                 slot: value.slot,
//                 slotted_instances: value.slotted_instances.get(),
//             }
//         }
//     }
//     impl From<base_types::post_generation::ActiveSlot> for RActiveSlot {
//         fn from(value: base_types::post_generation::ActiveSlot) -> Self {
//             Self {
//                 slot: value.slot,
//                 slotted_instances: RwSignal::new(value.slotted_instances),
//             }
//         }
//     }
//     impl<T, RTSchema: EditRGSO<Schema = RTSchema>> From<RGSOWrapper<T, RTSchema>> for GSOWrapper<T>
//     where
//         T: Clone + std::fmt::Debug,
//     {
//         fn from(value: RGSOWrapper<T, RTSchema>) -> Self {
//             Self {
//                 id: value.id,
//                 fields: value
//                     .fields
//                     .into_iter()
//                     .map(|(id, val)| (id, val.get()))
//                     .collect(),
//                 outgoing_slots: value
//                     .outgoing_slots
//                     .into_iter()
//                     .map(|(id, val)| (id, val.into()))
//                     .collect(),
//                 incoming_slots: value.incoming_slots.get().into_iter().collect(),

//                 operative: value.operative,
//                 template: value.template,
//                 _phantom: std::marker::PhantomData,
//             }
//         }
//     }
// }
