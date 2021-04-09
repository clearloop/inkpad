//! Ceres supported host functions
#![cfg_attr(not(feature = "std"), no_std)]
use ceres_std::{vec, Vec};

mod chain;
mod contract;
mod derive;
mod fun;
mod storage;
mod transfer;

pub use derive::{Host, ReturnCode};

/// Pallet contract host functions
pub fn pallet_contracts(
) -> Vec<ceres_executor::derive::HostParcel<&'static str, &'static str, ceres_sandbox::Sandbox>> {
    vec![
        chain::Gas::pack(),
        chain::BlockNumber::pack(),
        contract::SealTombstoneDeposit::pack(),
        contract::SealRentAllowance::pack(),
        contract::SealSetRentAllowance::pack(),
        transfer::SealValueTransferred::pack(),
        transfer::SealCaller::pack(),
        transfer::SealAddress::pack(),
        fun::SealInput::pack(),
        fun::SealReturn::pack(),
        fun::SealTerminate::pack(),
        storage::SealGetStorage::pack(),
        storage::SealClearStorage::pack(),
        storage::SealSetStorage::pack(),
    ]
}
