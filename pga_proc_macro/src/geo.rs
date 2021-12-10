use super::*;
use crate::multivector::Multivector;

pub fn define() -> TokenStream {
    let f64 = define_f64();
    let blades = Blade::iter().map(Blade::geo);
    let grades = Grade::iter().flat_map(|lhs| Grade::iter().map(move |rhs| Grade::geo(lhs, rhs)));

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
    fn geo(lhs: Self, rhs: Self) -> TokenStream {
        let f64 = lhs.geo_f64();

        quote! {
            #f64
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
}

trait Geometric<Rhs> {
    type Output;
    fn geo(self, rhs: Rhs) -> Self::Output;
}

impl Geometric<Self> for Grade {
    type Output = Multivector;
    fn geo(self, rhs: Self) -> Self::Output {
        let mut mv = Multivector::default();

        for lhs_blade in self.blades() {
            for rhs_blade in rhs.blades() {
                if let Product::Value(output, _) = lhs_blade * rhs_blade {
                    match output.grade() {
                        0 => mv.s = Some(()),
                        1 => {
                            mv.v = Some(match mv.v {
                                None => output.grade_type(),
                                Some(gt) => gt + output.grade_type(),
                            })
                        }
                        2 => {
                            mv.b = Some(match mv.b {
                                None => output.grade_type(),
                                Some(gt) => gt + output.grade_type(),
                            })
                        }
                        3 => {
                            mv.t = Some(match mv.t {
                                None => output.grade_type(),
                                Some(gt) => gt + output.grade_type(),
                            })
                        }
                        4 => mv.a = Some(()),
                        _ => unreachable!(),
                    }
                }
            }
        }

        mv
    }
}
