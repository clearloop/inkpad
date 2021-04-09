//! Transfer Entry
use crate::Sandbox;
use ceres_executor::Result;
use ceres_std::Vec;

/// Transfer Entry
pub struct TransferEntry {
    to: [u8; 32],
    value: u64,
    data: Vec<u8>,
}

impl Sandbox {
    /// Transfer value to account
    pub fn transfer(&mut self, to: [u8; 32], value: u64) -> Result<()> {
        self.transfers.push(TransferEntry {
            to,
            value,
            data: Vec::new(),
        });
        Ok(())
    }

    /// Call other contract
    pub fn call(&mut self, to: [u8; 32], value: u64, data: Vec<u8>) -> Result<()> {
        self.transfers.push(TransferEntry { to, value, data });

        Ok(())
    }

    pub fn caller(&self) -> [u8; 32] {
        [0; 32]
    }

    pub fn address(&self) -> [u8; 32] {
        [1; 32]
    }

    pub fn balance(&self) -> u64 {
        228
    }

    pub fn value_transferred(&self) -> u64 {
        1337
    }

    pub fn now(&self) -> &u64 {
        &1111
    }

    pub fn minimum_balance(&self) -> u64 {
        16
    }
}
