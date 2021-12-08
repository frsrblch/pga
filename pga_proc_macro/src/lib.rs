extern crate proc_macro;
use proc_macro2::TokenStream;
use quote::quote;

mod product;

mod blades;
mod grades;
mod zero;

mod add;
mod mul;

#[proc_macro]
pub fn pga(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let zero = zero::define();
    let blades = blades::Basis::iter().map(|b| b.define());
    let grades = grades::Grade::iter().map(|g| g.define());

    let addition = add::define();
    let multiplication = mul::define();

    let tokens = quote! {
        #zero
        #(#blades)*
        #(#grades)*

        #addition
        #multiplication
    };

    tokens.into()
}
