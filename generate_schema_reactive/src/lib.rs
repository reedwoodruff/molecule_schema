/*!
Code generation crate which ingests a schema and outputs static types to be included in a project.
*/

use base_types::post_generation::StandaloneRGSOConcrete;
pub use to_composite_id_macro;

use base_types::common::Uid;
use base_types::constraint_schema::*;
use base_types::constraint_schema_item::ConstraintSchemaItem;
use base_types::primitives::*;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use std::collections::HashMap;
use std::path::Path;
use utils::get_all_slots_enum_name;
use utils::get_all_subclasses;
use utils::get_operative_subclass_enum_name;
use utils::get_primitive_value_enum_variant_name;
use utils::get_slot_trait_enum_name;
use utils::get_template_get_slot_fn_name;
use utils::get_template_get_slots_trait_name;
use utils::get_template_slot_enum_name;

use crate::utils::get_all_operatives_which_implement_trait_set;
use crate::utils::get_operative_variant_name;
use crate::utils::get_operative_wrapped_name;
use crate::utils::get_primitive_type;
use crate::utils::get_template_get_field_fn_name;
use crate::utils::get_template_get_field_trait_name;
use crate::utils::get_template_get_slot_fn_name_id_only;

mod generate_operative_streams;
mod generate_trait_impl_streams;
mod utils;

struct FieldFnDetails {
    fn_name: TokenStream,
    fn_signature: TokenStream,
    field_return_type: TokenStream,
    field_return_type_enum_name: TokenStream,
}
struct SlotFnDetails {
    fn_name: TokenStream,
    fn_signature: TokenStream,
    return_enum_type: TokenStream,
    is_trait_slot: bool,
    id_only_signature: TokenStream,
    id_only_name: TokenStream,
    is_single_slot_bound: bool,
}
struct IntermediateFieldTraitInfo {
    trait_name: TokenStream,
    trait_fns: HashMap<Uid, FieldFnDetails>,
}
struct IntermediateSlotTraitInfo {
    trait_name: TokenStream,
    trait_fns: HashMap<Uid, SlotFnDetails>,
}
struct MetaData {
    template_field_trait_info: HashMap<Uid, IntermediateFieldTraitInfo>,
    template_slots_trait_info: HashMap<Uid, IntermediateSlotTraitInfo>,
}

fn impl_RGSO_for_enum(enum_name: TokenStream, members: Vec<syn::Ident>) -> TokenStream {
    quote! {
        impl RGSO for #enum_name {
            type Schema = Schema;
            fn operative(&self) -> &'static base_types::constraint_schema::LibraryOperative<base_types::primitives::PrimitiveTypes, base_types::primitives::PrimitiveValues> {
                match &self {
                #(Self::#members(item) => item.operative(),)*
                // _ => panic!(),
                }
            }
            fn get_id(&self) -> &base_types::common::Uid {
                match self {
                    #(Self::#members(item) => item.get_id(),)*
                    // _ => panic!(),
                }
            }
            fn template(&self) -> &'static base_types::constraint_schema::LibraryTemplate<base_types::primitives::PrimitiveTypes, base_types::primitives::PrimitiveValues> {
                match self {
                    #(Self::#members(item) => item.template(),)*
                    // _ => panic!(),
                }
            }
            fn outgoing_slots(&self) -> std::collections::BTreeMap<&base_types::common::Uid, &RActiveSlot>{
                match self {
                    #(Self::#members(item) => item.outgoing_slots(),)*
                    // _ => panic!(),
                }
            }
            fn incoming_slots(&self) -> leptos::prelude::RwSignal<Vec<base_types::post_generation::SlotRef>>{
                match self {
                    #(Self::#members(item) => item.incoming_slots(),)*
                    // _ => panic!(),
                }
            }
            fn fields(&self) -> &std::collections::HashMap<base_types::common::Uid, leptos::prelude::RwSignal<PrimitiveValues>>{
                match self {
                    #(Self::#members(item) => item.fields(),)*
                    // _ => panic!(),
                }
            }
        }
    }
}

