// use super::*;

// use anyhow::{Error, Result};
// use std::{collections::HashMap, rc::Rc};

// use validator::Validate;

// #[derive(Debug, Clone)]
// pub enum SampleSchema {
//     Sentence(GSOWrapper<Sentence, SampleSchema>),
//     Word(GSOWrapper<Word, SampleSchema>),
// }
// type SampleG = BaseGraphEnvironment<SampleSchema>;

// // impl<T: Clone + std::fmt::Debug> Instantiable for GSOWrapper<T> {
// //     type Schema = SampleSchema;

// //     fn instantiate(&self) -> Self::Schema {
// //         todo!()
// //     }

// //     fn get_instance_id(&self) -> &Uid {
// //         todo!()
// //     }
// //     // type Graph = G;
// // }

// impl FieldEditable for SampleSchema {
//     fn apply_field_edit(&mut self, field_edit: FieldEdit) {
//         todo!()
//     }
// }
// impl GSO for SampleSchema {
//     type Schema = SampleSchema;

//     fn get_id(&self) -> &Uid {
//         todo!()
//     }

//     fn get_operative(&self) -> Rc<LibraryOperative<PrimitiveTypes, PrimitiveValues>> {
//         todo!()
//     }

//     fn get_template(&self) -> Rc<LibraryTemplate<PrimitiveTypes, PrimitiveValues>> {
//         todo!()
//     }

//     fn get_slots(&self) -> &HashMap<Uid, ActiveSlot> {
//         todo!()
//     }

//     fn get_parent_slots(&self) -> &Vec<SlotRef> {
//         todo!()
//     }

//     fn add_parent_slot(&mut self, slot_ref: &SlotRef) -> &mut Self {
//         todo!()
//     }

//     fn add_child_to_slot(&mut self, slot_ref: &SlotRef) -> &mut Self {
//         todo!()
//     }

//     fn remove_child_from_slot(&mut self, slot_ref: &SlotRef) -> &mut Self {
//         todo!()
//     }

//     fn remove_parent(&mut self, parent_id: &Uid, slot_id: Option<&Uid>) -> Vec<SlotRef> {
//         todo!()
//     }

//     fn set_history(&mut self, history: Option<HistoryRef<Self::Schema>>) {
//         todo!()
//     }
// }

// #[derive(Debug, Clone, Default)]
// struct Sentence {}
// impl FieldEditable for Sentence {
//     fn apply_field_edit(&mut self, field_edit: FieldEdit) {
//         todo!()
//     }
// }
// impl IntoSchema for Sentence {
//     type Schema = SampleSchema;

//     fn into_schema(instantiable: GSOWrapper<Self, Self::Schema>) -> Self::Schema {
//         // let test = instantiable as &GSOWrapper<Sentence>;
//         SampleSchema::Sentence(instantiable.to_owned())
//     }

//     // fn into_schema(instantiable: Rc<dyn Instantiable<Schema = Self::Schema>>) -> Self::Schema {
//     //     let test = instantiable as GSOWrapper<Self>;
//     //     SampleSchema::Sentence(test)
//     // }
// }
// #[derive(Validate, Clone, Debug, Default)]
// struct SentenceBuilder {}

// // impl Integrable<Word> for SentenceBuilder {
// //     fn get_slot_id() -> Uid {
// //         0
// //     }
// //     // fn integrate(&mut self, child_id: &Uid)  {
// //     //     self.add_instance_to_slot(&0, *child_id);
// //     //     ParentSlotRef {
// //     //         slot_id: 0,
// //     //         host_instance_id: self.id,
// //     //     }
// //     // }
// // }

// trait SentenceWord {}

// impl Producable<Sentence> for SentenceBuilder {
//     fn produce(&self) -> Sentence {
//         Sentence {}
//     }
// }

// impl Verifiable for SentenceBuilder {
//     fn verify(&self) -> Result<(), Error> {
//         self.validate()?;
//         Ok(())
//     }
// }

// impl Buildable for Sentence {
//     type Schema = SampleSchema;
//     type Builder = GSOWrapperBuilder<SentenceBuilder>;

//     fn get_operative_id() -> Uid {
//         10
//     }
//     fn initiate_build() -> GSOBuilder<Self::Builder, GSOWrapper<Self, SampleSchema>, Self::Schema> {
//         let op_slot = OperativeSlot {
//             tag: Tag {
//                 name: String::from("WordSlot"),
//                 id: 0,
//             },
//             operative_descriptor: crate::constraint_schema::OperativeVariants::LibraryOperative(23),
//             bounds: SlotBounds::Range(1, 10),
//         };
//         let mut operative_slots = HashMap::new();
//         // let slot_ref = OperativeSlot;
//         let active_slot = ActiveSlot {
//             slot: op_slot,
//             slotted_instances: vec![],
//         };
//         operative_slots.insert(0, active_slot);
//         let builder = GSOWrapperBuilder::<SentenceBuilder>::new(
//             SentenceBuilder::default(),
//             Some(operative_slots),
//             Rc::new(Tag {
//                 id: 10,
//                 name: "SentenceOp".to_string(),
//             }),
//             Rc::new(Tag {
//                 id: 11,
//                 name: "Sentence".to_string(),
//             }),
//         );
//         GSOBuilder::<Self::Builder, GSOWrapper<Self, SampleSchema>, SampleSchema>::new(builder)
//     }
// }
// pub trait SentenceWordSlot {
//     fn add_word_new(
//         &mut self,
//         word: InstantiableWrapper<GSOWrapper<Word, SampleSchema>, SampleSchema>,
//     ) -> &mut Self;
//     fn add_word_existing(&mut self, word_id: &Uid) -> &mut Self;
// }
// impl SentenceWordSlot
//     for GSOBuilder<
//         GSOWrapperBuilder<SentenceBuilder>,
//         GSOWrapper<Sentence, SampleSchema>,
//         SampleSchema,
//     >
// {
//     fn add_word_new(
//         &mut self,
//         word: InstantiableWrapper<GSOWrapper<Word, SampleSchema>, SampleSchema>,
//     ) -> &mut Self {
//         integrate_child(self, word, 0);
//         self
//     }
//     fn add_word_existing(&mut self, word_id: &Uid) -> &mut Self {
//         integrate_child_id(self, word_id, 0);
//         self
//     }
// }

