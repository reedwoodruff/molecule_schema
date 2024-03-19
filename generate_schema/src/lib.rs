use proc_macro::TokenStream;

use quote::quote;

use serde_types::constraint_schema::*;
use serde_types::constraint_schema_item::ConstraintSchemaInstantiable;
use serde_types::primitives::*;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated, Result as SynResult, Token, Type,
};


// mod generated_expand_output;
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

fn concat_unique_element(element: &Box<&dyn ConstraintSchemaInstantiable<TTypes = PrimitiveTypes, TValues = PrimitiveValues>>) -> String {
    element.get_tag().name.clone() + "_" + element.get_constraint_schema_instantiable_type().as_ref()
}
fn get_variant_name(element: &Box<&dyn ConstraintSchemaInstantiable<TTypes = PrimitiveTypes, TValues = PrimitiveValues>> ) -> syn::Ident {
    syn::Ident::new(&concat_unique_element(element), proc_macro2::Span::call_site())
}
fn get_variant_builder_name(element: &Box<&dyn ConstraintSchemaInstantiable<TTypes = PrimitiveTypes, TValues = PrimitiveValues>> ) -> syn::Ident {
    syn::Ident::new(&(concat_unique_element(element) + "Builder"), proc_macro2::Span::call_site())
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
                fn #method_name(&self, env: &dyn output_types::GraphEnvironment::<Schema = Schema>) -> std::borrow::Cow<#return_type>;
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
        let instantiable_name = get_variant_name(&instantiable);
        // let mut rolling_trait_impl_list = instantiable.get_trait_impls().clone();
        // let parent_template = reference_constraint_schema.template_library.get(instantiable.get_template_id()).expect("Referenced constraint object must exist");
        // rolling_trait_impl_list.extend(parent_template.get_trait_impls().clone());
        // if let Some(parent_library_operative_id) = instantiable.get_operative_library_id() {
        //     let parent_library_operative = reference_constraint_schema.operative_library.get(parent_library_operative_id).expect("Referenced library operative must exist");
        //     rolling_trait_impl_list.extend(parent_library_operative.get_trait_impls().clone());
        // }
        let mut rolling_trait_impl_list = instantiable.get_local_trait_impls().clone();
        rolling_trait_impl_list.extend(instantiable.get_ancestors_trait_impls(&reference_constraint_schema));

        let trait_streams = rolling_trait_impl_list.iter().map(|(trait_id, method_impls)| {
            let trait_def = reference_constraint_schema.traits.get(trait_id).expect("trait must exist");
            let trait_name = syn::Ident::new(&trait_def.tag.name, proc_macro2::Span::call_site());

            let method_impl_streams = method_impls.iter().map(|(method_id, impl_path)| {
                let mut method_impl_stream = quote! {panic!("No terminating path for impl implementation")};


                println!("on trait: {}", trait_id );
                println!("expected method: {}", method_id );
                println!("trait_def: {:?}", trait_def);
                let method_def = trait_def.methods.iter().find(|method| &method.tag.id == method_id).expect("method must exist");
                let method_name = syn::Ident::new(&method_def.tag.name, proc_macro2::Span::call_site());
                let method_return_type = get_primitive_type(&method_def.return_type);

                let mut prepend_call: proc_macro2::TokenStream = quote!{};
                let mut last_item_id = instantiable.get_tag().id;
                let mut last_item_type = instantiable.get_constraint_schema_instantiable_type();

                for (path_index, path_item) in impl_path.iter().enumerate() {
                    match path_item {
                        serde_types::constraint_schema::TraitMethodImplPath::Field  (field_id) => {
                            match last_item_type {
                                serde_types::constraint_schema::ConstraintSchemaInstantiableType::Template => {
                                    let template = reference_constraint_schema.template_library.get(&last_item_id).expect("constraint object must exist");
                                    println!("expected: {}", field_id);
                                    let field_constraint = template.field_constraints.iter().find(|constraint| &constraint.tag.id == field_id).expect("field must exist");
                                    let field_name = syn::Ident::new(&field_constraint.tag.name, proc_macro2::Span::call_site());
                                    method_impl_stream = quote!{ std::borrow::Cow::Borrowed(&self.#field_name) };
                                }
                                serde_types::constraint_schema::ConstraintSchemaInstantiableType::Instance => {
                                    let instance_ref = reference_constraint_schema.instance_library.get(&last_item_id).expect("instance must exist");
                                    println!("expected: {}", field_id);
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
                                        let variant_name = get_variant_name(&Box::new(operative_ref));
                                        let variant_name_string = variant_name.to_string();
                                        let _prepend_call_string = prepend_call.to_string();
                                        let template = reference_constraint_schema.template_library.get(&operative_ref.template_id).expect("constraint object must exist");
                                        let field_constraint = template.field_constraints.iter().find(|constraint| &constraint.tag.id == field_id).expect("field must exist");
                                        let field_name = syn::Ident::new(&field_constraint.tag.name, proc_macro2::Span::call_site());
                                        method_impl_stream = quote!{ std::borrow::Cow::Owned(match &#prepend_call {
                                            // Schema::Person(person) => person.#field_name.clone(),
                                            Schema::#variant_name(item) => item.#field_name.clone(),
                                            _ => {
                                                println!("prepend_call: {:?}", #prepend_call);
                                                println!("attempted variant_name: {:?}", #variant_name_string);
                                                panic!("no variant name");

                                            }                                            
                                            }) };
                                    }
                                }
                            }; 
                        }
                        serde_types::constraint_schema::TraitMethodImplPath::TraitMethod { trait_id, trait_method_id } => {
                            let inner_trait_def = reference_constraint_schema.traits.get(&trait_id).expect("trait must exist");
                            println!("running here!");
                            let inner_method_def = inner_trait_def.methods.iter().find(|method| &method.tag.id == trait_method_id).expect("method must exist");
                            let inner_method_name = syn::Ident::new(&inner_method_def.tag.name, proc_macro2::Span::call_site());

                            match last_item_type {
                                serde_types::constraint_schema::ConstraintSchemaInstantiableType::Template => {
                                    let _template = reference_constraint_schema.template_library.get(&last_item_id).expect("constraint object must exist");

                                    method_impl_stream = quote!{ self.#inner_method_name(graph_environment) };
                                }
                                serde_types::constraint_schema::ConstraintSchemaInstantiableType::Instance => {
                                    // Not sure how to do this yet -- the instances being
                                    // referenced are in the library and are not GSO's, but rather
                                    // Instantiable objects. So they don't have real trait impls at
                                    // this point.
                                }
                                serde_types::constraint_schema::ConstraintSchemaInstantiableType::Operative => {
                                    // let operative_ref = reference_constraint_schema.operative_library.get(&last_item_id).expect("operative must exist");

                                    method_impl_stream = quote!{ graph_environment.get_element(#prepend_call).expect("element must exist").#inner_method_name() };
                                }
                            }
                        }
                        serde_types::constraint_schema::TraitMethodImplPath::TraitOperativeConstituent{trait_operative_id, trait_id, trait_method_id} => {
                            let inner_trait_def = reference_constraint_schema.traits.get(trait_id).expect("trait must exist");
                            println!("running here too");
                            let inner_method_def = inner_trait_def.methods.iter().find(|method| &method.tag.id == trait_method_id).expect("method must exist");
                            let inner_method_name = syn::Ident::new(&inner_method_def.tag.name, proc_macro2::Span::call_site());
                            let variants_which_impl_trait = reference_constraint_schema.template_library.iter().filter(
                                |(_co_id, co)| {
                                    co.trait_impls.contains_key(trait_id)
                                }
                            ).map(|(_co_id, co)| {
                                    get_variant_name(&Box::new(co))
                            }).collect::<Vec<_>>();

                            println!("variants which impl trait: {:?}", variants_which_impl_trait);
                            match last_item_type {
                                serde_types::constraint_schema::ConstraintSchemaInstantiableType::Template => {
                                    let template = reference_constraint_schema.template_library.get(&last_item_id).expect("constraint object must exist");
                                    let trait_operative = template.trait_operatives.iter().find(|trait_op| &trait_op.tag.id == trait_operative_id).expect("trait operative must exist");
                                    let trait_operative_id = trait_operative.tag.id;

                                    method_impl_stream = quote!{ match graph_environment.get_element(&self.get_operative_by_id(&#trait_operative_id).expect("operative must exist.")).cloned().expect("element must exist") {
                                        #(Schema::#variants_which_impl_trait(variant) => {
                                        let val = variant.#inner_method_name(graph_environment);
                                        let val = val.into_owned();
                                        std::borrow::Cow::Owned(val)
                                        },)*
                                        _ => panic!(),
                                        } };
                                    break;
                                }
                                serde_types::constraint_schema::ConstraintSchemaInstantiableType::Instance => {
                                    break;
                                    // TODO
                                }
                                serde_types::constraint_schema::ConstraintSchemaInstantiableType::Operative => {

                                    method_impl_stream = quote!{ match graph_environment.get_element(
                                            &#prepend_call.get_operative_by_id(&#trait_operative_id).expect("operative must exist")
                                        ).cloned().expect("element must exist") {
                                        #(Schema::#variants_which_impl_trait(variant) => {
                                        let val = variant.#inner_method_name(graph_environment);
                                        let val = val.into_owned();
                                        std::borrow::Cow::Owned(val)
                                        },)*
                                            _ => {panic!()}
                                        }};
                                    break;
                                }
                            };
                        }

                        serde_types::constraint_schema::TraitMethodImplPath::InstanceConstituent(instance_constituent_id)=> {
                            last_item_id = *instance_constituent_id;
                            last_item_type =  serde_types::constraint_schema::ConstraintSchemaInstantiableType::Instance;
                        }
                        serde_types::constraint_schema::TraitMethodImplPath::LibraryOperativeConstituent(library_operative_constituent_id) => {
                            if path_index == 0 {
                                prepend_call = quote! {graph_environment.get_element(&self.get_operative_by_id(&#library_operative_constituent_id).expect("constraint object must exist")).expect("element must exist")};
                            } else {
                                prepend_call = quote! {graph_environment.get_element(#prepend_call.get_operative_by_id(&#library_operative_constituent_id)).expect("element must exist")};
                            }
                            last_item_id = *library_operative_constituent_id;
                            last_item_type =  serde_types::constraint_schema::ConstraintSchemaInstantiableType::Operative;
                        }
                        _ => {}
                    };

                };

                quote! {
                    fn #method_name(&self, graph_environment: &dyn output_types::GraphEnvironment::<Schema = Schema>) -> std::borrow::Cow<#method_return_type> {
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

    let mut generate_objects_streams = |instantiable: Box::<&dyn ConstraintSchemaInstantiable<TTypes = PrimitiveTypes, TValues = PrimitiveValues>>| -> proc_macro2::TokenStream {
        let mut field_names = Vec::<syn::Ident>::new();
        let mut field_names_setters = Vec::<syn::Ident>::new();
        let mut field_values = Vec::<proc_macro2::TokenStream>::new();
        let mut initial_values = Vec::<proc_macro2::TokenStream>::new();
        let struct_name = get_variant_name(&instantiable);
        element_names.push(struct_name.clone());

        let reference_template_id = instantiable.get_template_id();
        let (constraint_schema_tag_name, constraint_schema_tag_id) = (instantiable.get_tag().name.clone(), instantiable.get_tag().id);
        let constraint_schema_instantiable_type = instantiable.get_constraint_schema_instantiable_type().as_ref().to_owned();
        // let constraint_schema_tag = match instantiable.get_constraint_schema_instantiable_type() {
        //     serde_types::constraint_schema::ConstraintSchemaInstantiableType::Template => {
        //         output_types::ConstraintSchemaTag::Template(instantiable.get_tag().clone().into())
        //     }
        //     serde_types::constraint_schema::ConstraintSchemaInstantiableType::Instance => {
        //         output_types::ConstraintSchemaTag::Instance(instantiable.get_tag().clone().into())
        //     }
        //     serde_types::constraint_schema::ConstraintSchemaInstantiableType::Operative => {
        //         output_types::ConstraintSchemaTag::Operative(instantiable.get_tag().clone().into())
        //     }
        // };
        let reference_template = reference_constraint_schema.clone().template_library.get(reference_template_id).cloned().expect("instantiable must be based on a constraint object");

        // let mut fulfilled_ops_as_instance_ids = Vec::new();
        // let mut fulfilled_library_ops = Vec::new();
        // let mut fulfilled_trait_ops = Vec::new();
        // if let Some(ids) = reference_template.get_fulfilled_library_operatives() {
        //     ids.iter().for_each(|item| {
        //         fulfilled_ops_as_instance_ids.push(item.fulfilling_instance_id);
        //         match item.operative_id {
        //             OperativeVariants::LibraryOperative(lib_op_id) => {
        //                 fulfilled_library_ops.push(lib_op_id);
        //             }
        //             OperativeVariants::TraitOperative(trait_op_id) => {
        //                 fulfilled_trait_ops.push(trait_op_id);
        //             }
        //         }
        //     })
        // }

        // let library_operatives: Vec<_> = reference_template.library_operatives.iter().filter(|op_id| !fulfilled_library_ops.contains(op_id)).map(|op_id| {
        //     reference_constraint_schema.operative_library.get(op_id).expect("Operative Library should contain the operative ID referenced within a template's constituents")
        // }).collect(); 
        let library_operatives = instantiable.get_all_unfulfilled_library_operatives(&reference_constraint_schema);
        let library_operative_names: Vec<_> = library_operatives.iter().map(|op| {
            syn::Ident::new(&op.tag.name, proc_macro2::Span::call_site())
        }).collect();
        let _library_operative_setter_new: Vec<_> = library_operatives.iter().map(|op| {
            syn::Ident::new(&("set_new_".to_string() + &op.tag.name), proc_macro2::Span::call_site())
        }).collect();
        let library_operative_setter_existing: Vec<_> = library_operatives.iter().map(|op| {
            syn::Ident::new(&("set_".to_string() + &op.tag.name), proc_macro2::Span::call_site())
        }).collect();
        let library_operative_ids: Vec<_> = library_operatives.iter().map(|op| {
            op.tag.id
        }).collect();
        let _simple_operatives = reference_template.library_operatives.iter().map(|&val| quote! { #val }).collect::<Vec<_>>();

        // let trait_operatives = &reference_template.trait_operatives.iter().filter(|trait_op| !fulfilled_trait_ops.contains(&trait_op.tag.id)).collect::<Vec<_>>();
        let trait_operatives = instantiable.get_all_unfulfilled_trait_operatives(&reference_constraint_schema);
        let _trait_operative_trait_names = trait_operatives.iter().map(|op| {
            let trait_id = op.trait_id;
            let trait_name = &reference_constraint_schema.traits.get(&trait_id).expect("trait must exist").tag.name;
            trait_name
        });
        let trait_operative_names: Vec<_> = trait_operatives.iter().map(|op| {
            syn::Ident::new(&op.tag.name, proc_macro2::Span::call_site())
        }).collect();
        let _trait_operative_setter_new: Vec<_> = trait_operatives.iter().map(|op| {
            syn::Ident::new(&("set_new_".to_string() + &op.tag.name), proc_macro2::Span::call_site())
        }).collect();
        let trait_operative_setter_existing: Vec<_> = trait_operatives.iter().map(|op| {
            syn::Ident::new(&("set_".to_string() + &op.tag.name), proc_macro2::Span::call_site())
        }).collect();
        let trait_operative_ids: Vec<_> = trait_operatives.iter().map(|op| {
            op.tag.id
        }).collect();

        // let mut all_instance_ids = constraint_schema_generated.template_library.get(&reference_template.get_template_id()).unwrap().instances.clone();
        // all_instance_ids.extend(fulfilled_ops_as_instance_ids);
        let all_instances = instantiable.get_all_constituent_instances(&constraint_schema_generated);
        let _all_instance_names = all_instances.iter().map(|item| {
            syn::Ident::new(&item.tag.name.clone(), proc_macro2::Span::call_site())
        });

        // let fulfilled_field_constraints = instantiable.get_fulfilled_fields().cloned().unwrap_or(Vec::<FulfilledFieldConstraint<PrimitiveTypes,PrimitiveValues>>::new());
        // let fulfilled_field_constraints = instantiable.get_local_fulfilled_fields().unwrap_or(&Vec::<FulfilledFieldConstraint<PrimitiveTypes,PrimitiveValues>>::new());
        // let fulfilled_field_constraint_ids = fulfilled_field_constraints.iter().map(|field| field.tag.id).collect::<Vec<_>>();
        // reference_template.field_constraints.iter().filter(|field| !fulfilled_field_constraint_ids.contains(&field.tag.id)).for_each(|field| {
        println!("unfulfilled_fields: {:?}", instantiable.get_all_unfulfilled_fields(&reference_constraint_schema));
        instantiable.get_all_unfulfilled_fields(&reference_constraint_schema).iter().for_each(|field| {
                let name = syn::Ident::new(&field.tag.name, proc_macro2::Span::call_site());
                field_names.push(name.clone());
                field_names_setters.push(syn::Ident::new(
                    &("set_".to_string() + &field.tag.name),
                    proc_macro2::Span::call_site(),
                ));
                let field_value = get_primitive_type(&field.value_type);
                if let PrimitiveTypes::Option(_inner) = &field.value_type {
                    initial_values.push(quote! {Some(None)});
                } else {
                    initial_values.push(quote! {None});
                }
                field_values.push(field_value.clone());
            });

        let struct_builder_name = get_variant_builder_name(&instantiable);
        let item_trait_stream = generate_trait_impl_streams(instantiable);
        // let str_rep = item_trait_stream.to_string();

        quote! {
            #[derive(Debug, Clone)]
            pub struct #struct_name {
                #(#field_names: #field_values,)*
                template_id: output_types::Uid,
                constraint_schema_tag: output_types::ConstraintSchemaTag,
                id: output_types::Uid,
                operatives: HashMap<Uid, Uid>,
                // instances: HashMap<Uid, Uid>,
            }

            impl output_types::GSO for #struct_name {
                type Builder = #struct_builder_name;

                fn initiate_build() -> #struct_builder_name {
                    #struct_builder_name::new()
                }
                fn get_operative_by_id(&self, operative_id: &output_types::Uid) -> Option<output_types::Uid> {
                    self.operatives.get(operative_id).cloned() 
                }
                fn get_template_id(&self) -> output_types::Uid {
                    self.template_id
                }
                fn get_id(&self) -> Uid {
                    self.id
                }
                fn get_constraint_schema_tag(&self) -> &output_types::ConstraintSchemaTag {
                    &self.constraint_schema_tag
                }
            }

            #item_trait_stream

            pub struct #struct_builder_name {
                #(#library_operative_names: Option<output_types::Uid>,)*
                #(#trait_operative_names: Option<output_types::Uid>,)*
                #(#field_names: Option<#field_values>,)*
            }
            impl #struct_builder_name {
                pub fn build(mut self) -> Result<Schema, bool> {
                    if self.check_completion() == true {
                        let mut operative_hashmap = HashMap::new();
                        #(operative_hashmap.insert(#library_operative_ids, self.#library_operative_names.unwrap());)*
                        #(operative_hashmap.insert(#trait_operative_ids, self.#trait_operative_names.unwrap());)*

                        // let mut instances_hashmap = HashMap::new();
                        // #(instances_hashmap.insert(#all_instance_ids, #all_instance_names);)*

                        Ok(Schema::#struct_name(#struct_name {
                            #(#field_names: self.#field_names.unwrap(),)*
                            id: uuid::Uuid::new_v4().as_u128(), 
                            constraint_schema_tag: match #constraint_schema_instantiable_type {
                                // let tag = output_types::Tag {id: #constraint_schema_tag_id, name: #constraint_schema_tag_name.to_string()};
                                "Template" => 
                                output_types::ConstraintSchemaTag::Template(output_types::Tag {id: #constraint_schema_tag_id, name: #constraint_schema_tag_name.to_string()}),
                                "Operative" => 
                                output_types::ConstraintSchemaTag::Operative(output_types::Tag {id: #constraint_schema_tag_id, name: #constraint_schema_tag_name.to_string()}),
                                "Instance" => 
                                output_types::ConstraintSchemaTag::Instance(output_types::Tag {id: #constraint_schema_tag_id, name: #constraint_schema_tag_name.to_string()}),
                                _ => panic!(),
                            },
                            template_id: #reference_template_id,
                            operatives: operative_hashmap,
                            // instances: instances_hashmap,
                        }))
                    } else {
                        Err(false)
                    }
                }

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

            }
        }
    };

    let template_streams = constraint_schema_generated.template_library.values().map(|el| {
        generate_objects_streams(Box::new(el))
    }).collect::<Vec<_>>();
    let operative_streams = constraint_schema_generated.operative_library.values().map(|el| {
        generate_objects_streams(Box::new(el))
    }).collect::<Vec<_>>();
    let instance_streams = constraint_schema_generated.instance_library.values().map(|el| {
        generate_objects_streams(Box::new(el))
    })
    .collect::<Vec<_>>();

    quote! {
        use output_types::GSO;
        // Helper trait, private to your module
        trait IsGraphEnvironment {}

        // Implement IsMyTrait for all T that implement MyTrait
        impl<T> IsGraphEnvironment for T where T: output_types::GraphEnvironment {}
        // let graph_environment = #graph_environment;
        let _check: &dyn IsGraphEnvironment = &#graph_environment;

        // const SCHEMA_JSON: &str = #data;
        #(#trait_definition_streams)*
        #(#template_streams)*
        #(#operative_streams)*
        #(#instance_streams)*

        #[derive(Debug, Clone)]
        enum Schema {
            #(#element_names(#element_names) ,)*
        }
        impl output_types::GSO for Schema {
            type Builder = ();

            fn get_constraint_schema_tag(&self) -> &output_types::ConstraintSchemaTag {
                match &self {
                #(Self::#element_names(item) => item.get_constraint_schema_tag(),)*
                _ => panic!(),
                }
            }
            fn get_id(&self) -> output_types::Uid {
                match self {
                    #(Self::#element_names(item) => item.get_id(),)*
                    _ => panic!(),
                }
            }
            fn get_template_id(&self) -> output_types::Uid {
                match self {
                    #(Self::#element_names(item) => item.get_id(),)*
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

