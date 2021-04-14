//! Ceres CLI Library
use structopt::StructOpt;

mod cmd;
mod result;
mod store;
mod tx;
mod util;

pub use self::{
    result::{Error, Result},
    store::Storage,
    tx::Tx,
};

/// Run CLI
pub fn run() {
    let matches = cmd::Opt::from_args();

    println!("{:?}", matches);
}
