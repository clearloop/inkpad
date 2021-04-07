#![allow(dead_code)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
#[macro_use]
extern crate alloc;

mod metadata;
mod result;
mod runtime;
mod storage;
mod util;

type StorageKey = [u8; 32];

pub use self::{
    metadata::Metadata,
    result::{Error, Result},
    runtime::Runtime,
    storage::Storage,
};
