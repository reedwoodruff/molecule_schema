use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::quote;
use serde_types::common::*;
use serde_types::constraint_schema::*;
use serde_types::primitives::*;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::Comma,
    Data, DeriveInput, Fields, Result as SynResult, Token, Type,
};
struct TypeList(Punctuated<Type, Token![,]>);

impl Parse for TypeList {
    fn parse(input: ParseStream) -> SynResult<Self> {
        Ok(TypeList(input.parse_terminated(Type::parse, Token![,])?))
    }
}

fn get_primitive_value(ty: &PrimitiveTypes) -> proc_macro2::TokenStream {
    match ty {
        PrimitiveTypes::String => quote! {String},
        PrimitiveTypes::U32 => quote! {u32},
        PrimitiveTypes::I32 => quote! {i32},
        PrimitiveTypes::F32 => quote! {f32},
        PrimitiveTypes::Bool => quote! {bool},
        PrimitiveTypes::Char => quote! {char},
        PrimitiveTypes::Option(inner) => {
            let inner = get_primitive_value(inner);
            quote! {Option<#inner>}
        }
        _ => panic!("Not a PrimitiveType"),
    }
}
#[proc_macro]
pub fn generate_constraint_schema(input: TokenStream) -> TokenStream {
    let data = std::fs::read_to_string("generate_schema_macro/resources/schema.json");
    let data = data.expect("schema json must be present");
    let constraint_schema_generated: ConstraintSchema<PrimitiveTypes, PrimitiveValues> =
        serde_json::from_str::<ConstraintSchema<PrimitiveTypes, PrimitiveValues>>(&*data)
            .expect("json should be formatted correctly");

    // The goal here is as follows:
    // 1. Map the constraint objects to individual structs which have:
    //      - The same structure as defined in the field constraints
    //      - A constructor function which ensures that all constraints are met (edge and field)
    //      - Some reference to the internal structure of the template (maybe just reference to the
    //      constraint_schema id )
    //      - Helper methods for adding and removing edges (but not mandatory ones)
    //  2. Create an enum with a variant for each struct

    let mut element_names = Vec::<syn::Ident>::new();
    let reference_constraint_schema = constraint_schema_generated.clone();
    let schema_elements = constraint_schema_generated
        .constraint_objects
        .iter()
        .map(|(id, el)| {
            let name = el.tag.name.clone();
            let mut field_names = Vec::<syn::Ident>::new();
            let mut field_names_setters = Vec::<syn::Ident>::new();
            let mut field_values = Vec::<proc_macro2::TokenStream>::new();
            let mut initial_values = Vec::<proc_macro2::TokenStream>::new();
            let operatives: Vec<_> = el.operatives.iter().map(|op_id| {
                reference_constraint_schema.operative_library.get(op_id).expect("Operative Library should contain the operative ID referenced within a template's constituents")
            }).collect(); 
            let fields = el
                .field_constraints
                .iter()
                .for_each(|field| {
                    let name = syn::Ident::new(&field.tag.name, proc_macro2::Span::call_site());
                    field_names.push(name.clone());
                    field_names_setters.push(syn::Ident::new(
                        &("set_".to_string() + &field.tag.name),
                        proc_macro2::Span::call_site(),
                    ));
                    let field_value = get_primitive_value(&field.value_type);
                    if let PrimitiveTypes::Option(inner) = &field.value_type {
                        initial_values.push(quote! {Some(None)});
                    } else {
                        initial_values.push(quote! {None});
                    }
                    field_values.push(field_value.clone());
                });
            // let fields_clone = fields.clone();
            let struct_name = syn::Ident::new(&name, proc_macro2::Span::call_site());
            element_names.push(struct_name.clone());

            let struct_builder_name = name + "Builder";
            let struct_builder_name =
                syn::Ident::new(&struct_builder_name, proc_macro2::Span::call_site());

            quote! {
                #[derive(Debug, Clone)]
                pub struct #struct_name {
                    #(#field_names: #field_values,)*
                    constraint_schema_id: serde_types::common::Uid,
                }
                impl #struct_name {
                    fn initiate_build() -> #struct_builder_name {
                        #struct_builder_name::new()
                    }
                }
                impl GSO for #struct_name {
                    fn get_constraint_schema_id(&self) -> serde_types::common::Uid {
                        self.constraint_schema_id
                    }
                }
                pub struct #struct_builder_name {
                    // remaining_operatives: Vec<serde_types::constraint_schema::LibraryOperative<serde_types::primitives::PrimitiveTypes, serde_types::primitives::PrimitiveValues>>,
                    #(#field_names: Option<#field_values>,)*
                }
                impl #struct_builder_name {
                    pub fn new() -> Self{
                        Self {
                            // remaining_operatives: #operatives,
                            #(#field_names: #initial_values,)*
                        }
                    }
                    fn check_completion(&self) -> bool {
                        let mut complete = true;
                        #(if self.#field_names.is_none() {complete = false;})*
                        return complete
                    }
                    #(pub fn #field_names_setters(mut self, val: #field_values) -> Self {
                        self.#field_names = Some(val);
                        self
                    })*
                    pub fn build(mut self) -> Result<#struct_name, bool> {
                        if self.check_completion() == true {
                        Ok(#struct_name {
                            #(#field_names: self.#field_names.unwrap(),)*
                            constraint_schema_id: #id,
                        })
                        } else {
                            Err(false)
                        }
                    }
                }
            }
        });
    quote! {
        // const SCHEMA_JSON: &str = #data;
        pub trait GSO {
            fn get_constraint_schema_id(&self) -> serde_types::common::Uid;
        }
        #(#schema_elements)*
        #[derive(Debug)]
        enum Schema {
            #(#element_names(#element_names) ,)*
        }
        impl GSO for Schema {
            fn get_constraint_schema_id(&self) -> serde_types::common::Uid {
                match &self {
                #(Self::#element_names(item) => item.get_constraint_schema_id(),)*
                _ => panic!(),
            }
            }
        }
       // const constraint_schema: ConstraintSchema<PrimitiveTypes, PrimitiveValues> = #constraint_schema_generated;
        //  let constraint_schema: serde_types::constraint_schema::ConstraintSchema<serde_types::primitives::PrimitiveTypes, serde_types::primitives::PrimitiveValues> =
        // serde_json::from_str(SCHEMA_JSON).unwrap();
    }
    .into()
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, marker::PhantomData};

    use serde_types::common::*;
    use serde_types::constraint_schema::*;

    use super::*;
    use serde::{Deserialize, Serialize};
    use serde_json::{from_str, to_string_pretty};
    #[test]
    fn test() {
        type TTypesImpl = PrimitiveTypes;
        type TValuesImpl = PrimitiveValues;

        let mut constraint_objects = HashMap::new();

        constraint_objects.insert(
            0,
            ConstraintObject {
                trait_impls: HashMap::new(),
                tag: Tag {
                    name: "Person".to_string(),
                    id: 0,
                },
                field_constraints: vec![
                    FieldConstraint {
                        tag: Tag {
                            id: 0,
                            name: "name".to_string(),
                        },
                        value_type: TTypesImpl::String,
                    },
                    FieldConstraint {
                        tag: Tag {
                            id: 1,
                            name: "age".to_string(),
                        },
                        value_type: TTypesImpl::Option(Box::new(TTypesImpl::U32)),
                    },
                ],
                // edge_constraints: vec![],
                // constituents: vec![],
                operatives: vec![],
                instances: vec![],
            },
        );
        constraint_objects.insert(
            1,
            ConstraintObject {
                trait_impls: HashMap::from([(
                    0,
                    TraitImpl {
                        trait_id: 0,
                        methods: HashMap::from([(
                            0,
                            TraitMethodImpl {
                                trait_method_id: 0,
                                fulfillment_path: vec![TraitPath::Field(0)],
                            },
                        )]),
                    },
                )]),
                tag: Tag {
                    name: "Sock".to_string(),
                    id: 1,
                },
                field_constraints: vec![FieldConstraint {
                    tag: Tag {
                        id: 0,
                        name: "color".to_string(),
                    },
                    value_type: TTypesImpl::String,
                }],
                // edge_constraints: vec![],
                // constituents: vec![],
                operatives: vec![],
                instances: vec![],
            },
        );
        constraint_objects.insert(
            2,
            ConstraintObject {
                trait_impls: HashMap::new(),
                tag: Tag {
                    name: "HasColoredObject".to_string(),
                    id: 2,
                },
                field_constraints: vec![],
                // edge_constraints: vec![
                //     FuzzyEdgeDescriptor::new()
                //         .dir(Dir::Recv)
                //         .edge_type(EdgeType::Slot(0))
                //         .add_target_schema_trait(0),
                //     FuzzyEdgeDescriptor::new()
                //         .dir(Dir::Emit)
                //         .edge_type(EdgeType::Slot(1)),
                // ],
                // constituents: vec![],
                operatives: vec![900, 901],
                instances: vec![800],
            },
        );

        let mut traits = HashMap::new();
        let color_trait = TraitDef {
            tag: Tag {
                id: 0,
                name: "Color".to_string(),
            },
            methods: vec![TraitMethodDef {
                tag: Tag {
                    id: 0,
                    name: "getColor".to_string(),
                },
                return_type: TTypesImpl::String,
            }],
        };
        traits.insert(0, color_trait);

        let mut operatives = HashMap::new();
        let locked_age_field = FulfilledFieldConstraint {
            constraint_tag: Tag {
                id: 1,
                name: "age".to_string(),
            },
            value_type: TTypesImpl::Option(Box::new(TTypesImpl::U32)),
            value: TValuesImpl::Option(Box::new(TValuesImpl::U32(99))),
        };
        let owner_operative = LibraryOperative::TemplateOperative {
            constraint_object_id: 0,
            id: 900,
            fulfilled_operatives: vec![],
            locked_fields: vec![locked_age_field],
        };
        let ownee_operative: LibraryOperative<TTypesImpl, TValuesImpl> =
            LibraryOperative::TraitOperative { trait_id: 0 };
        operatives.insert(900, owner_operative);
        operatives.insert(901, ownee_operative);

        let test_schema: ConstraintSchema<TTypesImpl, TValuesImpl> = ConstraintSchema {
            constraint_objects: constraint_objects,
            instance_library: Default::default(),
            operative_library: operatives,
            traits: traits,
        };
        println!("{}", to_string_pretty(&test_schema).unwrap());
        // panic!();
    }
    // #[test]
    // fn run_macro() {
    //     generate_constraint_schema!();
    // }
}
