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

type LibOp = LibraryOperative<PrimitiveTypes, PrimitiveValues>;
type LibTemplate = LibraryTemplate<PrimitiveTypes, PrimitiveValues>;

pub trait GraphEnvironment {
    type TTypes: ConstraintTraits;
    type TValues: ConstraintTraits;
    type TSchema: GSO;

    fn get_element(&self, id: &Uid) -> Option<&Self::TSchema>;
    fn instantiate_element(&mut self, element: Self::TSchema) -> Uid;
    fn get_constraint_schema(&self) -> &ConstraintSchema<Self::TTypes, Self::TValues>;
}

pub trait GSO: std::fmt::Debug {
    /// Instance ID
    fn get_id(&self) -> &Uid;
    // fn get_constraint_schema_operative_tag(&self) -> Rc<LibOp>;
    // fn get_constraint_schema_template_tag(&self) -> Rc<LibTemplate>;
    fn get_slot_by_id(&self, slot_id: &Uid) -> Option<&ActiveSlot> {
        self.get_slots().get(slot_id)
    }
    fn get_slots(&self) -> &HashMap<Uid, ActiveSlot>;
    fn get_parent_slots(&self) -> &Vec<SlotRef>;
}

#[derive(Clone, Debug)]
pub struct SlotRef {
    pub host_instance_id: Uid,
    pub slot_id: Uid,
}

pub trait Slotted {}

#[derive(Clone, Debug)]
pub struct ActiveSlot {
    slot: OperativeSlot,
    slotted_instances: Vec<Uid>,
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
    parent_slots: Vec<SlotRef>,
    pub data: T,
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

    fn get_parent_slots(&self) -> &Vec<SlotRef> {
        &self.parent_slots
    }
}
#[derive(Clone, Debug)]
pub struct GSOWrapperBuilder<T> {
    id: Uid,
    slots: HashMap<Uid, ActiveSlot>,
    parent_slots: Vec<SlotRef>,
    pub data: T,
    // operative: Rc<LibOp>,
    // template: Rc<LibTemplate>,
}

impl<T: Clone + std::fmt::Debug> GSOWrapperBuilder<T> {
    fn new(data: T, // , operative: Rc<LibOp>, template: Rc<LibTemplate>
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().as_u128(),
            slots: HashMap::new(),
            parent_slots: Vec::new(),
            data,
            // operative,
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
impl<F, T, G: GraphEnvironment> Producable<GSOWrapper<T>, G> for GSOWrapperBuilder<F>
where
    F: Producable<T, G>,
{
    fn produce(&self) -> GSOWrapper<T> {
        GSOWrapper::<T> {
            id: self.id.clone(),
            slots: self.slots.clone(),
            parent_slots: self.parent_slots.clone(),
            data: self.data.produce(),
            // operative: self.operative.clone(),
            // template: self.template.clone(),
        }
    }
}
#[derive(Debug, Display)]
enum BoundCheckError {
    OutOfRange,
}
impl std::error::Error for BoundCheckError {}
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
                    Some(Error::new(BoundCheckError::OutOfRange))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        if slot_errors.is_empty() {
            return Ok(());
        }
        // TODO make this return all of the errors
        Err(Error::new(BoundCheckError::OutOfRange))
    }
}
// impl<F, T, G: GraphEnvironment> Finalizable<GSOWrapper<T>, G> for GSOWrapperBuilder<F> where
//     F: Finalizable<T, G>
// {
// }
impl<F, T, G: GraphEnvironment> Finalizable<T, G> for F
where
    F: Verifiable + Producable<T, G>,
    T: Instantiable<Graph = G>,
{
}

pub trait Buildable<G: GraphEnvironment>
where
    Self: Sized + 'static,
    GSOWrapper<Self>: Instantiable<Graph = G>,
{
    type Builder: Finalizable<GSOWrapper<Self>, <GSOWrapper<Self> as Instantiable>::Graph> + Default;

    fn initiate_build() -> GSOBuilder<Self::Builder, GSOWrapper<Self>, G> {
        GSOBuilder::<Self::Builder, GSOWrapper<Self>, G>::new()
    }
}

pub trait Verifiable {
    fn verify(&self) -> Result<(), Error>;
}
pub trait Instantiable: GSO {
    type Graph: GraphEnvironment;

    fn instantiate(&self) -> Result<(), Error>;
    fn get_id(&self) -> &Uid;
}
type InstantiableElements<G> = Vec<Box<dyn Instantiable<Graph = G>>>;

pub struct InstantiableWrapper<T, G: GraphEnvironment>
where
    T: Instantiable<Graph = G>,
{
    prereq_instantiables: InstantiableElements<G>,
    instantiable_instance: T,
}

