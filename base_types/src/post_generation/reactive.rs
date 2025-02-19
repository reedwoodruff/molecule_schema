// pub mod from_reactive;
pub use crate::common::Uid;
use crate::{
    common::u128_to_string,
    constraint_schema::{LibraryOperative, LibraryTemplate, OperativeSlot, SlotBounds},
};
pub use typenum;

use crate::{
    common::ConstraintTraits,
    constraint_schema::ConstraintSchema,
    primitives::{PrimitiveTypes, PrimitiveValues},
};

use crate::post_generation::{
    ElementCreationError, HistoryFieldEdit, SlotRef, TaggedAction, Verifiable,
};
use leptos::prelude::*;
use std::{collections::HashMap, hash::Hash, marker::PhantomData, str::FromStr};

pub mod hidden {
    use super::*;
    pub trait EditRGSO: RGSO {
        fn add_incoming(&self, slot_ref: SlotRef) -> &Self;
        fn add_outgoing(&self, slot_ref: SlotRef) -> &Self;
        fn remove_outgoing(&self, slot_ref: &SlotRef) -> &Self;
        fn remove_incoming(&self, parent_id: &Uid, slot_id: Option<&Uid>) -> Vec<SlotRef>;
        fn update_field(&self, field_edit: HistoryFieldEdit) -> &Self;
        fn get_graph(&self) -> &std::sync::Arc<RBaseGraphEnvironment<Self::Schema>>;
    }
    impl<T: HasSlotEnum, TSchema> EditRGSO for RGSOConcrete<T, TSchema>
    where
        <T as HasSlotEnum>::SlotEnum: Clone,
    {
        fn update_field(&self, field_edit: HistoryFieldEdit) -> &Self {
            self.fields
                .get(&field_edit.field_id)
                .unwrap()
                .set(field_edit.new_value);
            self
        }
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
                        false
                    } else {
                        true
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
        fn get_graph(&self) -> &std::sync::Arc<RBaseGraphEnvironment<Self::Schema>> {
            &self.graph
        }
    }
    impl<T: HasSlotEnum, TSchema: 'static> RGSOConcrete<T, TSchema>
    where
        <T as HasSlotEnum>::SlotEnum: FromStr,
    {
        pub fn from_standalone(
            value: StandaloneRGSOConcrete,
            graph: SharedGraph<TSchema>,
            constraint_schema: &'static ConstraintSchema<PrimitiveTypes, PrimitiveValues>, // operative_ref: &'static LibraryOperative<PrimitiveTypes, PrimitiveValues>,
                                                                                           // template_ref: &'static LibraryTemplate<PrimitiveTypes, PrimitiveValues>,
        ) -> Self {
            let operative_ref = constraint_schema
                .operative_library
                .get(&value.operative)
                .unwrap();
            let template_ref = constraint_schema
                .template_library
                .get(&value.template)
                .unwrap();
            let mut initial_btree =
                std::collections::BTreeMap::<Uid, SpecializedRActiveSlot<T::SlotEnum>>::new();
            template_ref
                .operative_slots
                .iter()
                .for_each(|(slot_id, _slot)| {
                    let operative_slot_ref = template_ref.operative_slots.get(&slot_id).unwrap();
                    let enum_variant = T::SlotEnum::from_str(&u128_to_string(slot_id.clone()))
                        .ok()
                        .expect("Slot enum variant mismatched with slot id");
                    initial_btree.insert(
                        slot_id.clone(),
                        SpecializedRActiveSlot::<T::SlotEnum> {
                            base: RActiveSlot {
                                slot: operative_slot_ref,
                                slotted_instances: RwSignal::new(vec![]),
                            },
                            slot_enum: enum_variant,
                        },
                    );
                });
            let new_outgoing_slots =
                value
                    .outgoing_slots
                    .iter()
                    .fold(initial_btree, |mut agg, slot_ref| {
                        agg.entry(slot_ref.slot_id).and_modify(|r_active_slot| {
                            r_active_slot
                                .base
                                .slotted_instances
                                .update_untracked(|prev| prev.push(slot_ref.target_instance_id));
                        });
                        agg
                    });
            Self {
                id: value.id,
                fields: value
                    .fields
                    .into_iter()
                    .map(|(id, field)| (id, RwSignal::new(field)))
                    .collect::<HashMap<_, _>>(),
                graph: graph.into(),
                outgoing_slots: new_outgoing_slots,
                incoming_slots: RwSignal::new(value.incoming_slots),
                operative: operative_ref,
                template: template_ref,
                _phantom: PhantomData,
            }
        }
    }
}

use hidden::EditRGSO;

use super::StandaloneRGSOConcrete;
pub trait FromNonReactive<NTSchema>
where
    Self: EditRGSO<Schema = Self>,
{
    fn from_non_reactive(
        value: NTSchema,
        graph: std::sync::Arc<RBaseGraphEnvironment<Self>>,
    ) -> Self;
}
pub fn saturate_wrapper<
    T: Clone + std::fmt::Debug + HasSlotEnum,
    RTSchema: EditRGSO<Schema = RTSchema>,
