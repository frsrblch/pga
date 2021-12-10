use super::*;

pub fn define() -> TokenStream {
    let zero = zero_multiplication();
    let blades = Blade::iter().map(blade_multiplication);
    let grades = Grade::iter().map(grade_multiplication);

    quote! {
        #zero
        #(#blades)*
        #(#grades)*
    }
}

fn zero_multiplication() -> TokenStream {
    quote! {
        impl<T> std::ops::Mul<T> for Zero {
            type Output = Zero;

            fn mul(self, _: T) -> Self::Output {
                Zero
            }
        }

        impl std::ops::Div<f64> for Zero {
            type Output = Zero;

            fn div(self, _: f64) -> Self::Output {
                Zero
            }
        }

        impl std::ops::MulAssign<f64> for Zero {
            fn mul_assign(&mut self, _: f64) {}
        }

        impl std::ops::DivAssign<f64> for Zero {
            fn div_assign(&mut self, _: f64) {}
        }
    }
}

fn blade_multiplication(lhs: Blade) -> TokenStream {
    let other_blades = Blade::iter().map(move |rhs| match lhs * rhs {
        Product::Value(output, sign) => {
            let output_blade = output.ident();

            quote! {
                impl std::ops::Mul<#rhs> for #lhs {
                    type Output = #output_blade;

                    fn mul(self, rhs: #rhs) -> Self::Output {
                        (#sign self.0 * rhs.0).into()
                    }
                }
            }
        }
        Product::Zero => {
            return quote! {
                impl std::ops::Mul<#rhs> for #lhs {
                    type Output = Zero;

                    fn mul(self, _: #rhs) -> Self::Output {
                        Zero
                    }
                }
            };
        }
    });

    quote! {
        impl std::ops::Mul<f64> for #lhs {
            type Output = Self;

            fn mul(self, rhs: f64) -> Self::Output {
                (self.0 * rhs).into()
            }
        }

        impl std::ops::Div<f64> for #lhs {
            type Output = Self;

            fn div(self, rhs: f64) -> Self::Output {
                (self.0 / rhs).into()
            }
        }

        impl std::ops::MulAssign<f64> for #lhs {
            fn mul_assign(&mut self, rhs: f64) {
                self.0 *= rhs;
            }
        }

        impl std::ops::DivAssign<f64> for #lhs {
            fn div_assign(&mut self, rhs: f64) {
                self.0 /= rhs;
            }
        }

        impl std::ops::Mul<#lhs> for f64 {
            type Output = #lhs;

            fn mul(self, rhs: #lhs) -> Self::Output {
                #lhs(self * rhs.0)
            }
        }

        #(#other_blades)*
    }
}

fn grade_multiplication(grade: Grade) -> TokenStream {
    let fields_mul_assign = grade.blades().map(|b| {
        let f = b.field();
        quote! { self.#f *= rhs; }
    });

    let fields_div_assign = grade.blades().map(|b| {
        let f = b.field();
        quote! { self.#f /= rhs; }
    });

    quote! {
        impl std::ops::MulAssign<f64> for #grade {
            fn mul_assign(&mut self, rhs: f64) {
                #(#fields_mul_assign)*
            }
        }

        impl std::ops::DivAssign<f64> for #grade {
            fn div_assign(&mut self, rhs: f64) {
                #(#fields_div_assign)*
            }
        }
    }
}
