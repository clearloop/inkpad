#![allow(dead_code)]
#[macro_use]
extern crate alloc;

mod metadata;
mod resolver;
mod result;
mod runtime;
mod sandbox;
mod seal;
mod storage;
mod util;

type StorageKey = [u8; 32];

pub use self::{
    metadata::Metadata,
    resolver::Resolver,
    result::{Error, Result},
    runtime::Runtime,
    sandbox::Sandbox,
    storage::Storage,
};