>(
    non_reactive: crate::post_generation::GSOConcrete<T>,
    graph: std::sync::Arc<RBaseGraphEnvironment<RTSchema>>,
) -> RGSOConcrete<T, RTSchema>
where
    <T as HasSlotEnum>::SlotEnum: FromStr,
{
    RGSOConcrete::<T, RTSchema> {
        id: non_reactive.id,
        graph,
        fields: non_reactive
            .fields
            .into_iter()
            .map(|(id, val)| (id, RwSignal::new(val)))
            .collect(),
        outgoing_slots: non_reactive
            .outgoing_slots
            .into_iter()
            .map(|(id, val)| {
                (
                    id,
                    SpecializedRActiveSlot::<T::SlotEnum> {
                        slot_enum: match T::SlotEnum::from_str(&u128_to_string(val.slot.tag.id)) {
                            Ok(variant) => variant,
                            Err(_err) => unreachable!(),
                        },
                        base: val.into(),
                    },
                )
            })
            .collect(),
        incoming_slots: RwSignal::new(non_reactive.incoming_slots.into_iter().collect()),
        operative: non_reactive.operative,
        template: non_reactive.template,
        _phantom: std::marker::PhantomData,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BlueprintId {
    Existing(Uid),
    Temporary(String),
}
// impl From<BlueprintId> for Uid {
//     fn from(value: BlueprintId) -> Self {
//         todo!()
//     }
// }
impl From<Uid> for BlueprintId {
    fn from(value: Uid) -> Self {
        BlueprintId::Existing(value)
    }
}
impl From<&Uid> for BlueprintId {
    fn from(value: &Uid) -> Self {
        BlueprintId::Existing(*value)
    }
}
impl From<String> for BlueprintId {
    fn from(value: String) -> Self {
        BlueprintId::Temporary(value)
    }
}
impl From<&str> for BlueprintId {
    fn from(value: &str) -> Self {
        BlueprintId::Temporary(value.to_string())
    }
}
// impl Into<BlueprintId> for Uid {
//     fn into(self) -> BlueprintId {
//         BlueprintId::Existing(self)
//     }
// }
// impl Into<BlueprintId> for &Uid {
//     fn into(self) -> BlueprintId {
//         BlueprintId::Existing(self.clone())
//     }
// }
// impl Into<BlueprintId> for &str {
//     fn into(self) -> BlueprintId {
//         BlueprintId::Temporary(self.to_string())
//     }
// }
// impl Into<BlueprintId> for String {
//     fn into(self) -> BlueprintId {
//         BlueprintId::Temporary(self)
//     }
// }

// impl BlueprintId {
//     fn new_temporary(name: &str) -> Self {
//         BlueprintId::Temporary(name.to_string())
//     }
// }

pub trait RProducable<T> {
    type Schema;
    fn produce(&self) -> T;
}

#[derive(Clone, Debug)]
pub struct RHistoryContainer<TSchema> {
    pub undo: Vec<Blueprint<TSchema>>,
    pub redo: Vec<Blueprint<TSchema>>,
}

#[derive(Debug, Clone)]
pub struct RBaseGraphEnvironment<TSchema: 'static> {
    pub created_instances: RwSignal<std::collections::HashMap<Uid, TSchema>>,
    pub constraint_schema: &'static ConstraintSchema<PrimitiveTypes, PrimitiveValues>,
    pub history: std::sync::Arc<std::sync::Mutex<RHistoryContainer<TSchema>>>,
}
impl<TSchema: Send + Sync> RBaseGraphEnvironment<TSchema> {
    pub fn new(
        constraint_schema: &'static ConstraintSchema<PrimitiveTypes, PrimitiveValues>,
    ) -> Self {
        Self {
            created_instances: RwSignal::new(std::collections::HashMap::new()),
            constraint_schema,
            history: std::sync::Arc::new(std::sync::Mutex::new(RHistoryContainer {
                undo: Vec::new(),
                redo: Vec::new(),
            })),
        }
    }
    pub fn initialize(&self, created_instances: std::collections::HashMap<Uid, TSchema>) {
        self.created_instances.set(created_instances);
    }
}

impl<TSchema: EditRGSO + Send + Sync> RBaseGraphEnvironment<TSchema> {
    fn process_blueprint(&self, blueprint: Blueprint<TSchema>) {
        leptos::logging::log!("starting processing of blueprint");
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
                    prev.remove(instance.get_id());
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
        blueprint
            .field_updates
            .into_iter()
            .for_each(|field_update| {
                self.created_instances.with(|created_instances| {
                    created_instances
                        .get(&field_update.0)
                        .unwrap()
                        .update_field(field_update.1);
                });
            });
        leptos::logging::log!("finished processing of blueprint");
    }
    fn push_undo(&self, blueprint: Blueprint<TSchema>) {
        self.history.as_ref().lock().unwrap().undo.push(blueprint);
    }
    fn push_redo(&self, blueprint: Blueprint<TSchema>) {
        self.history.as_ref().lock().unwrap().redo.push(blueprint);
    }
    fn clear_redo(&self) {
        self.history.as_ref().lock().unwrap().redo.clear();
    }
}
impl<TSchema: EditRGSO + Send + Sync> RGraphEnvironment for RBaseGraphEnvironment<TSchema> {
    type Schema = TSchema;
    type Types = PrimitiveTypes;
    type Values = PrimitiveValues;

    fn get_constraint_schema(&self) -> &ConstraintSchema<Self::Types, Self::Values> {
        self.constraint_schema
    }

    fn get(&self, id: &Uid) -> Option<Self::Schema> {
        let test = self
            .created_instances
            .with_untracked(|created_instances| created_instances.get(id).cloned());
        test
    }

    fn undo(&self) {
        let undo_item = self.history.as_ref().lock().unwrap().undo.pop();
        if undo_item.is_none() {
            return;
        }
        let undo_item = undo_item.unwrap().reverse();

        self.process_blueprint(undo_item.clone());
        self.push_redo(undo_item)
    }

    fn redo(&self) {
        let redo_item = self.history.as_ref().lock().unwrap().redo.pop();
        if redo_item.is_none() {
            return;
        }
        let redo_item = redo_item.unwrap().reverse();
        self.process_blueprint(redo_item.clone());
        self.push_undo(redo_item);
    }
}
#[cfg(feature = "serde")]
impl<TSchema: Send + Sync + Clone + Into<StandaloneRGSOConcrete> + 'static> serde::Serialize
    for RBaseGraphEnvironment<TSchema>
{
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let standalone_instances = self
            .created_instances
            .get()
            .into_values()
            // .map(|inst| <StandaloneRGSOConcrete as From<TSchema>>::from(inst))
            .map(|inst| inst.into())
            .collect::<Vec<StandaloneRGSOConcrete>>();
        let mut s = serializer.serialize_seq(Some(standalone_instances.len()))?;
        for item in standalone_instances {
            serde::ser::SerializeSeq::serialize_element(&mut s, &item)?;
        }
        serde::ser::SerializeSeq::end(s)
    }
}

pub trait RGraphEnvironment {
    type Types: ConstraintTraits;
    type Values: ConstraintTraits;
    type Schema: RGSO;

    fn get(&self, id: &Uid) -> Option<Self::Schema>;
    fn get_constraint_schema(&self) -> &ConstraintSchema<Self::Types, Self::Values>;
    fn undo(&self);
    fn redo(&self);
}

/// Reactive Generated Schema Object
pub trait RGSO: std::fmt::Debug + Clone {
    type Schema;
    /// Instance ID
    fn get_id(&self) -> &Uid;
    // fn get_name(&self) -> &String;
    fn operative(&self) -> &'static LibraryOperative<PrimitiveTypes, PrimitiveValues>;
    fn template(&self) -> &'static LibraryTemplate<PrimitiveTypes, PrimitiveValues>;
    fn slot_by_id<E: Into<Uid>>(&self, slot_id: E) -> Option<&RActiveSlot> {
        self.outgoing_slots().get(&slot_id.into()).map(|slot| *slot)
    }
    fn outgoing_slots(&self) -> std::collections::BTreeMap<&Uid, &RActiveSlot>;
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
    fn fields(&self) -> &std::collections::HashMap<Uid, RwSignal<PrimitiveValues>>;
}
// impl From<RActiveSlot> for Uid {
//     fn from(value: RActiveSlot) -> Self {
//         todo!()
//     }
// }

