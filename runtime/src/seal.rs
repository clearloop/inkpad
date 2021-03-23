//! Seal functions
use crate::{Error, Result, Sandbox, StorageKey};
use parity_scale_codec::Encode;

pub enum ReturnCode {
    /// API call successful.
    Success,
    /// The called function trapped and has its state changes reverted.
    /// In this case no output buffer is returned.
    CalleeTrapped,
    /// The called function ran to completion but decided to revert its state.
    /// An output buffer is returned when one was supplied.
    CalleeReverted,
    /// The passed key does not exist in storage.
    KeyNotFound,
    /// Transfer failed because it would have brought the sender's total balance below the
    /// subsistence threshold.
    BelowSubsistenceThreshold,
    /// Transfer failed for other reasons. Most probably reserved or locked balance of the
    /// sender prevents the transfer.
    TransferFailed,
    /// The newly created contract is below the subsistence threshold after executing
    /// its constructor.
    NewContractNotFunded,
    /// No code could be found at the supplied code hash.
    CodeNotFound,
    /// The contract that was called is either no contract at all (a plain account)
    /// or is a tombstone.
    NotCallable,
}

/// seal_get_storage
pub fn seal_get_storage(
    sandbox: &mut Sandbox,
    key_ptr: u32,
    out_ptr: u32,
    out_len_ptr: u32,
) -> Result<ReturnCode> {
    let mut key: StorageKey = [0; 32];
    sandbox.read_sandbox_memory_into_buf(key_ptr, &mut key)?;
    if let Some(value) = sandbox.get_storage(&key)? {
        sandbox.write_sandbox_output(out_ptr, out_len_ptr, &value)?;
        Ok(ReturnCode::Success)
    } else {
        Ok(ReturnCode::KeyNotFound)
    }
}

/// seal_set_storage
pub fn seal_set_storage(
    sandbox: &mut Sandbox,
    key_ptr: u32,
    value_ptr: u32,
    value_len: u32,
) -> Result<()> {
    let mut key: StorageKey = [0; 32];
    sandbox.read_sandbox_memory_into_buf(key_ptr, &mut key)?;
    let value = sandbox.read_sandbox_memory(value_ptr, value_len)?;
    sandbox.set_storage(&key, value)?;

    Ok(())
}

/// seal_input
pub fn seal_input(sandbox: &mut Sandbox, out_ptr: u32, out_len_ptr: u32) -> Result<()> {
    if let Some(input) = sandbox.input_data.take() {
        sandbox.write_sandbox_output(out_ptr, out_len_ptr, &input)?;
        Ok(())
    } else {
        // Modify me
        Err(Error::OutOfBounds)
    }
}

/// *TODO*: replace `1337` with a dynamic value
///
/// seal_value_transferred
pub fn seal_value_transferred(sandbox: &mut Sandbox, out_ptr: u32, out_len_ptr: u32) -> Result<()> {
    Ok(sandbox.write_sandbox_output(out_ptr, out_len_ptr, &1337.encode())?)
}

/// *FIX_ME*
///
/// seal_return
pub fn seal_return(sandbox: &Sandbox, flags: u32, data_ptr: u32, data_len: u32) -> Result<()> {
    Ok(())
}
