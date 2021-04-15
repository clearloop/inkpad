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
    let store = Storage::new()?;
    let rt = store.rt(&opt.contract)?;

    match opt.command {
        Command::Info => cmd::list::exec(&rt)?,
        Command::List => cmd::info::exec(&rt)?,
        Command::Deploy(tx) => cmd::deploy::exec(&rt, tx)?,
        Command::Call(tx) => cmd::call::exec(&rt, tx)?,
    }

    Ok(())
}
