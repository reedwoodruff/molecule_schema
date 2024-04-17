use super::*;

use anyhow::{Error, Result};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    marker::PhantomData,
    rc::Rc,
};
use strum_macros::Display;

use validator::Validate;

#[derive(Debug)]
enum SampleSchema {
    Sentence(Sentence),
    Word(Word),
}
type SampleG = BaseGraphEnvironment<SampleSchema>;

impl GSO for SampleSchema {
    fn get_id(&self) -> &Uid {
        todo!()
    }

    fn get_slots(&self) -> &HashMap<Uid, ActiveSlot> {
        todo!()
    }

    fn get_parent_slots(&self) -> &Vec<ParentSlotRef> {
        todo!()
    }

    fn get_constraint_schema_operative_tag(&self) -> Rc<Tag> {
        todo!()
    }

    fn get_constraint_schema_template_tag(&self) -> Rc<Tag> {
        todo!()
    }
}

#[derive(Debug, Clone, Default)]
struct Sentence {}
#[derive(Validate, Clone, Debug, Default)]
struct SentenceBuilder {}

// impl Integrable<Word> for SentenceBuilder {
//     fn get_slot_id() -> Uid {
//         0
//     }
//     // fn integrate(&mut self, child_id: &Uid)  {
//     //     self.add_instance_to_slot(&0, *child_id);
//     //     ParentSlotRef {
//     //         slot_id: 0,
//     //         host_instance_id: self.id,
//     //     }
//     // }
// }

trait SentenceWord {}

impl Producable<Sentence> for SentenceBuilder {
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

impl Buildable for Sentence {
    type Builder = GSOWrapperBuilder<SentenceBuilder>;

    fn get_operative_id() -> Uid {
        10
    }
    fn initiate_build() -> GSOBuilder<Self::Builder, GSOWrapper<Self>> {
        let op_slot = OperativeSlot {
            tag: Tag {
                name: String::from("WordSlot"),
                id: 0,
            },
            operative_descriptor: crate::constraint_schema::OperativeVariants::LibraryOperative(23),
            bounds: SlotBounds::Range(1, 10),
        };
        let mut operative_slots = HashMap::new();
        // let slot_ref = OperativeSlot;
        let active_slot = ActiveSlot {
            slot: op_slot,
            slotted_instances: vec![],
        };
        operative_slots.insert(0, active_slot);
        let builder = GSOWrapperBuilder::<SentenceBuilder>::new(
            SentenceBuilder::default(),
            Some(operative_slots),
            Rc::new(Tag {
                id: 10,
                name: "SentenceOp".to_string(),
            }),
            Rc::new(Tag {
                id: 11,
                name: "Sentence".to_string(),
            }),
        );
        GSOBuilder::<Self::Builder, GSOWrapper<Self>>::new(builder)
    }
}
pub trait SentenceWordSlot {
    fn add_word_new(&mut self, word: InstantiableWrapper<GSOWrapper<Word>>) -> &mut Self;
    fn add_word_existing(&mut self, word_id: &Uid) -> &mut Self;
}
impl SentenceWordSlot for GSOBuilder<GSOWrapperBuilder<SentenceBuilder>, GSOWrapper<Sentence>> {
    fn add_word_new(&mut self, word: InstantiableWrapper<GSOWrapper<Word>>) -> &mut Self {
        integrate_child(self, word, 0);
        self
    }
    fn add_word_existing(&mut self, word_id: &Uid) -> &mut Self {
        integrate_child_id(self, word_id, 0);
        self
    }
}

#[derive(Default, Debug, Clone)]
struct Word {
    display: String,
}

#[derive(Debug, Default, Validate, Clone)]
struct WordBuilder {
    #[validate(required)]
    display: Option<String>,
}
pub trait SetDisplay {
    fn set_display(&mut self, new_display: &str) -> &mut Self;
}
impl SetDisplay for WordBuilder {
    fn set_display(&mut self, new_display: &str) -> &mut Self {
        self.display = Some(new_display.to_string());
        self
    }
}

impl Verifiable for WordBuilder {
    fn verify(&self) -> Result<(), Error> {
        self.validate()?;
        Ok(())
    }
}
impl Producable<Word> for WordBuilder {
    fn produce(&self) -> Word {
        Word {
            display: self.display.as_ref().unwrap().clone(),
        }
    }
}

impl<F: SetDisplay + Finalizable<T>, T> SetDisplay for GSOBuilder<F, T>
where
    T: Instantiable,
{
    fn set_display(&mut self, new_display: &str) -> &mut Self {
        self.wip_instance.set_display(new_display);
        self
    }
}
impl SetDisplay for GSOWrapperBuilder<WordBuilder> {
    fn set_display(&mut self, new_display: &str) -> &mut Self {
        self.data.set_display(new_display);
        self
    }
}

impl Buildable for Word {
    type Builder = GSOWrapperBuilder<WordBuilder>;

    fn initiate_build() -> GSOBuilder<Self::Builder, GSOWrapper<Self>> {
        GSOBuilder::<Self::Builder, GSOWrapper<Self>>::new(GSOWrapperBuilder::<WordBuilder>::new(
            WordBuilder::default(),
            None,
            Rc::new(Tag {
                id: 1,
                name: "WordOp".to_string(),
            }),
            Rc::new(Tag {
                id: 2,
                name: "Word".to_string(),
            }),
        ))
    }
    fn get_operative_id() -> Uid {
        1
    }
}

#[test]
fn test_builder() {
    let mut new_word = Word::initiate_build();
    new_word.set_display("Humgub");
    let new_word = new_word.build().unwrap();
    let mut new_word2 = Word::initiate_build();
    new_word2.set_display("Humgubbery");
    let new_word2 = new_word2.build().unwrap();

    let mut sentence = Sentence::initiate_build();
    sentence.add_word_new(new_word);
    sentence.add_word_new(new_word2);
    sentence.add_word_existing(&55);
    let sentence = sentence.build().unwrap();
    for line in sentence.flatten() {
        println!("{:#?}", line);
    }

    panic!();
}
