//! Chain state
use crate::util::al;
use ceres_std::Vec;
use parity_scale_codec::{Decode, Encode};
use serde::{Deserialize, Serialize};

/// Chain State
#[derive(Clone, Encode, Decode, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transaction {
    caller: [u8; 32],
    address: [u8; 32],
    balance: u64,
    value_transferred: u64,
    now: [u8; 32],
    minimum_balance: u64,
}

impl Default for Transaction {
    fn default() -> Transaction {
        Transaction {
            caller: [0; 32],
            address: [0; 32],
            balance: 42,
            value_transferred: 0,
            now: [0; 32],
            minimum_balance: 0,
        }
    }
}

impl Transaction {
    pub fn caller(&self) -> [u8; 32] {
        self.caller
    }

    pub fn set_caller(&mut self, caller: [u8; 32]) {
        self.caller = caller;
    }

    pub fn address(&self) -> [u8; 32] {
        self.address
    }

    pub fn set_address(&mut self, address: [u8; 32]) {
        self.address = address;
    }

    pub fn balance(&self) -> Vec<u8> {
        al(self.balance.encode(), 16)
    }

    pub fn set_balance(&mut self, balance: u64) {
        self.balance = balance;
    }

    pub fn value_transferred(&self) -> Vec<u8> {
        al(self.value_transferred.encode(), 16)
    }

    pub fn set_value_transferred(&mut self, value_transferred: u64) {
        self.value_transferred = value_transferred;
    }

    pub fn now(&self) -> [u8; 32] {
        self.now
    }

    pub fn set_now(&mut self, now: [u8; 32]) {
        self.now = now;
    }

    pub fn minimum_balance(&self) -> Vec<u8> {
        al(self.minimum_balance.encode(), 16)
    }

    pub fn set_minimum_balance(&mut self, minimum_balance: u64) {
        self.minimum_balance = minimum_balance;
    }
}
