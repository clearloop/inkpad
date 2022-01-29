//! Restore interface
use crate::derive::Host;
use inkpad_derive::host;
use inkpad_executor::{derive::Value, Error, Result};
use inkpad_sandbox::Sandbox;

// Was used to restore the given destination contract sacrificing the caller.
//
// # Note
//
// The state rent functionality was removed. This is stub only exists for
// backwards compatiblity
#[host(seal0)]
pub fn seal_restore_to(
    _dest_ptr: u32,
    _dest_len: u32,
    _code_hash_ptr: u32,
    _code_hash_len: u32,
    _rent_allowance_ptr: u32,
    _rent_allowance_len: u32,
    _delta_ptr: u32,
    _delta_count: u32,
) -> Result<Option<Value>> {
    Ok(None)
}

// Was used to restore the given destination contract sacrificing the caller.
//
// # Note
//
// The state rent functionality was removed. This is stub only exists for
// backwards compatiblity
#[host(seal1)]
pub fn seal_restore_to(
    _dest_ptr: u32,
    _code_hash_ptr: u32,
    _rent_allowance_ptr: u32,
    _delta_ptr: u32,
    _delta_count: u32,
) -> Result<Option<Value>> {
    Ok(None)
}
