use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::Comma,
    Result as SynResult, Token, Type,
};
struct TypeList(Punctuated<Type, Token![,]>);

impl Parse for TypeList {
    fn parse(input: ParseStream) -> SynResult<Self> {
        Ok(TypeList(input.parse_terminated(Type::parse, Token![,])?))
    }
}

enum TestNodeTypes {
    TestNodeA,
    TestNodeB,
}

#[proc_macro]
pub fn generate_schema(input: TokenStream) -> TokenStream {
    // let constraint_schema = input as molecule_schema::constraint_schema::ConstraintSchema<>;
    let schema_enum = quote! {
        enum Schema {
           Nodes(TestNodeTypes),
        }
    };

    // let values_enum = quote! {
    //     enum Values {
    //         #(#value_variants),*
    //     }
    // };

    let output = quote! {
        #schema_enum
    };

    output.into()
}