/**
Given a path to a schema JSON file, returns Rust source code containing types to enable the building of instances of schema objects.
*/
pub fn generate_concrete_schema_reactive(
    // constraint_schema: ConstraintSchema<PrimitiveTypes, PrimitiveValues>,
    raw_json_schema: String,
    raw_json_initial_population: Option<String>,
) -> String {
    // The goal here is as follows:
    // 1. Map the constraint objects to individual structs which have:
    //      - The same structure as defined in the field constraints
    //      - A constructor which ensures that any instance of the constraint object satisfies its constraints:
    //          - All required fields are populated
    //          - All required slots are filled (respecting each slot_range)
    //      - Helper methods for adding and removing edges (but not mandatory ones)
    //  2. Create an enum with a variant for each struct

    let mut meta = MetaData {
        template_field_trait_info: HashMap::new(),
        template_slots_trait_info: HashMap::new(),
    };
    let constraint_schema: ConstraintSchema<PrimitiveTypes, PrimitiveValues> =
        serde_json::from_str(&raw_json_schema).expect("Schema formatted incorrectly");

    // Creates traits which represent geting the fields for each template
    // This should be implemented by every operative which is a subclass of the template
    let get_template_fields_traits_streams = constraint_schema
        .template_library
        .values()
        .map(|template| {
            let mut fns_map = HashMap::new();
            let fn_streams = template
                .field_constraints
                .values()
                .map(|field_constraint| {
                    let field_getter_fn_name =
                        get_template_get_field_fn_name(&field_constraint.tag.name);
                    let value_type = get_primitive_type(&field_constraint.value_type);
                    let enum_variant_name =
                        get_primitive_value_enum_variant_name(&field_constraint.value_type);
                    let stream = quote! { fn #field_getter_fn_name(&self) -> #value_type };
                    fns_map.insert(
                        field_constraint.tag.id,
                        FieldFnDetails {
                            fn_name: field_getter_fn_name.clone().into_token_stream(),
                            fn_signature: stream.clone(),
                            field_return_type: value_type,
                            field_return_type_enum_name: enum_variant_name,
                        },
                    );
                    stream
                })
                .collect::<Vec<_>>();
            let get_fields_trait_name = get_template_get_field_trait_name(&template.tag.name);
            meta.template_field_trait_info.insert(
                template.tag.id,
                IntermediateFieldTraitInfo {
                    trait_name: get_fields_trait_name.clone().into_token_stream(),
                    trait_fns: fns_map,
                },
            );
            quote! {
                pub trait #get_fields_trait_name {
                    #(#fn_streams;)*
                }
            }
        })
        .collect::<Vec<_>>();

    // Creates traits which represent getting the operative slots for every template
    // This should be implemented by every operative which is a subclass of the template
    let get_template_slots_traits_streams = constraint_schema.template_library.values().map(|template| {
        let mut fns_map = HashMap::new();
        let fn_streams = template.operative_slots.values().map(|operative_slot| {
            let is_single_slot_bound = matches!(operative_slot.bounds, SlotBounds::Single);
            let slot_getter_fn_name = get_template_get_slot_fn_name( &operative_slot.tag.name);
            let return_enum_type = get_template_slot_enum_name(&constraint_schema, operative_slot);
            let stream = match is_single_slot_bound {
                true => quote!{ fn #slot_getter_fn_name(&self) -> #return_enum_type },
                false => quote!{ fn #slot_getter_fn_name(&self) -> Vec<#return_enum_type> },
            };
            let id_only_slot_getter_fn_name = get_template_get_slot_fn_name_id_only(&operative_slot.tag.name);
            let id_only_stream = match is_single_slot_bound {
                true => quote!{ fn #id_only_slot_getter_fn_name(&self) -> base_types::common::Uid},
                false => quote!{ fn #id_only_slot_getter_fn_name(&self) -> Vec<base_types::common::Uid>},
            };
            let is_trait_slot = match operative_slot.operative_descriptor {
                OperativeVariants::LibraryOperative(_) => false,
                OperativeVariants::TraitOperative(_) => true,
            };
            fns_map.insert(operative_slot.tag.id,
                SlotFnDetails {
                    fn_name: slot_getter_fn_name.clone().into_token_stream(),
                    fn_signature: stream.clone(),
                    id_only_signature: id_only_stream.clone(),
                    id_only_name: id_only_slot_getter_fn_name.into_token_stream(),
                    return_enum_type, is_trait_slot ,
                    is_single_slot_bound
                });
            quote!{#stream;#id_only_stream;}
        }).collect::<Vec<_>>();
        let get_slots_trait_name = get_template_get_slots_trait_name(&template.tag.name);
        meta.template_slots_trait_info.insert(template.tag.id, IntermediateSlotTraitInfo { trait_name: get_slots_trait_name.clone().into_token_stream(), trait_fns: fns_map, });
        quote! {
            pub trait #get_slots_trait_name {
                #(#fn_streams)*
            }
        }
    }).collect::<Vec<_>>();

    // Gets all operatives which have subclasses, and creates an enum with each subclass as a variant
    // Each enum should implement the field getter and slot getter of the root template so as not to have to match as much as an end consumer
    let subclass_enums_stream =
        constraint_schema
            .operative_library
            .values()
            .filter_map(|operative| {
                let template_id = operative.get_template_id();
                let subclass_op_names = get_all_subclasses(&constraint_schema, &operative.tag.id)
                    .iter()
                    .map(|op| get_operative_variant_name(&op.tag.name))
                    .collect::<Vec<_>>();
                let subclass_op_names2 = subclass_op_names.clone();
                let enum_name =
                    get_operative_subclass_enum_name(&constraint_schema, &operative.tag.id);
                let op_wrapped_name = get_all_subclasses(&constraint_schema, &operative.tag.id)
                    .iter()
                    .map(|op| get_operative_wrapped_name(&op.tag.name))
                    .collect::<Vec<_>>();
                let IntermediateFieldTraitInfo {
                    trait_name: field_trait_name,
                    trait_fns: field_trait_fns,
                } = &meta.template_field_trait_info.get(template_id).unwrap();
                let field_streams = field_trait_fns.iter().fold(
                    Vec::new(),
                    |mut agg,
                     (
                        id,
                        FieldFnDetails {
                            fn_signature,
                            fn_name,
                            ..
                        },
                    )| {
                        let intermediate =
                            &subclass_op_names
                                .iter()
                                .fold(Vec::new(), |mut agg, subclass| {
                                    agg.push(
                                        quote! {#enum_name::#subclass(val) => val.#fn_name(),},
                                    );
                                    agg
                                });
                        agg.push(quote! {
                           #fn_signature {
                               match self {
                                   #(#intermediate)*
                                   // _ => panic!(),
                               }
                           }
                        });
                        agg
                    },
                );

                let IntermediateSlotTraitInfo {
                    trait_name: slot_trait_name,
                    trait_fns: slot_trait_fns,
                } = meta.template_slots_trait_info.get(template_id).unwrap();
                let slot_streams = slot_trait_fns.iter().fold(
                    Vec::new(),
                    |mut agg,
                     (
                        id,
                        SlotFnDetails {
                            fn_name,
                            fn_signature,
                            return_enum_type,
                            is_trait_slot,
                            id_only_signature,
                            id_only_name,
                            is_single_slot_bound,
                        },
                    )| {
                        let intermediate = &subclass_op_names.iter().fold(
                            (Vec::new(), Vec::new()),
                            |mut agg, subclass| {
                                agg.0
                                    .push(quote! {#enum_name::#subclass(val) => val.#fn_name(),});
                                agg.1.push(
                                    quote! {#enum_name::#subclass(val) => val.#id_only_name(),},
                                );
                                agg
                            },
                        );
                        let variant_streams = intermediate.0.clone();
                        let id_only_variant_streams = intermediate.1.clone();
                        agg.push(quote! {
                            #fn_signature {
                                match self {
                                #(#variant_streams)*
                                // _ => panic!(),
                                }
                            }
                            #id_only_signature {
                                match self {
                                #(#id_only_variant_streams)*
                                // _ => panic!(),
                                }
                            }
                        });
                        agg
                    },
                );

                let rgso_impl = impl_RGSO_for_enum(enum_name.clone(), subclass_op_names.clone());
                if subclass_op_names.len() <= 1 {
                    None
                } else {
                    // TODO: Make this enum implement all traits implemented by the superclass by passing the method call down to its variants
                    Some(quote! {
                     #[derive(Debug, Clone)]
                        pub enum #enum_name {
                            #(#subclass_op_names(#op_wrapped_name),)*
                        }
                         impl PartialEq for #enum_name {
                             fn eq(&self, other: &Self) -> bool {
                                 self.get_id() == other.get_id()
                             }
                         }
                        impl #field_trait_name for #enum_name {
                            #(#field_streams)*
                        }
                        impl #slot_trait_name for #enum_name {
                            #(#slot_streams)*
                        }
                        #rgso_impl
                    })
                }
            });

    // Checks every trait-operative slot, finds all unique trait combos, and creates an enum which represents all operatives which fulfill these trait combos
    let slot_trait_enums_stream = constraint_schema
        .template_library
        .values()
        .filter_map(|template| {
            let trait_ops = template
                .operative_slots
                .values()
                .filter_map(|slot| match &slot.operative_descriptor {
                    OperativeVariants::LibraryOperative(_) => None,
                    OperativeVariants::TraitOperative(trait_op) => Some(trait_op),
                })
                .collect::<Vec<_>>();
            if trait_ops.is_empty() {
                None
            } else {
                Some(trait_ops)
            }
        })
        .flatten()
        .fold(Vec::new(), |mut acc, trait_op| {
            let mut sorted = trait_op.trait_ids.clone();
            sorted.sort();
            if acc.contains(&sorted) {
                acc
            } else {
                acc.push(sorted);
                acc
            }
        })
        .iter()
        .map(|unique_trait_combo| {
            let enum_name = get_slot_trait_enum_name(&constraint_schema, unique_trait_combo);
            let fulfilling_ops = get_all_operatives_which_implement_trait_set(
                &constraint_schema,
                unique_trait_combo,
            );
            let fulfilling_ops_names = fulfilling_ops
                .iter()
                .map(|op| get_operative_variant_name(&op.tag.name))
                .collect::<Vec<_>>();
            let fulfilling_ops_wrapped_names = fulfilling_ops
                .iter()
                .map(|op| get_operative_wrapped_name(&op.tag.name));
            let rgso_impl = impl_RGSO_for_enum(
                enum_name.clone().into_token_stream(),
                fulfilling_ops_names.clone(),
            );

            // TODO: Make this enum implement all of these traits by passing the method call down to its variants
            quote! {
                #[derive(Debug, Clone)]
                pub enum #enum_name {
                    #(#fulfilling_ops_names(#fulfilling_ops_wrapped_names),)*
                }
                impl PartialEq for #enum_name {
                    fn eq(&self, other: &Self) -> bool {
                        self.get_id() == other.get_id()
                    }
                }
                #rgso_impl
            }
        })
        .collect::<Vec<_>>();

    // Creates the traits as defined in the schema
    let trait_definition_streams = constraint_schema.traits.values().map(|trait_def| {
        let trait_name = syn::Ident::new(&trait_def.tag.name, proc_macro2::Span::call_site());
        let fn_streams = trait_def.methods.values().map(|method_def| {
            let method_name = syn::Ident::new(&method_def.tag.name, proc_macro2::Span::call_site());
            let return_type = utils::get_primitive_type(&method_def.return_type);
            quote! {
                fn #method_name(&self) -> #return_type;
            }
        });
        quote! {
            pub trait #trait_name {
                #(#fn_streams)*
            }
        }
    });

    // Gather all slots and create an enum which maps to their ids
    // This to be used when searching an instance's incoming slots
    let all_slots_enum_prep = constraint_schema.template_library.values().fold(
        (Vec::<syn::Ident>::new(), Vec::<Uid>::new()),
        |mut agg, template| {
            let slots = template.operative_slots.values().fold(
                (Vec::<syn::Ident>::new(), Vec::<Uid>::new()),
                |mut inner_agg, slot| {
                    inner_agg
                        .0
                        .push(get_all_slots_enum_name(&template.tag.name, &slot.tag.name));
                    inner_agg.1.push(slot.tag.id);
                    inner_agg
                },
            );
            agg.0.extend(slots.0);
            agg.1.extend(slots.1);
            agg
        },
    );
    let all_slots_enum = {
        let variant_names = all_slots_enum_prep.0;
        let variant_ids = all_slots_enum_prep.1;
        quote! {
            pub enum AllSlots {
                #(#variant_names,)*
            }
            impl From<AllSlots> for Uid {
                fn from(value: AllSlots) -> Self {
                    match value {
                        #(AllSlots::#variant_names => #variant_ids,)*
                    }
                }
            }
        }
    };

    let reference_constraint_schema: ConstraintSchema<PrimitiveTypes, PrimitiveValues> =
        constraint_schema.clone();

    let library_operative_streams = constraint_schema
        .operative_library
        .values()
        .map(|el| {
            generate_operative_streams::generate_operative_streams(
                Box::new(el),
                &reference_constraint_schema,
                &meta,
            )
        })
        .collect::<Vec<_>>();
    let instance_streams = constraint_schema
        .instance_library
        .values()
        .map(|el| {
            generate_operative_streams::generate_operative_streams(
                Box::new(el),
                &reference_constraint_schema,
                &meta,
            )
        })
        .collect::<Vec<_>>();

    let all_lib_op_names = constraint_schema
        .operative_library
        .values()
        .map(|el| get_operative_variant_name(&el.tag.name))
        .collect::<Vec<_>>();

    let schema_name = syn::Ident::new("Schema", proc_macro2::Span::call_site());

    let schema_rgso_impl =
        impl_RGSO_for_enum(schema_name.into_token_stream(), all_lib_op_names.clone());

    let repatriate_num_match_stream = constraint_schema.operative_library.iter().map(
        |(id, op)| {
            let struct_name = get_operative_variant_name(&op.get_tag().name);
            quote! {
                #id => super::#struct_name::into_schema(
                    base_types::post_generation::reactive::RGSOConcrete::<super::#struct_name, Schema>::from_standalone(
                        value, graph, &super::CONSTRAINT_SCHEMA
                    )
                ),
            }
        },
    ).collect::<Vec<_>>();

    let doctored_initialize_graph_stream = {
        if let Some(initial_population) = raw_json_initial_population {
            quote! {
                pub fn initialize_graph() -> SharedGraph<Schema> {
                base_types::utils::initialize_graph_populated(&CONSTRAINT_SCHEMA, #initial_population)
                }
            }
        } else {
            quote! {
                pub fn initialize_graph() -> SharedGraph<Schema> {
                base_types::utils::initialize_graph_unpopulated(&CONSTRAINT_SCHEMA)
                }
            }
        }
    };

    let final_output = quote! {
        pub mod prelude {
        use base_types::post_generation::reactive::hidden::EditRGSO;
        pub use base_types::post_generation::reactive::*;
        use base_types::post_generation::*;
        use base_types::primitives::*;
        pub use base_types::post_generation::non_reactive::StandaloneRGSOConcrete;
        pub use leptos::prelude::*;
        use typenum::*;
        use base_types::utils::*;

        pub trait StaticTypestate {
            type InitialSlotTypestate;
            type EmptyFieldTypestate;
            type FulfilledFieldTypestate;
        }
        pub trait Incorporatable<T: std::clone::Clone + std::fmt::Debug + HasSlotEnum, TSchema>
        where <T as HasSlotEnum>::SlotEnum: std::clone::Clone + std::fmt::Debug + Send + Sync
        {
            fn get_inner_builder(self) -> SubgraphBuilder<T, TSchema>;
        }
        impl <T, TSchema: hidden::EditRGSO<Schema = TSchema> + 'static> Incorporatable<T, TSchema> for ExistingBuilder<T, TSchema>
            where T: Clone + std::fmt::Debug + Send + Sync + HasSlotEnum,
            <T as HasSlotEnum>::SlotEnum: std::clone::Clone + std::fmt::Debug + Send + Sync
            {
            fn get_inner_builder(self) -> SubgraphBuilder<T, TSchema> {
                self.inner_builder
            }
        }
        impl <T, TSchema: hidden::EditRGSO<Schema = TSchema>  +'static, FieldsTS, SlotsTS> Incorporatable<T, TSchema> for FreshBuilder<T, TSchema, FieldsTS, SlotsTS>
            where
                RGSOConcreteBuilder<T, TSchema>: RProducable<RGSOConcrete<T, TSchema>>,
                T: Send + Sync + RIntoSchema<Schema = TSchema> + Clone + std::fmt::Debug + 'static + HasSlotEnum,
                <T as HasSlotEnum>::SlotEnum: std::clone::Clone + std::fmt::Debug + Send + Sync
            {
                fn get_inner_builder(self) -> SubgraphBuilder<T, TSchema> {
                    self.inner_builder
                }
            }

        pub struct ExistingBuilder<T: std::clone::Clone + std::fmt::Debug + HasSlotEnum, TSchema>
        where TSchema: 'static,
            <T as HasSlotEnum>::SlotEnum: Clone + std::fmt::Debug + Send + Sync,
        {
            inner_builder: SubgraphBuilder<T, TSchema>
        }

        // Purpose of the FreshBuilder is to hide internal details which are exposed on the SubgraphBuilder
        // FreshBuilder enables carrying the current typestate of the Builder,
        // which allows conditional exposure of methods based on validity of the current structure.
        pub struct FreshBuilder<T: std::clone::Clone + std::fmt::Debug + HasSlotEnum, TSchema, FieldsTS, SlotsTS>
            where TSchema: 'static,
            <T as HasSlotEnum>::SlotEnum: Clone + std::fmt::Debug + Send + Sync,
        {
            inner_builder: SubgraphBuilder<T, TSchema>,
            _fields_typestate: std::marker::PhantomData<FieldsTS>,
            _slots_typestate: std::marker::PhantomData<SlotsTS>,
        }
        impl <T, TSchema: hidden::EditRGSO<Schema = TSchema> + 'static> ExistingBuilder<T, TSchema>
        where
            TSchema: Send + Sync,
            RGSOConcreteBuilder<T, TSchema>: RProducable<RGSOConcrete<T, TSchema>>,
            T: Send + Sync + RIntoSchema<Schema = TSchema> + Clone + std::fmt::Debug + 'static + HasSlotEnum,
            <T as HasSlotEnum>::SlotEnum: Clone + std::fmt::Debug + Send + Sync,
        {
            pub fn get_id(&self) -> &Uid {
                self.inner_builder.get_id()
            }
            pub fn execute(&self) -> Result<ExecutionResult, ElementCreationError> {
                self.inner_builder.execute()
            }
            pub fn incorporate<C: Send + Sync + std::fmt::Debug + Clone + RIntoSchema<Schema = TSchema> + 'static + HasSlotEnum>(&mut self, other_builder: impl Incorporatable<C, TSchema>)
            where
                <C as HasSlotEnum>::SlotEnum: Clone + std::fmt::Debug + Send + Sync,
            {
                self.inner_builder.incorporate(&other_builder.get_inner_builder())
            }
            pub fn set_temp_id(mut self, temp_id: &str) -> Self {
                self.inner_builder.set_temp_id(temp_id);
                self
            }
        }
        impl <T, TSchema: hidden::EditRGSO<Schema = TSchema> + 'static, FieldsTS, SlotsTS> FreshBuilder<T, TSchema, FieldsTS, SlotsTS>
            where
                TSchema: Send + Sync,
                RGSOConcreteBuilder<T, TSchema>: RProducable<RGSOConcrete<T, TSchema>>,
                T: Send + Sync + RIntoSchema<Schema = TSchema> + Clone + std::fmt::Debug + 'static + HasSlotEnum,
                <T as HasSlotEnum>::SlotEnum: Clone + std::fmt::Debug + Send + Sync,
            {
            pub fn get_id(&self) -> &Uid {
                self.inner_builder.get_id()
            }
            pub fn execute(&self) -> Result<ExecutionResult, ElementCreationError> {
                self.inner_builder.execute()
            }
            pub fn incorporate<C: Send + Sync + std::fmt::Debug + Clone + RIntoSchema<Schema = TSchema> + 'static + HasSlotEnum>(&mut self, other_builder: impl Incorporatable<C, TSchema>)
            where
                <C as HasSlotEnum>::SlotEnum: Clone + std::fmt::Debug + Send + Sync,
            {
                self.inner_builder.incorporate(&other_builder.get_inner_builder())
            }

            pub fn set_temp_id(mut self, temp_id: &str) -> Self {
                self.inner_builder.set_temp_id(temp_id);
                self
            }
        }

        // #trait_file_stream
        #(#library_operative_streams)*
        #(#get_template_fields_traits_streams)*
        #(#get_template_slots_traits_streams)*
        #(#subclass_enums_stream)*
        #(#slot_trait_enums_stream)*
        #(#trait_definition_streams)*

        fn validate_signal_is_some<T: Send + Sync + 'static>(signal: &leptos::prelude::RwSignal<Option<T>>) -> Result<(), base_types::post_generation::ElementCreationError> {
            signal.with(|val| {if val.is_some() {return Ok(())} return Err(ElementCreationError::RequiredFieldIsEmpty);})
        }

        lazy_static::lazy_static!{
            pub static ref CONSTRAINT_SCHEMA: base_types::constraint_schema::ConstraintSchema<PrimitiveTypes, PrimitiveValues>
            = serde_json::from_str::<base_types::constraint_schema::ConstraintSchema<PrimitiveTypes, PrimitiveValues>>(#raw_json_schema).expect("Schema formatted incorrectly");
        }

        #all_slots_enum
        #(#instance_streams)*

        #[derive(Debug, Clone)]
        pub enum Schema {
            #(#all_lib_op_names(RGSOConcrete<#all_lib_op_names, Schema>),)*
        }
        #[derive(Debug, Clone)]
        pub enum NonReactiveSchema {
            #(#all_lib_op_names(base_types::post_generation::GSOConcrete<#all_lib_op_names>),)*
        }
        impl From<Schema> for NonReactiveSchema {
            fn from(value: Schema) -> Self {
                match value {
                    #(Schema::#all_lib_op_names(val) => NonReactiveSchema::#all_lib_op_names(val.into()),)*
                }
            }
        }
        impl FromNonReactive<NonReactiveSchema> for Schema {
            fn from_non_reactive(value: NonReactiveSchema, graph: std::sync::Arc<RBaseGraphEnvironment<Schema>>) -> Self {
                match value {
                    #(NonReactiveSchema::#all_lib_op_names(val) => Schema::#all_lib_op_names(saturate_wrapper(val,graph)),)*
                }
            }
        }
        impl PartialEq for Schema {
            fn eq(&self, other: &Self) -> bool {
                self.get_id() == other.get_id()
            }
        }


        #schema_rgso_impl

        // This mod to keep the methods from these impls from being accessible to the end user
        pub(super) mod private_impl {
            use super::Schema;
            use base_types::post_generation::*;
            use base_types::post_generation::reactive::*;

            impl hidden::EditRGSO for Schema {
                fn remove_outgoing(& self, slot_ref: &base_types::post_generation::SlotRef) -> & Self{
                    match self {
                        #(Self::#all_lib_op_names(item) => {item.remove_outgoing(slot_ref); self},)*
                        // _ => panic!(),
                    }
                }
                fn remove_incoming(& self, parent_id: &base_types::common::Uid, slot_id: Option<&base_types::common::Uid>) -> Vec<base_types::post_generation::SlotRef> {
                    match self {
                        #(Self::#all_lib_op_names(item) => item.remove_incoming(parent_id, slot_id),)*
                        // _ => panic!(),
                    }
                }
                fn get_graph(&self) -> &std::sync::Arc<RBaseGraphEnvironment<Schema>> {
                    match self {
                        #(Self::#all_lib_op_names(item) => item.get_graph(),)*
                        // _ => panic!(),
                    }
                }
                fn add_outgoing(& self, slot_ref: base_types::post_generation::SlotRef) -> & Self {
                    match self {
                        #(Self::#all_lib_op_names(item) => {item.add_outgoing(slot_ref); self},)*
                        // _ => panic!(),
                    }
                }
                fn add_incoming(& self, slot_ref: base_types::post_generation::SlotRef) ->  &Self {
                    match self {
                        #(Self::#all_lib_op_names(item) => {item.add_incoming(slot_ref); self},)*
                        // _ => panic!(),
                    }
                }
                fn update_field(& self, field_edit: base_types::post_generation::non_reactive::HistoryFieldEdit) ->  &Self {
                    match self {
                        #(Self::#all_lib_op_names(item) => {item.update_field(field_edit); self},)*
                        // _ => panic!(),
                    }
                }
            }
            impl base_types::post_generation::reactive::from_reactive::FromStandalone for Schema {
                type Schema = Schema;
                fn from_standalone(value: base_types::post_generation::StandaloneRGSOConcrete, graph: SharedGraph<Self::Schema>, ) -> Self::Schema {
                   match value.operative {
                       #(#repatriate_num_match_stream)*
                       _ => unreachable!()
                   }
               }
            }
        }

        impl From<Schema> for base_types::post_generation::StandaloneRGSOConcrete {
           fn from(value: Schema) -> Self {
               let outgoing_slots = value.outgoing_slots().values().fold(Vec::new(), |mut agg, slot| {
                    let new_slot_refs = slot.slotted_instances.get().iter().map(|instance_id| {
                        base_types::post_generation::SlotRef {
                            host_instance_id: value.get_id().clone(),
                            slot_id: slot.slot.tag.id.clone(),
                            target_instance_id: instance_id.clone(),
                        }
                    }).collect::<Vec<_>>();
                    agg.extend(new_slot_refs);
                    agg
                });
               Self {
                   id: value.get_id().clone(),
                   fields: value.fields().into_iter().map( |(field_id, field_value)| {
                       (field_id.clone(), field_value.get())
                   }).collect(),
                   outgoing_slots,
                   incoming_slots: value.incoming_slots().get(),
                   operative: value.operative().tag.id.clone(),
                   template: value.template().tag.id.clone(),
               }
           }
        }

        #doctored_initialize_graph_stream

        }
    };
    final_output.to_string()
}

