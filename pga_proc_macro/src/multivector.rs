use crate::grades::GradeType;
use crate::*;

// TODO consider having types for even- and odd-graded elements (motors and flectors)

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
            pub s: S,
            pub v: V,
            pub b: B,
            pub t: T,
            pub a: A,
        }
    }
}
