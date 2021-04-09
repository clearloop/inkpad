//! Chain interfaces
use crate::derive::Host;
use ceres_derive::host;
use ceres_executor::{
    derive::{ReturnValue, Value},
    Error, Result,
};
use ceres_sandbox::Sandbox;

/// Define a function `fn init_env<E: Ext>() -> HostFunctionSet<E>` that returns
/// a function set which can be imported by an executed contract.
#[host(seal0)]
pub fn gas(_amount: u32) -> Result<ReturnValue> {
    Ok(ReturnValue::Unit)
}

/// Stores the current block number of the current contract into the supplied buffer.
///
/// The value is stored to linear memory at the address pointed to by `out_ptr`.
/// `out_len_ptr` must point to a u32 value that describes the available space at
/// `out_ptr`. This call overwrites it with the size of the value. If the available
/// space at `out_ptr` is less than the size of the value a trap is triggered.
#[host(seal0)]
pub fn block_number(out_ptr: u32, out_len_ptr: u32) -> Result<ReturnValue> {
    sandbox.write_sandbox_output(out_ptr, out_len_ptr, &sandbox.block_number())?;
    Ok(ReturnValue::Unit)
}
