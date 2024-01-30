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
use output_types;

struct TypeList(Punctuated<Type, Token![,]>);

impl Parse for TypeList {
    fn parse(input: ParseStream) -> SynResult<Self> {
        Ok(TypeList(input.parse_terminated(Type::parse, Token![,])?))
    }
}

fn get_primitive_type(ty: &PrimitiveTypes) -> proc_macro2::TokenStream {
    match ty {
        PrimitiveTypes::String => quote! {String},
        PrimitiveTypes::U32 => quote! {u32},
        PrimitiveTypes::I32 => quote! {i32},
        PrimitiveTypes::F32 => quote! {f32},
        PrimitiveTypes::Bool => quote! {bool},
        PrimitiveTypes::Char => quote! {char},
        PrimitiveTypes::Option(inner) => {
            let inner = get_primitive_type(inner);
            quote! {Option<#inner>}
        }
        _ => panic!("Not a PrimitiveType"),
    }
}
fn get_primitive_value(ty: &PrimitiveValues) -> proc_macro2::TokenStream {
    match ty {
        PrimitiveValues::I32(val) => quote! {#val},
        PrimitiveValues::U32(val) => quote! {#val},
        PrimitiveValues::F32(val) => quote! {#val},
        PrimitiveValues::String(val) => quote! {#val},
        PrimitiveValues::Bool(val) => quote! {#val},
        PrimitiveValues::Char(val) => quote! {#val},
        PrimitiveValues::Option(val) => {
            let inner = get_primitive_value(val);
            quote! {#inner}
        },
    }
}
#[proc_macro]
pub fn generate_concrete_schema(input: TokenStream) -> TokenStream {
    let graph_environment = syn::parse_macro_input!(input as syn::Expr); 

    // let data = std::fs::read_to_string("generate_schema_macro/resources/schema.json");
    // let data = data.expect("schema json must be present");
    // let constraint_schema_generated: ConstraintSchema<PrimitiveTypes, PrimitiveValues> =
    //     serde_json::from_str::<ConstraintSchema<PrimitiveTypes, PrimitiveValues>>(&data)
    //         .expect("json should be formatted correctly");

    constraint_schema::constraint_schema!();

    // The goal here is as follows:
    // 1. Map the constraint objects to individual structs which have:
    //      - The same structure as defined in the field constraints
    //      - A constructor function which ensures that all constraints are met (edge and field)
    //      - Some reference to the internal structure of the template (maybe just reference to the
    //      constraint_schema id )
    //      - Helper methods for adding and removing edges (but not mandatory ones)
    //  2. Create an enum with a variant for each struct

    let trait_definition_streams = constraint_schema_generated.traits.values().map(| trait_def| {
        let trait_name = syn::Ident::new(&trait_def.tag.name, proc_macro2::Span::call_site());
        let fn_streams = trait_def.methods.iter().map(|method_def| {
            let method_name = syn::Ident::new(&method_def.tag.name, proc_macro2::Span::call_site());
            let return_type = get_primitive_type(&method_def.return_type);    
            quote! {
                fn #method_name(&self) -> std::borrow::Cow<#return_type>;
            }
        });
        quote! {
            pub trait #trait_name {
                #(#fn_streams)*
            }
        }
    });
    let mut element_names = Vec::<syn::Ident>::new();
    let reference_constraint_schema: ConstraintSchema<PrimitiveTypes, PrimitiveValues> = constraint_schema_generated.clone();

    let generate_trait_impl_streams = |instantiable: Box::<&dyn ConstraintSchemaInstantiable<TTypes = PrimitiveTypes, TValues = PrimitiveValues>>| -> proc_macro2::TokenStream {
        let instantiable_name = syn::Ident::new(&instantiable.get_tag().name, proc_macro2::Span::call_site());
        let mut rolling_trait_impl_list = instantiable.get_trait_impls().clone();
        if let Some(parent_constraint_object_id) = instantiable.get_constraint_object_id() {
            let parent_constraint_object = reference_constraint_schema.constraint_objects.get(parent_constraint_object_id).expect("Referenced constraint object must exist");
            rolling_trait_impl_list.extend(parent_constraint_object.get_trait_impls().clone());
        }
        if let Some(parent_library_operative_id) = instantiable.get_operative_library_id() {
            let parent_library_operative = reference_constraint_schema.operative_library.get(parent_library_operative_id).expect("Referenced library operative must exist");
            rolling_trait_impl_list.extend(parent_library_operative.get_trait_impls().clone());
        }

        let trait_streams = rolling_trait_impl_list.iter().map(|(trait_id, method_impls)| {
            let trait_def = reference_constraint_schema.traits.get(trait_id).expect("trait must exist");
            let trait_name = syn::Ident::new(&trait_def.tag.name, proc_macro2::Span::call_site());

            let method_impl_streams = method_impls.iter().map(|(trait_id, impl_path)| {
                let mut method_impl_stream = quote! {panic!("No terminating path for impl implementation")};

                let method_def = trait_def.methods.iter().find(|method| &method.tag.id == trait_id).expect("method must exist");
                let method_name = syn::Ident::new(&method_def.tag.name, proc_macro2::Span::call_site());
                let method_return_type = get_primitive_type(&method_def.return_type);

                let mut prepend_call: proc_macro2::TokenStream = quote!{};
                let mut last_item_id = instantiable.get_tag().id;
                let mut last_item_type = instantiable.get_constraint_schema_instantiable_type();

                impl_path.iter().enumerate().for_each(|(path_index, path_item)| {
                    match path_item {
                        serde_types::constraint_schema::TraitMethodImplPath::Field  (field_id) => {
                            match last_item_type {
                                serde_types::constraint_schema::ConstraintSchemaInstantiableType::ConstraintObject => {
                                    let constraint_object = reference_constraint_schema.constraint_objects.get(&last_item_id).expect("constraint object must exist");
                                    let field_constraint = constraint_object.field_constraints.iter().find(|constraint| &constraint.tag.id == field_id).expect("field must exist");
                                    let field_name = syn::Ident::new(&field_constraint.tag.name, proc_macro2::Span::call_site());
                                    method_impl_stream = quote!{ std::borrow::Cow::Borrowed(&self.#field_name) };
                                }
                                serde_types::constraint_schema::ConstraintSchemaInstantiableType::Instance => {
                                    let instance_ref = reference_constraint_schema.instance_library.get(&last_item_id).expect("instance must exist");
                                    let fulfilled_constraint = instance_ref.data.iter().find(|field| &field.tag.id == field_id).expect("field must exist");
                                    let value = get_primitive_value(&fulfilled_constraint.value);
                                    method_impl_stream = quote! { std::borrow::Cow::Owned(#value) };
                                }
                                serde_types::constraint_schema::ConstraintSchemaInstantiableType::Operative => {
                                    let operative_ref = reference_constraint_schema.operative_library.get(&last_item_id).expect("operative must exist");
                                    let fulfilled_constraint = operative_ref.locked_fields.iter().find(|field| &field.tag.id == field_id);
                                    if let Some(fulfilled_constraint) = fulfilled_constraint {
                                        let value = get_primitive_value(&fulfilled_constraint.value);
                                        method_impl_stream = quote! { std::borrow::Cow::Owned(#value) };
                                    } else {
                                        let constraint_object = reference_constraint_schema.constraint_objects.get(&operative_ref.constraint_object_id).expect("constraint object must exist");
                                        let field_constraint = constraint_object.field_constraints.iter().find(|constraint| &constraint.tag.id == field_id).expect("field must exist");
                                        let field_name = &field_constraint.tag.name;
                                        method_impl_stream = quote!{ std::borrow::Cow::Borrowed(&#prepend_call.#field_name) };
                                    }
                                }
                            }; 
                        }
                        serde_types::constraint_schema::TraitMethodImplPath::TraitOperativeConstituent{trait_operative_id, trait_id, trait_method_id} => {
                            let inner_trait_def = reference_constraint_schema.traits.get(trait_id).expect("trait must exist");
                            let inner_method_def = inner_trait_def.methods.iter().find(|method| &method.tag.id == trait_id).expect("method must exist");
                            let inner_method_name = syn::Ident::new(&inner_method_def.tag.name, proc_macro2::Span::call_site());

                            match last_item_type {
                                serde_types::constraint_schema::ConstraintSchemaInstantiableType::ConstraintObject => {
                                    let constraint_object = reference_constraint_schema.constraint_objects.get(&last_item_id).expect("constraint object must exist");
                                    let trait_operative = constraint_object.trait_operatives.iter().find(|trait_op| &trait_op.tag.id == trait_operative_id).expect("trait operative must exist");
                                    let trait_operative_id = trait_operative.tag.id;

                                    method_impl_stream = quote!{ #graph_environment.get_element(self.get_operative_by_id(#trait_operative_id)).#inner_method_name() };
                                }
                                serde_types::constraint_schema::ConstraintSchemaInstantiableType::Instance => {
                                    // Not sure how to do this yet -- the instances being
                                    // referenced are in the library and are not GSO's, but rather
                                    // Instantiable objects. So they don't have real trait impls at
                                    // this point.
                                    //
                                    // let instance_ref = reference_constraint_schema.instance_library.get(&last_item_id).expect("instance must exist");
                                    //
                                    // let trait_ref = reference_constraint_schema.traits.get(&trait_id).expect("trait must exist");
                                    // let trait_method_name = &trait_ref.methods.iter().find(|method| &method.tag.id == trait_method_id).expect("trait method must exist").tag.name;
                                    // let trait_method_name = syn::Ident::new(trait_method_name, proc_macro2::Span::call_site());
                                    //
                                    // method_impl_stream = quote!{ #graph_environment.get_element(self.get_operative_by_id(#trait_operative_id)).#trait_method_name() };
                                }
                                serde_types::constraint_schema::ConstraintSchemaInstantiableType::Operative => {
                                    // let operative_ref = reference_constraint_schema.operative_library.get(&last_item_id).expect("operative must exist");

                                    method_impl_stream = quote!{ #graph_environment.get_element(#prepend_call.get_operative_by_id(#trait_operative_id)).#inner_method_name() };
                                }
                            };

                        }
                        serde_types::constraint_schema::TraitMethodImplPath::InstanceConstituent(instance_constituent_id)=> {
                            last_item_id = *instance_constituent_id;
                            last_item_type =  serde_types::constraint_schema::ConstraintSchemaInstantiableType::Instance;
                        }
                        serde_types::constraint_schema::TraitMethodImplPath::LibraryOperativeConstituent(library_operative_constituent_id) => {
                            if path_index == 0 {
                                prepend_call = quote! {graph_environment.get_element(&self.get_operative_by_id(&library_operative_constituent_id).expect("constraint object must exist"))};
                            } else {
                                prepend_call = quote! {graph_environment.get_element(#prepend_call.get_operative_by_id(library_operative_constituent_id))};
                            }
                            last_item_id = *library_operative_constituent_id;
                            last_item_type =  serde_types::constraint_schema::ConstraintSchemaInstantiableType::Operative;
                        }
                        _ => {}
                    };

                });

                quote! {
                    fn #method_name(&self) -> std::borrow::Cow<#method_return_type> {
                        #method_impl_stream
                    }
                }
            });
            quote! {
                impl #trait_name for #instantiable_name {
                    #(#method_impl_streams)*
                }
            }
        });
        quote! {
            #(#trait_streams)*
        }
    };

    let constraint_objects_streams = constraint_schema_generated
        .constraint_objects
        .iter()
        .map(|(id, el)| {
            let name = el.tag.name.clone();
            let mut field_names = Vec::<syn::Ident>::new();
            let mut field_names_setters = Vec::<syn::Ident>::new();
            let mut field_values = Vec::<proc_macro2::TokenStream>::new();
            let mut initial_values = Vec::<proc_macro2::TokenStream>::new();
            let struct_name = syn::Ident::new(&name, proc_macro2::Span::call_site());
            element_names.push(struct_name.clone());

            let library_operatives: Vec<_> = el.library_operatives.iter().map(|op_id| {
                reference_constraint_schema.operative_library.get(op_id).expect("Operative Library should contain the operative ID referenced within a template's constituents")
            }).collect(); 
            let library_operative_names: Vec<_> = library_operatives.iter().map(|op| {
                syn::Ident::new(&op.tag.name, proc_macro2::Span::call_site())
            }).collect();
            let library_operative_setter_new: Vec<_> = library_operatives.iter().map(|op| {
                syn::Ident::new(&("set_new_".to_string() + &op.tag.name), proc_macro2::Span::call_site())
            }).collect();
            let library_operative_setter_existing: Vec<_> = library_operatives.iter().map(|op| {
                syn::Ident::new(&("set_".to_string() + &op.tag.name), proc_macro2::Span::call_site())
            }).collect();
            let library_operative_ids: Vec<_> = library_operatives.iter().map(|op| {
                op.tag.id
            }).collect();
            let simple_operatives = el.library_operatives.iter().map(|&val| quote! { #val }).collect::<Vec<_>>();

            let trait_operatives = &el.trait_operatives;
            let trait_operative_trait_names = trait_operatives.iter().map(|op| {
                let trait_id = op.trait_id;
                let trait_name = &reference_constraint_schema.traits.get(&trait_id).expect("trait must exist").tag.name;
                trait_name
            });
            let trait_operative_names: Vec<_> = trait_operatives.iter().map(|op| {
                syn::Ident::new(&op.tag.name, proc_macro2::Span::call_site())
            }).collect();
            let trait_operative_setter_new: Vec<_> = trait_operatives.iter().map(|op| {
                syn::Ident::new(&("set_new_".to_string() + &op.tag.name), proc_macro2::Span::call_site())
            }).collect();
            let trait_operative_setter_existing: Vec<_> = trait_operatives.iter().map(|op| {
                syn::Ident::new(&("set_".to_string() + &op.tag.name), proc_macro2::Span::call_site())
            }).collect();
            let trait_operative_ids: Vec<_> = trait_operatives.iter().map(|op| {
                op.tag.id
            }).collect();


            el .field_constraints .iter() .for_each(|field| {
                    let name = syn::Ident::new(&field.tag.name, proc_macro2::Span::call_site());
                    field_names.push(name.clone());
                    field_names_setters.push(syn::Ident::new(
                        &("set_".to_string() + &field.tag.name),
                        proc_macro2::Span::call_site(),
                    ));
                    let field_value = get_primitive_type(&field.value_type);
                    if let PrimitiveTypes::Option(inner) = &field.value_type {
                        initial_values.push(quote! {Some(None)});
                    } else {
                        initial_values.push(quote! {None});
                    }
                    field_values.push(field_value.clone());
                });

            let struct_builder_name = name + "Builder";
            let struct_builder_name =
                syn::Ident::new(&struct_builder_name, proc_macro2::Span::call_site());

            let item_trait_stream = generate_trait_impl_streams(Box::new(el));
            let str_rep = item_trait_stream.to_string();

            quote! {
                #[derive(Debug)]
                pub struct #struct_name {
                    #(#field_names: #field_values,)*
                    constraint_schema_id: output_types::Uid,
                    operatives: HashMap<Uid, Uid>,
                    id: output_types::Uid,
                }

                impl output_types::GSO for #struct_name {
                    type Builder = #struct_builder_name;

                    fn initiate_build() -> #struct_builder_name {
                        #struct_builder_name::new()
                    }
                    fn get_operative_by_id(&self, operative_id: &output_types::Uid) -> Option<output_types::Uid> {
                       self.operatives.get(operative_id).cloned() 
                    }
                    fn get_constraint_schema_id(&self) -> output_types::Uid {
                        self.constraint_schema_id
                    }
                }

                #item_trait_stream

                pub struct #struct_builder_name {
                    #(#library_operative_names: Option<output_types::Uid>,)*
                    #(#trait_operative_names: Option<output_types::Uid>,)*
                    #(#field_names: Option<#field_values>,)*
                }
                impl #struct_builder_name {
                    pub fn new() -> Self{
                        Self {
                            #(#library_operative_names: None,)*                
                            #(#trait_operative_names: None,)* 
                            #(#field_names: #initial_values,)*
                        }
                    }
                    fn check_completion(&self) -> bool {
                        let mut complete = true;
                        #(if self.#field_names.is_none() {complete = false;})*
                        #(if self.#library_operative_names.is_none() {complete = false;})*
                        #(if self.#trait_operative_names.is_none() {complete = false;})*
                        return complete
                    }

                    #(pub fn #field_names_setters(mut self, val: #field_values) -> Self {
                        self.#field_names = Some(val);
                        self
                    })*

                    // #(pub fn #library_operative_setter_new(mut self, val: #library_operative_names) -> Self {
                    //     self
                    // })*

                    #(pub fn #library_operative_setter_existing(mut self, val: output_types::Uid) -> Self {
                        self.#library_operative_names = Some(val);
                        self
                    })*

                    // #(pub fn #trait_operative_setter_new(mut self, val: impl #trait_operative_trait_names) -> Self {
                    //     self
                    // })*

                    #(pub fn #trait_operative_setter_existing(mut self, val: output_types::Uid) -> Self {
                        self.#trait_operative_names = Some(val);
                        self
                    })*

                    pub fn build(mut self) -> Result<#struct_name, bool> {
                        if self.check_completion() == true {
                            let mut operative_hashmap = HashMap::new();
                            #(operative_hashmap.insert(#library_operative_ids, self.#library_operative_names.unwrap());)*
                            #(operative_hashmap.insert(#trait_operative_ids, self.#trait_operative_names.unwrap());)*
                            Ok(#struct_name {
                                #(#field_names: self.#field_names.unwrap(),)*
                                id: uuid::Uuid::new_v4().as_u128(), 
                                constraint_schema_id: #id,
                                operatives: operative_hashmap,
                            })
                        } else {
                            Err(false)
                        }
                    }
                }
            }
        });

    quote! {
        use output_types::GSO;
        // Helper trait, private to your module
        trait IsGraphEnvironment {}

        // Implement IsMyTrait for all T that implement MyTrait
        impl<T> IsGraphEnvironment for T where T: output_types::GraphEnvironment {}
        let _check: &dyn IsGraphEnvironment = &#graph_environment;

        // const SCHEMA_JSON: &str = #data;
        #(#trait_definition_streams)*
        #(#constraint_objects_streams)*

        #[derive(Debug)]
        enum Schema {
            #(#element_names(#element_names) ,)*
        }
        impl output_types::GSO for Schema {
            type Builder = ();

            fn get_constraint_schema_id(&self) -> output_types::Uid {
                match &self {
                #(Self::#element_names(item) => item.get_constraint_schema_id(),)*
                _ => panic!(),
                }
            }
            fn get_operative_by_id(&self, operative_id: &output_types::Uid) -> Option<output_types::Uid> {
                match &self {
                #(Self::#element_names(item) => item.get_operative_by_id(operative_id),)*
                _ => panic!(),
                }
            }
            fn initiate_build() -> Self::Builder {
                ()
            }
            
        }
       // const constraint_schema: ConstraintSchema<PrimitiveTypes, PrimitiveValues> = #constraint_schema_generated;
        //  let constraint_schema: serde_types::constraint_schema::ConstraintSchema<serde_types::primitives::PrimitiveTypes, serde_types::primitives::PrimitiveValues> =
        // serde_json::from_str(SCHEMA_JSON).unwrap();
    }
    .into()
}