/// first argument is to the schema directory, second is to the molecule_schema project directory
/// if the third argument is specified, it should point to a json file of a populated graph,
/// and the graph will be prepoulated with that data
///
/// example usage: generate_crate!("resources/my_schema.json", "../molecule_schema");
/// example usage: generate_crate!("resources/my_schema.json", "../molecule_schema", Some("resources/my_starting_data.json"));
pub fn generate_crate(
    schema_location: &str,
    molecule_schema_workspace_location: &str,
    initial_population_location: Option<&str>,
) {
    use std::{env, fs, path::Path, process::Command};

    // let out_dir = std::env::var("OUT_DIR").unwrap();
    let generated_crate_dir = Path::new("target").join("generated_crate");
    let generated_cargo_toml_dir = generated_crate_dir.join("Cargo.toml");
    let generated_src_dir = generated_crate_dir.join("src");
    let generated_code_dir = generated_src_dir.join("lib.rs");
    let schema_path = Path::new(&schema_location);

    let raw_json_schema = std::fs::read_to_string(schema_location);
    let raw_json_schema = raw_json_schema.expect("schema json must be present");

    let initial_population = if let Some(initial_population_location) = initial_population_location
    {
        let json_initial_population = std::fs::read_to_string(initial_population_location)
            .expect("initial population location is incorrect");

        Some(json_initial_population)
    } else {
        None
    };

    let generated_code = generate_concrete_schema_reactive(raw_json_schema, initial_population);

    // Ensure directory exists
    fs::create_dir_all(&generated_crate_dir).unwrap();
    fs::create_dir_all(&generated_src_dir).unwrap();

    // Delete stale files
    fs::remove_file(&generated_code_dir);
    fs::remove_file(&generated_cargo_toml_dir);

    // Write out the crate's Cargo.toml and lib.rs
    fs::write(
            generated_cargo_toml_dir,
            format!(
                r#"
                    [package]
                    name = "generated_crate"
                    version = "0.1.0"
                    edition = "2021"

                    [lib]
                    path = "src/lib.rs"

                    [dependencies]
                    to_composite_id_macro = {{ path = "{}/to_composite_id_macro" }}
                    molecule_core = {{ path = "{}/molecule_core" }}
                    base_types = {{ path = "{}/base_types", features = ["serde"] }}
                    reactive_types = {{ path = "{}/reactive_types/" }}
                    lazy_static = "1.4"
                    strum = {{version = "0.26.1", features=["derive"]}}
                    strum_macros = "0.26.1"
                    serde = {{ version = "1", features = ["derive"] }}
                    serde_json = "1"
                    leptos = {{git = "https://github.com/leptos-rs/leptos", branch="main", features=["csr"]}}

                    [dependencies.uuid]
                    version = "1.4.1"
                    features = [
                        "v4",                # Lets you generate random UUIDs
                        "fast-rng",          # Use a faster (but still sufficiently random) RNG
                        "macro-diagnostics", # Enable better diagnostics for compile-time UUIDs
                        "js",
                    ]
                "#,
                molecule_schema_workspace_location, molecule_schema_workspace_location, molecule_schema_workspace_location, molecule_schema_workspace_location ,
            )
        )
        .unwrap();

    fs::write(generated_code_dir.clone(), generated_code).unwrap();

    let status = Command::new("rustfmt")
        .arg(&generated_code_dir)
        .status()
        .expect("Failed to run rustfmt");

    if !status.success() {
        panic!(
            "failed to format generated code. rustfmt failed with status: {:?}",
            status
        );
    }

    println!(
        "cargo::rerun-if-changed={}/generate_schema_reactive/src/lib.rs",
        molecule_schema_workspace_location
    );
    // println!("cargo::rerun-if-changed={}/generate_schema_reactive/src/generate_operative_stream.rs", ms_location);
    // println!("cargo::rerun-if-changed={}/generate_schema_reactive/src/generate_trait_impl_stream.rs", ms_location);
    // println!("cargo::rerun-if-changed={}/generate_schema_reactive/src/utils.rs", ms_location);
    println!("cargo::rerun-if-changed={}", schema_path.to_str().unwrap());

    // Emit a warning if the dependency isn't included
    println!("Make sure to add `generated_crate` as a dependency in Cargo.toml:\ngenerated_crate = {{ path = \"{}\"}}",
            generated_crate_dir.display());
}

// #[macro_export]
// first argument is to the schema directory, second is to the molecule_schema project directory

// example usage: generate_crate!("resources/my_schema.json", "../molecule_schema");
// macro_rules! generate_crate {
//     ($location:expr, $molecule_schema_location:expr) => {
//         crate::inner_generate_crate($location.to_string(), $molecule_schema_location.to_string())
//     };
// }
