//! Call a call method
use crate::{Result, Tx};
use ceres_runtime::{util::step_hex, Runtime};

/// Call a call method
pub fn exec(rt: &mut Runtime, tx: Tx) -> Result<()> {
    let mut args: Vec<Vec<u8>> = Vec::new();
    for arg in tx.args.iter() {
        args.push(step_hex(arg)?);
    }

    println!("result: {:?}", rt.call(&tx.method, args, Some(tx.tx()?))?);
    Ok(())
}
