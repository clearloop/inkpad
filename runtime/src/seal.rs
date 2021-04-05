//! Seal functions
//!
//! Update the argument implementation with proc-mc
use crate::{Error, Result, Sandbox, StorageKey};
use alloc::rc::Rc;
use core::cell::RefCell;
use wasmi::{RuntimeArgs, RuntimeValue};

/// Custom return code for wasm functions
#[repr(u32)]
pub enum ReturnCode {
    /// API call successful.
    Success = 0,
    /// The called function trapped and has its state changes reverted.
    /// In this case no output buffer is returned.
    CalleeTrapped = 1,
    /// The called function ran to completion but decided to revert its state.
    /// An output buffer is returned when one was supplied.
    CalleeReverted = 2,
    /// The passed key does not exist in storage.
    KeyNotFound = 3,
    /// Transfer failed because it would have brought the sender's total balance below the
    /// subsistence threshold.
    BelowSubsistenceThreshold = 4,
    /// Transfer failed for other reasons. Most probably reserved or locked balance of the
    /// sender prevents the transfer.
    TransferFailed = 5,
    /// The newly created contract is below the subsistence threshold after executing
    /// its constructor.
    NewContractNotFunded = 6,
    /// No code could be found at the supplied code hash.
    CodeNotFound = 7,
    /// The contract that was called is either no contract at all (a plain account)
    /// or is a tombstone.
    NotCallable = 8,
}

/// Retrieve the value under the given key from storage.
pub fn seal_get_storage(
    sandbox: Rc<RefCell<Sandbox>>,
    args: RuntimeArgs,
) -> Result<Option<RuntimeValue>> {
    let [key_ptr, out_ptr, out_len_ptr] = [
        args.nth_value_checked(0)?
            .try_into()
            .ok_or(Error::DecodeRuntimeValueFailed)?,
        args.nth_value_checked(1)?
            .try_into()
            .ok_or(Error::DecodeRuntimeValueFailed)?,
        args.nth_value_checked(2)?
            .try_into()
            .ok_or(Error::DecodeRuntimeValueFailed)?,
    ];

    let mut key: StorageKey = [0; 32];
    let mut bm = sandbox.borrow_mut();
    bm.read_sandbox_memory_into_buf(key_ptr, &mut key)?;
    if let Some(value) = bm.get_storage(&key)? {
        bm.write_sandbox_output(out_ptr, out_len_ptr, &value)?;
        Ok(Some(RuntimeValue::I32(ReturnCode::Success as i32)))
    } else {
        Ok(Some(RuntimeValue::I32(ReturnCode::KeyNotFound as i32)))
    }
}

/// Set the value at the given key in the contract storage.
///
/// The value length must not exceed the maximum defined by the contracts module parameters.
/// Storing an empty value is disallowed.
pub fn seal_set_storage(
    sandbox: Rc<RefCell<Sandbox>>,
    args: RuntimeArgs,
) -> Result<Option<RuntimeValue>> {
    let [key_ptr, value_ptr, value_len] = [
        args.nth_value_checked(0)?
            .try_into()
            .ok_or(Error::DecodeRuntimeValueFailed)?,
        args.nth_value_checked(1)?
            .try_into()
            .ok_or(Error::DecodeRuntimeValueFailed)?,
        args.nth_value_checked(2)?
            .try_into()
            .ok_or(Error::DecodeRuntimeValueFailed)?,
    ];

    let mut key: StorageKey = [0; 32];
    sandbox
        .borrow()
        .read_sandbox_memory_into_buf(key_ptr, &mut key)?;
    let value = sandbox.borrow().read_sandbox_memory(value_ptr, value_len)?;
    sandbox.borrow_mut().set_storage(&key, value)?;

    Ok(None)
}

/// seal_input
pub fn seal_input(
    sandbox: Rc<RefCell<Sandbox>>,
    args: RuntimeArgs,
) -> Result<Option<RuntimeValue>> {
    let [out_ptr, out_len_ptr] = [
        args.nth_value_checked(0)?
            .try_into()
            .ok_or(Error::DecodeRuntimeValueFailed)?,
        args.nth_value_checked(1)?
            .try_into()
            .ok_or(Error::DecodeRuntimeValueFailed)?,
    ];

    let mut bm = sandbox.borrow_mut();
    if let Some(input) = bm.input.take() {
        bm.write_sandbox_output(out_ptr, out_len_ptr, &input)?;
        Ok(None)
    } else {
        Err(Error::OutOfBounds)
    }
}

/// *TODO*: replace `1337` with a dynamic value
///
/// seal_value_transferred
///
/// Stores the value transferred along with this call or as endowment into
/// the supplied buffer.
///
/// AtLeat32Bits
pub fn seal_value_transferred(
    sandbox: Rc<RefCell<Sandbox>>,
    args: RuntimeArgs,
) -> Result<Option<RuntimeValue>> {
    let [out_ptr, out_len_ptr] = [
        args.nth_value_checked(0)?
            .try_into()
            .ok_or(Error::DecodeRuntimeValueFailed)?,
        args.nth_value_checked(1)?
            .try_into()
            .ok_or(Error::DecodeRuntimeValueFailed)?,
    ];

    sandbox
        .borrow_mut()
        .write_sandbox_output(out_ptr, out_len_ptr, &[0x00; 32])?;
    Ok(None)
}

/// Cease contract execution and save a data buffer as a result of the execution.
///
/// This function never returns as it stops execution of the caller.
/// This is the only way to return a data buffer to the caller. Returning from
/// execution without calling this function is equivalent to calling:
///
/// The flags argument is a bitfield that can be used to signal special return
pub fn seal_return(
    sandbox: Rc<RefCell<Sandbox>>,
    args: RuntimeArgs,
) -> Result<Option<RuntimeValue>> {
    let [flags, data_ptr, data_len] = [
        args.nth_value_checked(0)?
            .try_into()
            .ok_or(Error::DecodeRuntimeValueFailed)?,
        args.nth_value_checked(1)?
            .try_into()
            .ok_or(Error::DecodeRuntimeValueFailed)?,
        args.nth_value_checked(2)?
            .try_into()
            .ok_or(Error::DecodeRuntimeValueFailed)?,
    ];

    let mut bm = sandbox.borrow_mut();
    let data = bm.read_sandbox_memory(data_ptr, data_len)?;
    if flags == 0 {
        bm.ret = Some(data.clone());
    }

    drop(bm);
    Err(Error::ReturnData { flags, data })
}
