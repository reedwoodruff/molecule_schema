use schema_editor_generated_toolkit::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug, Hash, strum_macros::EnumDiscriminants)]
pub enum ExecutionSteps {
    MapFromInput {
        input: (),
    },
    MapToOutput {
        output: (),
    },
    GetField {
        field_to_get: FieldVariantTraitObject,
    },
    TraverseSlot {
        slot_to_traverse: SlotDescriptionTraitObject,
    },
    MutateSlot {
        reference_slot: SlotDescriptionTraitObject,
        add_to_slot: Vec<AddToSlotMutationDescriptor>,
        remove_from_slot: Vec<RemoveFromSlotMutationDescriptor>,
    },
    MutateField {
        template_field: FieldVariantTraitObject,
        new_value: ExecValPrimitives,
    },
    IteratorFilter {
        filter_steps: Vec<ExecutionSteps>,
    },
    IteratorMap {
        map_steps: Vec<ExecutionSteps>,
    },
    IteratorAggregator {
        output: (),
    },
    MultiTypeSplitter {
        arms: Vec<Vec<ExecutionSteps>>,
    },
    MultiTypeAggregator {
        output: ExecVal,
    },
}

impl ExecutionSteps {
    pub fn get_output_val(&self) {}
    pub fn get_allowed_next_steps(&self) {
        match self {
            ExecutionSteps::MapFromInput { input } => todo!(),
            ExecutionSteps::MapToOutput { output } => todo!(),
            ExecutionSteps::GetField { field_to_get } => todo!(),
            ExecutionSteps::TraverseSlot { slot_to_traverse } => todo!(),
            ExecutionSteps::MutateSlot {
                reference_slot,
                add_to_slot,
                remove_from_slot,
            } => todo!(),
            ExecutionSteps::MutateField {
                template_field,
                new_value,
            } => todo!(),
            ExecutionSteps::IteratorFilter { filter_steps } => todo!(),
            ExecutionSteps::IteratorMap { map_steps } => todo!(),
            ExecutionSteps::IteratorAggregator { output } => todo!(),
            ExecutionSteps::MultiTypeSplitter { arms } => todo!(),
            ExecutionSteps::MultiTypeAggregator { output } => todo!(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AddToSlotMutationDescriptor {}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RemoveFromSlotMutationDescriptor {}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum ExecVal {
    Bool,
    String,
    Int,
    SingleOperative {
        allowed_operative: RGSOConcrete<OperativeConcrete, Schema>,
    },
    MultiOperative {
        allowed_operatives: Vec<RGSOConcrete<OperativeConcrete, Schema>>,
    },
    TraitOperative {
        required_traits: Vec<RGSOConcrete<TraitConcrete, Schema>>,
    },
    CollectionBool,
    CollectionString,
    CollectionInt,
    CollectionSingleOperative {
        allowed_operative: RGSOConcrete<OperativeConcrete, Schema>,
    },
    CollectionMultiOperative {
        allowed_operatives: Vec<RGSOConcrete<OperativeConcrete, Schema>>,
    },
    CollectionTraitOperative {
        required_traits: Vec<RGSOConcrete<TraitConcrete, Schema>>,
    },
}

// impl ExecVal {
//     pub fn from_io_object(
//         io_object: GetNameFunctionIOTraitObject,
//         impling_operative: RGSOConcrete<OperativeConcrete, Schema>,
//     ) -> Self {
//         match io_object {
//             GetNameFunctionIOTraitObject::FunctionIOCollectionPrimitiveBool(_) => {
//                 ExecVal::CollectionBool
//             }
//             GetNameFunctionIOTraitObject::FunctionIOSingleOperative(item) => {
//                 ExecVal::SingleOperative {
//                     allowed_operative: item.get_allowedoperative_slot(),
//                 }
//             }
//             GetNameFunctionIOTraitObject::FunctionIOPrimitiveBool(_) => ExecVal::Bool,
//             GetNameFunctionIOTraitObject::FunctionIOCollectionMultiOperative(item) => {
//                 ExecVal::CollectionMultiOperative {
//                     allowed_operatives: item.get_allowedoperatives_slot(),
//                 }
//             }
//             GetNameFunctionIOTraitObject::FunctionIOCollectionTraitOperative(item) => {
//                 ExecVal::CollectionTraitOperative {
//                     required_traits: item.get_requiredtraits_slot(),
//                 }
//             }
//             GetNameFunctionIOTraitObject::FunctionIOCollectionPrimitiveString(_) => {
//                 ExecVal::CollectionString
//             }
//             GetNameFunctionIOTraitObject::FunctionIOTraitOperative(item) => {
//                 ExecVal::TraitOperative {
//                     required_traits: item.get_requiredtraits_slot(),
//                 }
//             }
//             GetNameFunctionIOTraitObject::FunctionIOSelf(_) => ExecVal::SingleOperative {
//                 allowed_operative: impling_operative,
//             },
//             GetNameFunctionIOTraitObject::FunctionIOCollectionSingleOperative(item) => {
//                 ExecVal::CollectionSingleOperative {
//                     allowed_operative: item.get_allowedoperative_slot(),
//                 }
//             }
//             GetNameFunctionIOTraitObject::FunctionIOPrimitiveInt(_) => ExecVal::Int,
//             GetNameFunctionIOTraitObject::FunctionIOMultiOperative(item) => {
//                 ExecVal::MultiOperative {
//                     allowed_operatives: item.get_allowedoperatives_slot(),
//                 }
//             }
//             GetNameFunctionIOTraitObject::FunctionIOPrimitiveString(_) => ExecVal::String,
//             GetNameFunctionIOTraitObject::FunctionIOCollectionPrimitiveInt(_) => {
//                 ExecVal::CollectionInt
//             }
//         }
//     }
//     fn get_allowed_next_steps(&self) -> Vec<ExecutionStepsDiscriminants> {
//         match self {
//             ExecVal::Bool => todo!(),
//             ExecVal::String => todo!(),
//             ExecVal::Int => todo!(),
//             ExecVal::SingleOperative { allowed_operative } => todo!(),
//             ExecVal::MultiOperative { allowed_operatives } => todo!(),
//             ExecVal::TraitOperative { required_traits } => todo!(),
//             ExecVal::CollectionBool => todo!(),
//             ExecVal::CollectionString => todo!(),
//             ExecVal::CollectionInt => todo!(),
//             ExecVal::CollectionSingleOperative { allowed_operative } => todo!(),
//             ExecVal::CollectionMultiOperative { allowed_operatives } => todo!(),
//             ExecVal::CollectionTraitOperative { required_traits } => todo!(),
//         }
//     }
// }
// impl From<ImplIOTraitObject> for ExecVal {
//     fn from(value: ImplIOTraitObject) -> Self {
//         match value {
//             ImplIOTraitObject::ImplCollectionPrimitiveInt(_) => ExecVal::CollectionInt,
//             ImplIOTraitObject::ImplIntermediateMultiOperative(item) => ExecVal::MultiOperative {
//                 allowed_operatives: item.get_allowedoperatives_slot(),
//             },
//             ImplIOTraitObject::ImplIntermediatePrimitiveBool(_) => ExecVal::Bool,
//             ImplIOTraitObject::ImplIntermediatePrimitiveInt(_) => ExecVal::Int,
//             ImplIOTraitObject::ImplIntermediatePrimitiveString(_) => ExecVal::String,
//             ImplIOTraitObject::ImpCollectionMultiOperative(item) => {
//                 ExecVal::CollectionMultiOperative {
//                     allowed_operatives: item.get_allowedoperatives_slot(),
//                 }
//             }
//             ImplIOTraitObject::ImplIntermediateSingleOperative(item) => ExecVal::SingleOperative {
//                 allowed_operative: item.get_allowedoperative_slot(),
//             },
//             ImplIOTraitObject::ImplCollectionTraitOperative(item) => {
//                 ExecVal::CollectionTraitOperative {
//                     required_traits: item.get_requiredtraits_slot(),
//                 }
//             }
//             ImplIOTraitObject::ImplCollectionSingleOperative(item) => {
//                 ExecVal::CollectionSingleOperative {
//                     allowed_operative: item.get_allowedoperative_slot(),
//                 }
//             }
//             ImplIOTraitObject::ImplCollectionPrimitiveString(_) => ExecVal::CollectionString,
//             ImplIOTraitObject::ImplIntermediateTraitOperative(item) => ExecVal::TraitOperative {
//                 required_traits: item.get_requiredtraits_slot(),
//             },
//             ImplIOTraitObject::ImplCollectionPrimitiveBool(_) => ExecVal::CollectionBool,
//         }
//     }
// }

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ExecValPrimitives {
    Bool,
    String,
    Int,
}

// impl From<ExecValPrimitives> for ExecVal {
//     fn from(value: ExecValPrimitives) -> Self {
//         match value {
//             ExecValPrimitives::Bool => ExecVal::Bool,
//             ExecValPrimitives::String => ExecVal::String,
//             ExecValPrimitives::Int => ExecVal::Int,
//         }
//     }
// }

// #[derive(Clone, Debug, PartialEq, Eq, Hash)]
// pub enum ExecValCollections {
//     CollectionBool,
//     CollectionString,
//     CollectionInt,
//     CollectionSingleOperative {
//         allowed_operative: RGSOConcrete<OperativeConcrete, Schema>,
//     },
//     CollectionMultiOperative {
//         allowed_operatives: Vec<RGSOConcrete<OperativeConcrete, Schema>>,
//     },
//     CollectionTraitOperative {
//         required_traits: Vec<RGSOConcrete<TraitConcrete, Schema>>,
//     },
// }

// impl From<ExecValCollections> for ExecVal {
//     fn from(value: ExecValCollections) -> Self {
//         match value {
//             ExecValCollections::CollectionBool => ExecVal::CollectionBool,
//             ExecValCollections::CollectionString => ExecVal::CollectionString,
//             ExecValCollections::CollectionInt => ExecVal::CollectionInt,
//             ExecValCollections::CollectionSingleOperative { allowed_operative } => {
//                 ExecVal::CollectionSingleOperative { allowed_operative }
//             }
//             ExecValCollections::CollectionMultiOperative { allowed_operatives } => {
//                 ExecVal::CollectionMultiOperative { allowed_operatives }
//             }
//             ExecValCollections::CollectionTraitOperative { required_traits } => {
//                 ExecVal::CollectionTraitOperative { required_traits }
//             }
//         }
//     }
// }
