use anyhow::{Error, Result};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    marker::PhantomData,
};

use validator::Validate;

use crate::{
    common::{ConstraintTraits, Tag, Uid},
    constraint_schema::{ConstraintSchema, OperativeSlot, SlotBounds},
};

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
    fn get_id(&self) -> Uid;
    fn get_constraint_schema_operative_tag(&self) -> &Tag;
    fn get_constraint_schema_template_tag(&self) -> &Tag;
    fn get_slot_by_id(&self, slot_id: &Uid) -> Option<&ActiveSlot> {
        self.get_slots().get(slot_id)
    }
    fn get_slots(&self) -> &HashMap<Uid, ActiveSlot>;
    fn get_parent_slots(&self) -> Vec<SlotRef>;
}
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

pub trait Buildable<G: GraphEnvironment>
where
    Self: Sized + 'static,
    Self: Instantiable<Graph = G>,
{
    type Builder: Finalizable<Self, Self::Graph>;

    fn initiate_build() -> GSOBuilder<Self::Builder, Self, G> {
        GSOBuilder::<Self::Builder, Self, G>::new()
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

pub trait Producable<T, G: GraphEnvironment>
where
    T: Instantiable<Graph = G>,
{
    fn produce(&self) -> T;
}

pub trait Finalizable<T: Instantiable<Graph = G>, G: GraphEnvironment>:
    Default + Verifiable + Producable<T, G>
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
    F: Finalizable<T, G>,
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
    fn integrate(&mut self, child: &C) -> &mut Self;
}

pub fn integrate_child<F, T, C, G: GraphEnvironment>(
    builder: &mut GSOBuilder<F, T, G>,
    child: InstantiableWrapper<C, G>,
) -> &mut GSOBuilder<F, T, G>
where
    F: Integrable<C> + Finalizable<T, G>,
    T: Instantiable<Graph = G>,
    C: Instantiable<Graph = G> + 'static,
{
    builder
        .wip_instance
        .integrate(child.get_instantiable_instance());
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
            // let id = uuid::Uuid::new_v4().as_u128();
            let id = element.get_id();
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
        fn get_id(&self) -> Uid {
            todo!()
        }

        fn get_constraint_schema_operative_tag(&self) -> &Tag {
            todo!()
        }

        fn get_constraint_schema_template_tag(&self) -> &Tag {
            todo!()
        }

        fn get_slots(&self) -> &HashMap<Uid, ActiveSlot> {
            todo!()
        }

        fn get_parent_slots(&self) -> Vec<SlotRef> {
            todo!()
        }
    }

    #[derive(Debug, Clone)]
    struct Sentence {
        operative_slots: HashMap<Uid, ActiveSlot>,
    }
    impl Instantiable for Sentence {
        type Graph = SampleG;

        fn instantiate(&self) -> Result<(), Error> {
            todo!()
        }

        fn get_id(&self) -> &Uid {
            todo!()
        }
    }
    impl GSO for Sentence {
        fn get_id(&self) -> Uid {
            todo!()
        }

        fn get_constraint_schema_operative_tag(&self) -> &Tag {
            todo!()
        }

        fn get_constraint_schema_template_tag(&self) -> &Tag {
            todo!()
        }

        fn get_slots(&self) -> &HashMap<Uid, ActiveSlot> {
            todo!()
        }

        fn get_parent_slots(&self) -> Vec<SlotRef> {
            todo!()
        }
    }
    impl Default for Sentence {
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
            Self { operative_slots }
        }
    }
    #[derive(Validate, Clone, Debug)]
    struct SentenceBuilder {
        operative_slots: HashMap<Uid, ActiveSlot>,
    }
    impl Default for SentenceBuilder {
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
            Self { operative_slots }
        }
    }

    impl Integrable<Word> for SentenceBuilder {
        fn integrate(&mut self, child: &Word) -> &mut Self {
            self.operative_slots
                .entry(0)
                .and_modify(|prev_ids| prev_ids.slotted_instances.push(child.id));
            self
        }
    }

    impl Producable<Sentence, SampleG> for SentenceBuilder {
        fn produce(&self) -> Sentence {
            Sentence {
                operative_slots: self.operative_slots.clone(),
            }
        }
    }

    impl Verifiable for SentenceBuilder {
        fn verify(&self) -> Result<(), Error> {
            self.validate()?;
            Ok(())
        }
    }

    impl Finalizable<Sentence, SampleG> for SentenceBuilder {}

    impl Buildable<SampleG> for Sentence {
        type Builder = SentenceBuilder;
    }
    impl GSOBuilder<SentenceBuilder, Sentence, SampleG> {
        pub fn add_word(&mut self, word: InstantiableWrapper<Word, SampleG>) -> &mut Self {
            integrate_child(self, word);
            self
        }
    }

    #[derive(Default, Debug, Clone)]
    struct Word {
        id: Uid,
        display: String,
    }
    impl Instantiable for Word {
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
    trait SetDisplay {
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
                id: get_next_id() as u128,
                display: self.display.as_ref().unwrap().clone(),
            }
        }
    }

    impl Finalizable<Word, SampleG> for WordBuilder {}

    impl<G: GraphEnvironment, F: SetDisplay + Finalizable<T, G>, T> GSOBuilder<F, T, G>
    where
        T: Instantiable<Graph = G>,
    {
        fn set_display(mut self, new_display: &str) -> Self {
            self.wip_instance.set_display(new_display);
            self
        }
    }

    impl GSO for Word {
        fn get_id(&self) -> Uid {
            // Get Instance ID
            todo!()
        }

        fn get_constraint_schema_template_tag(&self) -> &Tag {
            todo!()
        }

        fn get_constraint_schema_operative_tag(&self) -> &Tag {
            todo!()
        }

        fn get_slots(&self) -> &HashMap<Uid, ActiveSlot> {
            todo!()
        }

        fn get_parent_slots(&self) -> Vec<SlotRef> {
            todo!()
        }
    }

    impl Buildable<SampleG> for Word {
        type Builder = WordBuilder;

        fn initiate_build() -> GSOBuilder<Self::Builder, Self, Self::Graph> {
            GSOBuilder::<Self::Builder, Self, Self::Graph>::new()
        }
    }

    #[test]
    fn test_builder() {
        let new_word = Word::initiate_build()
            .set_display("Humgub")
            .build()
            .unwrap();
        let mut sentence_builder = Sentence::initiate_build();
        sentence_builder.add_word(new_word);
        let sentence = sentence_builder.build().unwrap();
        for line in sentence.flatten() {
            println!("{:?}", line);
        }

        panic!();
    }
}
