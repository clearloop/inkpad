extern crate proc_macro;
use proc_macro::TokenStream;

mod arg;
mod attr;
mod fun;

/// Derive custom function to wasm host functions
#[proc_macro_attribute]
pub fn host(attr: TokenStream, item: TokenStream) -> TokenStream {
    attr::host::parse(attr, item)
}
