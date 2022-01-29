//! Transaction arguments
use crate::{util::decode_addr, Result};
use inkpad_sandbox::Transaction;
use structopt::StructOpt;

/// Transaction arguments
#[derive(Debug, StructOpt)]
pub struct Tx {
    /// Calling method
    pub method: String,
    /// Contract caller
    #[structopt(long)]
    pub caller: Option<String>,
    /// Contract callee
    #[structopt(long)]
    pub address: Option<String>,
    /// contract balance
    #[structopt(long, short)]
    pub balance: Option<u64>,
    /// transferred value
    #[structopt(long, short)]
    pub value_transferred: Option<u64>,
    /// current time
    #[structopt(long, short)]
    pub now: Option<String>,
    /// minimum balance
    #[structopt(long, short)]
    pub minimum_balance: Option<u64>,
    /// Arguments
    #[structopt(long, short, name = "string,")]
    pub args: Vec<String>,
}

impl Tx {
    /// convert to `inkpad_sandbox::Transaction`
    pub fn tx(&self) -> Result<Transaction> {
        Ok(Transaction {
            caller: if let Some(caller) = self.caller.clone() {
                decode_addr(&caller)?
            } else {
                Default::default()
            },
            address: if let Some(addr) = self.address.clone() {
                decode_addr(&addr)?
            } else {
                Default::default()
            },
            balance: self.balance.unwrap_or(0),
            value_transferred: self.value_transferred.unwrap_or(0),
            // TODO: a format of time
            now: Default::default(),
            minimum_balance: self.minimum_balance.unwrap_or_default(),
        })
    }
}
