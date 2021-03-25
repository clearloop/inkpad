extern crate proc_macro;

mod sig;

use proc_macro::TokenStream;

/// Generate sigature of wasmi of custom seal functions
#[proc_macro_attribute]
pub fn sig(attr: TokenStream, item: TokenStream) -> TokenStream {
    sig::parse(attr, item)
}
