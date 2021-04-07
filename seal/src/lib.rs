#![cfg_attr(not(feature = "std"), no_std)]
use ceres_executor::derive::{ReturnValue, Value};
use ceres_sandbox::{Sandbox, StorageKey};
use ceres_std::Rc;
use core::cell::RefCell;

mod result;
mod ret;

pub use self::{
    result::{Error, Result},
    ret::ReturnCode,
};

/// Retrieve the value under the given key from storage.
pub fn seal_get_storage(
    sandbox: Rc<RefCell<Sandbox>>,
    args: &[Value],
) -> Result<Option<ReturnValue>> {
    if args.len() != 3 {
        return Err(Error::WrongArugmentLength);
    }
    let [key_ptr, out_ptr, out_len_ptr] = [args[0].into(), args[1].into(), args[2].into()];

    let mut key: StorageKey = [0; 32];
    let mut bm = sandbox.borrow_mut();
    bm.read_sandbox_memory_into_buf(key_ptr, &mut key)?;
    if let Some(value) = bm.get_storage(&key)? {
        bm.write_sandbox_output(out_ptr, out_len_ptr, &value)?;
        Ok(Some(Value::I32(ReturnCode::Success as i32).into()))
    } else {
        Ok(Some(Value::I32(ReturnCode::KeyNotFound as i32).into()))
    }
}

/// Set the value at the given key in the contract storage.
///
/// The value length must not exceed the maximum defined by the contracts module parameters.
/// Storing an empty value is disallowed.
pub fn seal_set_storage(
    sandbox: Rc<RefCell<Sandbox>>,
    args: &[Value],
) -> Result<Option<ReturnValue>> {
    if args.len() != 3 {
        return Err(Error::WrongArugmentLength);
    }
    let [key_ptr, value_ptr, value_len] = [args[0].into(), args[1].into(), args[2].into()];

    let mut key: StorageKey = [0; 32];
    sandbox
        .borrow()
        .read_sandbox_memory_into_buf(key_ptr, &mut key)?;
    let value = sandbox.borrow().read_sandbox_memory(value_ptr, value_len)?;
    sandbox.borrow_mut().set_storage(&key, value)?;

    Ok(None)
}

/// seal_input
pub fn seal_input(sandbox: Rc<RefCell<Sandbox>>, args: &[Value]) -> Result<Option<ReturnValue>> {
    if args.len() != 2 {
        return Err(Error::WrongArugmentLength);
    }
    let [out_ptr, out_len_ptr] = [args[0].into(), args[1].into()];

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
    args: &[Value],
) -> Result<Option<ReturnValue>> {
    if args.len() != 2 {
        return Err(Error::WrongArugmentLength);
    }
    let [out_ptr, out_len_ptr] = [args[0].into(), args[1].into()];

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
pub fn seal_return(sandbox: Rc<RefCell<Sandbox>>, args: &[Value]) -> Result<Option<ReturnValue>> {
    if args.len() != 3 {
        return Err(Error::WrongArugmentLength);
    }
    let [flags, data_ptr, data_len] = [args[0].into(), args[1].into(), args[2].into()];

    let mut bm = sandbox.borrow_mut();
    let data = bm.read_sandbox_memory(data_ptr, data_len)?;
    if flags == 0 {
        bm.ret = Some(data.clone());
    }

    drop(bm);
    Err(Error::ReturnData { flags, data })
}
