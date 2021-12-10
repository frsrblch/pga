use crate::blades::{E0123, S};
use crate::grades::GradeType;
use crate::*;
use proc_macro2::{Ident, Span};

// TODO consider having types for even- and odd-graded elements (motors and flectors)

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

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Multivector {
    pub s: Option<()>,
    pub v: Option<GradeType>,
    pub b: Option<GradeType>,
    pub t: Option<GradeType>,
    pub a: Option<()>,
}

// TODO Multivector needs to return a
impl ToTokens for Multivector {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.token_stream());
    }
}

impl Multivector {
    pub fn count_some(&self) -> usize {
        self.s.is_some() as usize
            + self.v.is_some() as usize
            + self.b.is_some() as usize
            + self.t.is_some() as usize
            + self.a.is_some() as usize
    }

    fn token_stream(&self) -> proc_macro2::TokenStream {
        let mut stream = proc_macro2::TokenStream::new();

        match self.count_some() {
            0 => stream.append(zero::ident()),
            1 => todo!("single grade"),
            _ => {
                stream.append(proc_macro2::Ident::new("Multivector", Span::call_site()));
                todo!("generic types");
            }
        }

        stream
    }
}
