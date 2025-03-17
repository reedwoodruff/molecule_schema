use base_types::common::Uid;
use schema_editor_generated_toolkit::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExecutionNodeTerminal {
    Input(RGSOConcrete<ImplStepMapFromInput, Schema>),
    Output(RGSOConcrete<ImplStepMapToOutput, Schema>),
}

impl ExecutionNodeTerminal {
    pub fn get_id(&self) -> &Uid {
        match self {
            ExecutionNodeTerminal::Input(input) => input.get_id(),
            ExecutionNodeTerminal::Output(output) => output.get_id(),
        }
    }
}

/// Represents a node in the execution flow graph
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExecutionNode {
    /// A data node that holds information
    Data(RGSOConcrete<ImplData, Schema>),
    /// A step node that performs computation
    Step(ImplStepVariantTraitObject),
    /// A terminal node (output of the function)
    Terminal(ExecutionNodeTerminal),
}
impl ExecutionNode {
    pub fn get_id(&self) -> &Uid {
        match self {
            ExecutionNode::Data(data) => data.get_id(),
            ExecutionNode::Step(step) => step.get_id(),
            ExecutionNode::Terminal(terminal) => terminal.get_id(),
        }
    }
}

/// Represents the type of slot in the execution flow graph
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SlotType {
    Dependency,
    Internal,
    Output,
}

/// Represents an edge in the execution flow graph
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionEdge {
    pub slot_name: String,
    pub slot_type: SlotType,
    pub from: ExecutionNode,
    pub to: ExecutionNode,
}

/// Represents the full execution flow graph
#[derive(Debug, Clone)]
pub struct ExecutionGraph {
    /// All nodes in the graph
    pub nodes: HashMap<Uid, ExecutionNode>,
    /// All edges in the graph
    pub edges: Vec<ExecutionEdge>,
    /// Mapping from step nodes to their data dependencies
    pub step_data_dependencies: HashMap<Uid, Vec<Uid>>,
    /// Mapping from data nodes to steps that depend on them
    pub data_dependent_steps: HashMap<Uid, Vec<Uid>>,
    /// The terminals (outputs) of the execution
    pub terminals: Vec<Uid>,
    /// The inputs of the execution
    pub inputs: Vec<Uid>,
}

