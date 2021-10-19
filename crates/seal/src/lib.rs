//! Ceres supported host functions
#![cfg_attr(not(feature = "std"), no_std)]
use ceres_std::{vec, Vec};

mod chain;
mod contract;
mod crypto;
mod derive;
mod event;
mod fun;
mod instantiate;
mod restore;
mod storage;
mod transfer;

pub use self::derive::Host;
use ceres_sandbox::RuntimeInterfaces;

/// Seal calls
pub type SealCall =
    ceres_executor::derive::HostCall<&'static str, &'static str, ceres_sandbox::Sandbox>;

/// Pallet contract host functions
pub fn pallet_contracts(ri: Option<impl RuntimeInterfaces>) -> Vec<SealCall> {
    let mut wasm = vec![
        chain::Seal0Gas::pack(),
        chain::Seal0SealGasLeft::pack(),
        chain::Seal0BlockNumber::pack(),
        chain::Seal0SealWeightToFee::pack(),
        chain::Seal0SealCallChainExtension::pack(),
        contract::Seal0SealTombstoneDeposit::pack(),
        contract::Seal0SealRentAllowance::pack(),
        contract::Seal0SealSetRentAllowance::pack(),
        event::Seal0SealDepositEvent::pack(),
        fun::Seal0SealInput::pack(),
        fun::Seal0SealReturn::pack(),
        fun::Seal0SealTerminate::pack(),
        restore::Seal0SealRestoreTo::pack(),
        storage::Seal0SealGetStorage::pack(),
        storage::Seal0SealClearStorage::pack(),
        storage::Seal0SealSetStorage::pack(),
        transfer::Seal0SealTransfer::pack(),
        transfer::Seal0SealAddress::pack(),
        transfer::Seal0SealBalance::pack(),
        transfer::Seal0SealMinimumBalance::pack(),
        transfer::Seal0SealCaller::pack(),
        transfer::Seal0SealValueTransferred::pack(),
        instantiate::Seal0SealCall::pack(),
        instantiate::Seal0SealInstantiate::pack(),
        // #[seal1]
        contract::Seal1SealSetRentAllowance::pack(),
        fun::Seal1SealTerminate::pack(),
        instantiate::Seal1SealInstantiate::pack(),
        restore::Seal1SealRestoreTo::pack(),
        // #[__unstable__]
        chain::UnstableSealCallRuntime::pack(),
        crypto::UnstableSealEcdsaRecover::pack(),
        instantiate::UnstableSealCall::pack(),
    ];

    if let Some(interfaces) = ri {
        wasm.append(&mut RuntimeInterfaces::pack(&interfaces))
    }

    wasm
}
