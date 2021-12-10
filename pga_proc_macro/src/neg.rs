use super::*;
use crate::blades::Blade;
use crate::grades::Grade;

pub fn define() -> TokenStream {
    let neg_blades = Blade::iter().map(neg_blade);
    let neg_grades = Grade::iter().map(neg_grade);

    quote! {
        #(#neg_blades)*
        #(#neg_grades)*
    }
}

fn neg_blade(blade: Blade) -> TokenStream {
    quote! {
        impl std::ops::Neg for #blade {
            type Output = Self;

            fn neg(self) -> Self::Output {
                (-self.0).into()
            }
        }
    }
}

fn neg_grade(grade: Grade) -> TokenStream {
    let fields = grade.blades().map(|b| {
        let f = b.field();
        quote! { #f: -self.#f, }
    });

    quote! {
        impl std::ops::Neg for #grade {
            type Output = Self;

            fn neg(self) -> Self::Output {
                #grade {
                    #(#fields)*
                }
            }
        }
    }
}
