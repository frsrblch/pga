use crate::blades::{E0123, S};
use crate::geo::GeoProd;
use crate::grades::GradeType;
use crate::*;
use std::ops::Mul;

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

impl ToTokens for Multivector {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        use proc_macro2::{Ident, Span, TokenStream};

        match self.count_some() {
            0 | 1 => panic!("use Grade/Blade ident"),
            _ => {
                let mv = Ident::new("Multivector", Span::call_site());

                let s = if self.s.is_some() {
                    Ident::new("f64", Span::call_site())
                } else {
                    zero::ident()
                };

                let v = if let Some(ty) = self.v {
                    Grade { k: 1, ty }.ident()
                } else {
                    zero::ident()
                };

                let b = if let Some(ty) = self.b {
                    Grade { k: 2, ty }.ident()
                } else {
                    zero::ident()
                };

                let t = if let Some(ty) = self.t {
                    Grade { k: 3, ty }.ident()
                } else {
                    zero::ident()
                };

                let a = if self.a.is_some() {
                    E0123.ident()
                } else {
                    zero::ident()
                };

                tokens.append_all(quote! {
                    #mv < #s, #v, #b, #t, #a >
                });
            }
        }
    }
}

impl std::iter::Sum<Product<Blade>> for Multivector {
    fn sum<I: Iterator<Item = Product<Blade>>>(iter: I) -> Self {
        iter.fold(Multivector::default(), |mv, b| mv + b)
    }
}

impl std::ops::Add<Product<Blade>> for Multivector {
    type Output = Self;
    fn add(mut self, rhs: Product<Blade>) -> Self {
        if let Product::Value(rhs, _) = rhs {
            match rhs.grade() {
                0 => self.s = Some(()),
                4 => self.a = Some(()),
                1 => self.v += rhs.grade_type(),
                2 => self.b += rhs.grade_type(),
                3 => self.t += rhs.grade_type(),
                _ => unreachable!(),
            }
        }

        self
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

    pub fn geo_prod(&self) -> GeoProd {
        match self.count_some() {
            0 => GeoProd::Zero,
            1 => {
                if self.s.is_some() {
                    GeoProd::Blade(S)
                } else if self.a.is_some() {
                    GeoProd::Blade(E0123)
                } else if let Some(ty) = self.v {
                    GeoProd::Grade(Grade { k: 1, ty })
                } else if let Some(ty) = self.b {
                    GeoProd::Grade(Grade { k: 2, ty })
                } else if let Some(ty) = self.t {
                    GeoProd::Grade(Grade { k: 3, ty })
                } else {
                    unreachable!()
                }
            }
            _ => GeoProd::Multi(*self),
        }
    }
}
