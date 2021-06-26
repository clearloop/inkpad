//! Transfer Entry
use crate::Sandbox;
use ceres_executor::{Result, ReturnData, ReturnFlags};
use ceres_std::Vec;

/// Transfer Entry
pub struct TransferEntry {
    pub to: [u8; 32],
    pub value: u64,
    pub data: Vec<u8>,
}

impl Sandbox {
    /// Transfer value to account
    pub fn transfer(&mut self, to: [u8; 32], value: u64) -> Result<()> {
        self.ext.transfers.push(TransferEntry {
            to,
            value,
            data: Vec::new(),
        });
        Ok(())
    }

    /// Call other contract
    pub fn call(&mut self, to: [u8; 32], value: u64, data: Vec<u8>) -> Result<ReturnData> {
        self.ext.transfers.push(TransferEntry { to, value, data });

        Ok(ReturnData {
            flags: ReturnFlags::empty(),
            data: Vec::new(),
        })
    }

    pub fn caller(&self) -> [u8; 32] {
        self.tx.caller()
    }

    pub fn address(&self) -> [u8; 32] {
        self.tx.address()
    }

    pub fn balance(&self) -> Vec<u8> {
        self.tx.balance()
    }

    pub fn value_transferred(&self) -> Vec<u8> {
        self.tx.value_transferred()
    }

    pub fn now(&self) -> [u8; 32] {
        self.tx.now()
    }

    pub fn minimum_balance(&self) -> Vec<u8> {
        self.tx.minimum_balance()
    }
}
