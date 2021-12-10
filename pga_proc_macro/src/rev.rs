use super::*;

pub fn define() -> TokenStream {
    let f64 = f64_reverse();
    let blades = Blade::iter().map(Blade::reverse);
    let grades = Grade::iter().map(Grade::reverse);

    quote! {
        pub trait Reverse {
            fn rev(self) -> Self;
        }

        pub trait Antireverse {
            fn antirev(self) -> Self;
        }

        impl<T, U> Antireverse for T
        where
            T: LeftComp<Output=U>,
            U: Reverse + RightComp<Output=T>,
        {
            fn antirev(self) -> Self {
                self.l_comp().rev().r_comp()
            }
        }

        #f64
        #(#blades)*
        #(#grades)*
    }
}

fn f64_reverse() -> TokenStream {
    quote! {
        impl Reverse for f64 {
            fn rev(self) -> Self {
                self
            }
        }
    }
}

impl Blade {
    fn reverse(self) -> TokenStream {
        let sign = match self.grade() {
            0 | 1 | 4 => Sign::Pos,
            _ => Sign::Neg,
        };

        quote! {
            impl Reverse for #self {
                fn rev(self) -> Self {
                    #sign self
                }
            }
        }
    }
}

impl Grade {
    fn reverse(self) -> TokenStream {
        let fields = self.blades().map(|b| {
            let f = b.field();
            quote! { #f: self.#f.rev(), }
        });

        quote! {
            impl Reverse for #self {
                fn rev(self) -> Self {
                    #self {
                        #(#fields)*
                    }
                }
            }
        }
    }
}
