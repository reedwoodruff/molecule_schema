use axum::{Json, http::StatusCode};
use base_types::{common::u128_to_string, primitives::PrimitiveValues };
use neo4rs::*;
use crate::app::prelude::CONSTRAINT_SCHEMA;

pub async fn save_graph(Json(payload): Json<Vec<base_types::traits::StandaloneRGSOWrapper>>) -> (StatusCode, ()) {
    let uri = "neo4j://localhost:7687";
   let user = "neo4j";
   let pass = "********";

   let graph = Graph::new(uri, user, pass).await.unwrap();

   let mut txn = graph.start_txn().await.unwrap();

    let (node_creation_queries, relationship_creation_queries)= payload.iter().fold((Vec::new(), Vec::new()),|mut agg, node| {
        println!("creating node: {}", node.id);
        let template = CONSTRAINT_SCHEMA.template_library.get(&node.template).unwrap();
        let operative = CONSTRAINT_SCHEMA.operative_library.get(&node.operative).unwrap();
        let string_node_id = base_types::common::u128_to_string(node.id);
        let string_template_id = base_types::common::u128_to_string(node.template);
        let string_operative_id = base_types::common::u128_to_string(node.operative);

         // Base query
        let mut query = format!("CREATE (n:{} {{", operative.tag.name);

        // Add each key-value pair to the query
        query.push_str(&format!("{}: ${}, ", "id", "id"));
        query.push_str(&format!("{}: ${}, ", "template_id", "template_id"));
        query.push_str(&format!("{}: ${}, ", "operative_id", "operative_id"));

        for (key, value) in &node.fields {
            let string_id = base_types::common::u128_to_string(*key);
            let replaced_id = string_id.replace("-", "_" );
            query.push_str(&format!("`{}`: $value_{}, ", string_id, replaced_id));
        }
        // Remove the trailing comma and space, then close the property map
        query.pop();
        query.pop();
        query.push_str("})");

        let mut creation_query = neo4rs::query(&query).
        param("id", string_node_id.clone()).
        param("template_id", string_template_id.clone()).
        param("operative_id", string_operative_id.clone());

        for (key, value) in &node.fields {
            let string_id = base_types::common::u128_to_string(*key);
            let param_name = "value_".to_string() + &string_id.clone().to_string();
            let replaced_id = param_name.replace("-", "_" );
            match value {
                PrimitiveValues::String(val) => {
                    creation_query = creation_query.param(&replaced_id, val.clone());
                }
                PrimitiveValues::Int(val) => {
                    creation_query = creation_query.param(&replaced_id, *val);
                }
                PrimitiveValues::Bool(val) => {
                    creation_query = creation_query.param(&replaced_id, *val);
                }
                PrimitiveValues::Option(val) => {
                    match val.as_ref() {
                        Some(val) => {                        
                            match val {
                                PrimitiveValues::String(val) => {
                                    creation_query = creation_query.param(&replaced_id, val.clone());
                                }
                                PrimitiveValues::Int(val) => {
                                    creation_query = creation_query.param(&replaced_id, *val);
                                }
                                PrimitiveValues::Bool(val) => {
                                    creation_query = creation_query.param(&replaced_id, *val);
                                }
                                _ => panic!()
                            }
                        }
,
                        None => todo!(),
                    }
                
                }
                PrimitiveValues::List(val) => {
                }
            }
        }

        let new_edge_queries = node.outgoing_slots.iter().map(|slot| {
            let string_target_id = u128_to_string(slot.target_instance_id);
            let string_slot_id = u128_to_string(slot.slot_id);
            let slot_name = template.operative_slots.get(&slot.slot_id).unwrap().tag.name.clone();
            let format_string = format!("MATCH (a {{id: $id }}), (b {{id: $id2}}) CREATE (a) -[:{} {{id: $slot_id }}]->(b)", slot_name);
            let edge_query = neo4rs::query(
                &format_string
            );
            edge_query.param("id", string_node_id.clone())
            .param("id2", string_target_id)
            .param("slot_id", string_slot_id)
        }).collect::<Vec<_>>();

        agg.1.extend(new_edge_queries);
        agg.0.push(creation_query);
        agg
    });
    println!("have all queries");
    let mut all_queries = vec![];
    all_queries.extend(node_creation_queries);
    all_queries.extend(relationship_creation_queries);
   let result = txn.run_queries(all_queries).await;
   assert!(result.is_ok());
   println!("successful run");
   txn.commit().await.unwrap();
   println!("completed transaction");
}
