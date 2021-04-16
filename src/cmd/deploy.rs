//! Call a deploy method
use crate::{Result, Tx};
use ceres_runtime::Runtime;

/// Call a deploy method
pub fn exec(rt: &mut Runtime, tx: Tx) -> Result<()> {
    let mut args: Vec<&str> = Vec::new();
    tx.args.iter().for_each(|s| args.push(s.as_str()));
    rt.deploy(&tx.method, &args, Some(tx.tx()?))?;
    rt.flush()?;
    println!("Deploy contract succeed!");
    Ok(())
}
