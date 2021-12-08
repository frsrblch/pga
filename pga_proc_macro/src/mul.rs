use super::*;
use crate::blades::Basis;
use crate::product::Product;

pub fn define() -> TokenStream {
    let blades = blade_multiplication();

    quote! {
        impl<T> std::ops::Mul<T> for Zero {
            type Output = Zero;

            fn mul(self, _: T) -> Self::Output {
                Zero
            }
        }

        #(#blades)*
    }
}

fn blade_multiplication() -> impl Iterator<Item = TokenStream> + 'static {
    Basis::iter().map(|lhs| {
        let ref blade = lhs.ident();

        let other_blades = Basis::iter().map(move |rhs| {
            let rhs_blade = rhs.ident();

            match lhs * rhs {
                Product::Value(output, sign) => {
                    let output_blade = output.ident();
                    let sign = sign.tokens();

                    quote! {
                        impl std::ops::Mul<#rhs_blade> for #blade {
                            type Output = #output_blade;

                            fn mul(self, rhs: #rhs_blade) -> Self::Output {
                                (#sign self.0 * rhs.0).into()
                            }
                        }
                    }
                }
                Product::Zero => {
                    return quote! {
                        impl std::ops::Mul<#rhs_blade> for #blade {
                            type Output = Zero;

                            fn mul(self, _: #rhs_blade) -> Self::Output {
                                Zero
                            }
                        }
                    };
                }
            }
        });

        quote! {
            impl std::ops::Mul<f64> for #blade {
                type Output = Self;

                fn mul(self, rhs: f64) -> Self::Output {
                    (self.0 * rhs).into()
                }
            }

            impl std::ops::Mul<#blade> for f64 {
                type Output = #blade;

                fn mul(self, rhs: #blade) -> Self::Output {
                    #blade(self * rhs.0)
                }
            }

            #(#other_blades)*
        }
    })
}
