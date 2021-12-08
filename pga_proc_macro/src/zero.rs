use super::*;

pub fn define() -> TokenStream {
    quote! {
        #[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Zero;
    }
}
