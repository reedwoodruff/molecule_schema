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

pub trait GSO {
    /// Instance ID
    fn get_id(&self) -> Uid;
    fn get_constraint_schema_operative_tag(&self) -> &Tag;
    fn get_constraint_schema_template_tag(&self) -> &Tag;
    fn get_operative_by_id(&self, operative_id: &Uid) -> Option<Uid>;
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

pub trait Buildable
where
    Self: Sized,
{
    type Builder: Finalizable<Self>;

    fn initiate_build() -> GSOBuilder<Self::Builder, Self> {
        GSOBuilder::<Self::Builder, Self>::new()
    }
}

pub trait Finalizable<T>: Default {
    fn finalize(&self) -> Result<T, Error>;
}

// pub struct GSOEditor {}

#[derive(Default, Debug, Clone)]
struct GSOBuilder<F, T, R = ()>
where
    F: Finalizable<T>,
{
    wip_instance: F,
    saved_state: Option<R>,
    _phantom: PhantomData<T>,
}

pub trait CombinableGSOBuilder<F, T, R> {
    fn combine(&self) -> T;
}

impl<F, T, R> GSOBuilder<F, T, R>
where
    F: Finalizable<T>,
    R: Clone,
{
    fn build(&self) -> Result<T, Error> {
        self.wip_instance.finalize()
    }
    fn get_saved_state(&self) -> Option<R> {
        self.saved_state.as_ref().cloned()
    }
    fn new_with_saved_state(state: R) -> Self {
        Self {
            wip_instance: F::default(),
            saved_state: Some(state),
            _phantom: PhantomData,
        }
    }
    fn new() -> Self {
        Self {
            wip_instance: F::default(),
            saved_state: None,
            _phantom: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[derive(Debug, Clone)]
    struct Sentence {
        operative_slots: HashMap<Uid, ActiveSlot>,
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

    trait ManipulateWordSlot {
        // fn add_word_new(
        //     self,
        // ) -> GSOBuilder<WordBuilder, Word, GSOBuilder<SentenceBuilder, Sentence>>;
        fn add_word_existing(&mut self, instance_id: Uid) -> &mut Self;
    }
    impl ManipulateWordSlot for SentenceBuilder {
        // fn add_word_new(
        //     self,
        // ) -> GSOBuilder<WordBuilder, Word, GSOBuilder<SentenceBuilder, Sentence>> {
        //     GSOBuilder::<WordBuilder, Word, GSOBuilder<SentenceBuilder, Sentence>>::new_with_saved_state(self)
        // }
        fn add_word_existing(&mut self, instance_id: Uid) -> &mut Self {
            self.operative_slots
                .entry(0)
                .and_modify(|slot| slot.slotted_instances.push(instance_id));
            self
        }
    }
    impl Finalizable<GSOBuilder<SentenceBuilder, Sentence>>
        for GSOBuilder<WordBuilder, Word, GSOBuilder<SentenceBuilder, Sentence>>
    {
        fn finalize(&self) -> Result<GSOBuilder<SentenceBuilder, Sentence>, Error> {
            let mut existing_sentence = self.get_saved_state().unwrap();
            let new_word = self.wip_instance.finalize()?;
            println!("new_word: {:?}", new_word);
            existing_sentence.add_word_existing(new_word.id);
            println!("new_sentence: {:?}", existing_sentence);
            Ok(existing_sentence)
        }
    }

    impl Finalizable<Sentence> for SentenceBuilder {
        fn finalize(&self) -> Result<Sentence, Error> {
            self.validate()?;
            Ok(Sentence {
                operative_slots: self.operative_slots.clone(),
            })
        }
    }

    impl Buildable for Sentence {
        type Builder = SentenceBuilder;
    }
    // impl<F: ManipulateWordSlot + Finalizable<T>, T> GSOBuilder<F, T> {
    //     pub fn add_word_new(
    //         self,
    //     ) -> GSOBuilder<WordBuilder, Word, GSOBuilder<SentenceBuilder, Sentence>> {
    //         // self.wip_instance.add_word_new()
    //         GSOBuilder::<WordBuilder, Word, GSOBuilder<SentenceBuilder, Sentence>>::new_with_saved_state(self)
    //     }
    // }
    impl GSOBuilder<SentenceBuilder, Sentence> {
        pub fn add_word_new(
            self,
        ) -> GSOBuilder<WordBuilder, Word, GSOBuilder<SentenceBuilder, Sentence>> {
            // self.wip_instance.add_word_new()
            GSOBuilder::<WordBuilder, Word, GSOBuilder<SentenceBuilder, Sentence>>::new_with_saved_state(self)
        }
        pub fn add_word_existing(&mut self, id: Uid) {
            self.wip_instance.add_word_existing(id);
        }
    }

    #[derive(Default, Debug, Clone)]
    struct Word {
        id: Uid,
        display: String,
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

    impl Finalizable<Word> for WordBuilder {
        fn finalize(&self) -> Result<Word, Error> {
            self.validate()?;
            Ok(Word {
                id: 111,
                display: self.display.as_ref().unwrap().clone(),
            })
        }
    }
    impl<F: SetDisplay + Finalizable<T>, T, R> GSOBuilder<F, T, R> {
        fn set_display(&mut self, new_display: &str) -> &mut Self {
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

        fn get_operative_by_id(&self, operative_id: &Uid) -> Option<Uid> {
            todo!()
        }
    }

    impl Buildable for Word {
        type Builder = WordBuilder;

        fn initiate_build() -> GSOBuilder<Self::Builder, Self> {
            GSOBuilder::<Self::Builder, Self>::new()
        }
    }
    #[test]
    fn test_builder() {
        let new_word = Word::initiate_build()
            .set_display("Humgub")
            .build()
            .unwrap();
        // let test: Box<dyn Any> = Box::new(new_word);
        let new_sentence = Sentence::initiate_build()
            .add_word_new()
            .set_display("VOIU")
            .finalize()
            .unwrap()
            .build()
            .unwrap();
        println!("{:?}", new_sentence);
        panic!();
    }
}
