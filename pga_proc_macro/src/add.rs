use super::*;
use crate::blades::Basis;

pub fn define() -> TokenStream {
    let blade_addition = blade_addition();

    quote! {
        impl<T> std::ops::Add<T> for Zero {
            type Output = T;
            #[inline]
            fn add(self, rhs: T) -> Self::Output {
                rhs
            }
        }

        #(#blade_addition)*
    }
}

fn blade_addition() -> impl Iterator<Item = TokenStream> + 'static {
    Basis::iter().map(|b| {
        let blade = b.ident();

        quote! {
            impl std::ops::Add<Zero> for #blade {
                type Output = Self;
                #[inline]
                fn add(self, _: Zero) -> Self::Output {
                    self
                }
            }

            impl std::ops::Add for #blade {
                type Output = Self;
                #[inline]
                fn add(self, rhs: Self) -> Self {
                    (self.0 + rhs.0).into()
                }
            }
        }
    })
}
