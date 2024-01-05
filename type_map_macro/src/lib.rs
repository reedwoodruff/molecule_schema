// use proc_macro::TokenStream;
// use proc_macro2::TokenTree;
// use quote::quote;
// use syn::{
//     parse::{Parse, ParseStream},
//     parse_macro_input,
//     punctuated::Punctuated,
//     token::Comma,
//     Result as SynResult, Token, Type,
// };

// struct TypeList(Punctuated<Type, Token![,]>);

// impl Parse for TypeList {
//     fn parse(input: ParseStream) -> SynResult<Self> {
//         Ok(TypeList(input.parse_terminated(Type::parse, Token![,])?))
//     }
// }

// #[proc_macro]
// pub fn type_map(input: TokenStream) -> TokenStream {
//     // let tokens: proc_macro2::TokenStream = tokens.into();
//     // let mut tokens = tokens.into_iter();
//     let types = parse_macro_input!(input as TypeList);

//     let type_variants = types.0.iter().map(|ty| {
//         let type_name = quote! {#ty }.to_string().replace(" ", "");
//         let variant_name = syn::Ident::new(&type_name, proc_macro2::Span::call_site());
//         quote! {#variant_name}
//     });
//     let types_enum = quote! {
//         enum Types {
//             #(#type_variants),*
//         }
//     };

//     let value_variants = types.0.iter().map(|ty| {
//         let type_name = quote! { #ty }.to_string().replace(" ", "");
//         let variant_name = syn::Ident::new(&type_name, proc_macro2::Span::call_site());
//         quote! { #variant_name(#ty) }
//     });

//     let values_enum = quote! {
//         enum Values {
//             #(#value_variants),*
//         }
//     };

//     // let match_arms = types.0.iter().map(|ty| {
//     //     let type_name = quote! {#ty}.to_string().replace(" ", "");
//     //     let variant_name = syn::Ident::new(&type_name, proc_macro2::Span::call_site());
//     //     quote! {
//     //         if let Some(val) = any.downcast_ref::<#ty>() {
//     //             return Values::#variant_name(val.clone());
//     //         }
//     //     }
//     // });
//     // let lookup_fn = quote! {
//     //     fn lookup(any: &dyn Any) -> Values {
//     //         #(#match_arms)*
//     //         panic!("No matching type found");
//     //     }
//     // };
//     // let match_arms = types.0.iter().map(|ty| {
//     //     let type_name = quote! {#ty}.to_string().replace(" ", "");
//     //     let variant_name = syn::Ident::new(&type_name, proc_macro2::Span::call_site());
//     //     quote! {
//     //         Types::#variant_name => Values::#variant_name(val.clone()),
//     //     }
//     // });
//     // let lookup_fn = quote! {
//     //     fn lookup(type_enum: Types) -> Values {
//     //         match type_enum {
//     //             #(#match_arms)*
//     //             _ => panic!("No matching type found"),
//     //         }
//     //     }
//     // };

//     let output = quote! {
//         #types_enum

//         #values_enum

//         // #lookup_fn
//     };

//     output.into()
// }

// use proc_macro::TokenStream;
// use proc_macro2::TokenTree;
// use quote::quote;
// use syn::{
//     parse::{Parse, ParseStream},
//     parse_macro_input,
//     punctuated::Punctuated,
//     token::Comma,
//     Result as SynResult, Token, Type,
// };

// struct TypeList(Punctuated<Type, Token![,]>);

// impl Parse for TypeList {
//     fn parse(input: ParseStream) -> SynResult<Self> {
//         Ok(TypeList(input.parse_terminated(Type::parse, Token![,])?))
//     }
// }

// #[proc_macro]
// pub fn type_map(input: TokenStream) -> TokenStream {
//     // let tokens: proc_macro2::TokenStream = tokens.into();
//     // let mut tokens = tokens.into_iter();
//     let types = parse_macro_input!(input as TypeList);

//     let values = types.0.iter().map(|ty| {
//         let type_name = quote! {#ty }.to_string().replace(" ", "");
//         let value_name = syn::Ident::new(
//             &format!("{}Value", type_name),
//             proc_macro2::Span::call_site(),
//         );
//         (value_name, type_name)
//     });
//     let value_structs = values.clone().map(|(value_name, type_name)| {
//         quote! {
//             struct #value_name {}
//             impl TypedValue for #value_name {
//                 type ValueType = #type_name;
//             }
//         }
//     });
//     let value_structs = quote! {
//         #(#value_structs)*
//     };

//     let type_variants = types.0.iter().map(|ty| {
//         let type_name = quote! {#ty }.to_string().replace(" ", "");
//         let variant_name = syn::Ident::new(&type_name, proc_macro2::Span::call_site());
//         let value_name = syn::Ident::new(
//             &format!("{}Value", type_name),
//             proc_macro2::Span::call_site(),
//         );
//         quote! {#variant_name(#value_name)}
//     });
//     let types_enum = quote! {
//         enum Types {
//             #(#type_variants),*
//         }
//     };

//     let field_constraint = quote! {
//         struct FieldConstraint<T> {
//                 id: Uid,
//                 name: String,
//                 value_type: Types,
//                 locked_value: Option<bool>,
//         }
//     };

//     let field_constraint_impls = values.clone().map(
//         (|(value_name, type_name)| {
//             quote! {
//                 impl FieldConstraint<#value_name: TypedValue> {
//                     fn new(name: Types::#type_name) -> Self {
//                         Self {
//                             id: 0,
//                             name,
//                             value_type: #value_name::ValueType,
//                             locked_value: None,
//                         }
//                     }
//                     fn fulfill(&self, val: #type_name) -> FulfilledFieldConstraint<#value_name> {
//                         // TODO add error if already locked
//                         FulfilledFieldConstraint::<TTypes, TValues> {
//                             constraint_id: self.id,
//                             name: self.name.clone(),
//                             value_type: self.value_type.clone(),
//                             value: val,
//                         }
//                     }
//                 }
//             }
//         }),
//     );

//     // let field_constraint_methods = types.0.iter().map(|ty| {});
//     let output = quote! {
//         trait TypedValue {
//             type ValueType;
//         }
//         #[derive(Clone)]
//         struct FulfilledFieldConstraint<T: TypedValue> {
//             pub constraint_id: Uid,
//             pub name: String,
//             pub value_type: TTypes,
//             pub value: T::ValueType,
//         }
//         #value_structs
//         #types_enum
//     };

//     output.into()
// }

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(GenerateEnumFromSchema)]
pub fn generate_enum_from_schema(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Logic to iterate over ConstraintSchema and generate code
    // let enum_variants = input.constraint_objects.iter().map(|object| {
    // Generate enum variant and corresponding struct
    // });

    // Generate the final enum and structs
    let expanded = quote! {
        // enum GeneratedEnum {
        //     #(#enum_variants),*
        // }
    };

    TokenStream::from(expanded)
}
