//! Chain interfaces
use crate::derive::Host;
use ceres_derive::host;
use ceres_executor::{derive::Value, Error, Result};
use ceres_sandbox::Sandbox;

/// Define a function `fn init_env<E: Ext>() -> HostFunctionSet<E>` that returns
/// a function set which can be imported by an executed contract.
#[host(seal0)]
pub fn gas(_amount: u32) -> Result<Value> {
    Ok(Value::F32(0))
}

/// Stores the current block number of the current contract into the supplied buffer.
///
/// The value is stored to linear memory at the address pointed to by `out_ptr`.
/// `out_len_ptr` must point to a u32 value that describes the available space at
/// `out_ptr`. This call overwrites it with the size of the value. If the available
/// space at `out_ptr` is less than the size of the value a trap is triggered.
#[host(seal0)]
pub fn block_number(out_ptr: u32, out_len_ptr: u32) -> Result<Value> {
    sandbox.write_sandbox_output(out_ptr, out_len_ptr, &sandbox.block_number())?;
    Ok(Value::F32(0))
}

/// Stores the price for the specified amount of gas into the supplied buffer.
///
/// The value is stored to linear memory at the address pointed to by `out_ptr`.
/// `out_len_ptr` must point to a u32 value that describes the available space at
/// `out_ptr`. This call overwrites it with the size of the value. If the available
/// space at `out_ptr` is less than the size of the value a trap is triggered.
///
/// The data is encoded as T::Balance.
///
/// # Note
///
/// It is recommended to avoid specifying very small values for `gas` as the prices for a single
/// gas can be smaller than one.
#[host(seal0)]
pub fn seal_weight_to_fee(gas: u64, out_ptr: u32, out_len_ptr: u32) -> Result<Value> {
    sandbox.write_sandbox_output(out_ptr, out_len_ptr, &sandbox.get_weight_price(gas))?;
    Ok(Value::F32(0))
}

/// Stores the amount of gas left into the supplied buffer.
///
/// The value is stored to linear memory at the address pointed to by `out_ptr`.
/// `out_len_ptr` must point to a u32 value that describes the available space at
/// `out_ptr`. This call overwrites it with the size of the value. If the available
/// space at `out_ptr` is less than the size of the value a trap is triggered.
///
/// The data is encoded as Gas.
#[host(seal0)]
pub fn seal_gas_left(out_ptr: u32, out_len_ptr: u32) -> Result<Value> {
    sandbox.write_sandbox_output(
        out_ptr,
        out_len_ptr,
        &sandbox.ext.gas_meter.gas_left_bytes(),
    )?;
    Ok(Value::F32(0))
}
