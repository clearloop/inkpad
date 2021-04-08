use crate::derive::Host;
use ceres_derive::host;
use ceres_executor::{
    derive::{ReturnValue, Value},
    Error, Result,
};
use ceres_sandbox::Sandbox;

/// *TODO*: replace `1337` with a dynamic value
///
/// seal_value_transferred
///
/// Stores the value transferred along with this call or as endowment into
/// the supplied buffer.
///
/// AtLeat32Bits
#[host(seal0)]
pub fn seal_value_transferred(out_ptr: u32, out_len_ptr: u32) -> Result<ReturnValue> {
    sandbox.write_sandbox_output(out_ptr, out_len_ptr, &[0x00; 32])?;
    Ok(ReturnValue::Unit)
}
