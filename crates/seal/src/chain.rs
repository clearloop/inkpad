//! Chain interfaces
use crate::derive::Host;
use inkpad_derive::host;
use inkpad_executor::{derive::Value, Error, Result};
use inkpad_sandbox::Sandbox;

/// Define a function `fn init_env<E: Ext>() -> HostFunctionSet<E>` that returns
/// a function set which can be imported by an executed contract.
#[host(seal0)]
pub fn gas(_amount: u32) -> Result<Option<Value>> {
    Ok(None)
}

/// Stores the current block number of the current contract into the supplied buffer.
///
/// The value is stored to linear memory at the address pointed to by `out_ptr`.
/// `out_len_ptr` must point to a u32 value that describes the available space at
/// `out_ptr`. This call overwrites it with the size of the value. If the available
/// space at `out_ptr` is less than the size of the value a trap is triggered.
#[host(seal0)]
pub fn block_number(out_ptr: u32, out_len_ptr: u32) -> Result<Option<Value>> {
    sandbox.write_sandbox_output(out_ptr, out_len_ptr, &sandbox.block_number())?;
    Ok(None)
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
pub fn seal_weight_to_fee(gas: u64, out_ptr: u32, out_len_ptr: u32) -> Result<Option<Value>> {
    sandbox.write_sandbox_output(out_ptr, out_len_ptr, &sandbox.get_weight_price(gas))?;
    Ok(None)
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
pub fn seal_gas_left(out_ptr: u32, out_len_ptr: u32) -> Result<Option<Value>> {
    sandbox.write_sandbox_output(
        out_ptr,
        out_len_ptr,
        &sandbox.ext.gas_meter.gas_left_bytes(),
    )?;
    Ok(None)
}

// Call into the chain extension provided by the chain if any.
//
// Handling of the input values is up to the specific chain extension and so is the
// return value. The extension can decide to use the inputs as primitive inputs or as
// in/out arguments by interpreting them as pointers. Any caller of this function
// must therefore coordinate with the chain that it targets.
//
// # Note
//
// If no chain extension exists the contract will trap with the `NoChainExtension`
// module error.
#[host(seal0)]
pub fn seal_call_chain_extension(
    _func_id: u32,
    _input_ptr: u32,
    _input_len: u32,
    _output_ptr: u32,
    _output_len_ptr: u32,
) -> Result<Option<Value>> {
    unimplemented!()
}

// Call some dispatchable of the runtime.
//
// This function decodes the passed in data as the overarching `Call` type of the
// runtime and dispatches it. The weight as specified in the runtime is charged
// from the gas meter. Any weight refunds made by the dispatchable are considered.
//
// The filter specified by `Config::CallFilter` is attached to the origin of
// the dispatched call.
//
// # Parameters
//
// - `input_ptr`: the pointer into the linear memory where the input data is placed.
// - `input_len`: the length of the input data in bytes.
//
// # Return Value
//
// Returns `ReturnCode::Success` when the dispatchable was succesfully executed and
// returned `Ok`. When the dispatchable was exeuted but returned an error
// `ReturnCode::CallRuntimeReturnedError` is returned. The full error is not
// provided because it is not guaranteed to be stable.
//
// # Comparison with `ChainExtension`
//
// Just as a chain extension this API allows the runtime to extend the functionality
// of contracts. While making use of this function is generelly easier it cannot be
// used in call cases. Consider writing a chain extension if you need to do perform
// one of the following tasks:
//
// - Return data.
// - Provide functionality **exclusively** to contracts.
// - Provide custom weights.
// - Avoid the need to keep the `Call` data structure stable.
//
// # Unstable
//
// This function is unstable and subject to change (or removal) in the future. Do not
// deploy a contract using it to a production chain.
#[host(__unstable__)]
pub fn seal_call_runtime(_call_ptr: u32, _call_len: u32) -> Result<Option<Value>> {
    unimplemented!()
}