impl ExecutionGraph {
    /// Get all data dependencies for a specific step
    pub fn get_data_dependencies(&self, step_id: &Uid) -> Vec<&ExecutionNode> {
        if let Some(deps) = self.step_data_dependencies.get(step_id) {
            deps.iter()
                .map(|id| {
                    self.nodes
                        .iter()
                        .find(|(_, node)| match node {
                            ExecutionNode::Data(inner) => inner.get_id() == id,
                            _ => false,
                        })
                        .map(|(_, node)| node)
                        .unwrap()
                })
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get all steps that depend on a specific data node
    pub fn get_dependent_steps(&self, data_id: &Uid) -> Vec<&ExecutionNode> {
        if let Some(deps) = self.data_dependent_steps.get(data_id) {
            deps.iter()
                .map(|id| {
                    self.nodes
                        .iter()
                        .find(|(_, node)| match node {
                            ExecutionNode::Step(inner) => inner.get_id() == id,
                            _ => false,
                        })
                        .map(|(_, node)| node)
                        .unwrap()
                })
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Find the path from a data node to a terminal
    pub fn find_path_to_terminal(&self, data_id: &Uid) -> Vec<ExecutionNode> {
        let mut path = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        let this_node = self.nodes.get(data_id).unwrap();
        queue.push_back(this_node.clone());
        visited.insert(this_node.clone());

        while let Some(node) = queue.pop_front() {
            path.push(node.clone());

            for edge in &self.edges {
                if edge.from == node && !visited.contains(&edge.to) {
                    queue.push_back(edge.to.clone());
                    visited.insert(edge.to.clone());

                    if let ExecutionNode::Terminal(ExecutionNodeTerminal::Output(_)) = edge.to {
                        // We've reached a terminal, so we can stop the search
                        path.push(edge.to.clone());
                        return path;
                    }
                }
            }
        }

        path
    }

    /// Find all data nodes that are inputs to the execution
    pub fn get_input_data_nodes(&self) -> Vec<&ExecutionNode> {
        self.inputs
            .iter()
            .map(|id| {
                self.nodes
                    .iter()
                    .find(|(_, node)| match node {
                        ExecutionNode::Data(inner) => inner.get_id() == id,
                        _ => false,
                    })
                    .map(|(_, node)| node)
                    .unwrap()
            })
            .collect()
    }

    /// Find all data nodes that are outputs of the execution
    pub fn get_output_data_nodes(&self) -> Vec<&ExecutionNode> {
        self.terminals
            .iter()
            .map(|id| {
                self.nodes
                    .iter()
                    .find(|(_, node)| match node {
                        ExecutionNode::Terminal(inner) => inner.get_id() == id,
                        _ => false,
                    })
                    .map(|(_, node)| node)
                    .unwrap()
            })
            .collect()
    }
}

/// Analyze a method implementation to build an execution graph
pub fn analyze_method_implementation(
    method_impl: &RGSOConcrete<MethodImplementation, Schema>,
) -> ExecutionGraph {
    let mut graph = ExecutionGraph {
        nodes: HashMap::new(),
        edges: Vec::new(),
        step_data_dependencies: HashMap::new(),
        data_dependent_steps: HashMap::new(),
        terminals: Vec::new(),
        inputs: Vec::new(),
    };

    // First, extract terminals (outputs) from the method implementation
    let output_terminals = method_impl.get_maptooutputs_slot();

    for terminal in output_terminals {
        graph.terminals.push(*terminal.get_id());
        graph.nodes.insert(
            *terminal.get_id(),
            ExecutionNode::Terminal(ExecutionNodeTerminal::Output(terminal.clone())),
        );

        // Now we need to traverse backwards from each terminal to find all data and step nodes
        build_graph_from_terminal(&mut graph, method_impl, *terminal.get_id());
    }

    // Identify input nodes (data nodes with no incoming edges)
    let all_data_nodes: HashSet<_> = graph
        .nodes
        .iter()
        .filter_map(|(_, node)| match node {
            ExecutionNode::Data(inner) => Some(inner.get_id()),
            _ => None,
        })
        .cloned()
        .collect();

    let input_nodes: Vec<_> = all_data_nodes
        .iter()
        .filter(|data_id| {
            !graph.edges.iter().any(|edge| match &edge.to {
                ExecutionNode::Data(inner) => inner.get_id() == *data_id,
                _ => false,
            })
        })
        .cloned()
        .collect();

    graph.inputs = input_nodes;

    graph
}

/// Helper function to recursively build the execution graph from a terminal node
fn build_graph_from_terminal(
    graph: &mut ExecutionGraph,
    method_impl: &RGSOConcrete<MethodImplementation, Schema>,
    terminal_id: Uid,
) {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back(ExecutionNode::Terminal(ExecutionNodeTerminal::Output(
        method_impl
            .get_maptooutputs_slot()
            .iter()
            .find(|t| t.get_id() == &terminal_id)
            .unwrap()
            .clone(),
    )));
    visited.insert(ExecutionNode::Terminal(ExecutionNodeTerminal::Output(
        method_impl
            .get_maptooutputs_slot()
            .iter()
            .find(|t| t.get_id() == &terminal_id)
            .unwrap()
            .clone(),
    )));

    while let Some(node) = queue.pop_front() {
        leptos::logging::log!("Processing node: {:#?}", node);
        // For each node, find its dependencies
        match &node {
            ExecutionNode::Terminal(ExecutionNodeTerminal::Output(terminal)) => {
                // Terminal nodes depend on data nodes
                let data = get_output_terminal_data_dependencies(terminal);
                let data_node = ExecutionNode::Data(data.clone());
                graph.nodes.insert(*data.get_id(), data_node.clone());

                let edge = ExecutionEdge {
                    slot_name: "input".to_string(),
                    slot_type: SlotType::Dependency,
                    from: data_node.clone(),
                    to: node.clone(),
                };
                graph.edges.push(edge);

                if !visited.contains(&data_node) {
                    queue.push_back(data_node.clone());
                    visited.insert(data_node.clone());
                }
            }
            ExecutionNode::Data(data) => {
                // Data nodes may come from step nodes
                if let Some(step_deps) = get_data_step_dependencies(data) {
                    for step in step_deps {
                        let step_node = match &step {
                            ImplStepVariantTraitObject::ImplStepMapFromInput(inner_step) => {
                                ExecutionNode::Terminal(ExecutionNodeTerminal::Input(
                                    inner_step.clone(),
                                ))
                            }
                            _ => ExecutionNode::Step(step.clone()),
                        };
                        graph.nodes.insert(*step.get_id(), step_node.clone());

                        let edge = ExecutionEdge {
                            slot_name: "upstream".to_string(),
                            slot_type: SlotType::Dependency,
                            from: step_node.clone(),
                            to: node.clone(),
                        };
                        graph.edges.push(edge);

                        // Track step -> data dependency for easy lookup
                        graph
                            .step_data_dependencies
                            .entry(*step.get_id())
                            .or_insert_with(Vec::new)
                            .push(*data.get_id());

                        // Track data -> step dependency for easy lookup
                        graph
                            .data_dependent_steps
                            .entry(*data.get_id())
                            .or_insert_with(Vec::new)
                            .push(*step.get_id());

                        if !visited.contains(&step_node) {
                            queue.push_back(step_node.clone());
                            visited.insert(step_node.clone());
                        }
                    }
                }

                // // Handle internal slots for ImplData nodes
                // let internal_slots = get_internal_slots_for_data(data);
                // for (slot_name, internal_data) in internal_slots {
                //     let internal_node = ExecutionNode::Data(internal_data.clone());
                //     graph
                //         .nodes
                //         .insert(*internal_data.get_id(), internal_node.clone());

                //     let edge = ExecutionEdge {
                //         slot_name,
                //         slot_type: SlotType::Internal,
                //         from: internal_node.clone(),
                //         to: node.clone(),
                //     };
                //     graph.edges.push(edge);

                //     if !visited.contains(&internal_node) {
                //         queue.push_back(internal_node.clone());
                //         visited.insert(internal_node.clone());
                //     }
                // }
            }
            ExecutionNode::Step(step) => {
                // Step nodes depend on data nodes
                let deps = get_step_data_dependencies(step);
                for edge in deps {
                    graph
                        .nodes
                        .insert(edge.to.get_id().clone(), edge.to.clone());
                    graph.edges.push(edge.clone());

                    if !visited.contains(&edge.to) {
                        queue.push_back(edge.to.clone());
                        visited.insert(edge.to.clone());
                    }
                }

                // // Handle non-flow dependencies (internal context)
                // for data in non_flow_deps {
                //     let data_node = ExecutionNode::Data(data.clone());
                //     graph.nodes.insert(*data.get_id(), data_node.clone());

                //     let edge = ExecutionEdge {
                //         slot_name: "internal".to_string(),
                //         slot_type: SlotType::Internal,
                //         from: data_node.clone(),
                //         to: node.clone(),
                //     };
                //     graph.edges.push(edge);

                //     if !visited.contains(&data_node) {
                //         queue.push_back(data_node.clone());
                //         visited.insert(data_node.clone());
                //     }
                // }
            }
            _ => {}
        }
    }
}

/// Helper function to get data dependencies for a terminal node
fn get_output_terminal_data_dependencies(
    terminal: &RGSOConcrete<ImplStepMapToOutput, Schema>,
) -> RGSOConcrete<ImplData, Schema> {
    terminal.get_input_slot()
}

/// Helper function to get step dependencies for a data node
fn get_data_step_dependencies(
    data: &RGSOConcrete<ImplData, Schema>,
) -> Option<Vec<ImplStepVariantTraitObject>> {
    // This would be implementation-specific based on how your data nodes are connected to steps
    // You'd need to implement this based on your schema
    let upstream = data.get_upstreamstep_slot();
    if upstream.is_empty() {
        None
    } else {
        Some(upstream.iter().map(|u| u.clone()).collect())
    }
}

// /// Helper function to get internal slots for a data node
// fn get_internal_slots_for_data(
//     data: &RGSOConcrete<ImplData, Schema>,
// ) -> Vec<(String, RGSOConcrete<ImplData, Schema>)> {
//     let mut internal_slots = Vec::new();

//     internal_slots.push(("DataType".to_string(), data.get_datatype_slot()));

//     // if let Some(allowed_operative) = data.get_allowedoperative_slot() {
//     //     internal_slots.push(("AllowedOperative".to_string(), allowed_operative));
//     // }

//     internal_slots
// }

/// Helper function to get data dependencies for a step node
pub(crate) fn get_step_data_dependencies(step: &ImplStepVariantTraitObject) -> Vec<ExecutionEdge> {
    match step {
        ImplStepVariantTraitObject::ImplStepIsType(rgsoconcrete) => {
            vec![
                ExecutionEdge {
                    slot_name: "InputOperative".to_string(),
                    slot_type: SlotType::Dependency,
                    from: ExecutionNode::Step(step.clone()),
                    to: ExecutionNode::Data(rgsoconcrete.get_inputoperative_slot()),
                },
                ExecutionEdge {
                    slot_name: "OutputBool".to_string(),
                    slot_type: SlotType::Output,
                    from: ExecutionNode::Step(step.clone()),
                    to: ExecutionNode::Data(rgsoconcrete.get_outputbool_slot()),
                },
                ExecutionEdge {
                    slot_name: "TypeCheckOperative".to_string(),
                    slot_type: SlotType::Internal,
                    from: ExecutionNode::Step(step.clone()),
                    to: ExecutionNode::Data(rgsoconcrete.get_typecheckoperative_slot()),
                },
            ]
        }
        ImplStepVariantTraitObject::ImplStepGetCollectionLength(rgsoconcrete) => {
            vec![
                ExecutionEdge {
                    slot_name: "InputCollection".to_string(),
                    slot_type: SlotType::Dependency,
                    from: ExecutionNode::Step(step.clone()),
                    to: ExecutionNode::Data(rgsoconcrete.get_inputcollection_slot()),
                },
                ExecutionEdge {
                    slot_name: "OutputInt".to_string(),
                    slot_type: SlotType::Output,
                    from: ExecutionNode::Step(step.clone()),
                    to: ExecutionNode::Data(rgsoconcrete.get_outputint_slot()),
                },
            ]
        }
        ImplStepVariantTraitObject::ImplStepIdentity(rgsoconcrete) => {
            vec![
                ExecutionEdge {
                    slot_name: "Input".to_string(),
                    slot_type: SlotType::Dependency,
                    from: ExecutionNode::Step(step.clone()),
                    to: ExecutionNode::Data(rgsoconcrete.get_input_slot()),
                },
                ExecutionEdge {
                    slot_name: "Output".to_string(),
                    slot_type: SlotType::Output,
                    from: ExecutionNode::Step(step.clone()),
                    to: ExecutionNode::Data(rgsoconcrete.get_output_slot()),
                },
            ]
        }
        ImplStepVariantTraitObject::ImplStepInvokeMethod(rgsoconcrete) => {
            let method_inputs = rgsoconcrete
                .get_methodinputs_slot()
                .iter()
                .map(|input| ExecutionEdge {
                    slot_name: "MethodInputs".to_string(),
                    slot_type: SlotType::Dependency,
                    from: ExecutionNode::Step(step.clone()),
                    to: ExecutionNode::Data(input.clone()),
                })
                .collect::<Vec<_>>();
            let method_outputs = rgsoconcrete
                .get_methodoutputs_slot()
                .iter()
                .map(|output| ExecutionEdge {
                    slot_name: "MethodOutputs".to_string(),
                    slot_type: SlotType::Output,
                    from: ExecutionNode::Step(step.clone()),
                    to: ExecutionNode::Data(output.clone()),
                })
                .collect::<Vec<_>>();
            let mut edges = vec![
                ExecutionEdge {
                    slot_name: "CallingOperative".to_string(),
                    slot_type: SlotType::Dependency,
                    from: ExecutionNode::Step(step.clone()),
                    to: ExecutionNode::Data(rgsoconcrete.get_callingoperative_slot()),
                },
                ExecutionEdge {
                    slot_name: "MethodName".to_string(),
                    slot_type: SlotType::Dependency,
                    from: ExecutionNode::Step(step.clone()),
                    to: ExecutionNode::Data(rgsoconcrete.get_methodname_slot()),
                },
            ];
            edges.extend(method_inputs);
            edges.extend(method_outputs);
            edges
        }
        ImplStepVariantTraitObject::ImplStepBitNot(rgsoconcrete) => {
            vec![
                ExecutionEdge {
                    slot_name: "InputBool".to_string(),
                    slot_type: SlotType::Dependency,
                    from: ExecutionNode::Step(step.clone()),
                    to: ExecutionNode::Data(rgsoconcrete.get_inputbool_slot()),
                },
                ExecutionEdge {
                    slot_name: "OutputBool".to_string(),
                    slot_type: SlotType::Output,
                    from: ExecutionNode::Step(step.clone()),
                    to: ExecutionNode::Data(rgsoconcrete.get_outputbool_slot()),
                },
            ]
        }
        ImplStepVariantTraitObject::ImplStepMathDivide(rgsoconcrete) => vec![
            ExecutionEdge {
                slot_name: "ArgumentOne".to_string(),
                slot_type: SlotType::Dependency,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_argumentone_slot()),
            },
            ExecutionEdge {
                slot_name: "ArgumentTwo".to_string(),
                slot_type: SlotType::Dependency,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_argumenttwo_slot()),
            },
            ExecutionEdge {
                slot_name: "OutputInt".to_string(),
                slot_type: SlotType::Output,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_outputint_slot()),
            },
        ],
        ImplStepVariantTraitObject::ImplStepBitOr(rgsoconcrete) => vec![
            ExecutionEdge {
                slot_name: "ArgumentOne".to_string(),
                slot_type: SlotType::Dependency,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_argumentone_slot()),
            },
            ExecutionEdge {
                slot_name: "ArgumentTwo".to_string(),
                slot_type: SlotType::Dependency,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_argumenttwo_slot()),
            },
            ExecutionEdge {
                slot_name: "OutputBool".to_string(),
                slot_type: SlotType::Output,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_outputbool_slot()),
            },
        ],
        ImplStepVariantTraitObject::ImplStepCompareEqual(rgsoconcrete) => vec![
            ExecutionEdge {
                slot_name: "ArgumentOne".to_string(),
                slot_type: SlotType::Dependency,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_argumentone_slot()),
            },
            ExecutionEdge {
                slot_name: "ArgumentTwo".to_string(),
                slot_type: SlotType::Dependency,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_argumenttwo_slot()),
            },
            ExecutionEdge {
                slot_name: "OutputBool".to_string(),
                slot_type: SlotType::Output,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_outputbool_slot()),
            },
        ],
        ImplStepVariantTraitObject::ImplStepBitAnd(rgsoconcrete) => vec![
            ExecutionEdge {
                slot_name: "ArgumentOne".to_string(),
                slot_type: SlotType::Dependency,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_argumentone_slot()),
            },
            ExecutionEdge {
                slot_name: "ArgumentTwo".to_string(),
                slot_type: SlotType::Dependency,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_argumenttwo_slot()),
            },
            ExecutionEdge {
                slot_name: "OutputBool".to_string(),
                slot_type: SlotType::Output,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_outputbool_slot()),
            },
        ],
        ImplStepVariantTraitObject::ImplStepMathAdd(rgsoconcrete) => vec![
            ExecutionEdge {
                slot_name: "ArgumentOne".to_string(),
                slot_type: SlotType::Dependency,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_argumentone_slot()),
            },
            ExecutionEdge {
                slot_name: "ArgumentTwo".to_string(),
                slot_type: SlotType::Dependency,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_argumenttwo_slot()),
            },
            ExecutionEdge {
                slot_name: "OutputInt".to_string(),
                slot_type: SlotType::Output,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_outputint_slot()),
            },
        ],
        ImplStepVariantTraitObject::ImplStepMathModulus(rgsoconcrete) => vec![
            ExecutionEdge {
                slot_name: "ArgumentOne".to_string(),
                slot_type: SlotType::Dependency,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_argumentone_slot()),
            },
            ExecutionEdge {
                slot_name: "ArgumentTwo".to_string(),
                slot_type: SlotType::Dependency,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_argumenttwo_slot()),
            },
            ExecutionEdge {
                slot_name: "OutputInt".to_string(),
                slot_type: SlotType::Output,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_outputint_slot()),
            },
        ],
        ImplStepVariantTraitObject::ImplStepMathMultiply(rgsoconcrete) => vec![
            ExecutionEdge {
                slot_name: "ArgumentOne".to_string(),
                slot_type: SlotType::Dependency,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_argumentone_slot()),
            },
            ExecutionEdge {
                slot_name: "ArgumentTwo".to_string(),
                slot_type: SlotType::Dependency,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_argumenttwo_slot()),
            },
            ExecutionEdge {
                slot_name: "OutputInt".to_string(),
                slot_type: SlotType::Output,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_outputint_slot()),
            },
        ],
        ImplStepVariantTraitObject::ImplStepMathSubtract(rgsoconcrete) => vec![
            ExecutionEdge {
                slot_name: "ArgumentOne".to_string(),
                slot_type: SlotType::Dependency,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_argumentone_slot()),
            },
            ExecutionEdge {
                slot_name: "ArgumentTwo".to_string(),
                slot_type: SlotType::Dependency,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_argumenttwo_slot()),
            },
            ExecutionEdge {
                slot_name: "OutputInt".to_string(),
                slot_type: SlotType::Output,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_outputint_slot()),
            },
        ],
        ImplStepVariantTraitObject::ImplStepCompareGreaterThan(rgsoconcrete) => vec![
            ExecutionEdge {
                slot_name: "ArgumentOne".to_string(),
                slot_type: SlotType::Dependency,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_argumentone_slot()),
            },
            ExecutionEdge {
                slot_name: "ArgumentTwo".to_string(),
                slot_type: SlotType::Dependency,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_argumenttwo_slot()),
            },
            ExecutionEdge {
                slot_name: "OutputBool".to_string(),
                slot_type: SlotType::Output,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_outputbool_slot()),
            },
        ],
        ImplStepVariantTraitObject::ImplStepCompareLessThan(rgsoconcrete) => vec![
            ExecutionEdge {
                slot_name: "ArgumentOne".to_string(),
                slot_type: SlotType::Dependency,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_argumentone_slot()),
            },
            ExecutionEdge {
                slot_name: "ArgumentTwo".to_string(),
                slot_type: SlotType::Dependency,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_argumenttwo_slot()),
            },
            ExecutionEdge {
                slot_name: "OutputBool".to_string(),
                slot_type: SlotType::Output,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_outputbool_slot()),
            },
        ],
        ImplStepVariantTraitObject::ImplStepIf(rgsoconcrete) => {
            let mut initial_vec =
                if let Some(output) = rgsoconcrete.get_output_slot().first().cloned() {
                    vec![ExecutionEdge {
                        slot_name: "Output".to_string(),
                        slot_type: SlotType::Output,
                        from: ExecutionNode::Step(step.clone()),
                        to: ExecutionNode::Data(output),
                    }]
                } else {
                    vec![]
                };
            initial_vec.extend(vec![
                ExecutionEdge {
                    slot_name: "Condition".to_string(),
                    slot_type: SlotType::Dependency,
                    from: ExecutionNode::Step(step.clone()),
                    to: ExecutionNode::Data(rgsoconcrete.get_condition_slot()),
                },
                ExecutionEdge {
                    slot_name: "TrueBranch".to_string(),
                    slot_type: SlotType::Internal,
                    from: ExecutionNode::Step(step.clone()),
                    to: ExecutionNode::Data(rgsoconcrete.get_truebranch_slot()),
                },
                ExecutionEdge {
                    slot_name: "FalseBranch".to_string(),
                    slot_type: SlotType::Internal,
                    from: ExecutionNode::Step(step.clone()),
                    to: ExecutionNode::Data(rgsoconcrete.get_falsebranch_slot()),
                },
            ]);
            initial_vec
        }
        ImplStepVariantTraitObject::ImplStepIteratorFilter(rgsoconcrete) => vec![
            ExecutionEdge {
                slot_name: "InputCollection".to_string(),
                slot_type: SlotType::Dependency,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_inputcollection_slot()),
            },
            ExecutionEdge {
                slot_name: "IterationEndBool".to_string(),
                slot_type: SlotType::Internal,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_iterationendbool_slot()),
            },
            ExecutionEdge {
                slot_name: "IterationStartItem".to_string(),
                slot_type: SlotType::Internal,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_iterationstartitem_slot()),
            },
            ExecutionEdge {
                slot_name: "OutputCollection".to_string(),
                slot_type: SlotType::Output,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_outputcollection_slot()),
            },
        ],
        ImplStepVariantTraitObject::ImplStepMutateSlot(rgsoconcrete) => vec![
            ExecutionEdge {
                slot_name: "MutatedOperative".to_string(),
                slot_type: SlotType::Dependency,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_mutatedoperative_slot()),
            },
            ExecutionEdge {
                slot_name: "NewValue".to_string(),
                slot_type: SlotType::Internal,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_newvalue_slot()),
            },
        ],
        ImplStepVariantTraitObject::ImplStepGetField(rgsoconcrete) => vec![
            ExecutionEdge {
                slot_name: "InputOperative".to_string(),
                slot_type: SlotType::Dependency,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_inputoperative_slot()),
            },
            ExecutionEdge {
                slot_name: "FieldName".to_string(),
                slot_type: SlotType::Dependency,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_fieldname_slot()),
            },
            ExecutionEdge {
                slot_name: "OutputField".to_string(),
                slot_type: SlotType::Output,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_outputfield_slot()),
            },
        ],
        ImplStepVariantTraitObject::ImplStepMutateField(rgsoconcrete) => vec![
            ExecutionEdge {
                slot_name: "MutatedOperative".to_string(),
                slot_type: SlotType::Dependency,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_mutatedoperative_slot()),
            },
            ExecutionEdge {
                slot_name: "NewValue".to_string(),
                slot_type: SlotType::Internal,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_newvalue_slot()),
            },
        ],
        ImplStepVariantTraitObject::ImplStepMultiTypeSplitter(rgsoconcrete) => vec![
            ExecutionEdge {
                slot_name: "InputMultiOperative".to_string(),
                slot_type: SlotType::Dependency,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_inputmultioperative_slot()),
            },
            ExecutionEdge {
                slot_name: "Convergence".to_string(),
                slot_type: SlotType::Internal,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_convergence_slot()),
            },
            ExecutionEdge {
                slot_name: "DiscriminantStarts".to_string(),
                slot_type: SlotType::Internal,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_discriminantstarts_slot()),
            },
            ExecutionEdge {
                slot_name: "Output".to_string(),
                slot_type: SlotType::Output,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_output_slot()),
            },
        ],
        ImplStepVariantTraitObject::ImplStepIteratorMap(rgsoconcrete) => vec![
            ExecutionEdge {
                slot_name: "InputCollection".to_string(),
                slot_type: SlotType::Dependency,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_inputcollection_slot()),
            },
            ExecutionEdge {
                slot_name: "IterationStartItem".to_string(),
                slot_type: SlotType::Internal,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_iterationstartitem_slot()),
            },
            ExecutionEdge {
                slot_name: "IterationEndItem".to_string(),
                slot_type: SlotType::Internal,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_iterationenditem_slot()),
            },
            ExecutionEdge {
                slot_name: "OutputCollection".to_string(),
                slot_type: SlotType::Output,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_outputcollection_slot()),
            },
        ],
        ImplStepVariantTraitObject::ImplStepTraverseSlot(rgsoconcrete) => vec![
            ExecutionEdge {
                slot_name: "InputOperative".to_string(),
                slot_type: SlotType::Dependency,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_inputoperative_slot()),
            },
            ExecutionEdge {
                slot_name: "SlotName".to_string(),
                slot_type: SlotType::Internal,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_slotname_slot()),
            },
            ExecutionEdge {
                slot_name: "OutputOperatives".to_string(),
                slot_type: SlotType::Output,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_outputoperatives_slot()),
            },
        ],
        ImplStepVariantTraitObject::ImplStepWhileLoop(rgsoconcrete) => vec![
            ExecutionEdge {
                slot_name: "InitialState".to_string(),
                slot_type: SlotType::Dependency,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_initialstate_slot()),
            },
            ExecutionEdge {
                slot_name: "LoopStateIngestor".to_string(),
                slot_type: SlotType::Internal,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_loopstateingestor_slot()),
            },
            ExecutionEdge {
                slot_name: "LoopExitState".to_string(),
                slot_type: SlotType::Internal,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_loopexitstate_slot()),
            },
            ExecutionEdge {
                slot_name: "ContinueWhileBool".to_string(),
                slot_type: SlotType::Internal,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_continuewhilebool_slot()),
            },
            ExecutionEdge {
                slot_name: "LoopStateStub".to_string(),
                slot_type: SlotType::Internal,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_loopstatestub_slot()),
            },
            ExecutionEdge {
                slot_name: "Output".to_string(),
                slot_type: SlotType::Output,
                from: ExecutionNode::Step(step.clone()),
                to: ExecutionNode::Data(rgsoconcrete.get_output_slot()),
            },
        ],
        ImplStepVariantTraitObject::ImplStepMapToOutput(rgsoconcrete) => {
            panic!("Should be marked as a terminal, not a step")
        }
        ImplStepVariantTraitObject::ImplStepMapFromInput(rgsoconcrete) => {
            panic!("Should be marked as a terminal, not a step")
        }
    }
}
