use std::fmt;

use std::{any::Any, cell::RefCell, collections::HashMap, marker::PhantomData, rc::Rc};
use strum_macros::Display;

use crate::common::StrUid;
use crate::{
    common::{ConstraintTraits, Uid},
    constraint_schema::{
        ConstraintSchema, LibraryOperative, LibraryTemplate, OperativeSlot, SlotBounds,
    },
    primitives::{PrimitiveTypes, PrimitiveValues},
};

pub type LibOp = LibraryOperative<PrimitiveTypes, PrimitiveValues>;
pub type LibTemplate = LibraryTemplate<PrimitiveTypes, PrimitiveValues>;

type Error = ElementCreationError;
#[derive(Debug, Display, Clone)]
pub enum ElementCreationError {
    RequiredFieldIsEmpty,
    BoundCheckOutOfRange(String),
    OutgoingElementIsWrongType { expected: String, recieved: String },
    OutgoingElementDoesntExist { id: Uid },
    NonexistentTempId { temp_id: String },
    DeletionError,
    Stack(Vec<ElementCreationError>),
}
impl std::error::Error for ElementCreationError {}

#[derive(Debug, Clone)]
pub enum TaggedAction {
    Normal,
    Undo,
    Redo,
}

pub type HistoryStack<TSchema> = Vec<Vec<HistoryItem<TSchema>>>;
pub type HistoryRef<TSchema> = Rc<RefCell<HistoryContainer<TSchema>>>;

// #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct BaseGraphEnvironment<TSchema> {
    pub created_instances: HashMap<Uid, TSchema>,
    // #[serde(skip)]
    pub constraint_schema: &'static ConstraintSchema<PrimitiveTypes, PrimitiveValues>,
}
impl<TSchema: 'static> BaseGraphEnvironment<TSchema> {
    pub fn new(
        constraint_schema: &'static ConstraintSchema<PrimitiveTypes, PrimitiveValues>,
    ) -> Self {
        Self {
            created_instances: HashMap::new(),
            constraint_schema,
        }
    }
}

pub trait GraphEnvironment {
    type Types: ConstraintTraits;
    type Values: ConstraintTraits;
    type Schema: 'static;

    fn get(&self, id: &Uid) -> Option<&Self::Schema>;
    fn create_connection(&mut self, connection: ConnectionAction) -> Result<(), Error>;
    fn instantiate_element<T>(
        &mut self,
        element: InstantiableWrapper<GSOConcrete<T>, Self::Schema>,
    ) -> Result<Uid, Error>
    where
        GSOConcrete<T>: Instantiable<Schema = Self::Schema>,
        Self: Sized,
        T: std::fmt::Debug + Clone + 'static;
    fn get_mut(&mut self, id: &Uid) -> Option<&mut Self::Schema>;
    fn get_constraint_schema(&self) -> &ConstraintSchema<Self::Types, Self::Values>;
    fn delete(&mut self, id: &Uid) -> Result<(), Error>;
    fn undo(&mut self);
    fn redo(&mut self);
}

#[derive(Debug, Clone)]
pub enum HistoryItem<TSchema> {
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

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StrSlotRef {
    pub host_instance_id: StrUid,
    pub target_instance_id: StrUid,
    pub slot_id: StrUid,
}
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SlotRef {
    pub host_instance_id: Uid,
    pub target_instance_id: Uid,
    pub slot_id: Uid,
}
impl From<SlotRef> for StrSlotRef {
    fn from(value: SlotRef) -> Self {
        Self {
            host_instance_id: value.host_instance_id.into(),
            target_instance_id: value.target_instance_id.into(),
            slot_id: value.slot_id.into(),
        }
    }
}
impl From<StrSlotRef> for SlotRef {
    fn from(value: StrSlotRef) -> Self {
        Self {
            host_instance_id: value.host_instance_id.into(),
            target_instance_id: value.target_instance_id.into(),
            slot_id: value.slot_id.into(),
        }
    }
}

pub trait Slotted {}

// #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
            base_types::post_generation::ActiveSlot {
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
pub struct HistoryContainer<TSchema> {
    pub undo: HistoryStack<TSchema>,
    pub redo: HistoryStack<TSchema>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
/// Normalizes GSOWrapper for network transfer
pub struct StandaloneRGSOConcrete {
    pub id: Uid,
    pub fields: std::collections::HashMap<Uid, PrimitiveValues>,
    pub outgoing_slots: Vec<SlotRef>,
    pub incoming_slots: Vec<SlotRef>,
    pub operative: Uid,
    pub template: Uid,
}
#[derive(Clone)]
/// Struct which abstracts all common parts of a generated schema object
pub struct GSOConcrete<T> {
    pub id: Uid,
    pub outgoing_slots: HashMap<Uid, ActiveSlot>,
    pub incoming_slots: Vec<SlotRef>,
    pub fields: HashMap<Uid, PrimitiveValues>,
    pub operative: &'static LibraryOperative<PrimitiveTypes, PrimitiveValues>,
    pub template: &'static LibraryTemplate<PrimitiveTypes, PrimitiveValues>,
    pub _phantom: PhantomData<T>,
}
impl<T: std::fmt::Debug> std::fmt::Debug for GSOConcrete<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GSOWrapper")
            .field("id", &self.id)
            .field(
                "slots",
                &self
                    .outgoing_slots
                    .values()
                    .map(|slot| (&slot.slot.tag.name, &slot.slotted_instances))
                    .collect::<HashMap<_, _>>(),
            )
            .field(
                "parent_slots",
                &self
                    .incoming_slots
                    .iter()
                    .map(|parent_slot| parent_slot.host_instance_id)
                    .collect::<Vec<_>>(),
            )
            // .field("parent_slots", &self.parent_slots)
            .field("data", &self.fields)
            .finish()
    }
}

#[derive(Clone, Debug)]
pub struct GSOConcreteBuilder<T> {
    id: Uid,
    slots: HashMap<Uid, ActiveSlot>,
    parent_slots: Vec<SlotRef>,
    pub data: HashMap<Uid, Option<PrimitiveValues>>,
    operative: &'static LibraryOperative<PrimitiveTypes, PrimitiveValues>,
    template: &'static LibraryTemplate<PrimitiveTypes, PrimitiveValues>,
    _phantom: PhantomData<T>,
}

impl<T: Clone + std::fmt::Debug> GSOConcreteBuilder<T> {
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

pub trait Buildable
where
    Self: Sized + 'static,
    GSOConcrete<Self>: Instantiable<Schema = Self::Schema>,
{
    type Builder: Finalizable<GSOConcrete<Self>>;
    type Schema;

    fn initiate_build() -> GSOBuilder<Self::Builder, GSOConcrete<Self>, Self::Schema>;
    fn get_operative_id() -> Uid;
}

pub trait Verifiable {
    fn verify(&self) -> Result<(), ElementCreationError>;
}
pub trait Instantiable: std::fmt::Debug + Any {
    type Schema;

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

pub trait IntoSchema
where
    Self: Sized,
{
    type Schema;
    fn into_schema(instantiable: GSOConcrete<Self>) -> Self::Schema;
}
