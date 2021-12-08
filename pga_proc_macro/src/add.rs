use super::*;
use crate::blades::Basis;
use crate::grades::Grade;

pub fn define() -> TokenStream {
    let blade_addition = blade_addition();
    let grade_addition = grade_addition();

    quote! {
        impl<T> std::ops::Add<T> for Zero {
            type Output = T;
            #[inline]
            fn add(self, rhs: T) -> Self::Output {
                rhs
            }
        }

        #(#blade_addition)*
        #(#grade_addition)*
    }
}

fn blade_addition() -> impl Iterator<Item = TokenStream> + 'static {
    Basis::iter().map(|lhs| {
        quote! {
            impl std::ops::Add<Zero> for #lhs {
                type Output = Self;
                #[inline]
                fn add(self, _: Zero) -> Self::Output {
                    self
                }
            }

            impl std::ops::Sub<Zero> for #lhs {
                type Output = Self;
                #[inline]
                fn sub(self, _: Zero) -> Self::Output {
                    self
                }
            }

            impl std::ops::Add for #lhs {
                type Output = Self;
                #[inline]
                fn add(self, rhs: Self) -> Self {
                    (self.0 + rhs.0).into()
                }
            }

            impl std::ops::Sub for #lhs {
                type Output = Self;
                #[inline]
                fn sub(self, rhs: Self) -> Self {
                    (self.0 - rhs.0).into()
                }
            }
        }
    })
}

fn grade_addition() -> impl Iterator<Item = TokenStream> + 'static {
    Grade::iter().map(|lhs| {
        let add_grades = Grade::iter().map(move |rhs| add_grades(lhs, rhs));
        let sub_grades = Grade::iter().map(move |rhs| sub_grades(lhs, rhs));

        quote! {
            impl std::ops::Add<Zero> for #lhs {
                type Output = #lhs;
                #[inline]
                fn add(self, _: Zero) -> Self {
                    self
                }
            }

            impl std::ops::Sub<Zero> for #lhs {
                type Output = #lhs;
                #[inline]
                fn sub(self, _: Zero) -> Self {
                    self
                }
            }

            #(#add_grades)*
            #(#sub_grades)*
        }
    })
}

fn add_grades(lhs: Grade, rhs: Grade) -> TokenStream {
    if lhs.k == rhs.k {
        let output = Grade {
            k: lhs.k,
            ty: lhs.ty + rhs.ty,
        };

        let fields = output.blades().map(|b| {
            let f = b.field();
            match (lhs.contains(b), rhs.contains(b)) {
                (true, true) => quote! { #f: self.#f + rhs.#f, },
                (true, false) => quote! { #f: self.#f, },
                (false, true) => quote! { #f: rhs.#f, },
                (false, false) => {
                    unreachable!("add_grades: at least one side should contain each blade")
                }
            }
        });

        quote! {
            impl std::ops::Add<#rhs> for #lhs {
                type Output = #output;
                #[inline]
                fn add(self, rhs: #rhs) -> Self::Output {
                    #output {
                        #(#fields)*
                    }
                }
            }
        }
    } else {
        // TODO disimilar grade addition = multivector
        quote! {}
    }
}

fn sub_grades(lhs: Grade, rhs: Grade) -> TokenStream {
    if lhs.k == rhs.k {
        let output = Grade {
            k: lhs.k,
            ty: lhs.ty + rhs.ty,
        };

        let fields = output.blades().map(|b| {
            let f = b.field();
            match (lhs.contains(b), rhs.contains(b)) {
                (true, true) => quote! { #f: self.#f - rhs.#f, },
                (true, false) => quote! { #f: self.#f, },
                (false, true) => quote! { #f: -rhs.#f, },
                (false, false) => {
                    unreachable!("add_grades: at least one side should contain each blade")
                }
            }
        });

        quote! {
            impl std::ops::Sub<#rhs> for #lhs {
                type Output = #output;
                #[inline]
                fn sub(self, rhs: #rhs) -> Self::Output {
                    #output {
                        #(#fields)*
                    }
                }
            }
        }
    } else {
        // TODO disimilar grade addition = multivector
        quote! {}
    }
}