// #[derive(Default, Debug, Clone)]
// struct Word {
//     display: String,
// }
// impl FieldEditable for Word {
//     fn apply_field_edit(&mut self, field_edit: FieldEdit) {
//         todo!()
//     }
// }
// impl IntoSchema for Word {
//     type Schema = SampleSchema;

//     // fn into_schema(instantiable: Rc<dyn Instantiable<Schema = Self::Schema>>) -> Self::Schema {
//     //     let test = instantiable as GSOWrapper<Self>;
//     //     SampleSchema::Word(test)
//     // }
//     fn into_schema(instantiable: GSOWrapper<Self, SampleSchema>) -> Self::Schema {
//         // let test = instantiable as &GSOWrapper<Sentence>;
//         SampleSchema::Word(instantiable.to_owned())
//     }
// }

// #[derive(Debug, Default, Validate, Clone)]
// struct WordBuilder {
//     #[validate(required)]
//     display: Option<String>,
// }
// pub trait SetDisplay {
//     fn set_display(&mut self, new_display: &str) -> &mut Self;
// }
// impl SetDisplay for WordBuilder {
//     fn set_display(&mut self, new_display: &str) -> &mut Self {
//         self.display = Some(new_display.to_string());
//         self
//     }
// }

// impl Verifiable for WordBuilder {
//     fn verify(&self) -> Result<(), Error> {
//         self.validate()?;
//         Ok(())
//     }
// }
// impl Producable<Word> for WordBuilder {
//     fn produce(&self) -> Word {
//         Word {
//             display: self.display.as_ref().unwrap().clone(),
//         }
//     }
// }

// impl<F: SetDisplay + Finalizable<T>, T> SetDisplay for GSOBuilder<F, T, SampleSchema>
// where
//     T: Instantiable<Schema = SampleSchema>,
// {
//     fn set_display(&mut self, new_display: &str) -> &mut Self {
//         self.wip_instance.set_display(new_display);
//         self
//     }
// }
// impl SetDisplay for GSOWrapperBuilder<WordBuilder> {
//     fn set_display(&mut self, new_display: &str) -> &mut Self {
//         self.data.set_display(new_display);
//         self
//     }
// }
// impl SetDisplay for GSOWrapper<Word, SampleSchema> {
//     fn set_display(&mut self, new_display: &str) -> &mut Self {
//         self.data.display = new_display.to_string();
//         self
//     }
// }

// impl Buildable for Word {
//     type Schema = SampleSchema;
//     type Builder = GSOWrapperBuilder<WordBuilder>;

//     fn initiate_build() -> GSOBuilder<Self::Builder, GSOWrapper<Self, SampleSchema>, SampleSchema> {
//         GSOBuilder::<Self::Builder, GSOWrapper<Self, SampleSchema>, SampleSchema>::new(
//             GSOWrapperBuilder::<WordBuilder>::new(
//                 WordBuilder::default(),
//                 None,
//                 Rc::new(Tag {
//                     id: 1,
//                     name: "WordOp".to_string(),
//                 }),
//                 Rc::new(Tag {
//                     id: 2,
//                     name: "Word".to_string(),
//                 }),
//             ),
//         )
//     }
//     fn get_operative_id() -> Uid {
//         1
//     }
// }

// #[test]
// fn test_builder() {
//     // let mut new_word = Word::initiate_build();
//     // new_word.set_display("Humgub");
//     // let new_word = new_word.build().unwrap();
//     // let mut new_word2 = Word::initiate_build();
//     // new_word2.set_display("Humgubbery");
//     // let new_word2 = new_word2.build().unwrap();

//     // let word1id = new_word.get_instantiable_instance().get_id().clone();

//     // let mut sentence = Sentence::initiate_build();
//     // sentence.add_word_new(new_word);
//     // sentence.add_word_new(new_word2);
//     // let sentence = sentence.build().unwrap();

//     // let mut env = BaseGraphEnvironment::<SampleSchema>::new_without_schema();

//     // let sentence_id = env.instantiate_element(sentence);
//     // // env.delete(&sentence_id);
//     // // let word = env.get_mut(&word1id).unwrap();
//     // println!("{:#?}", env);
//     // //  {
//     // //     Some(SampleSchema::Word(word)) => word,
//     // //     _ => unreachable!(),
//     // // };
//     // // word.data.display = "Goolo".to_string();
//     // // word.set_display("goob");
//     // // println!("{:#?}", word);

//     panic!()
// }
