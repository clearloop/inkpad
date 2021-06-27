//! Derive types && traits
use ceres_executor::{
    derive::{HostParcel, Value},
    Result,
};
use ceres_sandbox::Sandbox;

/// Host function trait
pub trait Host: Sized {
    /// Host function module
    fn module() -> &'static str;

    /// Host function name
    fn name() -> &'static str;

    /// Wrap host function
    fn wrap(sandbox: &mut Sandbox, args: &[Value]) -> Result<Option<Value>>;

    /// Pack instance
    fn pack() -> HostParcel<&'static str, &'static str, Sandbox> {
        (
            <Self as Host>::module(),
            <Self as Host>::name(),
            <Self as Host>::wrap,
        )
    }
}
