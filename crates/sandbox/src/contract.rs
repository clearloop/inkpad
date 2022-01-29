//! Contract entry
use crate::Sandbox;
use inkpad_std::Vec;
use parity_scale_codec::{Decode, Encode};

/// Gas Meter
#[derive(Default, Clone, Decode, Encode)]
pub struct GasMeter {
    pub gas_limit: u64,
    pub gas_left: u64,
}

impl GasMeter {
    pub fn new(gas: u64) -> GasMeter {
        GasMeter {
            gas_limit: gas,
            gas_left: gas,
        }
    }

    pub fn gas_left_bytes(&self) -> Vec<u8> {
        self.gas_left.encode()
    }

    pub fn with_nested<R, F>(&mut self, amount: u64, f: F) -> R
    where
        F: FnOnce(Option<&mut GasMeter>) -> R,
    {
        // NOTE that it is ok to allocate all available gas since it still ensured
        // by `charge` that it doesn't reach zero.
        if self.gas_left < amount {
            f(None)
        } else {
            self.gas_left -= amount;

            let mut nested = GasMeter::new(amount);
            let r = f(Some(&mut nested));

            self.gas_left += nested.gas_left;
            r
        }
    }
}

/// Information needed for rent calculations that can be requested by a contract.
#[derive(Default, Clone, Encode, Decode, PartialEq, Eq)]
pub struct RentParams {
    /// The total balance of the contract. Includes the balance transferred from the caller.
    total_balance: u64,
    /// The free balance of the contract. Includes the balance transferred from the caller.
    free_balance: u64,
    /// See crate [`Contracts::subsistence_threshold()`].
    subsistence_threshold: u64,
    /// See crate [`Config::DepositPerContract`].
    deposit_per_contract: u64,
    /// See crate [`Config::DepositPerStorageByte`].
    deposit_per_storage_byte: u64,
    /// See crate [`Config::DepositPerStorageItem`].
    deposit_per_storage_item: u64,
    /// See crate [`Ext::rent_allowance()`].
    rent_allowance: u64,
    /// See crate [`Config::RentFraction`].
    rent_fraction: u16,
    /// See crate [`AliveContractInfo::storage_size`].
    storage_size: u32,
    /// See crate [`Executable::aggregate_code_len()`].
    code_size: u32,
    /// See crate [`Executable::refcount()`].
    code_refcount: u32,
    /// Reserved for backwards compatible changes to this data structure.
    _reserved: Option<()>,
}

impl Sandbox {
    pub fn tombstone_deposit(&self) -> [u8; 32] {
        [1; 32]
    }

    pub fn rent_allowance(&self) -> [u8; 32] {
        self.ext.rent_allowance
    }

    pub fn set_rent_allowance(&mut self, rent_allowence: [u8; 32]) {
        self.ext.rent_allowance = rent_allowence;
    }

    pub fn rent_params(&self) -> Vec<u8> {
        self.ext.rent_params.encode()
    }
}
