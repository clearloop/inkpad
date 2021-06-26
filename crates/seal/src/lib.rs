//! Ceres supported host functions
#![cfg_attr(not(feature = "std"), no_std)]
use ceres_std::{vec, Vec};

mod chain;
mod contract;
mod derive;
mod event;
mod fun;
mod instantiate;
mod restore;
// mod ri;
mod storage;
mod transfer;

pub use self::derive::Host;
pub use ceres_sandbox::RuntimeInterfaces;

/// Seal calls
pub type SealCall =
    ceres_executor::derive::HostParcel<&'static str, &'static str, ceres_sandbox::Sandbox>;

/// Pallet contract host functions
pub fn pallet_contracts(ri: Option<impl RuntimeInterfaces>) -> Vec<SealCall> {
    let mut wasm = vec![
        chain::Gas::pack(),
        chain::BlockNumber::pack(),
        chain::SealWeightToFee::pack(),
        contract::SealTombstoneDeposit::pack(),
        contract::SealRentAllowance::pack(),
        contract::SealSetRentAllowance::pack(),
        event::SealDepositEvent::pack(),
        fun::SealInput::pack(),
        fun::SealReturn::pack(),
        fun::SealTerminate::pack(),
        restore::Restore::pack(),
        storage::SealGetStorage::pack(),
        storage::SealClearStorage::pack(),
        storage::SealSetStorage::pack(),
        transfer::SealAddress::pack(),
        transfer::SealBalance::pack(),
        transfer::SealCaller::pack(),
        transfer::SealValueTransferred::pack(),
        instantiate::SealCall::pack(),
        instantiate::SealInstantiate::pack(),
    ];

    if let Some(interfaces) = ri {
        wasm.append(&mut RuntimeInterfaces::pack(&interfaces))
    }

    wasm
}
