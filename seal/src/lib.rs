//! Ceres supported host functions
#![cfg_attr(not(feature = "std"), no_std)]
use ceres_std::{vec, Vec};

mod chain;
mod contract;
mod derive;
mod event;
mod fun;
// mod instantiate;
mod restore;
mod ri;
mod storage;
mod transfer;

pub use self::{
    derive::{Host, ReturnCode},
    ri::{NoRuntimeInterfaces, RuntimeInterfaces},
};

/// Pallet contract host functions
pub fn pallet_contracts(
    interfaces: impl ri::RuntimeInterfaces,
) -> Vec<ceres_executor::derive::HostParcel<&'static str, &'static str, ceres_sandbox::Sandbox>> {
    let mut wasm = vec![
        chain::Gas::pack(),
        chain::BlockNumber::pack(),
        chain::SealWeightToFee::pack(),
        contract::SealTombstoneDeposit::pack(),
        contract::SealRentAllowance::pack(),
        contract::SealSetRentAllowance::pack(),
        fun::SealInput::pack(),
        fun::SealReturn::pack(),
        fun::SealTerminate::pack(),
        restore::Restore::pack(),
        storage::SealGetStorage::pack(),
        storage::SealClearStorage::pack(),
        storage::SealSetStorage::pack(),
        transfer::SealValueTransferred::pack(),
        transfer::SealCaller::pack(),
        transfer::SealAddress::pack(),
    ];

    if ri::RuntimeInterfaces::enabled(&interfaces) {
        wasm.append(&mut ri::RuntimeInterfaces::pack(&interfaces))
    }

    wasm
}
