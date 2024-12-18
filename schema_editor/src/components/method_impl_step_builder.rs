use crate::components::{common::*, method_impl_builder::MethodImplBuilderContext};
use leptos::either::Either;
use schema_editor_generated_toolkit::prelude::*;

use super::method_impl_utils::{ExecVal, ExecutionSteps};

#[component]
pub fn MethodImplementationStepBuilder(step: Signal<ExecutionSteps>) -> impl IntoView {
    let MethodImplBuilderContext { impling_operative } =
        use_context::<MethodImplBuilderContext>().unwrap();
    let arrow_view = view! { <div>"→"</div> };
    let step_clone = step.clone();
    let show_arrow = move || match step_clone.clone().get() {
        ExecutionSteps::MapFromInput { input } => false,
        _ => true,
    };
    let step_clone = step.clone();
    let impling_operative_clone = impling_operative.clone();
    let step_view = move || {
        let impling_operative_clone = impling_operative_clone.clone();
        match step_clone.clone().get() {
            ExecutionSteps::MapFromInput { input } => {
                let exec_val = ExecVal::from_io_object(input.clone(), impling_operative_clone);
                view! {
                    <div class="method-impl entry">
                        <LeafSectionHeader>MapfromInput</LeafSectionHeader>
                        <div>"Input name: "{move || input.get_name()}</div>
                        <ExecValDisplay exec_val=exec_val />
                    </div>
                }
            }
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
            ExecutionSteps::MultiTypeSplitter { arms } => todo!(),
            ExecutionSteps::IteratorAggregator { output } => todo!(),
            ExecutionSteps::MultiTypeAggregator { output } => todo!(),
        }
    };
    view! {
        <Show when=show_arrow>{arrow_view.clone()}</Show>
        {step_view}
        <NextStep step=step />
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

#[component]
pub fn NextStep(step: Signal<ExecutionSteps>) -> impl IntoView {
    let selected_next_step = RwSignal::new(None);
    let next_step_options = Memo::new(move |_| step.get().get_allowed_next_steps());

    move || {
        if matches!(step.get(), ExecutionSteps::MapToOutput { output: _ }) {
            Either::Left(view! {})
        } else {
            Either::Right(view! {})
        }
    }
}
