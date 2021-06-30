#![allow(dead_code)]
#![cfg_attr(not(feature = "std"), no_std)]

mod metadata;
mod method;
mod result;
mod runtime;
pub mod util;

type StorageKey = [u8; 32];

pub use self::{
    metadata::Metadata,
    result::{Error, Result},
    runtime::Runtime,
};