impl<T, G: GraphEnvironment> InstantiableWrapper<T, G>
where
    T: Instantiable<Graph = G> + 'static,
{
    pub fn flatten(mut self) -> InstantiableElements<G> {
        self.prereq_instantiables
            .push(Box::new(self.instantiable_instance));
        self.prereq_instantiables
    }
    pub fn get_prereq_instantiables(&self) -> &InstantiableElements<G> {
        &self.prereq_instantiables
    }
    pub fn get_instantiable_instance(&self) -> &T {
        &self.instantiable_instance
    }
}
impl<T, G: GraphEnvironment> InstantiableWrapper<GSOWrapper<T>, G>
where
    GSOWrapper<T>: Instantiable<Graph = G>,
{
    pub fn add_parent_slot(&mut self, parent_slot: SlotRef) {
        self.instantiable_instance.parent_slots.push(parent_slot);
    }
}

pub trait Producable<T, G: GraphEnvironment> {
    fn produce(&self) -> T;
}

pub trait Finalizable<T: Instantiable<Graph = G>, G: GraphEnvironment>:
    Verifiable + Producable<T, G>
{
    fn finalize(&self) -> Result<T, Error> {
        self.verify()?;
        Ok(self.produce())
    }
}

#[derive(Default, Debug)]
pub struct GSOBuilder<F, T, G: GraphEnvironment>
where
    F: Finalizable<T, G>,
    T: Instantiable<Graph = G>,
{
    instantiables: Vec<Box<dyn Instantiable<Graph = G>>>,
    wip_instance: F,
    _phantom: PhantomData<(T, G)>,
}

impl<F, T, G: GraphEnvironment> GSOBuilder<F, T, G>
where
    F: Finalizable<T, G> + Default,
    T: Instantiable<Graph = G> + 'static,
{
    fn build(mut self) -> Result<InstantiableWrapper<T, G>, Error> {
        Ok(InstantiableWrapper {
            instantiable_instance: self.wip_instance.finalize()?,
            prereq_instantiables: self.instantiables,
        })
    }
    fn new() -> Self {
        Self {
            instantiables: vec![],
            wip_instance: F::default(),
            _phantom: PhantomData,
        }
    }
}

pub trait Integrable<C> {
    fn integrate(&mut self, child: &C) -> SlotRef;
}

// impl<F, T> Integrable<T> for GSOWrapperBuilder<F> {
//     fn integrate(&mut self, child: &T) -> &mut Self {}
// }
pub fn integrate_child<F, T, C, G: GraphEnvironment>(
    builder: &mut GSOBuilder<F, T, G>,
    mut child: InstantiableWrapper<GSOWrapper<C>, G>,
) -> &mut GSOBuilder<F, T, G>
where
    F: Integrable<GSOWrapper<C>> + Finalizable<T, G>,
    T: Instantiable<Graph = G>,
    GSOWrapper<C>: Instantiable<Graph = G> + 'static,
{
    let slot_ref = builder
        .wip_instance
        .integrate(child.get_instantiable_instance());
    child.add_parent_slot(slot_ref);
    builder.instantiables.extend(child.flatten());
    builder
}

#[cfg(test)]
mod tests {

    static COUNTER: AtomicUsize = AtomicUsize::new(9);
    fn get_next_id() -> usize {
        COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    }

    use std::sync::atomic::AtomicUsize;

    use crate::primitives::{PrimitiveTypes, PrimitiveValues};

    use super::*;

    #[derive(Debug)]
    struct SampleGraphEnvironment<TSchema: GSO> {
        created_instances: HashMap<Uid, TSchema>,
        constraint_schema: ConstraintSchema<PrimitiveTypes, PrimitiveValues>,
    }

    impl<TTSchema: GSO> GraphEnvironment for SampleGraphEnvironment<TTSchema> {
        type TSchema = TTSchema;
        type TTypes = PrimitiveTypes;
        type TValues = PrimitiveValues;

        fn get_constraint_schema(&self) -> &ConstraintSchema<Self::TTypes, Self::TValues> {
            &self.constraint_schema
        }

        fn get_element(&self, id: &Uid) -> Option<&Self::TSchema> {
            self.created_instances.get(id)
        }
        fn instantiate_element(&mut self, element: Self::TSchema) -> Uid {
            let id = *element.get_id();
            self.created_instances.insert(id, element);
            id
        }
    }

    #[derive(Debug)]
    enum SampleSchema {
        Sentence(Sentence),
        Word(Word),
    }
    type SampleG = SampleGraphEnvironment<SampleSchema>;

    impl GSO for SampleSchema {
        fn get_id(&self) -> &Uid {
            todo!()
        }

        fn get_slots(&self) -> &HashMap<Uid, ActiveSlot> {
            todo!()
        }

        fn get_parent_slots(&self) -> &Vec<SlotRef> {
            todo!()
        }
    }

    #[derive(Debug, Clone, Default)]
    struct Sentence {}
    impl Instantiable for GSOWrapper<Sentence> {
        type Graph = SampleG;

