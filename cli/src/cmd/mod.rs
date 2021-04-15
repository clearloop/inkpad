//! Command
use crate::Tx;
use structopt::StructOpt;

pub mod call;
pub mod deploy;
pub mod info;
pub mod list;

#[derive(Debug, StructOpt)]
#[structopt(name = "ceres")]
pub struct Opt {
    /// Target contract
    #[structopt(
        name = "*.contract | name | code-hash",
        help = "If empty, ceres will load the last contract which has been executed"
    )]
    pub contract: String,
    #[structopt(subcommand)]
    pub command: Command,
}

/// Ceres command tool
#[derive(Debug, StructOpt)]
pub enum Command {
    /// Lists all contracts
    List,
    /// Prints info of *.contract
    Info,
    /// Calls a deploy method
    Deploy(Tx),
    /// Calls a call method
    Call(Tx),
}
