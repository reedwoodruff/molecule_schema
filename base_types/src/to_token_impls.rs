use crate::constraint_schema::{OperativeSlot, OperativeVariants, SlotBounds, TraitOperative};

#[cfg(feature = "to_tokens")]
impl quote::ToTokens for OperativeSlot {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let tag = &self.tag;
        let bounds = &self.bounds;
        let operative_descriptor = self.operative_descriptor.clone();
        tokens.extend(quote::quote! {
            base_types::constraint_schema::OperativeSlot {
                tag: #tag,
                operative_descriptor: #operative_descriptor,
                bounds: #bounds,
            }
        })
    }
}

#[cfg(feature = "to_tokens")]
impl quote::ToTokens for SlotBounds {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ts = match *self {
            SlotBounds::Single => {
                quote::quote! {base_types::constraint_schema::SlotBounds::Single}
            }
            SlotBounds::LowerBound(lb) => {
                quote::quote! {base_types::constraint_schema::SlotBounds::LowerBound(#lb)}
            }
            SlotBounds::UpperBound(ub) => {
                quote::quote! {base_types::constraint_schema::SlotBounds::UpperBound(#ub)}
            }
            SlotBounds::Range(lb, ub) => {
                quote::quote! {base_types::constraint_schema::SlotBounds::Range(#lb,#ub)}
            }
            SlotBounds::LowerBoundOrZero(lb) => {
                quote::quote! {base_types::constraint_schema::SlotBounds::LowerBoundOrZero(#lb)}
            }
            SlotBounds::RangeOrZero(lb, ub) => {
                quote::quote! {base_types::constraint_schema::SlotBounds::RangeOrZero(#lb,#ub)}
            }
        };
        ts.to_tokens(tokens);
    }
}

#[cfg(feature = "to_tokens")]
impl quote::ToTokens for OperativeVariants {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ts = match self {
            OperativeVariants::TraitOperative(trait_op) => {
                quote::quote! {base_types::constraint_schema::OperativeVariants::TraitOperative(#trait_op)}
            }
            OperativeVariants::LibraryOperative(id) => {
                quote::quote! {base_types::constraint_schema::OperativeVariants::LibraryOperative(#id)}
            }
        };
        ts.to_tokens(tokens);
    }
}

#[cfg(feature = "to_tokens")]
impl quote::ToTokens for TraitOperative {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let trait_ids = &self.trait_ids;
        let tag = &self.tag;
        tokens.extend(quote::quote! {
            base_types::constraint_schema::TraitOperative {
                trait_ids: vec![#(#trait_ids,)*],
                tag: #tag,
            }
        })
    }
}
