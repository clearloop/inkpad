//! Storage functions
use crate::derive::Host;
use ceres_derive::host;
use ceres_executor::{derive::Value, Error, Result, ReturnCode};
use ceres_sandbox::{Sandbox, StorageKey};
use ceres_std::vec;

/// Retrieve the value under the given key from storage.
#[host(seal0)]
pub fn seal_get_storage(key_ptr: u32, out_ptr: u32, out_len_ptr: u32) -> Result<Option<Value>> {
    let mut key: StorageKey = [0; 32];
    sandbox.read_sandbox_memory_into_buf(key_ptr, &mut key)?;
    if let Some(value) = sandbox.get_storage(&key)? {
        sandbox.write_sandbox_output(out_ptr, out_len_ptr, &value)?;
        Ok(Some(Value::I32(ReturnCode::Success as i32)))
    } else {
        Err(Error::ExecuteFailed(ReturnCode::KeyNotFound))
    }
}

/// Clear the value at the given key in the contract storage.
#[host(seal0)]
pub fn seal_clear_storage(key_ptr: u32) -> Result<Option<Value>> {
    let mut key: StorageKey = [0; 32];
    sandbox.read_sandbox_memory_into_buf(key_ptr, &mut key)?;
    if sandbox.set_storage(key, vec![]).is_ok() {
        Ok(None)
    } else {
        Err(Error::SetStorageFailed)
    }
}

/// Set the value at the given key in the contract storage.
///
/// The value length must not exceed the maximum defined by the contracts module parameters.
/// Storing an empty value is disallowed.
#[host(seal0)]
pub fn seal_set_storage(key_ptr: u32, value_ptr: u32, value_len: u32) -> Result<Option<Value>> {
    let mut key: StorageKey = [0; 32];
    sandbox.read_sandbox_memory_into_buf(key_ptr, &mut key)?;
    let value = sandbox.read_sandbox_memory(value_ptr, value_len)?;

    sandbox.set_storage(key, value)?;

    Ok(None)
}
