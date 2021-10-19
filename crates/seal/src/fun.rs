use crate::derive::Host;
use ceres_derive::host;
use ceres_executor::{derive::Value, Error, Result, ReturnData, ReturnFlags, TrapCode};
use ceres_sandbox::Sandbox;

/// Stores the input passed by the caller into the supplied buffer.
#[host(seal0)]
pub fn seal_input(out_ptr: u32, out_len_ptr: u32) -> Result<Option<Value>> {
    if let Some(input) = sandbox.input.take() {
        log::debug!(
            "(seal_input) {:?} ({:?})",
            input,
            sandbox.cache.borrow().active()
        );
        sandbox.write_sandbox_output(out_ptr, out_len_ptr, &input)?;
        Ok(None)
    } else {
        Err(Error::OutOfBounds)
    }
}

/// Cease contract execution and save a data buffer as a result of the execution.
///
/// This function never returns as it stops execution of the caller.
/// This is the only way to return a data buffer to the caller. Returning from
/// execution without calling this function is equivalent to calling:
///
/// The flags argument is a bitfield that can be used to signal special return
#[host(seal0)]
pub fn seal_return(flags: u32, data_ptr: u32, data_len: u32) -> Result<Option<Value>> {
    let data = sandbox.read_sandbox_memory(data_ptr, data_len)?;
    if flags == 0 {
        sandbox.ret = Some(data.clone());
    }

    Err(Error::Return(ReturnData {
        flags: ReturnFlags::from_bits(flags).unwrap_or_default(),
        data,
    }))
}

/// Remove the calling account and transfer remaining balance.
///
/// This function never returns. Either the termination was successful and the
/// execution of the destroyed contract is halted. Or it failed during the termination
/// which is considered fatal and results in a trap + rollback.
#[host(seal0)]
pub fn seal_terminate(beneficiary_ptr: u32, _beneficiary_len: u32) -> Result<Option<Value>> {
    sandbox.terminate(beneficiary_ptr)?;
    Err(Error::Trap(TrapCode::Termination.into()))
}

/// Remove the calling account and transfer remaining balance.
///
/// This function never returns. Either the termination was successful and the
/// execution of the destroyed contract is halted. Or it failed during the termination
/// which is considered fatal and results in a trap + rollback.
#[host(seal1)]
pub fn seal_terminate(beneficiary_ptr: u32) -> Result<Option<Value>> {
    sandbox.terminate(beneficiary_ptr)?;
    Err(Error::Trap(TrapCode::Termination.into()))
}
