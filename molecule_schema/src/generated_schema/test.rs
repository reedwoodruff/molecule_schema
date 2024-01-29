use std::fs::File;
use std::io::Result;
use std::io::Write;

use molecule_types::*;

// Need some way of associating particular operative versions of elements with the template which
// they are part of. Each element may have unlimited operative versions -- each template may use it
// in a different way.
#[cfg(test)]
mod tests {
    use std::cell::{self, LazyCell};

    use super::super::*;
    use im::HashMap;
    use uuid::Uuid;

    #[test]
    fn test() {
        #[derive(Clone, Debug, PartialEq)]
        enum VTImpl {
            String,
            I32,
        }
        impl ConstraintTraits for VTImpl {}

        const NODE_A_TAG: SchemaTag<VTImpl> = SchemaTagBuilder::new("NodeA")
            .add_field("field1", VTImpl::String)
            .add_field("field2", VTImpl::I32)
            .build();;
        #[derive(Clone)]
        struct NodeA {
            pub field1: FieldDef<String>,
            pub field2: FieldDef<i32>,
        }
        #[derive(Clone)]
        struct NodeASubA {
            pub field1: String,
        }
        #[derive(Clone)]
        struct NodeASubB {
            pub field2: i32,
        }

        impl InstanceGSO<NodeA> {
            fn new(field1: String, field2: i32) -> Self {
                Self {
                    id: Uuid::new_v4().as_u128(),
                    schema_tag: NODE_A_TAG.id,
                    required_edges: Vec::new(),
                    optional_edges: Vec::new(),
                    structure_ids: None,
                    // Should be able to generate this all dynamically based on the NodeATag
                    data: NodeA {
                        field1: FieldDef {
                            name: "field1".to_string(),
                            id: NODE_A_TAG.fields[0].id,
                            value: field1,
                        },
                        field2: FieldDef {
                            name: "field2".to_string(),
                            id: NODE_A_TAG.fields[1].id,
                            value: field2,
                        },
                    },
                }
            }
        }

        #[derive(Clone)]
        struct NodeB {
            field3: FieldDef<String>,
            field4: FieldDef<i32>,
        }
        const NODE_B_TAG: SchemaTag<VTImpl> = SchemaTagBuilder::new("NodeB")
            .add_field("field3", VTImpl::String)
            .add_field("field4", VTImpl::I32)
            .build();
        impl InstanceGSO<NodeB> {
            fn new(field3: String, field4: i32) -> Self {
                Self {
                    id: Uuid::new_v4().as_u128(),
                    schema_tag: NODE_B_TAG.id,
                    required_edges: Vec::new(),
                    optional_edges: Vec::new(),
                    structure_ids: None,
                    data: NodeB {
                        field3: FieldDef {
                            name: "field3".to_string(),
                            id: NODE_B_TAG.fields[0].id,
                            value: field3,
                        },
                        field4: FieldDef {
                            name: "field4".to_string(),
                            id: NODE_B_TAG.fields[1].id,
                            value: field4,
                        },
                    },
                }
            }
        }

        #[derive(Clone)]
        struct TemplateA {}
        const TEMPLATE_A_TAG: SchemaTag<VTImpl> = SchemaTagBuilder::new("TemplateA").build();
        impl InstanceGSO<TemplateA> {
            fn new() -> Self {
                Self {
                    id: Uuid::new_v4().as_u128(),
                    schema_tag: TEMPLATE_A_TAG.id,
                    required_edges: Vec::new(),
                    optional_edges: Vec::new(),
                    structure_ids: Some(vec![0, 99]),
                    data: TemplateA {},
                }
            }
        }

        #[derive(Clone)]
        enum ExampleInstanceSchema {
            NodeA(InstanceGSO<NodeA>),
            NodeB(InstanceGSO<NodeB>),
            TemplateA(InstanceGSO<TemplateA>),
            // TemplateB(InstanceGSO<TemplateB>),
        }
        #[derive(Clone)]
        enum ExampleOperativeSchema {
            NodeAOp(OperativeGSO<NodeASubA, VTImpl>),
        }

        const INSTANCE_LIBRARY: LazyCell<HashMap<Uid, ExampleInstanceSchema>> =
            LazyCell::new(|| {
                let mut hash_map = HashMap::<Uid, ExampleInstanceSchema>::new();
                hash_map.insert(
                    0,
                    ExampleInstanceSchema::NodeA(InstanceGSO::<NodeA>::new(
                        "field1".to_string(),
                        1,
                    )),
                );
                hash_map.insert(
                    1,
                    ExampleInstanceSchema::TemplateA(InstanceGSO::<TemplateA>::new()),
                );
                hash_map
            });

        const OPERATIVE_LIBRARY: LazyCell<HashMap<Uid, ExampleOperativeSchema>> =
            LazyCell::new(|| {
                let mut hash_map = HashMap::new();
                let NodeAOperative = OperativeGSO::<NodeASubA, VTImpl> {
                    id: todo!(),
                    structure_ids: todo!(),
                    schema_tag: todo!(),
                    parent_template_tag: todo!(),
                    locked_edges: todo!(),
                    operative_edges: todo!(),
                    locked_data: todo!(),
                    operative_fields: todo!(),
                };
                hash_map.insert(99, ExampleOperativeSchema::NodeAOp(NodeAOperative));
                hash_map
            });
    }
}
#[test]
fn test2() {}
