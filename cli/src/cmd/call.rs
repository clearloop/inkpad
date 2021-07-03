//! Call a call method
use crate::{Result, Tx};
use ceres_runtime::Runtime;
use ceres_support::convert::step_hex;

/// Call a call method
pub fn exec(rt: &mut Runtime, tx: Tx) -> Result<()> {
    let mut args: Vec<Vec<u8>> = Vec::new();
    for arg in tx.args.iter() {
        args.push(step_hex(arg).ok_or("argument should be hex encoded")?);
    }

    println!(
        "\n\tresult: {:?}\n",
        rt.call(&tx.method, args, Some(tx.tx()?))?
    );
    Ok(())
}
