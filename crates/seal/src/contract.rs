//! Contract interfaces
use crate::derive::Host;
use inkpad_derive::host;
use inkpad_executor::{derive::Value, Error, Result};
use inkpad_sandbox::Sandbox;
use parity_scale_codec::Encode;

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
pub fn seal_tombstone_deposit(out_ptr: u32, out_len_ptr: u32) -> Result<Option<Value>> {
    sandbox.write_sandbox_output(out_ptr, out_len_ptr, &sandbox.tombstone_deposit())?;
    Ok(None)
}

/// Set rent allowance of the contract
///
/// - value_ptr: a pointer to the buffer with value, how much to allow for rent
///   Should be decodable as a `T::Balance`. Traps otherwise.
/// - value_len: length of the value buffer.
#[host(seal0)]
pub fn seal_set_rent_allowance(_value_ptr: u32, _value_len: u32) -> Result<Option<Value>> {
    Ok(None)
}

/// Set rent allowance of the contract
///
/// - value_ptr: a pointer to the buffer with value, how much to allow for rent
///   Should be decodable as a `T::Balance`. Traps otherwise.
/// - value_len: length of the value buffer.
#[host(seal1)]
pub fn seal_set_rent_allowance(_value_ptr: u32) -> Result<Option<Value>> {
    Ok(None)
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
pub fn seal_rent_allowance(out_ptr: u32, out_len_ptr: u32) -> Result<Option<Value>> {
    sandbox.write_sandbox_output(out_ptr, out_len_ptr, &sandbox.rent_allowance())?;
    Ok(None)
}

// Stores the contract deposit into the supplied buffer.
//
// The value is stored to linear memory at the address pointed to by `out_ptr`.
// `out_len_ptr` must point to a u32 value that describes the available space at
// `out_ptr`. This call overwrites it with the size of the value. If the available
// space at `out_ptr` is less than the size of the value a trap is triggered.
//
// The data is encoded as T::Balance.
//
// # Note
//
// The contract deposit is on top of the existential deposit. The sum
// is commonly referred as subsistence threshold in code. No contract initiated
// balance transfer can go below this threshold.
#[host(seal0)]
pub fn seal_contract_deposit(out_ptr: u32, out_len_ptr: u32) -> Result<Option<Value>> {
    sandbox.write_sandbox_output(out_ptr, out_len_ptr, &sandbox.ext.contract_deposit.encode())?;
    Ok(None)
}
