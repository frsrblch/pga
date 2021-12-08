extern crate proc_macro;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};

use blades::Basis;
use grades::Grade;

mod product;

mod blades;
mod grades;
mod zero;

mod add;
mod mul;
mod neg;

#[proc_macro]
pub fn pga(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let zero = zero::define();
    let blades = blades::Basis::iter().map(|b| b.define());
    let grades = grades::Grade::iter().map(|g| g.define());

    let addition = add::define();
    let multiplication = mul::define();
    let negation = neg::define();

    let tokens = quote! {
        #zero
        #(#blades)*
        #(#grades)*

        #addition
        #multiplication
        #negation
    };

    tokens.into()
}