pub trait Slotted {}

#[derive(Clone)]
pub struct RActiveSlot {
    pub slot: &'static OperativeSlot,
    pub slotted_instances: RwSignal<Vec<Uid>>,
}

#[derive(Clone, Debug)]
pub struct SpecializedRActiveSlot<TSlotEnum> {
    pub base: RActiveSlot,
    // pub slot: &'static OperativeSlot,
    // pub slotted_instances: RwSignal<Vec<Uid>>,
    pub slot_enum: TSlotEnum,
}
impl<TSlotEnum> From<SpecializedRActiveSlot<TSlotEnum>> for RActiveSlot {
    fn from(value: SpecializedRActiveSlot<TSlotEnum>) -> Self {
        value.base
    }
}
impl<TSlotEnum> AsRef<RActiveSlot> for SpecializedRActiveSlot<TSlotEnum> {
    fn as_ref(&self) -> &RActiveSlot {
        &self.base // Simply return a reference to the contained RActiveSlot
    }
}

impl<TSlotEnum> std::ops::Deref for SpecializedRActiveSlot<TSlotEnum> {
    type Target = RActiveSlot;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl<TSlotEnum> AsMut<RActiveSlot> for SpecializedRActiveSlot<TSlotEnum> {
    fn as_mut(&mut self) -> &mut RActiveSlot {
        &mut self.base
    }
}

impl std::fmt::Debug for RActiveSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

pub struct RGSOConcrete<T: HasSlotEnum, TSchema: 'static> {
    id: Uid,
    pub fields: std::collections::HashMap<Uid, RwSignal<PrimitiveValues>>,
    graph: std::sync::Arc<RBaseGraphEnvironment<TSchema>>,
    outgoing_slots: std::collections::BTreeMap<Uid, SpecializedRActiveSlot<T::SlotEnum>>,
    incoming_slots: RwSignal<Vec<SlotRef>>,
    operative: &'static LibraryOperative<PrimitiveTypes, PrimitiveValues>,
    template: &'static LibraryTemplate<PrimitiveTypes, PrimitiveValues>,
    _phantom: std::marker::PhantomData<T>,
}
impl<T: HasSlotEnum, TSchema: 'static> Clone for RGSOConcrete<T, TSchema>
where
    <T as HasSlotEnum>::SlotEnum: Clone,
{
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            fields: self.fields.clone(),
            graph: self.graph.clone(),
            outgoing_slots: self.outgoing_slots.clone(),
            incoming_slots: self.incoming_slots.clone(),
            operative: &self.operative,
            template: &self.template,
            _phantom: PhantomData,
        }
    }
}
impl<T: HasSlotEnum, TSchema> PartialEq for RGSOConcrete<T, TSchema> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl<T: HasSlotEnum, TSchema> PartialOrd for RGSOConcrete<T, TSchema> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}
impl<T: HasSlotEnum, TSchema> Ord for RGSOConcrete<T, TSchema> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl<T: HasSlotEnum, TSchema> Eq for RGSOConcrete<T, TSchema> {}
impl<T: HasSlotEnum, TSchema> Hash for RGSOConcrete<T, TSchema> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
impl<T: HasSlotEnum, TSchema> std::fmt::Debug for RGSOConcrete<T, TSchema> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
                    .fields
                    .iter()
                    .map(|(field_id, data)| {
                        (
                            self.template
                                .field_constraints
                                .get(field_id)
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

impl<T: HasSlotEnum, TSchema> RGSO for RGSOConcrete<T, TSchema>
where
    <T as HasSlotEnum>::SlotEnum: Clone,
{
    type Schema = TSchema;
    fn get_id(&self) -> &Uid {
        &self.id
    }
    // fn get_name(&self) -> &String {
    //     &self.operative.tag.name
    // }

    fn outgoing_slots(&self) -> std::collections::BTreeMap<&Uid, &RActiveSlot> {
        self.outgoing_slots
            .iter()
            .map(|(k, v)| (k, v.as_ref()))
            .collect()
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
    fn fields(&self) -> &std::collections::HashMap<Uid, RwSignal<PrimitiveValues>> {
        &self.fields
    }
}
impl<T: HasSlotEnum, TSchema> std::fmt::Display for RGSOConcrete<T, TSchema> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.operative().tag.name)
    }
}
// impl<T: HasSlotEnum, TSchema: 'static> Serialize for RGSOConcrete<T, TSchema> {
//     fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer {
//             let mut s = serializer.serialize_struct("RGSOConcrete", 3)?;
//                     s.serialize_field("name", &self.name)?;
//                     s.serialize_field("age", &self.age)?;
//                     s.serialize_field("phones", &self.phones)?;
//                     s.end()
//     }
// }

#[derive(Clone, Debug)]
pub struct RGSOConcreteBuilder<T: HasSlotEnum, TSchema: 'static> {
    id: Uid,
    slots: std::collections::BTreeMap<Uid, SpecializedRActiveSlot<T::SlotEnum>>,
    incoming_slots: RwSignal<Vec<SlotRef>>,
    pub data: std::collections::HashMap<Uid, RwSignal<Option<RwSignal<PrimitiveValues>>>>,
    operative: &'static LibraryOperative<PrimitiveTypes, PrimitiveValues>,
    template: &'static LibraryTemplate<PrimitiveTypes, PrimitiveValues>,
    graph: std::sync::Arc<RBaseGraphEnvironment<TSchema>>,
    temp_id: String,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: HasSlotEnum, TSchema> RGSOConcreteBuilder<T, TSchema> {
    pub fn new(
        data: std::collections::HashMap<Uid, RwSignal<Option<RwSignal<PrimitiveValues>>>>,
        slots: Option<std::collections::BTreeMap<Uid, SpecializedRActiveSlot<T::SlotEnum>>>,
        operative: &'static LibraryOperative<PrimitiveTypes, PrimitiveValues>,
        template: &'static LibraryTemplate<PrimitiveTypes, PrimitiveValues>,
        graph: std::sync::Arc<RBaseGraphEnvironment<TSchema>>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().as_u128(),
            slots: slots.unwrap_or_default(),
            incoming_slots: RwSignal::new(Vec::new()),
            data,
            operative,
            template,
            graph,
            _phantom: std::marker::PhantomData,
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
    pub fn get_temp_id(&self) -> &String {
        &self.temp_id
    }
}
impl<T: HasSlotEnum, TSchema> RProducable<RGSOConcrete<T, TSchema>>
    for RGSOConcreteBuilder<T, TSchema>
where
    <T as HasSlotEnum>::SlotEnum: Clone,
{
    type Schema = TSchema;
    fn produce(&self) -> RGSOConcrete<T, TSchema> {
        RGSOConcrete::<T, TSchema> {
            id: self.id,
            outgoing_slots: self.slots.clone(),
            incoming_slots: self.incoming_slots,
            graph: self.graph.clone(),
            fields: self
                .data
                .iter()
                .map(|(id, build_data)| (*id, build_data.get().unwrap()))
                .collect::<std::collections::HashMap<Uid, RwSignal<PrimitiveValues>>>(),
            operative: self.operative,
            template: self.template,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: HasSlotEnum, TSchema> Verifiable for RGSOConcreteBuilder<T, TSchema> {
    fn verify(&self) -> Result<(), crate::post_generation::ElementCreationError> {
        let field_errors = self
            .data
            .values()
            .filter_map(|field_val| {
                if field_val.with(|field_val| field_val.is_none()) {
                    return Some(ElementCreationError::RequiredFieldIsEmpty);
                }
                None
            })
            .collect::<Vec<_>>();
        let slot_errors = self
            .slots
            .values()
            .filter_map(|active_slot| {
                if !active_slot.check_current_conformity() {
                    Some(ElementCreationError::BoundCheckOutOfRange(format!(
                        "{}: {}\nBounds: {:?}, Attempted: {}",
                        self.operative.tag.name,
                        active_slot.slot.tag.name,
                        active_slot.slot.bounds,
                        active_slot.slotted_instances.get().len()
                    )))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let mut all_errors = Vec::new();
        all_errors.extend(field_errors);
        all_errors.extend(slot_errors);
        if !all_errors.is_empty() {
            leptos::logging::log!("{:#?}", all_errors);
            return Err(ElementCreationError::Stack(all_errors));
        }
        Ok(())
    }
}

pub trait RBuildable
where
    Self: Sized + Clone + std::fmt::Debug + 'static + HasSlotEnum,
    <Self as HasSlotEnum>::SlotEnum: Clone + std::fmt::Debug,
{
    type Schema;

    fn initiate_build(
        graph: impl Into<std::sync::Arc<RBaseGraphEnvironment<Self::Schema>>>,
    ) -> SubgraphBuilder<Self, Self::Schema>;
    fn initiate_edit(
        id: Uid,
        graph: impl Into<std::sync::Arc<RBaseGraphEnvironment<Self::Schema>>>,
    ) -> SubgraphBuilder<Self, Self::Schema>;
    fn get_operative_id() -> Uid;
}

pub trait RInstantiable: Send + Sync {
    type Schema;

    fn instantiate(&self) -> Result<Self::Schema, crate::post_generation::ElementCreationError>;
    fn get_id(&self) -> &Uid;
    fn get_temp_id(&self) -> &String;
    fn get_template(&self) -> &'static LibraryTemplate<PrimitiveTypes, PrimitiveValues>;
    fn add_incoming(&mut self, host_id: &Uid, slot_id: &Uid);
    fn add_outgoing(&mut self, target_id: &Uid, slot_id: &Uid);
}
// type RInstantiableElements<TSchema> = Vec<std::sync::Arc<dyn RInstantiable<Schema = TSchema>>>;

#[derive(Clone, Debug)]
pub struct Blueprint<TSchema> {
    added_instances: Vec<TSchema>,
    deleted_instances: Vec<TSchema>,
    add_outgoing_updates: std::collections::HashSet<(Uid, SlotRef)>,
    remove_outgoing_updates: std::collections::HashSet<(Uid, SlotRef)>,
    add_incoming_updates: std::collections::HashSet<(Uid, SlotRef)>,
    remove_incoming_updates: std::collections::HashSet<(Uid, SlotRef)>,
    field_updates: std::collections::HashSet<(Uid, HistoryFieldEdit)>,
    action_tag: Option<TaggedAction>,
}
impl<TSchema> Blueprint<TSchema> {
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
pub struct TempAddIncomingSlotRef {
    pub host_instance_id: BlueprintId,
    pub slot_id: Uid,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TempAddOutgoingSlotRef {
    pub target_instance_id: BlueprintId,
    pub slot_id: Uid,
}

pub struct ExecutionResult {
    pub temp_id_map: std::collections::HashMap<String, Uid>,
}
impl ExecutionResult {
    pub fn get_final_id(&self, temp_id: &str) -> Option<&Uid> {
        self.temp_id_map.get(temp_id)
    }
}

#[derive(Debug, Clone)]
pub struct SubgraphBuilder<T: HasSlotEnum + Clone + std::fmt::Debug, TSchema: 'static>
where
    <T as HasSlotEnum>::SlotEnum: std::fmt::Debug + Clone,
{
    pub instantiables:
        RwSignal<Vec<std::sync::Arc<std::sync::Mutex<dyn RInstantiable<Schema = TSchema>>>>>,
    pub cumulative_errors: RwSignal<std::vec::Vec<ElementCreationError>>,
    pub add_outgoing_updates: RwSignal<std::collections::HashSet<(Uid, SlotRef)>>,
    pub add_incoming_updates: RwSignal<std::collections::HashSet<(Uid, SlotRef)>>,
    pub remove_outgoing_updates: RwSignal<std::collections::HashSet<(Uid, SlotRef)>>,
    pub remove_incoming_updates: RwSignal<std::collections::HashSet<(Uid, SlotRef)>>,
    pub deleted_instances: RwSignal<std::collections::HashSet<Uid>>,
    pub to_delete_recursive: RwSignal<std::collections::HashSet<Uid>>,
    pub field_updates: RwSignal<std::collections::HashSet<(Uid, HistoryFieldEdit)>>,
    pub temp_add_incoming_updates:
        RwSignal<std::collections::HashSet<(String, TempAddIncomingSlotRef)>>,
    pub temp_add_outgoing_updates:
        RwSignal<std::collections::HashSet<(BlueprintId, TempAddOutgoingSlotRef)>>,
    // If I remember correctly, If there is a wip_instance, it means that this subgraphbuilder
    // represents a new item (rather than an edit)
    pub wip_instance: Option<RGSOConcreteBuilder<T, TSchema>>,
    pub id: Uid,
    pub graph: std::sync::Arc<RBaseGraphEnvironment<TSchema>>,
    pub _phantom: std::marker::PhantomData<T>,
}

impl<T: Send + Sync + HasSlotEnum, TSchema: EditRGSO + Send + Sync> SubgraphBuilder<T, TSchema>
where
    RGSOConcreteBuilder<T, TSchema>: RProducable<RGSOConcrete<T, TSchema>>,
    T: RIntoSchema<Schema = TSchema> + Clone + std::fmt::Debug + 'static,
    <T as HasSlotEnum>::SlotEnum: std::fmt::Debug + Clone + Send + Sync,
{
    // -------------
    // To be exposed
    // -------------
    pub fn get_id(&self) -> &Uid {
        &self.id
    }
    pub fn execute(&self) -> Result<ExecutionResult, ElementCreationError> {
        let graph = self.graph.clone();
        let (blueprint, execution_result) = self.clone().get_blueprint()?;
        graph.clear_redo();
        graph.push_undo(blueprint.clone());
        graph.process_blueprint(blueprint);
        Ok(execution_result)
    }
    pub fn incorporate<
        C: std::fmt::Debug
            + Clone
            + RIntoSchema<Schema = TSchema>
            + 'static
            + Send
            + Sync
            + HasSlotEnum,
    >(
        &mut self,
        other_builder: &SubgraphBuilder<C, TSchema>,
    ) where
        <C as HasSlotEnum>::SlotEnum: std::fmt::Debug + Clone + Send + Sync,
    {
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
                prev.push(std::sync::Arc::new(std::sync::Mutex::new(inner)));
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
        self.cumulative_errors
            .update(|prev| prev.extend(other_builder.cumulative_errors.get()));
    }
    pub fn set_temp_id(&mut self, temp_id: &str) -> &mut Self {
        if let Some(wip_instance) = &mut self.wip_instance {
            wip_instance.set_temp_id(temp_id);
        }
        self
    }

    // -------------
    // To be private
    // -------------
    pub fn delete_recursive_handler(&mut self, id: &Uid, is_root_deletion: bool) {
        let item = self.graph.get(id).unwrap();
        let pending_incoming_removals = self.remove_incoming_updates.with(|remove_updates| {
            remove_updates
                .iter()
                .filter(|update| update.0 == *id)
                .collect::<std::collections::HashSet<_>>()
                .len()
        });
        let pending_incoming_additions = self.add_incoming_updates.with(|add_updates| {
            add_updates
                .iter()
                .filter(|update| update.0 == *id)
                .collect::<std::collections::HashSet<_>>()
                .len()
        });
        leptos::logging::log!("{}: \nIncoming Slots: {}\nPending incoming additions: {}\nPending incoming removals: {}", item.template().tag.name, item.incoming_slots().get().len(), pending_incoming_additions, pending_incoming_removals);
        leptos::logging::log!("{:#?}", item.outgoing_slots().values());
        let total_incoming = item.incoming_slots().get().len() + pending_incoming_additions;
        if (total_incoming == pending_incoming_removals) || is_root_deletion {
            self.delete(id);
            let slotted_instances = item
                .outgoing_slots()
                .values()
                .flat_map(|slot| slot.slotted_instances.get())
                .for_each(|instance_id| self.delete_recursive_handler(&instance_id, false));
            slotted_instances
        }
    }
    // Perform final calculations to gather all changes
    pub fn get_blueprint(
        mut self,
    ) -> Result<(Blueprint<TSchema>, ExecutionResult), ElementCreationError> {
        let mut all_errors = self.cumulative_errors.get();
        let mut new_instantiables = self.instantiables.get();
        if let Some(instance) = &self.wip_instance {
            new_instantiables.push(std::sync::Arc::new(std::sync::Mutex::new(instance.clone())));
        }

        let temp_id_map = new_instantiables
            .iter()
            .map(|instantiable| {
                let (id, temp_id) = {
                    let lock = instantiable.lock().unwrap();
                    (lock.get_id().clone(), lock.get_temp_id().clone())
                };
                (temp_id, id)
            })
            .collect::<std::collections::HashMap<_, _>>();

        // Perform any incoming or outgoing updates for temporary ids
        // To explain the process a bit, when a new node is created via a FreshBuilder, an
        // instantiable is created representing that node, and that instantiable is carried along
        // with the bundle of all instantiables through the whole subgraphbuilder process.
        // When a user references a temp id somewhere else in the graph, the instantiable has to be
        // updated to reflect whatever edge is created, but you can't be sure that the instantiable
        // has actually been created yet because the user can reference the tempid at any point in
        // the tree.
        // TODO: Known issue -- when temp_id is not set very first, there can be some sequences of actions
        // which cause the old "temp_id" (which is the actual id) to be stored for certain updates
        // When that old id is searched for in the temp map it is no longer there because it is replaced with a user-provided temp-id.
        let temp_incoming_execution_errors = self.temp_add_incoming_updates.with(|updates| {
            updates
                .iter()
                .filter_map(|update| {
                    let final_host_id = match &update.1.host_instance_id {
                        BlueprintId::Existing(existing_id) => Ok(*existing_id),
                        BlueprintId::Temporary(temp_id) => temp_id_map
                            .get(temp_id)
                            .cloned()
                            .ok_or_else(|| ElementCreationError::NonexistentTempId {
                                temp_id: temp_id.clone(),
                            }),
                    };
                    if let Some(error) = final_host_id.clone().err() {
                        return Some(error);
                    }
                    let final_host_id = final_host_id.unwrap();
                    if let Some(instantiable) = new_instantiables.iter_mut().find(|instantiable| {
                        instantiable.lock().unwrap().get_temp_id() == &update.0
                    }) {
                        instantiable
                            .as_ref()
                            .lock()
                            .unwrap()
                            .add_incoming(&final_host_id, &update.1.slot_id);
                    } else {
                        return Some(ElementCreationError::NonexistentTempId {
                            temp_id: update.0.clone(),
                        });
                    };
                    None
                })
                .collect::<Vec<_>>()
        });
        all_errors.extend(temp_incoming_execution_errors);

        // Similarly, temp_outgoing refers to connections that need to be made but where the target
        // was only known by temp_id and not guaranteed to exist at the time. Since new nodes and
        // existing nodes are handled differently here (new nodes have to have their instantiable
        // updated whereas existing nodes just need to update the `outgoing updates`), that
        // information is carried in the temp_add_outgoing_updates.
        let temp_outgoing_execution_errors = self.temp_add_outgoing_updates.with(|updates| {
            updates
                .iter()
                .filter_map(|update| {
                    let final_target_id = match &update.1.target_instance_id {
                        BlueprintId::Existing(existing_id) => Ok(*existing_id),
                        BlueprintId::Temporary(temp_id) => temp_id_map
                            .get(temp_id)
                            .ok_or(ElementCreationError::NonexistentTempId {
                                temp_id: temp_id.clone(),
                            })
                            .cloned(),
                    };
                    if let Some(error) = final_target_id.clone().err() {
                        return Some(error);
                    }
                    let final_target_id = final_target_id.unwrap();

                    match &update.0 {
                        BlueprintId::Existing(existing_id) => {
                            self.add_outgoing_updates.update(|prev| {
                                prev.insert((
                                    *existing_id,
                                    SlotRef {
                                        target_instance_id: final_target_id,
                                        host_instance_id: *existing_id,
                                        slot_id: update.1.slot_id,
                                    },
                                ));
                            });
                        }
                        BlueprintId::Temporary(temp_id) => {
                            if let Some(instantiable) =
                                new_instantiables.iter_mut().find(|instantiable| {
                                    instantiable.lock().unwrap().get_temp_id() == temp_id
                                })
                            {
                                instantiable
                                    .as_ref()
                                    .lock()
                                    .unwrap()
                                    .add_outgoing(&final_target_id, &update.1.slot_id);
                            }
                        }
                    };
                    None
                })
                .collect::<Vec<_>>()
        });
        all_errors.extend(temp_outgoing_execution_errors);

        // Run through each node which was marked to be deleted recursively and add them to the delete list
        // Checks to see if slotted instances have any other references (incoming slots) and deletes them if no
        // and continues to check their children (and so on)
        //
        // TODO: This currently is not as smart or robust as it could be -- could use some investigation.
        // Starter point: if two nodes have only each other slotted as unaddressed slots, `delete_recursive_handler` currently
        // would never delete either of them (unless the parent was a root delete node) because the first would have an unaddressed
        // incoming edge (from the second) which would prevent the algorithm from continuing to the second and checking it.
        self.to_delete_recursive
            .get()
            .iter()
            .for_each(|to_delete_id| {
                self.delete_recursive_handler(to_delete_id, true);
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
                        created_instances.get(deleted_instance_id).unwrap().clone()
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
                                            .filter(|addition| addition.1.slot_id == **slot.0)
                                            .collect::<Vec<_>>()
                                            .len()
                                        - all_removals
                                            .clone()
                                            .filter(|addition| addition.1.slot_id == **slot.0)
                                            .collect::<Vec<_>>()
                                            .len();
                                    if !slot.1.check_bound_conformity(final_count) {
                                        Some(ElementCreationError::BoundCheckOutOfRange(format!(
                                            "{}: {}\nBounds: {:?}, Attempted: {}",
                                            self.graph.get(&update.0).unwrap().operative().tag.name,
                                            slot.1.slot.tag.name,
                                            slot.1.slot.bounds,
                                            final_count
                                        )))
                                    } else {
                                        None
                                    }
                                })
                                .collect::<Vec<_>>();
                            if errors.is_empty() {
                                None
                            } else {
                                Some(errors)
                            }
                        })
                        .flatten()
                        .collect::<Vec<_>>();
                    errors
                })
            });

        if !bounds_checks.is_empty() {
            leptos::logging::log!("{:#?}", bounds_checks);
            return Err(ElementCreationError::Stack(bounds_checks));
        }

        let (instantiated_elements, instantiation_errors) = new_instantiables.iter().fold(
            (Vec::with_capacity(new_instantiables.len()), Vec::new()),
            |mut agg, el| {
                match el.lock().unwrap().instantiate() {
                    Ok(instance) => agg.0.push(instance),
                    Err(error) => agg.1.push(error),
                }
                agg
            },
        );

        all_errors.extend(instantiation_errors);
        if !all_errors.is_empty() {
            return Err(ElementCreationError::Stack(all_errors));
        }

        let blueprint = Blueprint::<TSchema> {
            added_instances: instantiated_elements,
            add_outgoing_updates: self.add_outgoing_updates.get(),
            add_incoming_updates: self.add_incoming_updates.get(),
            remove_outgoing_updates: self.remove_outgoing_updates.get(),
            remove_incoming_updates: self.remove_incoming_updates.get(),
            deleted_instances: cloned_delete_instances,
            field_updates: self.field_updates.get(),
            action_tag: None,
        };
        Ok((blueprint, ExecutionResult { temp_id_map }))
    }
    pub fn get_graph(&self) -> &std::sync::Arc<RBaseGraphEnvironment<TSchema>> {
        &self.graph
    }
    pub fn new(
        builder_wrapper_instance: Option<RGSOConcreteBuilder<T, TSchema>>,
        id: Uid,
        graph: std::sync::Arc<RBaseGraphEnvironment<TSchema>>,
    ) -> Self {
        Self {
            graph,
            instantiables: RwSignal::new(vec![]),
            wip_instance: builder_wrapper_instance,
            id,
            cumulative_errors: RwSignal::new(std::vec::Vec::new()),
            add_outgoing_updates: RwSignal::new(std::collections::HashSet::new()),
            add_incoming_updates: RwSignal::new(std::collections::HashSet::new()),
            remove_outgoing_updates: RwSignal::new(std::collections::HashSet::new()),
            remove_incoming_updates: RwSignal::new(std::collections::HashSet::new()),
            temp_add_incoming_updates: RwSignal::new(std::collections::HashSet::new()),
            temp_add_outgoing_updates: RwSignal::new(std::collections::HashSet::new()),
            _phantom: std::marker::PhantomData,
            field_updates: RwSignal::new(std::collections::HashSet::new()),
            deleted_instances: RwSignal::new(std::collections::HashSet::new()),
            to_delete_recursive: RwSignal::new(std::collections::HashSet::new()),
        }
    }
    pub fn raw_add_outgoing_to_updates(&mut self, slot_ref: SlotRef) {
        self.add_outgoing_updates.update(|prev| {
            prev.insert((slot_ref.host_instance_id, slot_ref));
        });
    }
    pub fn raw_add_incoming_to_updates(&mut self, slot_ref: SlotRef) {
        self.add_incoming_updates.update(|prev| {
            prev.insert((slot_ref.target_instance_id, slot_ref));
        });
    }
    pub fn add_outgoing<
        C: std::fmt::Debug
            + Clone
            + RIntoSchema<Schema = TSchema>
            + 'static
            + Send
            + Sync
            + HasSlotEnum,
    >(
        &mut self,
        slot_id: &Uid,
        // BlueprintId in this case meaning that:
        //    Existing: The ID is known
        //    Temporary: The ID is non known, only the temp_id
        target_id: BlueprintId,
        instantiable: Option<SubgraphBuilder<C, TSchema>>,
    ) where
        <C as HasSlotEnum>::SlotEnum: Clone + std::fmt::Debug + Send + Sync,
    {
        // If this is a newly created instance
        if let Some(instance) = &self.wip_instance {
            match &target_id {
                BlueprintId::Existing(existing_target_id) => {
                    let slot = instance.slots.get(slot_id).unwrap();
                    slot.slotted_instances.update(|prev| {
                        prev.push(*existing_target_id);
                    });
                }
                BlueprintId::Temporary(_temp_target_id) => {
                    self.temp_add_outgoing(
                        BlueprintId::Temporary(instance.get_temp_id().clone()),
                        TempAddOutgoingSlotRef {
                            target_instance_id: target_id.clone(),
                            slot_id: *slot_id,
                        },
                    );
                }
            }
        // If this is an existing element being edited
        } else {
            match &target_id {
                BlueprintId::Existing(existing_target_id) => {
                    self.raw_add_outgoing_to_updates(SlotRef {
                        host_instance_id: self.id,
                        target_instance_id: *existing_target_id,
                        slot_id: *slot_id,
                    });
                }
                BlueprintId::Temporary(_temp_target_id) => self.temp_add_outgoing(
                    BlueprintId::Existing(*self.get_id()),
                    TempAddOutgoingSlotRef {
                        target_instance_id: target_id,
                        slot_id: *slot_id,
                    },
                ),
            }
        }
        if let Some(instantiable) = instantiable {
            self.incorporate(&instantiable);
        }
    }
    pub fn remove_outgoing(&mut self, slot_ref: SlotRef) {
        self.remove_outgoing_updates.update(|prev| {
            prev.insert((slot_ref.host_instance_id, slot_ref.clone()));
        });
        self.remove_incoming_updates.update(|prev| {
            prev.insert((slot_ref.target_instance_id, slot_ref));
        });
    }
    pub fn add_incoming<
        C: std::fmt::Debug
            + Clone
            + RIntoSchema<Schema = TSchema>
            + 'static
            + Send
            + Sync
            + HasSlotEnum,
    >(
        &mut self,
        slot_ref: SlotRef,
        instantiable: Option<SubgraphBuilder<C, TSchema>>,
    ) where
        <C as HasSlotEnum>::SlotEnum: Clone + std::fmt::Debug + Send + Sync,
    {
        if let Some(instance) = &self.wip_instance {
            instance
                .incoming_slots
                .update(|prev| prev.push(slot_ref.clone()))
        } else {
            self.add_incoming_updates.update(|prev| {
                prev.insert((slot_ref.target_instance_id, slot_ref));
            });
        }
        if let Some(instantiable) = instantiable {
            self.incorporate(&instantiable);
        }
    }
    pub fn edit_field(&mut self, field_id: Uid, value: PrimitiveValues) {
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
                // TODO: It seems like there could be a better way to do this than looking up the
                // value like this.
                // At the very least, for ExistingBuilders which are entered into with `.edit()`,
                // we would have access to the concrete node at the time of `ExistingBuilder` creation.
                // The difficulty comes in that ExistingBuilders are also created through the process
                // of adding some outgoing node, in which case we'd only have the id and would still
                // have to do the lookup like this at some point.
                let prev_value = self
                    .graph
                    .get(self.get_id())
                    .unwrap()
                    .fields()
                    .get(&field_id)
                    .unwrap()
                    .get();
                prev.insert((
                    *self.get_id(),
                    HistoryFieldEdit {
                        instance_id: *self.get_id(),
                        field_id,
                        new_value: value,
                        prev_value,
                    },
                ));
            })
        }
    }

    // Add the node to the delete list and remove any references to or from it
    pub fn delete(&mut self, to_delete_id: &Uid) {
        self.deleted_instances.update(|prev| {
            prev.insert(*to_delete_id);
        });
        let existing_instance = self.graph.get(to_delete_id).unwrap();
        self.remove_outgoing_updates.update(|removes| {
            existing_instance.incoming_slots().with(|incoming_slots| {
                incoming_slots.iter().for_each(|incoming_slot| {
                    removes.insert((incoming_slot.host_instance_id, incoming_slot.clone()));
                })
            })
        });
        self.remove_incoming_updates.update(|removes| {
            existing_instance
                .outgoing_slots()
                .values()
                .for_each(|slot| {
                    slot.slotted_instances.with(|slotted_instances| {
                        slotted_instances.iter().for_each(|target_instance_id| {
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
    pub fn delete_recursive(&mut self) {
        self.to_delete_recursive.update(|prev| {
            prev.insert(self.id);
        });
    }
    pub fn temp_add_incoming(&mut self, host_id: &str, temp_slot_ref: TempAddIncomingSlotRef) {
        self.temp_add_incoming_updates
            .update(|temp_add_incoming_updates| {
                temp_add_incoming_updates.insert((host_id.to_string(), temp_slot_ref));
            });
    }
    pub fn temp_add_outgoing(
        &mut self,
        target_id: BlueprintId,
        temp_slot_ref: TempAddOutgoingSlotRef,
    ) {
        self.temp_add_outgoing_updates
            .update(|temp_add_outgoing_updates| {
                temp_add_outgoing_updates.insert((target_id, temp_slot_ref));
            });
    }
    pub fn add_error(&mut self, error: ElementCreationError) {
        self.cumulative_errors.update(|prev| prev.push(error));
    }
}

impl<T: Send + Sync + HasSlotEnum, TSchema: Send + Sync + 'static> RInstantiable
    for RGSOConcreteBuilder<T, TSchema>
where
    T: RIntoSchema<Schema = TSchema> + 'static,
    <T as HasSlotEnum>::SlotEnum: Clone + std::fmt::Debug + Send + Sync,
{
    type Schema = TSchema;

    fn instantiate(&self) -> Result<Self::Schema, crate::post_generation::ElementCreationError> {
        self.verify()?;
        Ok(T::into_schema(self.produce()))
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
                host_instance_id: *host_id,
                slot_id: *slot_id,
                target_instance_id: self.id,
            })
        });
    }

    fn add_outgoing(&mut self, target_id: &Uid, slot_id: &Uid) {
        self.slots
            .get(slot_id)
            .unwrap()
            .slotted_instances
            .update(|slotted_instances| slotted_instances.push(*target_id));
    }
}

pub trait RIntoSchema
where
    Self: Sized + HasSlotEnum,
{
    type Schema: EditRGSO;
    fn into_schema(instantiable: RGSOConcrete<Self, Self::Schema>) -> Self::Schema;
}

pub trait REditable<T>
where
    Self: Sized,
    T: std::clone::Clone + std::fmt::Debug + HasSlotEnum,
    <T as HasSlotEnum>::SlotEnum: Clone + std::fmt::Debug,
{
    type Schema;
    fn initiate_edit(&self) -> SubgraphBuilder<T, Self::Schema>;
}
impl<T, TSchema> REditable<T> for RGSOConcrete<T, TSchema>
where
    T: RIntoSchema<Schema = TSchema> + RBuildable<Schema = TSchema> + 'static + HasSlotEnum,
    <T as HasSlotEnum>::SlotEnum: Clone + std::fmt::Debug,
{
    type Schema = TSchema;
    fn initiate_edit(&self) -> SubgraphBuilder<T, Self::Schema> {
        T::initiate_edit(*self.get_id(), self.get_graph().clone())
    }
}

#[derive(Clone, Debug)]
pub struct SharedGraph<TSchema: 'static>(std::sync::Arc<RBaseGraphEnvironment<TSchema>>);
impl<TSchema> From<std::sync::Arc<RBaseGraphEnvironment<TSchema>>> for SharedGraph<TSchema> {
    fn from(value: std::sync::Arc<RBaseGraphEnvironment<TSchema>>) -> SharedGraph<TSchema> {
        SharedGraph(value)
    }
}
impl<TSchema> From<SharedGraph<TSchema>> for std::sync::Arc<RBaseGraphEnvironment<TSchema>> {
    fn from(value: SharedGraph<TSchema>) -> std::sync::Arc<RBaseGraphEnvironment<TSchema>> {
        value.0
    }
}
impl<TSchema> std::ops::Deref for SharedGraph<TSchema> {
    type Target = std::sync::Arc<RBaseGraphEnvironment<TSchema>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait HasSlotEnum {
    type SlotEnum: Send + Sync + Clone + std::fmt::Debug;
}

impl<T: HasSlotEnum, TSchema> RGSOConcrete<T, TSchema>
where
    <T as HasSlotEnum>::SlotEnum: PartialEq,
{
    pub fn outgoing_slots_with_enum(
        &self,
    ) -> &std::collections::BTreeMap<Uid, SpecializedRActiveSlot<T::SlotEnum>> {
        &self.outgoing_slots
    }
    pub fn slot_by_enum(&self, variant: T::SlotEnum) -> &SpecializedRActiveSlot<T::SlotEnum> {
        self.outgoing_slots_with_enum()
            .values()
            .find(|slot| slot.slot_enum == variant)
            .unwrap()
    }
}

pub mod from_reactive {
    use std::{
        collections::HashMap,
        sync::{Arc, Mutex},
    };

    use super::{HasSlotEnum, SharedGraph, SpecializedRActiveSlot};
    use crate::post_generation::{BaseGraphEnvironment, GSOConcrete};
    use leptos::prelude::*;

    use super::{
        FromNonReactive, RActiveSlot, RBaseGraphEnvironment, RGSOConcrete, RHistoryContainer,
    };
    impl<RTSchema: Send + Sync, TSchema: Send + Sync> From<SharedGraph<RTSchema>>
        for BaseGraphEnvironment<TSchema>
    where
        RTSchema: Into<TSchema> + Clone,
    {
        fn from(value: SharedGraph<RTSchema>) -> Self {
            Self {
                created_instances: value
                    .0
                    .created_instances
                    .get()
                    .into_iter()
                    .map(|(id, val)| (id, val.into()))
                    .collect(),
                constraint_schema: value.0.constraint_schema,
            }
        }
    }
    impl<RTSchema: Send + Sync, TSchema: Send + Sync> From<BaseGraphEnvironment<TSchema>>
        for SharedGraph<RTSchema>
    where
        RTSchema: FromNonReactive<TSchema>,
    {
        fn from(value: BaseGraphEnvironment<TSchema>) -> Self {
            let new_graph = RBaseGraphEnvironment::<RTSchema> {
                created_instances: RwSignal::new(HashMap::new()),
                constraint_schema: value.constraint_schema,
                history: Arc::new(Mutex::new(RHistoryContainer {
                    undo: vec![],
                    redo: vec![],
                })),
            };
            let rc_graph = Arc::new(new_graph);
            let members = value.created_instances.into_iter().map(|(id, val)| {
                (
                    id,
                    <RTSchema as FromNonReactive<TSchema>>::from_non_reactive(
                        val,
                        rc_graph.clone(),
                    ),
                )
            });
            rc_graph.initialize(members.collect());
            rc_graph.into()
        }
    }
    impl From<RActiveSlot> for crate::post_generation::ActiveSlot {
        fn from(value: RActiveSlot) -> Self {
            Self {
                slot: value.slot,
                slotted_instances: value.slotted_instances.get(),
            }
        }
    }
    impl From<crate::post_generation::ActiveSlot> for RActiveSlot {
        fn from(value: crate::post_generation::ActiveSlot) -> Self {
            Self {
                slot: value.slot,
                slotted_instances: RwSignal::new(value.slotted_instances),
            }
        }
    }
    impl<T, RTSchema> From<RGSOConcrete<T, RTSchema>> for GSOConcrete<T>
    where
        T: Clone + std::fmt::Debug + super::HasSlotEnum,
    {
        fn from(value: RGSOConcrete<T, RTSchema>) -> Self {
            Self {
                id: value.id,
                fields: value
                    .fields
                    .into_iter()
                    .map(|(id, val)| (id, val.get()))
                    .collect(),
                outgoing_slots: value
                    .outgoing_slots
                    .into_iter()
                    .map(|(id, val)| {
                        (
                            id,
                            <SpecializedRActiveSlot<<T as HasSlotEnum>::SlotEnum> as Into<
                                RActiveSlot,
                            >>::into(val)
                            .into(),
                        )
                    })
                    .collect(),
                incoming_slots: value.incoming_slots.get().into_iter().collect(),

                operative: value.operative,
                template: value.template,
                _phantom: std::marker::PhantomData,
            }
        }
    }

    pub trait FromStandalone {
        type Schema;
        fn from_standalone(
            value: crate::post_generation::StandaloneRGSOConcrete,
            graph: SharedGraph<Self::Schema>,
        ) -> Self::Schema;
    }
}

pub trait RootConstraints<TSchema>:
    HasSlotEnum
    + Send
    + Sync
    + Clone
    + RBuildable<Schema = TSchema>
    + RIntoSchema<Schema = TSchema>
    + 'static
{
}
impl<T, TSchema> RootConstraints<TSchema> for T where
    T: HasSlotEnum
        + Send
        + Sync
        + Clone
        + RBuildable<Schema = TSchema>
        + RIntoSchema<Schema = TSchema>
        + 'static
{
}

pub trait Incorporatable<T: std::clone::Clone + std::fmt::Debug + HasSlotEnum, TSchema>
where
    <T as HasSlotEnum>::SlotEnum: std::clone::Clone + std::fmt::Debug + Send + Sync,
{
    fn get_inner_builder(&self) -> &SubgraphBuilder<T, TSchema>;
}

impl<T, U: std::clone::Clone + std::fmt::Debug + HasSlotEnum, V> Incorporatable<U, V> for &T
where
    T: Incorporatable<U, V>,
{
    fn get_inner_builder(&self) -> &SubgraphBuilder<U, V> {
        (**self).get_inner_builder()
    }
}
impl<T: RootConstraints<TSchema>, TSchema> Incorporatable<T, TSchema>
    for Box<dyn Incorporatable<T, TSchema>>
{
    fn get_inner_builder(&self) -> &SubgraphBuilder<T, TSchema> {
        self.as_ref().get_inner_builder()
    }
}
