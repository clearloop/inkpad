//! Ceres CLI Library
use std::path::PathBuf;
use structopt::StructOpt;

mod call;
mod deploy;
mod info;

/// Ceres command tool
#[derive(Debug, StructOpt)]
#[structopt(name = "ceres")]
pub enum Opt {
    /// Prints info of *.contract
    Info {
        /// The path of ink! contrainct ( *.contract )
        #[structopt(long, short)]
        path: Option<PathBuf>,
        /// Code hash (hex string)
        #[structopt(long, short)]
        code_hash: Option<String>,
    },
    /// Call a deploy method
    Deploy {
        /// The path of ink! contrainct ( *.contract )
        #[structopt(long, short)]
        path: Option<PathBuf>,
        /// Code hash
        #[structopt(long, short)]
        code_hash: Option<String>,
        /// Arguments
        args: Vec<String>,
        /// Transaction config
        #[structopt(long, short)]
        tx: bool,
    },
    /// Call a call method
    Call {
        /// The path of ink! contrainct ( *.contract )
        #[structopt(long, short)]
        path: Option<PathBuf>,
        /// Code hash
        #[structopt(long, short)]
        code_hash: Option<String>,
        /// Arguments
        args: Vec<String>,
        /// Transaction config
        #[structopt(long, short)]
        tx: bool,
    },
}

/// Run CLI
pub fn run() {
    let matches = Opt::from_args();

    println!("{:?}", matches);
}
