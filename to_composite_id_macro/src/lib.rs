use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, LitInt};

#[proc_macro]
pub fn to_comp_id(input: TokenStream) -> TokenStream {
    let value = parse_macro_input!(input as LitInt)
        .base10_parse::<u32>()
        .unwrap();

    let a = (value >> 24) & 0xFF;
    let b = (value >> 16) & 0xFF;
    let c = (value >> 8) & 0xFF;
    let d = value & 0xFF;

    let a_type = quote::format_ident!("U{}", a, span = Span::call_site());
    let b_type = quote::format_ident!("U{}", b, span = Span::call_site());
    let c_type = quote::format_ident!("U{}", c, span = Span::call_site());
    let d_type = quote::format_ident!("U{}", d, span = Span::call_site());

    let expanded = quote! {
        molecule_core::CompId<typenum::#a_type, typenum::#b_type, typenum::#c_type, typenum::#d_type>
    };

    expanded.into()
}
