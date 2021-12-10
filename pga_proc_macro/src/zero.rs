use super::*;
use proc_macro2::{Ident, Span};

pub fn define() -> TokenStream {
    let zero = ident();

    quote! {
        #[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct #zero;
    }
}

pub fn ident() -> Ident {
    Ident::new("Zero", Span::call_site())
}
