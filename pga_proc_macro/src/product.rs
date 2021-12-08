use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Product<T> {
    Zero,
    Value(T, Sign),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Sign {
    Pos,
    Neg,
}

impl Sign {
    pub fn tokens(&self) -> TokenStream {
        match self {
            Self::Neg => quote! { - },
            Self::Pos => quote! {},
        }
    }
}
