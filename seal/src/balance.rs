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

// **TODO**
//
// Require transfer interface
//
// /// Transfer some value to another account.
// #[host(seal0)]
// pub fn seal_transfer(
//     account_ptr: u32,
//     account_len: u32,
//     value_ptr: u32,
//     value_len: u32,
// ) -> Result<ReturnValue> {
//     let callee: [u8; 32] = sandbox.read_sandbox_memory_as(account_ptr, account_len)?;
//     let value: [u8; 32] = sandbox.read_sandbox_memory_as(value_ptr, value_len)?;
//
//     let result = sandbox.transfer(&callee, value);
// }
