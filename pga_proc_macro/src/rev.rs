use super::*;

pub fn define() -> TokenStream {
    let f64 = f64_reverse();
    let blades = blades_reverse();
    let grades = grades_reverse();

    quote! {
        pub trait Reverse {
            fn rev(self) -> Self;
        }

        pub trait Antireverse {
            fn antirev(self) -> Self;
        }

        #f64
        #blades
        #grades
    }
}

fn f64_reverse() -> TokenStream {
    quote! {
        impl Reverse for f64 {
            fn rev(self) -> Self {
                self
            }
        }

        impl Antireverse for f64 {
            fn antirev(self) -> Self {
                self
            }
        }
    }
}

fn blades_reverse() -> TokenStream {
    let reverse = Basis::iter().map(blade_reverse);
    let antireverse = Basis::iter().map(blade_antireverse);

    quote! {
        #(#reverse)*
        #(#antireverse)*
    }
}

fn blade_reverse(blade: Basis) -> TokenStream {
    let sign = match blade.grade() {
        0 | 1 | 4 => Sign::Pos,
        _ => Sign::Neg,
    };

    quote! {
        impl Reverse for #blade {
            fn rev(self) -> Self {
                #sign self
            }
        }
    }
}

fn blade_antireverse(blade: Basis) -> TokenStream {
    let sign = match blade.grade() {
        0 | 3 | 4 => Sign::Pos,
        _ => Sign::Neg,
    };

    quote! {
        impl Antireverse for #blade {
            fn antirev(self) -> Self {
                #sign self
            }
        }
    }
}

fn grades_reverse() -> TokenStream {
    let reverse = Grade::iter().map(grade_reverse);
    let antireverse = Grade::iter().map(grade_antireverse);

    quote! {
        #(#reverse)*
        #(#antireverse)*
    }
}

fn grade_reverse(grade: Grade) -> TokenStream {
    let fields = grade.blades().map(|b| {
        let f = b.field();
        quote! { #f: self.#f.rev(), }
    });

    quote! {
        impl Reverse for #grade {
            fn rev(self) -> Self {
                #grade {
                    #(#fields)*
                }
            }
        }
    }
}

fn grade_antireverse(grade: Grade) -> TokenStream {
    let fields = grade.blades().map(|b| {
        let f = b.field();
        quote! { #f: self.#f.antirev(), }
    });

    quote! {
        impl Antireverse for #grade {
            fn antirev(self) -> Self {
                #grade {
                    #(#fields)*
                }
            }
        }
    }
}
