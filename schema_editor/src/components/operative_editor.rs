use std::str::FromStr;

use crate::components::{
    common::*,
    workspace::{WorkspaceState, WorkspaceTab},
};
use leptos::either::EitherOf3;
use schema_editor_generated_toolkit::{
    prelude::*, slot_markers::OperativeConcreteLockedFieldsAcceptableTargetMarker,
};
use strum::{Display, EnumIter, EnumString};
#[derive(EnumIter, Display, EnumString, PartialEq, Clone)]
enum BoolOptions {
    #[strum(serialize = "true")]
    True,
    #[strum(serialize = "false")]
    False,
}

#[component]
pub fn OperativeEditor(operative: RGSOConcrete<OperativeConcrete, Schema>) -> impl IntoView {
    let derivative_operative_name = RwSignal::new(operative.get_name_field());
    let ctx = use_context::<SharedGraph<Schema>>().unwrap();
    let WorkspaceState {
        schema,
        selected_tab,
    } = use_context::<WorkspaceState>().unwrap();
    let ctx_clone = ctx.clone();
    let schema_clone = schema.clone();
    let selected_tab = selected_tab.clone();
    let operative_clone = operative.clone();

    let create_derivative_operative = move |_| {
        // Really not liking being forced to do two graph actions -- need to figure out how to fix the api.
        let derivative_operative_name = derivative_operative_name.clone().get();
        let mut editor = schema.edit(ctx_clone.clone()).add_new_operatives(|op| {
            op.set_name(derivative_operative_name.clone())
                .add_existing_roottemplate(
                    operative_clone.get_roottemplate_slot().get_id(),
                    |item| item,
                )
                .add_existing_parentoperative(operative_clone.get_id(), |na| na)
                .set_temp_id("new_op")
        });
        editor.incorporate(
            operative_clone
                .edit(ctx_clone.clone())
                .add_temp_childrenoperatives("new_op"),
        );
        let new_op_id = editor
            .execute()
            .unwrap()
            .get_final_id("new_op")
            .unwrap()
            .clone();

        let locked_fields = operative_clone.get_lockedfields_slot();
        if locked_fields.len() > 0 {
            match ctx_clone.get(&new_op_id).unwrap() {
                Schema::OperativeConcrete(item) => {
                    let mut editor = item.edit(ctx_clone.clone());
                    for locked_field in operative_clone.get_lockedfields_slot() {
                        match locked_field {
                            FulfilledFieldVariantTraitObject::BoolFulfilledField(_) => {
                                editor = editor.add_existing_lockedfields::<BoolFulfilledField>(
                                    locked_field.get_id(),
                                    |na| na,
                                )
                            }
                            FulfilledFieldVariantTraitObject::IntFulfilledField(_) => {
                                editor = editor.add_existing_lockedfields::<IntFulfilledField>(
                                    locked_field.get_id(),
                                    |na| na,
                                )
                            }
                            FulfilledFieldVariantTraitObject::StringFulfilledField(_) => {
                                editor = editor.add_existing_lockedfields::<StringFulfilledField>(
                                    locked_field.get_id(),
                                    |na| na,
                                )
                            }
                        }
                    }
                    editor.execute().unwrap();
                }
                _ => panic!(),
            };
        }
    };

    let operative_clone = operative.clone();
    let ctx_clone = ctx.clone();
    let update_name = move |new_val: String| {
        let editor = operative_clone.edit(ctx_clone.clone());
        editor.set_name(new_val).execute().unwrap();
    };

    let operative_clone = operative.clone();
    let ctx_clone = ctx.clone();
    let selected_tab_clone = selected_tab.clone();
    let delete_operative = move |_| {
        let ctx_clone = ctx_clone.clone();
        operative_clone
            .edit(ctx_clone)
            .delete_recursive()
            .execute()
            .unwrap();
        selected_tab_clone.set(WorkspaceTab::Operative(RwSignal::new(None)))
    };

    let operative_clone = operative.clone();
    let non_locked_fields = Memo::new(move |_| {
        let locked_fields = operative_clone
            .get_lockedfields_slot()
            .into_iter()
            .map(|item| match item {
                FulfilledFieldVariantTraitObject::BoolFulfilledField(item) => {
                    item.get_constraintreference_slot().get_id().clone()
                }
                FulfilledFieldVariantTraitObject::IntFulfilledField(item) => {
                    item.get_constraintreference_slot().get_id().clone()
                }
                FulfilledFieldVariantTraitObject::StringFulfilledField(item) => {
                    item.get_constraintreference_slot().get_id().clone()
                }
            })
            .collect::<Vec<_>>();
        operative_clone
            .get_roottemplate_slot()
            .get_fields_slot()
            .into_iter()
            .filter(|field| !locked_fields.contains(field.get_id()))
            .collect::<Vec<_>>()
    });

    let operative_clone = operative.clone();
    let ctx_clone = ctx.clone();
    let non_locked_field_view = move |field: GetNameFieldVariantTraitObject| {
        let field_clone = field.clone();
        let string_of_thing: GetNameFieldVariantTraitObjectDiscriminants = field.clone().into();
        let operative_clone = operative_clone.clone();
        let ctx_clone = ctx_clone.clone();
        let on_click_lock = move |_| {
            match field_clone {
                GetNameFieldVariantTraitObject::StringTemplateField(_) => {
                    let mut editor = operative_clone
                        .edit(ctx_clone.clone())
                        .add_new_lockedfields::<StringFulfilledField, _>(|locked_field| {
                            locked_field
                                .set_temp_id("the_field")
                                .set_value("".to_string())
                                .add_existing_fulfiller(operative_clone.get_id(), |na| na)
                                .add_existing_constraintreference(field_clone.get_id(), |na| na)
                        });
                    recurse_add_locked_field::<StringFulfilledField>(
                        operative_clone.get_childrenoperatives_slot(),
                        &mut editor,
                        &ctx_clone,
                    );
                    editor.execute().unwrap();
                }
                GetNameFieldVariantTraitObject::BoolTemplateField(_) => {
                    let mut editor = operative_clone
                        .edit(ctx_clone.clone())
                        .add_new_lockedfields::<BoolFulfilledField, _>(|locked_field| {
                            locked_field
                                .set_temp_id("the_field")
                                .set_value(true)
                                .add_existing_fulfiller(operative_clone.get_id(), |na| na)
                                .add_existing_constraintreference(field_clone.get_id(), |na| na)
                        });
                    recurse_add_locked_field::<BoolFulfilledField>(
                        operative_clone.get_childrenoperatives_slot(),
                        &mut editor,
                        &ctx_clone,
                    );
                    editor.execute().unwrap();
                }
                GetNameFieldVariantTraitObject::IntTemplateField(_) => {
                    let mut editor = operative_clone
                        .edit(ctx_clone.clone())
                        .add_new_lockedfields::<IntFulfilledField, _>(|locked_field| {
                            locked_field
                                .set_temp_id("the_field")
                                .set_value(0)
                                .add_existing_fulfiller(operative_clone.get_id(), |na| na)
                                .add_existing_constraintreference(field_clone.get_id(), |na| na)
                        });
                    recurse_add_locked_field::<IntFulfilledField>(
                        operative_clone.get_childrenoperatives_slot(),
                        &mut editor,
                        &ctx_clone,
                    );
                    editor.execute().unwrap();
                }
            };
        };
        view! {
            <LeafSection>
            <LeafSectionHeader>
            {move || field.get_name()}
            </LeafSectionHeader>
            {string_of_thing.to_string()} <Button on:click=on_click_lock>Lock</Button>
            </LeafSection>
        }
    };

    let operative_clone = operative.clone();
    let locked_fields: Memo<(Vec<_>, Vec<_>)> = Memo::new(move |_| {
        operative_clone
            .get_lockedfields_slot()
            .into_iter()
            .partition(|locked_field| match locked_field {
                FulfilledFieldVariantTraitObject::BoolFulfilledField(item) => {
                    item.get_fulfiller_slot().get_id() == operative_clone.get_id()
                }
                FulfilledFieldVariantTraitObject::IntFulfilledField(item) => {
                    item.get_fulfiller_slot().get_id() == operative_clone.get_id()
                }
                FulfilledFieldVariantTraitObject::StringFulfilledField(item) => {
                    item.get_fulfiller_slot().get_id() == operative_clone.get_id()
                }
            })
    });

    let unowned_locked_field_view = move |field: FulfilledFieldVariantTraitObject| {
        let field_view = move || match field.clone() {
            FulfilledFieldVariantTraitObject::BoolFulfilledField(value) => {
                let value_clone = value.clone();
                let name = move || value_clone.get_constraintreference_slot().get_name();
                let value_clone = value.clone();
                let value = move || value_clone.get_value_field();
                EitherOf3::A(view! {
                    <LeafSectionHeader>
                    {name}
                    </LeafSectionHeader>
                    {value}
                })
            }
            FulfilledFieldVariantTraitObject::IntFulfilledField(value) => {
                let value_clone = value.clone();
                let name = move || value_clone.get_constraintreference_slot().get_name();
                let value_clone = value.clone();
                let value = move || value_clone.get_value_field();
                EitherOf3::B(view! {
                    <LeafSectionHeader>
                    {name}
                    </LeafSectionHeader>
                    {value}
                })
            }
            FulfilledFieldVariantTraitObject::StringFulfilledField(value) => {
                let value_clone = value.clone();
                let name = move || value_clone.get_constraintreference_slot().get_name();
                let value_clone = value.clone();
                let value = move || value_clone.get_value_field();
                EitherOf3::C(view! {
                    <LeafSectionHeader>
                    {name}
                    </LeafSectionHeader>
                    {value}
                })
            }
        };
        view! {<LeafSection>
            {field_view}
        </LeafSection>}
    };
    let ctx_clone = ctx.clone();
    let owned_locked_field_view = move |field: FulfilledFieldVariantTraitObject| {
        let field_clone = field.clone();
        let ctx_clone = ctx_clone.clone();
        let ctx_clone_2 = ctx_clone.clone();
        let field_view = move || match field_clone.clone() {
            FulfilledFieldVariantTraitObject::BoolFulfilledField(value) => {
                let value_clone = value.clone();
                let name = move || value_clone.get_constraintreference_slot().get_name();
                let ctx_clone = ctx_clone.clone();
                let value_clone = value.clone();
                let field_value = move || {
                    BoolOptions::from_str(&value_clone.get_value_field().to_string()).unwrap()
                };
                let value_clone = value.clone();
                let setter = Callback::new(move |new_val: BoolOptions| {
                    let bool_val: bool = new_val.to_string().parse().unwrap();
                    value_clone
                        .edit(ctx_clone.clone())
                        .set_value(bool_val)
                        .execute()
                        .unwrap();
                });
                EitherOf3::A(view! {
                    <LeafSectionHeader>
                    {name}
                    </LeafSectionHeader>
                    <ManagedEnumSelect getter=field_value setter=setter/>
                })
            }
            FulfilledFieldVariantTraitObject::IntFulfilledField(value) => {
                let value_clone = value.clone();
                let name = move || value_clone.get_constraintreference_slot().get_name();
                let value_clone = value.clone();
                let field_value = move || value_clone.get_value_field().to_string();
                let ctx_clone = ctx_clone.clone();
                let value_clone = value.clone();
                let setter = move |item: String| {
                    let num_val: u32 = item.parse().expect("bad number input");
                    value_clone
                        .edit(ctx_clone.clone())
                        .set_value(num_val)
                        .execute()
                        .unwrap();
                };
                EitherOf3::B(view! {
                    <LeafSectionHeader>
                    {name}
                    </LeafSectionHeader>
                    <ToggleManagedTextInput prop:type="number" prop:min=0 getter=field_value setter=setter/>
                })
            }
            FulfilledFieldVariantTraitObject::StringFulfilledField(value) => {
                let value_clone = value.clone();
                let name = move || value_clone.get_constraintreference_slot().get_name();
                let value_clone = value.clone();
                let field_value = move || value_clone.get_value_field();
                let ctx_clone = ctx_clone.clone();
                let value_clone = value.clone();

                let setter = move |item: String| {
                    value_clone
                        .edit(ctx_clone.clone())
                        .set_value(item)
                        .execute()
                        .unwrap();
                };
                EitherOf3::C(view! {
                    <LeafSectionHeader>
                    {name}
                    </LeafSectionHeader>
                    <ToggleManagedTextInput getter=field_value setter=setter/>
                })
            }
        };

        let field_clone = field.clone();
        let ctx_clone = ctx_clone_2.clone();
        let on_click_unlock = move |_| {
            match field_clone.clone() {
                FulfilledFieldVariantTraitObject::BoolFulfilledField(inner_field) => inner_field
                    .edit(ctx_clone.clone())
                    .delete()
                    .execute()
                    .unwrap(),
                FulfilledFieldVariantTraitObject::IntFulfilledField(inner_field) => inner_field
                    .edit(ctx_clone.clone())
                    .delete()
                    .execute()
                    .unwrap(),
                FulfilledFieldVariantTraitObject::StringFulfilledField(inner_field) => inner_field
                    .edit(ctx_clone.clone())
                    .delete()
                    .execute()
                    .unwrap(),
            };
        };
        view! {<LeafSection>
            {field_view}
            <Button on:click=on_click_unlock>Unlock</Button>
        </LeafSection>}
    };

    let operative_clone = operative.clone();
    let operative_clone_3 = operative.clone();
    view! {
        <Section>
            <SectionHeader>Overview</SectionHeader>
            <SubSection>
                <SubSectionHeader>
                    Name:
                </SubSectionHeader>
                <ToggleManagedTextInput getter=move || operative_clone.get_name_field() setter=update_name />
            </SubSection>
            <SubSection>
                <Button on:click=delete_operative>Delete Item</Button>
            </SubSection>
        </Section>

        <Section>
            <SectionHeader>Create Derivatives</SectionHeader>
            <SignalTextInput value=derivative_operative_name/><Button on:click=create_derivative_operative>Create derivative operative</Button>
        </Section>
        <Section>
            <SectionHeader>Fields</SectionHeader>
            <SubSection>
                <For each=move||locked_fields.get().1 key=|item| item.get_id().clone() children=unowned_locked_field_view />
                <For each=move||locked_fields.get().0 key=|item| item.get_id().clone() children=owned_locked_field_view />
                <For each=move || non_locked_fields.get() key=|item| item.get_id().clone() children=non_locked_field_view />
            </SubSection>
        </Section>
    }
}

fn recurse_add_locked_field<
    T: OperativeConcreteLockedFieldsAcceptableTargetMarker
        + RootConstraints<Schema>
        + schema_editor_generated_toolkit::prelude::StaticTypestate,
>(
    children: Vec<RGSOConcrete<OperativeConcrete, Schema>>,
    mut_editor: &mut ExistingBuilder<OperativeConcrete, Schema>,
    ctx_clone: &SharedGraph<Schema>,
) {
    children.into_iter().for_each(|child| {
        mut_editor.incorporate(
            child
                .edit(ctx_clone.clone())
                .add_temp_lockedfields::<T>("the_field"),
        );
        recurse_add_locked_field::<T>(child.get_childrenoperatives_slot(), mut_editor, ctx_clone)
    });
}
