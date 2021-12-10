use super::*;

pub fn define() -> TokenStream {
    let f64 = f64_comp();
    let blades = Blade::define_comp();
    let grades = Grade::define_comp();

    quote! {
        pub trait LeftComp {
            type Output;
            fn l_comp(self) -> Self::Output;
        }

        pub trait RightComp {
            type Output;
            fn r_comp(self) -> Self::Output;
        }

        #f64
        #blades
        #grades
    }
}

fn f64_comp() -> TokenStream {
    quote! {
        impl LeftComp for f64 {
            type Output = E0123;
            fn l_comp(self) -> Self::Output {
                E0123(self)
            }
        }

        impl RightComp for f64 {
            type Output = E0123;
            fn r_comp(self) -> Self::Output {
                E0123(self)
            }
        }
    }
}

impl Blade {
    fn define_comp() -> TokenStream {
        let l_comp = Blade::iter().map(Blade::l_comp);
        let r_comp = Blade::iter().map(Blade::r_comp);

        quote! {
            #(#l_comp)*
            #(#r_comp)*
        }
    }

    fn l_comp(self) -> TokenStream {
        let output = !self;
        let sign = match self.grade() {
            1 | 2 => Sign::Neg,
            _ => Sign::Pos,
        };

        quote! {
            impl LeftComp for #self {
                type Output = #output;
                fn l_comp(self) -> Self::Output {
                    (#sign self.0).into()
                }
            }
        }
    }

    fn r_comp(self) -> TokenStream {
        let output = !self;
        let sign = match self.grade() {
            2 | 3 => Sign::Neg,
            _ => Sign::Pos,
        };

        quote! {
            impl RightComp for #self {
                type Output = #output;
                fn r_comp(self) -> Self::Output {
                    (#sign self.0).into()
                }
            }
        }
    }
}

impl Grade {
    fn define_comp() -> TokenStream {
        let l_comp = Grade::iter().map(Grade::l_comp);
        let r_comp = Grade::iter().map(Grade::r_comp);

        quote! {
            #(#l_comp)*
            #(#r_comp)*
        }
    }

    fn l_comp(self) -> TokenStream {
        let output = !self;

        let fields = output.blades().map(|b| {
            let f = b.field();
            let self_f = (!b).field();
            quote! { #f: self.#self_f.l_comp(), }
        });

        quote! {
            impl LeftComp for #self {
                type Output = #output;
                fn l_comp(self) -> Self::Output {
                    #output {
                        #(#fields)*
                    }
                }
            }
        }
    }

    fn r_comp(self) -> TokenStream {
        let output = !self;

        let fields = output.blades().map(|b| {
            let f = b.field();
            let self_f = (!b).field();
            quote! { #f: self.#self_f.r_comp(), }
        });

        quote! {
            impl RightComp for #self {
                type Output = #output;
                fn r_comp(self) -> Self::Output {
                    #output {
                        #(#fields)*
                    }
                }
            }
        }
    }
}
