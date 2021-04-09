//! Ceres supported host functions
#![cfg_attr(not(feature = "std"), no_std)]
use ceres_std::{vec, Vec};

mod balance;
mod chain;
mod derive;
mod fun;
mod storage;

pub use derive::{Host, ReturnCode};

/// Pallet contract host functions
pub fn pallet_contracts(
) -> Vec<ceres_executor::derive::HostParcel<&'static str, &'static str, ceres_sandbox::Sandbox>> {
    vec![
        balance::SealValueTransferred::pack(),
        fun::SealInput::pack(),
        fun::SealReturn::pack(),
        storage::SealGetStorage::pack(),
        storage::SealClearStorage::pack(),
        storage::SealSetStorage::pack(),
    ]
}
