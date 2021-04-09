//! Transfer Entry
use crate::{util::al, Sandbox};
use ceres_executor::Result;
use ceres_std::Vec;
use funty::{AtLeast16, IsNumber};
use parity_scale_codec::Encode;

/// Transfer Entry
pub struct TransferEntry {
    to: [u8; 32],
    value: u64,
    data: Vec<u8>,
}

// type Balance: IsNumber + AtLeast16 = u64;

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

    pub fn balance(&self) -> Vec<u8> {
        al(42.encode(), 16)
    }

    pub fn value_transferred(&self) -> Vec<u8> {
        al(0.encode(), 16)
    }

    pub fn now(&self) -> [u8; 32] {
        [0; 32]
    }

    pub fn minimum_balance(&self) -> Vec<u8> {
        al(0.encode(), 16)
    }
}
