//! Command
use crate::Tx;
use structopt::StructOpt;

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
