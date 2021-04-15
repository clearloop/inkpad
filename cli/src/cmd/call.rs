//! Call a call method
use crate::{Result, Tx};
use ceres_runtime::Runtime;

/// Call a call method
pub fn exec(rt: &mut Runtime, tx: Tx) -> Result<()> {
    let mut args: Vec<&str> = Vec::new();
    tx.args.iter().for_each(|s| args.push(s.as_str()));
    println!("result: {:?}", rt.call(&tx.method, &args, Some(tx.tx()?))?);
    rt.flush()?;
    Ok(())
}
