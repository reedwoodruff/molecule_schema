// use leptos::SignalGet;
// use std::collections::HashMap;
// use std::io::Write;

// use base_types::common::Uid;

// use base_types::traits::reactive::RBaseGraphEnvironment;
// use base_types::traits::{BaseGraphEnvironment, GraphEnvironment, GSO};
// use generate_schema_reactive::generate_concrete_schema_reactive;

// #[test]
// fn test_macro() {
//     println!("starting test");
//     println!("=========================++!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!+++===================================");
//     std::io::stdout().flush().unwrap();
//     // constraint_schema::constraint_schema!();
//     // let mut sge_instance = BaseGraphEnvironment::new(constraint_schema_generated);
//     generate_concrete_schema_reactive!();
//     let mut sge_instance = RBaseGraphEnvironment::new(&CONSTRAINT_SCHEMA);

//     let new_word3 = WordOp::initiate_build()
//         .set_display("CREATED_FIRST_WORD".to_string())
//         .build(&sge_instance)
//         .unwrap();
//     let word_3_id = *new_word3.get_instantiable_instance().get_id();
//     let new_word4 = WordOp::initiate_build()
//         .set_display("CREATED_FIRST_WORD_2".to_string())
//         .build(&sge_instance)
//         .unwrap();
//     let new_linear_displayable_first = LinearDisplayableOp::initiate_build()
//         .add_new_latter(new_word3)
//         .add_new_former(new_word4)
//         .build(&sge_instance)
//         .unwrap();

//     let first_displayable_id = sge_instance
//         .instantiate_element(new_linear_displayable_first)
//         .unwrap();

//     let new_word = WordOp::initiate_build()
//         .set_display("Clunk".to_string())
//         .build(&sge_instance)
//         .unwrap();
//     let word1id = *new_word.get_instantiable_instance().get_id();
//     let new_word2 = WordOp::initiate_build()
//         .set_display("Clip".to_string())
//         .build(&sge_instance)
//         .unwrap();

//     let new_linear_displayable = LinearDisplayableOp::initiate_build()
//         .add_new_latter(new_word)
//         .add_new_former(new_word2)
//         .build(&sge_instance)
//         .unwrap();
//     let _second_displayable_id = *new_linear_displayable.get_instantiable_instance().get_id();

//     let new_punctuation_op = PunctuationOp::initiate_build()
//         .set_display(".".to_string())
//         .build(&sge_instance)
//         .unwrap();
//     let new_sen = SentenceOp::initiate_build()
//         .add_new_linear_displayable(new_linear_displayable)
//         .add_existing_linear_displayable(&first_displayable_id)
//         // .add_existing_linear_displayable(&word_3_id)
//         // .add_new_final_punctuation(new_punctuation_op)
//         .build(&sge_instance)
//         .unwrap();

//     let sent_id = sge_instance.instantiate_element(new_sen).unwrap();

//     let word = match sge_instance.get(&word1id).unwrap() {
//         Schema::WordOp(word) => word,
//         _ => panic!(),
//     };
//     word.set_display("clong".to_string());

//     let latest_linear_displayable = LinearDisplayableOp::initiate_build()
//         .add_existing_former(&word_3_id)
//         .add_existing_latter(&word1id)
//         .build(&sge_instance)
//         .unwrap();

//     let latest_linear_displayable_id = sge_instance
//         .instantiate_element(latest_linear_displayable)
//         .unwrap();

//     let sentence = match sge_instance.get(&sent_id).unwrap() {
//         Schema::SentenceOp(sentence) => sentence,
//         _ => panic!(),
//     };
//     // let action = sentence.add_new_linear_displayable(latest_linear_displayable);
//     let action = sentence.add_existing_linear_displayable(&latest_linear_displayable_id);
//     sge_instance.create_connection(action).unwrap();
//     // let new_word_last = WordOp::initiate_build()
//     //     .set_display("HOoho".to_string())
//     //     .build(&sge_instance)
//     //     .unwrap();

//     // sentence.add_new_linear_displayable(new_word_last);
//     // sge_instance.instantiate_element(action);

//     sge_instance.undo();
//     sge_instance.undo();
//     sge_instance.undo();
//     sge_instance.undo();
//     sge_instance.undo();

//     sge_instance.redo();
//     sge_instance.redo();
//     sge_instance.redo();
//     sge_instance.redo();
//     sge_instance.redo();

//     sge_instance.undo();
//     sge_instance.undo();
//     sge_instance.undo();
//     sge_instance.undo();
//     sge_instance.undo();

//     sge_instance.redo();
//     sge_instance.redo();
//     sge_instance.redo();
//     sge_instance.redo();
//     sge_instance.redo();
//     // sge_instance.delete(&second_displayable_id).unwrap();
//     // // sge_instance.delete(&first_displayable_id).unwrap();
//     // // for instance in sge_instance.created_instances.values() {
//     // //     println!("{:#?}", instance);
//     // // }
//     println!("{:#?}", sge_instance.history.borrow().undo);

//     println!("{:#?}", sge_instance.history.borrow().redo);
//     println!("{:#?}", sge_instance.created_instances.get().values());
//     panic!();
//     // let mut new_sen = SentenceOp::initiate_build();
//     // let mut new_linear_displayable = LinearDisplayableOp::initiate_build();
//     // let new_linear_displayable = new_linear_displayable.build().unwrap();
//     // new_sen.add_new_linear_displayable(new_linear_displayable);

//     // LinearDisplayableOp::initiate_build().
// }
