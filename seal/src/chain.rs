//! Chain interfaces
use crate::derive::Host;
use ceres_derive::host;
use ceres_executor::{
    derive::{ReturnValue, Value},
    Error, Result,
};
use ceres_sandbox::Sandbox;

/// Define a function `fn init_env<E: Ext>() -> HostFunctionSet<E>` that returns
/// a function set which can be imported by an executed contract.
#[host(seal0)]
pub fn gas(_amount: u32) -> Result<ReturnValue> {
    Ok(ReturnValue::Unit)
}
