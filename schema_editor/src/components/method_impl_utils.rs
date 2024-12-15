use schema_editor_generated_toolkit::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum ExecutionSteps {
    MapFromInput {
        input: GetNameFunctionIOTraitObject,
    },
    MapToOutput {
        output: GetNameFunctionIOTraitObject,
    },
    GetField,
    TraverseSlot,
    MutateSlot,
    MutateField,
    IteratorFilter,
    IteratorMap,
    MultiTypeSplitter,
}

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

impl ExecVal {
    pub fn from_io_object(
        io_object: GetNameFunctionIOTraitObject,
        impling_operative: RGSOConcrete<OperativeConcrete, Schema>,
    ) -> Self {
        match io_object {
            GetNameFunctionIOTraitObject::FunctionIOCollectionPrimitiveBool(_) => {
                ExecVal::CollectionBool
            }
            GetNameFunctionIOTraitObject::FunctionIOSingleOperative(item) => {
                ExecVal::SingleOperative {
                    allowed_operative: item.get_allowedoperative_slot(),
                }
            }
            GetNameFunctionIOTraitObject::FunctionIOPrimitiveBool(_) => ExecVal::Bool,
            GetNameFunctionIOTraitObject::FunctionIOCollectionMultiOperative(item) => {
                ExecVal::CollectionMultiOperative {
                    allowed_operatives: item.get_allowedoperatives_slot(),
                }
            }
            GetNameFunctionIOTraitObject::FunctionIOCollectionTraitOperative(item) => {
                ExecVal::CollectionTraitOperative {
                    required_traits: item.get_requiredtraits_slot(),
                }
            }
            GetNameFunctionIOTraitObject::FunctionIOCollectionPrimitiveString(_) => {
                ExecVal::CollectionString
            }
            GetNameFunctionIOTraitObject::FunctionIOTraitOperative(item) => {
                ExecVal::TraitOperative {
                    required_traits: item.get_requiredtraits_slot(),
                }
            }
            GetNameFunctionIOTraitObject::FunctionIOSelf(_) => ExecVal::SingleOperative {
                allowed_operative: impling_operative,
            },
            GetNameFunctionIOTraitObject::FunctionIOCollectionSingleOperative(item) => {
                ExecVal::CollectionSingleOperative {
                    allowed_operative: item.get_allowedoperative_slot(),
                }
            }
            GetNameFunctionIOTraitObject::FunctionIOPrimitiveInt(_) => ExecVal::Int,
            GetNameFunctionIOTraitObject::FunctionIOMultiOperative(item) => {
                ExecVal::MultiOperative {
                    allowed_operatives: item.get_allowedoperatives_slot(),
                }
            }
            GetNameFunctionIOTraitObject::FunctionIOPrimitiveString(_) => ExecVal::String,
            GetNameFunctionIOTraitObject::FunctionIOCollectionPrimitiveInt(_) => {
                ExecVal::CollectionInt
            }
        }
    }
}
impl From<ImplIOTraitObject> for ExecVal {
    fn from(value: ImplIOTraitObject) -> Self {
        match value {
            ImplIOTraitObject::ImplCollectionPrimitiveInt(_) => ExecVal::CollectionInt,
            ImplIOTraitObject::ImplIntermediateMultiOperative(item) => ExecVal::MultiOperative {
                allowed_operatives: item.get_allowedoperatives_slot(),
            },
            ImplIOTraitObject::ImplIntermediatePrimitiveBool(_) => ExecVal::Bool,
            ImplIOTraitObject::ImplIntermediatePrimitiveInt(_) => ExecVal::Int,
            ImplIOTraitObject::ImplIntermediatePrimitiveString(_) => ExecVal::String,
            ImplIOTraitObject::ImpCollectionMultiOperative(item) => {
                ExecVal::CollectionMultiOperative {
                    allowed_operatives: item.get_allowedoperatives_slot(),
                }
            }
            ImplIOTraitObject::ImplIntermediateSingleOperative(item) => ExecVal::SingleOperative {
                allowed_operative: item.get_allowedoperative_slot(),
            },
            ImplIOTraitObject::ImplCollectionTraitOperative(item) => {
                ExecVal::CollectionTraitOperative {
                    required_traits: item.get_requiredtraits_slot(),
                }
            }
            ImplIOTraitObject::ImplCollectionSingleOperative(item) => {
                ExecVal::CollectionSingleOperative {
                    allowed_operative: item.get_allowedoperative_slot(),
                }
            }
            ImplIOTraitObject::ImplCollectionPrimitiveString(_) => ExecVal::CollectionString,
            ImplIOTraitObject::ImplIntermediateTraitOperative(item) => ExecVal::TraitOperative {
                required_traits: item.get_requiredtraits_slot(),
            },
            ImplIOTraitObject::ImplCollectionPrimitiveBool(_) => ExecVal::CollectionBool,
        }
    }
}
