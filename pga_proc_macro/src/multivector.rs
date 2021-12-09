use crate::grades::GradeType;
use crate::*;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Multivector {
    s: Option<()>,
    v: Option<GradeType>,
    b: Option<GradeType>,
    t: Option<GradeType>,
    a: Option<()>,
}

pub fn define() -> TokenStream {
    quote! {
        #[derive(Debug, Default, Copy, Clone, PartialEq)]
        pub struct Multivector<S, V, B, T, A> {
            s: S,
            v: V,
            b: B,
            t: T,
            a: A,
        }
    }
}
