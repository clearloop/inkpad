#![allow(dead_code)]
#![cfg_attr(not(feature = "std"), no_std)]

mod method;
mod result;
mod runtime;
pub mod util;

type StorageKey = [u8; 32];

pub use self::{
    result::{Error, Result},
    runtime::Runtime,
};
