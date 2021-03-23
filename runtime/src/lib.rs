#![cfg_attr(not(feature = "std"), no_std)]
#![allow(dead_code)]
#[cfg(not(feature = "std"))]
extern crate alloc;

mod metadata;
mod result;
mod sandbox;
mod seal;

type StorageKey = [u8; 32];

pub use self::{
    metadata::Metadata,
    result::{Error, Result},
    sandbox::Sandbox,
};
