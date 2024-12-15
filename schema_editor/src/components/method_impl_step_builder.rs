use crate::components::{common::*, method_impl_builder::MethodImplBuilderContext};
use schema_editor_generated_toolkit::prelude::*;

use super::method_impl_utils::{ExecVal, ExecutionSteps};

#[component]
pub fn MethodImplementationStepBuilder(step: ExecutionSteps) -> impl IntoView {
    let MethodImplBuilderContext { impling_operative } =
        use_context::<MethodImplBuilderContext>().unwrap();
    let arrow_view = view! { <div>"→"</div> };
    let step_clone = step.clone();
    let show_arrow = move || match step_clone.clone() {
        ExecutionSteps::MapFromInput { input } => false,
        ExecutionSteps::MapToOutput { output } => true,
        ExecutionSteps::GetField => true,
        ExecutionSteps::TraverseSlot => true,
        ExecutionSteps::MutateSlot => true,
        ExecutionSteps::MutateField => true,
        ExecutionSteps::IteratorFilter => true,
        ExecutionSteps::IteratorMap => true,
        ExecutionSteps::MultiTypeSplitter => true,
    };
    let step_clone = step.clone();
    let impling_operative_clone = impling_operative.clone();
    let step_view = move || {
        let impling_operative_clone = impling_operative_clone.clone();
        match step_clone.clone() {
            ExecutionSteps::MapFromInput { input } => {
                let exec_val = ExecVal::from_io_object(input.clone(), impling_operative_clone);
                view! {
                    <div class="method-impl entry">
                        <LeafSectionHeader>Map from Input</LeafSectionHeader>
                        <div>"Input name: "{move || input.get_name()}</div>
                        <ExecValDisplay exec_val=exec_val />
                    </div>
                }
            }
            ExecutionSteps::MapToOutput { output } => todo!(),
            ExecutionSteps::GetField => todo!(),
            ExecutionSteps::TraverseSlot => todo!(),
            ExecutionSteps::MutateSlot => todo!(),
            ExecutionSteps::MutateField => todo!(),
            ExecutionSteps::IteratorFilter => todo!(),
            ExecutionSteps::IteratorMap => todo!(),
            ExecutionSteps::MultiTypeSplitter => todo!(),
        }
    };
    view! {
        <Show when=show_arrow>{arrow_view.clone()}</Show>
        {step_view}
    }
}
#[component]
pub fn ExecValDisplay(exec_val: ExecVal) -> impl IntoView {
    let display_view = move || match exec_val.clone() {
        ExecVal::Bool => view! { <div class="single-exec-val">Bool</div> }.into_any(),
        ExecVal::String => view! { <div class="single-exec-val">String</div> }.into_any(),
        ExecVal::Int => view! { <div class="single-exec-val">Int</div> }.into_any(),
        ExecVal::SingleOperative { allowed_operative } => view! {
            <div class="single-exec-val">
                "Single Operative: " {move || allowed_operative.get_name()}
            </div>
        }
        .into_any(),
        ExecVal::MultiOperative { allowed_operatives } => view! {
            <div class="single-exec-val">
                "Multi Operative: ["
                {move || {
                    allowed_operatives
                        .clone()
                        .into_iter()
                        .map(|item| item.get_name())
                        .collect::<Vec<_>>()
                        .join(", ")
                }} "]"
            </div>
        }
        .into_any(),
        ExecVal::TraitOperative { required_traits } => view! {
            <div class="single-exec-val">
                "Trait Operative. Required traits: ["
                {move || {
                    required_traits
                        .clone()
                        .into_iter()
                        .map(|item| item.get_name())
                        .collect::<Vec<_>>()
                        .join(", ")
                }} "]"
            </div>
        }
        .into_any(),
        ExecVal::CollectionBool => {
            view! { <div class="collection-exec-val">Collection of Bools</div> }.into_any()
        }
        ExecVal::CollectionString => {
            view! { <div class="collection-exec-val">Collection of Strings</div> }.into_any()
        }
        ExecVal::CollectionInt => {
            view! { <div class="collection-exec-val">Collection of Ints</div> }.into_any()
        }
        ExecVal::CollectionSingleOperative { allowed_operative } => view! {
            <div class="collection-exec-val">
                "Collection of Single Operatives: " {move || allowed_operative.get_name()}
            </div>
        }
        .into_any(),
        ExecVal::CollectionMultiOperative { allowed_operatives } => view! {
            <div class="collection-exec-val">
                "Collection of Multi Operatives: ["
                {move || {
                    allowed_operatives
                        .clone()
                        .into_iter()
                        .map(|item| item.get_name())
                        .collect::<Vec<_>>()
                        .join(", ")
                }} "]"
            </div>
        }
        .into_any(),
        ExecVal::CollectionTraitOperative { required_traits } => view! {
            <div class="collection-exec-val">
                "Collection of Trait Operatives. Required traits: ["
                {move || {
                    required_traits
                        .clone()
                        .into_iter()
                        .map(|item| item.get_name())
                        .collect::<Vec<_>>()
                        .join(", ")
                }} "]"
            </div>
        }
        .into_any(),
    };
    view! { <div class="exec-val-container">{display_view}</div> }
}