        fn instantiate(&self) -> Result<(), Error> {
            todo!()
        }

        fn get_id(&self) -> &Uid {
            todo!()
        }
    }
    #[derive(Validate, Clone, Debug, Default)]
    struct SentenceBuilder {}

    impl Integrable<GSOWrapper<Word>> for GSOWrapperBuilder<SentenceBuilder> {
        fn integrate(&mut self, child: &GSOWrapper<Word>) -> SlotRef {
            self.add_instance_to_slot(&0, child.id);
            SlotRef {
                slot_id: 0,
                host_instance_id: self.id,
            }
        }
    }
    impl Default for GSOWrapperBuilder<SentenceBuilder> {
        fn default() -> Self {
            let op_slot = OperativeSlot {
                tag: Tag {
                    name: String::from("WordSlot"),
                    id: 0,
                },
                operative_descriptor: crate::constraint_schema::OperativeVariants::LibraryOperative(
                    23,
                ),
                bounds: SlotBounds::Range(1, 10),
            };
            let mut operative_slots = HashMap::new();
            // let slot_ref = OperativeSlot;
            let active_slot = ActiveSlot {
                slot: op_slot,
                slotted_instances: vec![],
            };
            operative_slots.insert(0, active_slot);
            let mut new_builder = GSOWrapperBuilder::<SentenceBuilder>::new(SentenceBuilder {});
            new_builder.replace_slots(operative_slots);
            new_builder
        }
    }

    impl Producable<Sentence, SampleG> for SentenceBuilder {
        fn produce(&self) -> Sentence {
            Sentence {}
        }
    }

    impl Verifiable for SentenceBuilder {
        fn verify(&self) -> Result<(), Error> {
            self.validate()?;
            Ok(())
        }
    }

    impl Buildable<SampleG> for Sentence {
        type Builder = GSOWrapperBuilder<SentenceBuilder>;
    }
    impl GSOBuilder<GSOWrapperBuilder<SentenceBuilder>, GSOWrapper<Sentence>, SampleG> {
        pub fn add_word(
            &mut self,
            word: InstantiableWrapper<GSOWrapper<Word>, SampleG>,
        ) -> &mut Self {
            integrate_child(self, word);
            self
        }
    }

    #[derive(Default, Debug, Clone)]
    struct Word {
        display: String,
    }
    impl Instantiable for GSOWrapper<Word> {
        type Graph = SampleG;

        fn instantiate(&self) -> Result<(), Error> {
            todo!()
        }

        fn get_id(&self) -> &Uid {
            todo!()
        }
    }

    #[derive(Debug, Default, Validate, Clone)]
    struct WordBuilder {
        #[validate(required)]
        display: Option<String>,
    }
    pub trait SetDisplay {
        fn set_display(&mut self, new_display: &str);
    }
    impl SetDisplay for WordBuilder {
        fn set_display(&mut self, new_display: &str) {
            self.display = Some(new_display.to_string());
        }
    }

    impl Verifiable for WordBuilder {
        fn verify(&self) -> Result<(), Error> {
            self.validate()?;
            Ok(())
        }
    }
    impl Producable<Word, SampleG> for WordBuilder {
        fn produce(&self) -> Word {
            Word {
                display: self.display.as_ref().unwrap().clone(),
            }
        }
    }

    impl<G: GraphEnvironment, F: SetDisplay + Finalizable<T, G>, T> GSOBuilder<F, T, G>
    where
        T: Instantiable<Graph = G>,
    {
        pub fn set_display(mut self, new_display: &str) -> Self {
            self.wip_instance.set_display(new_display);
            self
        }
    }
    impl Default for GSOWrapperBuilder<WordBuilder> {
        fn default() -> Self {
            GSOWrapperBuilder::<WordBuilder>::new(WordBuilder::default())
        }
    }
    impl SetDisplay for GSOWrapperBuilder<WordBuilder> {
        fn set_display(&mut self, new_display: &str) {
            self.data.set_display(new_display);
        }
    }

    impl Buildable<SampleG> for Word {
        type Builder = GSOWrapperBuilder<WordBuilder>;

        fn initiate_build() -> GSOBuilder<Self::Builder, GSOWrapper<Self>, SampleG> {
            GSOBuilder::<Self::Builder, GSOWrapper<Self>, SampleG>::new()
        }
    }

    #[test]
    fn test_builder() {
        let new_word = Word::initiate_build()
            .set_display("Humgub")
            .build()
            .unwrap();
        let new_word2 = Word::initiate_build()
            .set_display("Humgubbery")
            .build()
            .unwrap();

        let mut sentence = Sentence::initiate_build();
        sentence.add_word(new_word);
        sentence.add_word(new_word2);
        let sentence = sentence.build().unwrap();
        for line in sentence.flatten() {
            println!("{:#?}", line);
        }

        panic!();
    }
}
