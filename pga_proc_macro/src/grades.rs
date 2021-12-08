use super::*;
use crate::blades::Basis;
use proc_macro2::{Ident, Span};
use std::fmt::Display;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Grade {
    k: u8,
    ty: GradeType,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GradeType {
    Whole,
    Bulk,
    Weight,
}

impl GradeType {
    pub fn iter() -> std::slice::Iter<'static, Self> {
        [Self::Whole, Self::Bulk, Self::Weight].iter()
    }
}

impl Grade {
    pub fn iter() -> impl Iterator<Item = Self> + 'static {
        (1..4).flat_map(|k| GradeType::iter().map(move |&ty| Grade { k, ty }))
    }

    pub fn define(&self) -> TokenStream {
        // don't define f64 or E0123
        if self.k == 0 || self.k == 4 {
            return quote! {};
        }

        let grade = self.ident();
        let blades = Basis::iter()
            // TODO extract filtering into fn Grade::contains(Basis)
            .filter(|b| b.grade() == self.k)
            .filter(|b| match self.ty {
                GradeType::Whole => true,
                GradeType::Bulk => !b.get(0),
                GradeType::Weight => b.get(0),
            })
            .map(|b| {
                let blade = b.ident();
                let field = b.field();
                quote! {
                    #field: #blade,
                }
            });

        quote! {
            #[derive(Debug, Default, Copy, Clone, PartialEq)]
            pub struct #grade {
                #(#blades)*
            }
        }
    }

    pub fn ident(&self) -> Ident {
        Ident::new(&self.to_string(), Span::call_site())
    }
}

impl Display for Grade {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let name = match self.k {
            0 => return write!(f, "f64"),
            4 => return write!(f, "{}", Basis(0b_00001111)),
            1 => "Vector",
            2 => "Bivector",
            3 => "Trivector",
            _ => unreachable!("invalid grade"),
        };

        let suffix = match self.ty {
            GradeType::Whole => "",
            GradeType::Bulk => "Bulk",
            GradeType::Weight => "Weight",
        };

        write!(f, "{}{}", name, suffix)
    }
}
