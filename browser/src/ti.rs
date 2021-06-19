//! Type interfaces
use crate::result::err_check;
use ceres_sandbox::Transaction as Inner;
use serde::{Deserialize, Serialize};

/// vector to hash
fn hash(mut src: Vec<u8>) -> [u8; 32] {
    let mut dest: [u8; 32] = [0; 32];
    dest.copy_from_slice(&mut src);
    dest
}

/**
 * Contract transaction
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub caller: String,
    pub address: String,
    pub balance: u64,
    pub value_transferred: u64,
    pub now: String,
    pub minimum_balance: u64,
}

impl From<Transaction> for Inner {
    fn from(tx: Transaction) -> Inner {
        Inner {
            caller: hash(err_check(hex::decode(tx.caller))),
            address: hash(err_check(hex::decode(tx.address))),
            balance: tx.balance,
            value_transferred: tx.value_transferred,
            now: hash(err_check(hex::decode(tx.now))),
            minimum_balance: tx.minimum_balance,
        }
    }
}
