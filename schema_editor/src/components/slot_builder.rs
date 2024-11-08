use leptos::either::Either;
use schema_editor_generated_toolkit::prelude::*;

use crate::components::{
    common::{Button, SignalEnumSelect, SignalSelectWithOptions, SignalTextInput},
    workspace::WorkspaceState,
};

#[component]
pub fn SlotBuilder(
    template: RGSOConcrete<TemplateConcrete, Schema>, // builder: FreshBuilder<
                                                      //     TemplateSlot,
                                                      //     Schema,
                                                      //     <TemplateSlot as StaticTypestate>::EmptyFieldTypestate,
                                                      //     <TemplateSlot as StaticTypestate>::InitialSlotTypestate,
                                                      // >,
) -> impl IntoView {
    let ctx = use_context::<SharedGraph<Schema>>().unwrap();
    let ctx_clone = ctx.clone();
    let WorkspaceState {
        schema,
        selected_tab,
    } = use_context::<WorkspaceState>().unwrap();
    let schema_clone = schema.clone();

    let name = RwSignal::new("new_slot".to_string());
    let slot_type =
        RwSignal::new(TemplateSlotVariantTraitObjectDiscriminants::ConreteOperativeVariant);

    let dropdown_selected_trait = RwSignal::new(None); // TraitConcrete
    let selected_trait_list = RwSignal::new(Vec::new()); // TraitConcrete
    let final_selected_trait_list = Memo::new(move |_| {
        selected_trait_list
            .get()
            .into_iter()
            .filter(|item| schema_clone.get_traits_slot().contains(item))
            .collect::<Vec<_>>()
    });
    // let selected_operative_for_slot = RwSignal::new(None); //OperativeConcrete

    let schema_clone = schema.clone();

    let trait_slot_details_view = move || {
        let ctx_clone = ctx_clone.clone();
        let schema_clone = schema_clone.clone();
        let on_click_add_trait = move |_| match dropdown_selected_trait.get() {
            Some(new_trait_item) => {
                dropdown_selected_trait.set(None);
                selected_trait_list.update(|prev| prev.push(new_trait_item));
            }
            None => (),
        };
        let trait_options = Signal::derive(move || {
            schema_clone
                .get_traits_slot()
                .into_iter()
                .filter(|item| {
                    selected_trait_list.with(|selected_list| !selected_list.contains(item))
                })
                .collect::<Vec<_>>()
        });
        let template_clone = template.clone();
        let on_click_save_trait_slot = move |_| {
            template_clone.edit(ctx_clone.clone());
            // .add_new_templateslots(|new_template_slot| {
            //     new_template_slot
            //         .set_name(name.get())
            //         .add_new_operativevariant::<TraitOperativeVariant, _>(|new_op_var| {
            //             fn do_the_thing<T, Y, Z>(op_var: T, item: Y) -> Z
            //             where T: Can_add_existing_traits{

            //             }
            //             final_selected_trait_list.get().into_iter().fold(
            //                 new_op_var,
            //                 |agg, trait_connection| {
            //                     new_op_var
            //                         .add_existing_traits(trait_connection.get_id(), |na| na)
            //                 },
            //             )
            //         })
            // })
        };
        view! {
            <div>
                <SignalSelectWithOptions value=dropdown_selected_trait options=trait_options empty_allowed=true/>
                <Button on:click=on_click_add_trait>Add</Button>
            </div>
            <div>
                <For each=move ||final_selected_trait_list.get() key=|selected_item| selected_item.get_id().clone() let:selected_item>
                {
                    let selected_item_clone = selected_item.clone();
                    let item_name = move || selected_item_clone.get_name();
                    let on_click = move |_|{
                        selected_trait_list.update(|prev| prev.retain(|prev_item| prev_item.get_id() != selected_item.get_id()))
                    };
                    view!{
                        <span>{item_name}</span><Button on:click=on_click>X</Button>,
                    }
                }
                </For>
            </div>
            <div>
                <Button on:click=on_click_save_trait_slot>Save New Slot</Button>
            </div>
        }
    };
    let concrete_op_slot_details_view = move || {
        view! {
            ConcreteOp stuff
        }
    };
    let schema_clone = schema.clone();
    view! {
    <div>
        <div>
            Slot name: <SignalTextInput value=name />
        </div>
        <div>
            Slot type: <SignalEnumSelect value=slot_type/>
        </div>
        <div>
        {move || match slot_type.get() {
                TemplateSlotVariantTraitObjectDiscriminants::TraitOperativeVariant => {
                    Either::Left(trait_slot_details_view.clone())
                }
                TemplateSlotVariantTraitObjectDiscriminants::ConreteOperativeVariant => {
                    Either::Right(concrete_op_slot_details_view)
                }
        }}
        </div>
    </div>
    }
}
