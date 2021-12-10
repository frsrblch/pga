use super::*;
use proc_macro2::{Ident, Span};
use std::fmt::Display;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Grade {
    pub k: u8,
    pub ty: GradeType,
}

impl ToTokens for Grade {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(self.ident());
    }
}

impl std::ops::Not for Grade {
    type Output = Self;
    fn not(self) -> Self {
        Grade {
            k: 4 - self.k,
            ty: !self.ty,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GradeType {
    Whole,
    Bulk,
    Weight,
}

impl std::ops::Add for GradeType {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        use GradeType::*;
        match (self, rhs) {
            (Bulk, Bulk) => Bulk,
            (Weight, Weight) => Weight,
            _ => Whole,
        }
    }
}

impl std::ops::Not for GradeType {
    type Output = Self;
    fn not(self) -> Self {
        match self {
            Self::Whole => Self::Whole,
            Self::Bulk => Self::Weight,
            Self::Weight => Self::Bulk,
        }
    }
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

        let blades = Blade::iter().filter(|b| self.contains(*b)).map(|b| {
            let f = b.field();
            quote! {
                #f: #b,
            }
        });

        quote! {
            #[derive(Debug, Default, Copy, Clone, PartialEq)]
            pub struct #self {
                #(#blades)*
            }
        }
    }

    pub fn ident(&self) -> Ident {
        Ident::new(&self.to_string(), Span::call_site())
    }

    pub fn contains(&self, blade: Blade) -> bool {
        self.k == blade.grade()
            && match self.ty {
                GradeType::Whole => true,
                GradeType::Bulk => !blade.get(0),
                GradeType::Weight => blade.get(0),
            }
    }

    pub fn blades(&self) -> impl Iterator<Item = Blade> + '_ {
        Blade::iter().filter(move |b| self.contains(*b))
    }
}

impl Display for Grade {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let name = match self.k {
            0 => return write!(f, "f64"),
            4 => return write!(f, "{}", Blade(0b_00001111)),
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
