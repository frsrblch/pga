use super::*;

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

impl ToTokens for Sign {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if let Sign::Neg = *self {
            tokens.append_all(quote! {-});
        }
    }
}
