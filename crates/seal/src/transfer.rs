use crate::derive::Host;
use ceres_derive::host;
use ceres_executor::{derive::Value, Error, Result};
use ceres_sandbox::Sandbox;

/// *TODO*: replace `1337` with a dynamic value
///
/// seal_value_transferred
///
/// Stores the value transferred along with this call or as endowment into
/// the supplied buffer.
///
/// AtLeat32Bits
#[host(seal0)]
pub fn seal_value_transferred(out_ptr: u32, out_len_ptr: u32) -> Result<Option<Value>> {
    sandbox.write_sandbox_output(out_ptr, out_len_ptr, &sandbox.value_transferred())?;
    Ok(None)
}

/// Transfer some value to another account.
#[host(seal0)]
pub fn seal_transfer(
    account_ptr: u32,
    account_len: u32,
    value_ptr: u32,
    value_len: u32,
) -> Result<Option<Value>> {
    let callee: [u8; 32] = sandbox.read_sandbox_memory_as(account_ptr, account_len)?;
    let value: u64 = sandbox.read_sandbox_memory_as(value_ptr, value_len)?;
    sandbox.transfer(callee, value)?;

    // ReturnCode::Success
    Ok(None)
}

/// Stores the address of the caller into the supplied buffer.
///
/// The value is stored to linear memory at the address pointed to by `out_ptr`.
/// `out_len_ptr` must point to a u32 value that describes the available space at
/// `out_ptr`. This call overwrites it with the size of the value. If the available
/// space at `out_ptr` is less than the size of the value a trap is triggered.
#[host(seal0)]
pub fn seal_caller(out_ptr: u32, out_len_ptr: u32) -> Result<Option<Value>> {
    sandbox.write_sandbox_output(out_ptr, out_len_ptr, &sandbox.caller())?;
    Ok(None)
}

/// Stores the address of the current contract into the supplied buffer.
///
/// The value is stored to linear memory at the address pointed to by `out_ptr`.
/// `out_len_ptr` must point to a u32 value that describes the available space at
/// `out_ptr`. This call overwrites it with the size of the value. If the available
/// space at `out_ptr` is less than the size of the value a trap is triggered.
#[host(seal0)]
pub fn seal_address(out_ptr: u32, out_len_ptr: u32) -> Result<Option<Value>> {
    sandbox.write_sandbox_output(out_ptr, out_len_ptr, &sandbox.address())?;
    Ok(None)
}

/// Stores the balance of the current account into the supplied buffer.
///
/// The value is stored to linear memory at the address pointed to by `out_ptr`.
/// `out_len_ptr` must point to a u32 value that describes the available space at
/// `out_ptr`. This call overwrites it with the size of the value. If the available
/// space at `out_ptr` is less than the size of the value a trap is triggered.
///
/// The data is encoded as T::Balance.
#[host(seal0)]
pub fn seal_balance(out_ptr: u32, out_len_ptr: u32) -> Result<Option<Value>> {
    sandbox.write_sandbox_output(out_ptr, out_len_ptr, &sandbox.balance())?;
    Ok(None)
}

/// Load the latest block timestamp into the supplied buffer
///
/// The value is stored to linear memory at the address pointed to by `out_ptr`.
/// `out_len_ptr` must point to a u32 value that describes the available space at
/// `out_ptr`. This call overwrites it with the size of the value. If the available
/// space at `out_ptr` is less than the size of the value a trap is triggered.
#[host(seal0)]
pub fn seal_now(out_ptr: u32, out_len_ptr: u32) -> Result<Option<Value>> {
    sandbox.write_sandbox_output(out_ptr, out_len_ptr, &sandbox.now())?;
    Ok(None)
}

/// Stores the minimum balance (a.k.a. existential deposit) into the supplied buffer.
///
/// The data is encoded as T::Balance.
#[host(seal0)]
pub fn seal_minimum_balance(out_ptr: u32, out_len_ptr: u32) -> Result<Option<Value>> {
    sandbox.write_sandbox_output(out_ptr, out_len_ptr, &sandbox.minimum_balance())?;
    Ok(None)
}
