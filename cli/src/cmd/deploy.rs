//! Call a deploy method
use crate::{Result, Tx};
use inkpad_runtime::Runtime;
use inkpad_support::convert::step_hex;

/// Call a deploy method
pub fn exec(rt: &mut Runtime, tx: Tx) -> Result<()> {
    let mut args: Vec<Vec<u8>> = Vec::new();
    for arg in tx.args.iter() {
        args.push(step_hex(arg).ok_or("Arguments should be hex encoded")?);
    }

    rt.deploy(&tx.method, args, Some(tx.tx()?))?;

    println!("\n\tDeploy contract succeed!\n");
    Ok(())
}
