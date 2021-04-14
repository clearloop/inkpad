//! Ceres CLI Library
use ceres_sandbox::Transaction;
use structopt::StructOpt;
use thiserror::Error;

mod call;
mod deploy;
mod info;

/// Ceres CLI Error
#[derive(Error, Debug)]
pub enum Error {
    #[error("Could not parse command `{0}`")]
    CouldNotParseCommand(String),
    #[error("Decode ss58 address `{0}`")]
    DecodeAddressFailed(String),
}

/// Ceres result
pub type Result<T> = core::result::Result<T, Error>;

/// Decode address to [u8; 32]
///
/// ```
/// use ceres_cli::decode_addr;
///
/// assert!(
///   ceres_cli::decode_addr(
///     "0x46da65a1be5b49d639a934e27b8a773c3fc2540f488df4c2afb9880ee34a6346"
///   ).is_ok()
/// );
/// ```
pub fn decode_addr(addr: &str) -> Result<[u8; 32]> {
    let mut slice = hex::decode(if addr.starts_with("0x") {
        &addr[2..]
    } else {
        &addr[..]
    })
    .map_err(|_| Error::DecodeAddressFailed(addr.into()))?;
    if slice.len() != 32 {
        return Err(Error::DecodeAddressFailed(addr.into()));
    }

    let mut res: [u8; 32] = [0; 32];
    res.copy_from_slice(&mut slice);
    Ok(res)
}

/// Transaction arguments
#[derive(Debug, StructOpt)]
pub struct Tx {
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
}

impl Tx {
    fn tx(self) -> Result<Transaction> {
        Ok(Transaction {
            caller: if let Some(caller) = self.caller {
                decode_addr(&caller)?
            } else {
                Default::default()
            },
            address: if let Some(addr) = self.address {
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

/// Ceres command tool
#[derive(Debug, StructOpt)]
#[structopt(name = "ceres")]
pub enum Opt {
    /// Prints info of *.contract
    Info {
        /// Target contract
        #[structopt(long, short, name = "*.contract | name | code-hash")]
        contract: String,
    },
    /// Call a deploy method
    Deploy {
        /// Target contract
        #[structopt(long, short, name = "*.contract | name | code-hash")]
        contract: String,
        /// Arguments
        #[structopt(long, short)]
        args: Vec<String>,
        /// Transaction config
        #[structopt(flatten)]
        tx: Tx,
    },
    /// Call a call method
    Call {
        /// Target contract
        #[structopt(long, short, name = "*.contract | name | code-hash")]
        contract: String,
        /// Arguments
        args: Vec<String>,
        /// Transaction config
        #[structopt(flatten)]
        tx: Tx,
    },
}

/// Run CLI
pub fn run() {
    let matches = Opt::from_args();

    println!("{:?}", matches);
}
