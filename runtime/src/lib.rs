// #![cfg_attr(not(feature = "std"), no_std)]
#![allow(dead_code)]
#[macro_use]
extern crate alloc;

mod mem;
mod metadata;
mod resolver;
mod result;
mod sandbox;
mod seal;

type StorageKey = [u8; 32];

pub use self::{
    mem::scan_imports,
    metadata::Metadata,
    resolver::Resolver,
    result::{Error, Result},
    sandbox::Sandbox,
};
