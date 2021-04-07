//! Host funcitons
use crate::{
    value::{ReturnValue, Value},
    Result,
};

/// Host function generic type
pub type HostFuncType<T> = fn(&mut T, &[Value]) -> Result<ReturnValue>;
