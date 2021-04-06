//! Host funcitons
use crate::{
    value::{ReturnValue, Value},
    Error,
};

/// Host function generic type
pub type HostFuncType<T> = fn(&mut T, &[Value]) -> Result<ReturnValue, Error>;
