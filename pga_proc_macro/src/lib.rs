extern crate proc_macro;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};

use blades::Basis;
use grades::Grade;
use product::{Product, Sign};

mod product;

mod blades;
mod grades;
mod multivector;
mod zero;

mod add;
mod mul;
mod neg;
mod rev;

#[proc_macro]
pub fn pga(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let zero = zero::define();
    let blades = blades::Basis::iter().map(|b| b.define());
    let grades = grades::Grade::iter().map(|g| g.define());
    let multivector = multivector::define();

    let addition = add::define();
    let multiplication = mul::define();
    let negation = neg::define();
    let reverse = rev::define();

    let tokens = quote! {
        #zero
        #(#blades)*
        #(#grades)*
        #multivector

        #addition
        #multiplication
        #negation
        #reverse
    };

    // std::fs::write("tokens.txt", tokens.to_string()).ok();

    tokens.into()
}
