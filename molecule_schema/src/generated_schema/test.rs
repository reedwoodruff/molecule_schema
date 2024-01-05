use std::fs::File;
use std::io::Result;
use std::io::Write;

use crate::common::ConstraintTraits;
use crate::constraint_schema::ConstraintSchema;
use molecule_types::*;

#[cfg(test)]
mod tests {
    use super::super::*;
    use im::HashMap;
    use uuid::Uuid;

    #[test]
    fn test() {
        #[derive(Clone)]
        struct NodeA {
            pub field1: String,
            pub field2: i32,
        }

        impl InstanceGSO<NodeA> {
            fn new(field1: String, field2: i32) -> Self {
                Self {
                    id: Uuid::new_v4().as_u128(),
                    schema_tag: SchemaTag::new("NodeA".to_string()),
                    edges: Vec::new(),
                    edge_constraints: None,
                    structure_ids: None,
                    data: NodeA { field1, field2 },
                }
            }
        }

        #[derive(Clone)]
        struct NodeB {
            field3: String,
            field4: u32,
        }
        impl InstanceGSO<NodeB> {
            fn new(field3: String, field4: u32) -> Self {
                Self {
                    id: Uuid::new_v4().as_u128(),
                    schema_tag: SchemaTag::new("NodeB".to_string()),
                    edges: Vec::new(),
                    edge_constraints: None,
                    structure_ids: None,
                    data: NodeB { field3, field4 },
                }
            }
        }
        #[derive(Clone)]
        struct TemplateA {}
        impl InstanceGSO<TemplateA> {
            fn new() -> Self {
                Self {
                    id: Uuid::new_v4().as_u128(),
                    schema_tag: SchemaTag::new("TemplateA".to_string()),
                    edges: Vec::new(),
                    edge_constraints: None,
                    structure_ids: Some(vec![0, 99]),
                    data: TemplateA {},
                }
            }
        }

        #[derive(Clone)]
        enum ExampleSchema {
            NodeA(InstanceGSO<NodeA>),
            NodeB(InstanceGSO<NodeB>),
            TemplateA(InstanceGSO<TemplateA>),
        }

        impl GSOCommon for ExampleSchema {
            fn id(&self) -> &Uid {
                match self {
                    ExampleSchema::NodeA(item) => item.id(),
                    ExampleSchema::NodeB(item) => item.id(),
                    ExampleSchema::TemplateA(item) => item.id(),
                }
            }
            fn structure_ids(&self) -> &Option<Vec<Uid>> {
                match self {
                    ExampleSchema::NodeA(item) => item.structure_ids(),
                    ExampleSchema::NodeB(item) => item.structure_ids(),
                    ExampleSchema::TemplateA(item) => item.structure_ids(),
                }
            }
            fn schema_tag(&self) -> &SchemaTag {
                match self {
                    ExampleSchema::NodeA(item) => item.schema_tag(),
                    ExampleSchema::NodeB(item) => item.schema_tag(),
                    ExampleSchema::TemplateA(item) => item.schema_tag(),
                }
            }
            fn edge_constraints(&self) -> &Option<u32> {
                match self {
                    ExampleSchema::NodeA(item) => item.edge_constraints(),
                    ExampleSchema::NodeB(item) => item.edge_constraints(),
                    ExampleSchema::TemplateA(item) => item.edge_constraints(),
                }
            }
            fn edges(&self) -> &Vec<EdgeInstance> {
                match self {
                    ExampleSchema::NodeA(item) => item.edges(),
                    ExampleSchema::NodeB(item) => item.edges(),
                    ExampleSchema::TemplateA(item) => item.edges(),
                }
            }
        }

        const INSTANCE_LIBRARY: std::cell::LazyCell<HashMap<Uid, ExampleSchema>> =
            std::cell::LazyCell::new(|| {
                let mut hash_map = HashMap::<Uid, ExampleSchema>::new();
                hash_map.insert(
                    0,
                    ExampleSchema::NodeA(InstanceGSO::<NodeA>::new("field1".to_string(), 1)),
                );
                hash_map.insert(
                    1,
                    ExampleSchema::TemplateA(InstanceGSO::<TemplateA> {
                        id: 1,
                        structure_ids: Some(vec![0, 99]),
                        schema_tag: SchemaTag::new("TemplateA".to_string()),
                        edges: Vec::new(),
                        edge_constraints: None,
                        data: TemplateA {},
                    }),
                );
                hash_map
            });

        // const OPERATIVE_LIBRARY: HashMap<Uid, i32> = HashMap::new();
    }
}
