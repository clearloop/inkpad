//! Ceres CLI Library
use structopt::StructOpt;

mod cmd;
mod result;
mod store;
mod tx;
pub mod util;

use self::cmd::{Command, Opt};
pub use self::{
    result::{Error, Result},
    store::Storage,
    tx::Tx,
};

/// Run CLI
pub fn run() -> Result<()> {
    let opt = Opt::from_args();
    let mut store = Storage::new()?;
    let mut rt = store.rt(&opt.contract.unwrap_or_else(|| "".to_string()))?;

    match opt.command {
        Command::List => cmd::list::exec(&store)?,
        Command::Info => cmd::info::exec(&rt)?,
        Command::Deploy(tx) => cmd::deploy::exec(&mut rt, tx)?,
        Command::Call(tx) => cmd::call::exec(&mut rt, tx)?,
    }

    store.0.flush()?;
    Ok(())
}
