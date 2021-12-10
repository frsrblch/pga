extern crate proc_macro;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};

use blades::Blade;
use grades::Grade;
use product::{Product, Sign};

mod product;

mod blades;
mod grades;
mod multivector;
mod zero;

mod add;
mod bulk_weight;
mod comp;
mod geo;
mod mul;
mod neg;
mod rev;

#[proc_macro]
pub fn pga(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let zero = zero::define();
    let blades = blades::Blade::iter().map(|b| b.define());
    let grades = grades::Grade::iter().map(|g| g.define());
    let multivector = multivector::define();

    let addition = add::define();
    let multiplication = mul::define();
    let negation = neg::define();
    let complement = comp::define();
    let reverse = rev::define();
    let bulk_weight = bulk_weight::define();
    let geometric = geo::define();

    let tokens = quote! {
        #zero
        #(#blades)*
        #(#grades)*
        #multivector

        #addition
        #multiplication
        #negation
        #complement
        #reverse
        #bulk_weight
        #geometric
    };

    std::fs::write("./target/tokens.txt", tokens.to_string()).ok();

    tokens.into()
}
