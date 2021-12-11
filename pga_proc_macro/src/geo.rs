use super::*;
use crate::grades::GradeType;
use crate::multivector::Multivector;
use std::ops::Mul;

pub fn define() -> TokenStream {
    let f64 = define_f64();
    let blades = Blade::iter().map(Blade::geo);
    let grades = Grade::iter().map(Grade::define_geo);

    quote! {
        pub trait Geometric<Rhs> {
            type Output;
            fn geo(self, rhs: Rhs) -> Self::Output;
        }

        pub trait Antigeometric<Rhs> {
            type Output;
            fn antigeo(self, rhs: Rhs) -> Self::Output;
        }

        impl<T, TComp, Rhs, RhsComp, OutputComp> Antigeometric<Rhs> for T
        where
            T: LeftComp<Output = TComp>,
            Rhs: LeftComp<Output = RhsComp>,
            TComp: Geometric<RhsComp, Output = OutputComp>,
            OutputComp: RightComp,
        {
            type Output = <OutputComp as RightComp>::Output;

            fn antigeo(self, rhs: Rhs) -> Self::Output {
                self.l_comp().geo(rhs.l_comp()).r_comp()
            }
        }

        #f64
        #(#blades)*
        #(#grades)*
    }
}

fn define_f64() -> TokenStream {
    quote! {
        impl<Rhs> Geometric<Rhs> for f64
        where
            f64: std::ops::Mul<Rhs>,
        {
            type Output = <f64 as std::ops::Mul<Rhs>>::Output;

            fn geo(self, rhs: Rhs) -> Self::Output {
                self * rhs
            }
        }
    }
}

impl Blade {
    fn geo(self) -> TokenStream {
        quote! {
            impl<Rhs> Geometric<Rhs> for #self
            where
                Self: std::ops::Mul<Rhs>,
            {
                type Output = <Self as std::ops::Mul<Rhs>>::Output;

                fn geo(self, rhs: Rhs) -> Self::Output {
                    self * rhs
                }
            }
        }
    }
}

impl Grade {
    fn define_geo(self) -> TokenStream {
        let f64 = self.geo_f64();

        let grades = Grade::iter().map(|rhs| Grade::geo_self(self, rhs));

        quote! {
            #f64
            #(#grades)*
        }
    }

    fn geo_f64(self) -> TokenStream {
        quote! {
            impl Geometric<f64> for #self {
                type Output = #self;

                fn geo(self, rhs: f64) -> Self::Output {
                    self * rhs
                }
            }
        }
    }

    fn geo_self(lhs: Self, rhs: Self) -> TokenStream {
        let output = lhs.geo(rhs);

        let constructor = match output {
            GeoProd::Zero => zero::ident().to_token_stream(),
            GeoProd::Blade(blade) => {
                let sum = lhs
                    .blades()
                    .flat_map(|lhs_b| {
                        rhs.blades().map(move |rhs_b| {
                            let lhs_f = lhs_b.field();
                            let rhs_f = rhs_b.field();
                            quote! {
                                self.#lhs_f * rhs.#rhs_f
                            }
                        })
                    })
                    .collect::<syn::punctuated::Punctuated<_, syn::token::Add>>();

                if sum.is_empty() {
                    quote! { Default::default() }
                } else {
                    sum.to_token_stream()
                }
            }
            GeoProd::Grade(grade) => {
                let fields = grade.blades().map(|b| {
                    let f = b.field();

                    let sum = lhs
                        .blades()
                        .flat_map(|lhs_b| {
                            rhs.blades().flat_map(move |rhs_b| {
                                let p = lhs_b * rhs_b;
                                match p {
                                    Product::Value(p, _) if p == b => {
                                        let lhs_f = lhs_b.field();
                                        let rhs_f = rhs_b.field();
                                        Some(quote! { self.#lhs_f * rhs.#rhs_f })
                                    }
                                    _ => None,
                                }
                            })
                        })
                        .collect::<syn::punctuated::Punctuated<_, syn::token::Add>>();

                    if sum.is_empty() {
                        quote! { #f: Default::default(), }
                    } else {
                        quote! { #f: #sum, }
                    }
                });

                quote! {
                    #grade {
                        #(#fields)*
                    }
                }
            }
            GeoProd::Multi(mv) => {
                // TODO simplify the process of summing the products
                // let s = if mv.s.is_some() {
                // } else {
                //     zero::ident().to_token_stream()
                // };

                quote! {
                    todo!()
                }
            }
        };

        quote! {
            impl Geometric<#rhs> for #lhs {
                type Output = #output;
                fn geo(self, rhs: #rhs) -> Self::Output {
                    #constructor
                }
            }
        }
    }
}

trait Geometric<Rhs> {
    type Output;
    fn geo(self, rhs: Rhs) -> Self::Output;
}

impl Geometric<Self> for Grade {
    type Output = GeoProd;
    fn geo(self, rhs: Self) -> Self::Output {
        self.blades()
            .flat_map(|lhs| rhs.blades().map(move |rhs| lhs * rhs))
            .sum::<Multivector>()
            .geo_prod()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GeoProd {
    Zero,
    Blade(Blade),
    Grade(Grade),
    Multi(Multivector),
}

impl ToTokens for GeoProd {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            GeoProd::Zero => tokens.append(zero::ident()),
            GeoProd::Blade(b) => b.to_tokens(tokens),
            GeoProd::Grade(g) => g.to_tokens(tokens),
            GeoProd::Multi(mv) => mv.to_tokens(tokens),
        }
    }
}

#[test]
fn geometric_vectors() {
    let v = Grade {
        k: 1,
        ty: GradeType::Whole,
    };
    let sb = v.geo(v);
    assert_eq!(
        GeoProd::Multi(Multivector {
            s: Some(()),
            v: None,
            b: Some(GradeType::Whole),
            t: None,
            a: None
        }),
        sb
    )
}
