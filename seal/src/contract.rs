//! Contract interfaces
use crate::derive::Host;
use ceres_derive::host;
use ceres_executor::{
    derive::{ReturnValue, Value},
    Error, Result,
};
use ceres_sandbox::Sandbox;

/// Stores the tombstone deposit into the supplied buffer.
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
/// The tombstone deposit is on top of the existential deposit. So in order for
/// a contract to leave a tombstone the balance of the contract must not go
/// below the sum of existential deposit and the tombstone deposit. The sum
/// is commonly referred as subsistence threshold in code.
#[host(seal0)]
pub fn seal_tombstone_deposit(out_ptr: u32, out_len_ptr: u32) -> Result<ReturnValue> {
    sandbox.write_sandbox_output(out_ptr, out_len_ptr, &sandbox.tombstone_deposit())?;
    Ok(ReturnValue::Unit)
}

/// Set rent allowance of the contract
///
/// - value_ptr: a pointer to the buffer with value, how much to allow for rent
///   Should be decodable as a `T::Balance`. Traps otherwise.
/// - value_len: length of the value buffer.
#[host(seal0)]
pub fn seal_set_rent_allowance(value_ptr: u32, value_len: u32) -> Result<ReturnValue> {
    let value = sandbox.read_sandbox_memory_as(value_ptr, value_len)?;
    sandbox.set_rent_allowance(value);
    Ok(ReturnValue::Unit)
}

/// Stores the rent allowance into the supplied buffer.
///
/// The value is stored to linear memory at the address pointed to by `out_ptr`.
/// `out_len_ptr` must point to a u32 value that describes the available space at
/// `out_ptr`. This call overwrites it with the size of the value. If the available
/// space at `out_ptr` is less than the size of the value a trap is triggered.
///
/// The data is encoded as T::Balance.
#[host(seal0)]
pub fn seal_rent_allowance(out_ptr: u32, out_len_ptr: u32) -> Result<ReturnValue> {
    sandbox.write_sandbox_output(out_ptr, out_len_ptr, &sandbox.rent_allowance())?;
    Ok(ReturnValue::Unit)
}
