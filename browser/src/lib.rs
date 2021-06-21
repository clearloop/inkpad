mod result;
mod ri;
mod runtime;
mod ti;
mod tree;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub use self::{
    result::{Error, Result},
    ri::Interface,
    runtime::Runtime,
    ti::Transaction,
    tree::Tree,
};
