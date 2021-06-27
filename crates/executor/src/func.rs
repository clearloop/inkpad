//! Host funcitons
use crate::{value::Value, Result};

/// Host function generic type
pub type HostFuncType<T> = fn(&mut T, &[Value]) -> Result<Value>;
