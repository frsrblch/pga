use super::*;
use grades::GradeType;

pub fn define() -> TokenStream {
    let grade_bulk = Grade::iter().map(bulk);
    let grade_weight = Grade::iter().map(weight);

    quote! {
        pub trait Bulk {
            type Output;
            fn bulk(self) -> Self::Output;
        }

        pub trait Weight {
            type Output;
            fn weight(self) -> Self::Output;
        }

        #(#grade_bulk)*
        #(#grade_weight)*
    }
}

fn bulk(grade: Grade) -> TokenStream {
    if grade.ty == GradeType::Weight {
        return quote! {
            impl Bulk for #grade {
                type Output = Zero;
                fn bulk(self) -> Zero {
                    Zero
                }
            }
        };
    }

    let output = Grade {
        k: grade.k,
        ty: GradeType::Bulk,
    };

    let fields = output.blades().map(|b| {
        let f = b.field();
        quote! { #f: self.#f, }
    });

    quote! {
        impl Bulk for #grade {
            type Output = #output;
            fn bulk(self) -> Self::Output {
                #output {
                    #(#fields)*
                }
            }
        }
    }
}

fn weight(grade: Grade) -> TokenStream {
    if grade.ty == GradeType::Bulk {
        return quote! {
            impl Weight for #grade {
                type Output = Zero;
                fn weight(self) -> Zero {
                    Zero
                }
            }
        };
    }

    let output = Grade {
        k: grade.k,
        ty: GradeType::Weight,
    };

    let fields = output.blades().map(|b| {
        let f = b.field();
        quote! { #f: self.#f, }
    });

    quote! {
        impl Weight for #grade {
            type Output = #output;
            fn weight(self) -> Self::Output {
                #output {
                    #(#fields)*
                }
            }
        }
    }
}
