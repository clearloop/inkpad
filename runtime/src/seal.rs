//! Seal functions
//!
//! Update the argument implementation with proc-mc
use crate::{Error, Result, Sandbox, StorageKey};
use alloc::rc::Rc;
use core::cell::RefCell;
use parity_scale_codec::Encode;
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

/// seal_get_storage
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
    sandbox
        .borrow_mut()
        .read_sandbox_memory_into_buf(key_ptr, &mut key)?;
    if let Some(value) = sandbox.borrow().get_storage(&key)? {
        sandbox
            .borrow_mut()
            .write_sandbox_output(out_ptr, out_len_ptr, &value)?;
        Ok(Some(RuntimeValue::I32(ReturnCode::Success as i32)))
    } else {
        Ok(Some(RuntimeValue::I32(ReturnCode::KeyNotFound as i32)))
    }
}

/// seal_set_storage
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

    let mut b = sandbox.borrow_mut();
    if let Some(input) = b.input.take() {
        b.write_sandbox_output(out_ptr, out_len_ptr, &input)?;
        Ok(None)
    } else {
        Err(Error::OutOfBounds)
    }
}

/// *TODO*: replace `1337` with a dynamic value
///
/// seal_value_transferred
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
        .write_sandbox_output(out_ptr, out_len_ptr, &1337.encode())?;
    Ok(None)
}

/// *FIX_ME*
///
/// seal_return
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

    Err(Error::ReturnData {
        flags,
        data: sandbox.borrow().read_sandbox_memory(data_ptr, data_len)?,
    })
}
