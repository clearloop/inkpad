use crate::derive::Host;
use ceres_derive::host;
use ceres_executor::{
    derive::{ReturnValue, Value},
    Error, Result,
};
use ceres_sandbox::Sandbox;

/// seal_input
#[host(seal0)]
pub fn seal_input(out_ptr: u32, out_len_ptr: u32) -> Result<ReturnValue> {
    if let Some(input) = sandbox.input.take() {
        sandbox.write_sandbox_output(out_ptr, out_len_ptr, &input)?;
        Ok(ReturnValue::Unit)
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
pub fn seal_return(flags: u32, data_ptr: u32, data_len: u32) -> Result<ReturnValue> {
    let data = sandbox.read_sandbox_memory(data_ptr, data_len)?;
    if flags == 0 {
        sandbox.ret = Some(data.clone());
    }

    Err(Error::ReturnData { flags, data })
}
