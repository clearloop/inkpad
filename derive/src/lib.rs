extern crate proc_macro;
use proc_macro::TokenStream;

mod host;

/// Derive custom function to wasm host functions
#[proc_macro_attribute]
pub fn host(attr: TokenStream, item: TokenStream) -> TokenStream {
    host::host(attr, item)
}
