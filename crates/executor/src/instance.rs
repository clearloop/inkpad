//! Ceres executor instance
#[cfg(not(feature = "std"))]
use crate::wasmi as e;
#[cfg(feature = "std")]
use crate::wasmtime as e;
use crate::{builder::Builder, derive, Result, Value};

/// Instance instance
pub struct Instance<T>(e::Instance<T>);

impl<T> Instance<T> {
    /// Instantiate a module with the given env builder
    pub fn new(code: &[u8], builder: &Builder<T>, state: &mut T) -> Result<Self> {
        Ok(Instance(<e::Instance<T> as derive::Instance<T>>::new(
            code, builder, state,
        )?))
    }

    /// invoke an exported function
    pub fn invoke(&mut self, name: &str, args: &[Value], state: &mut T) -> Result<Value> {
        derive::Instance::invoke(&mut self.0, name, args, state)
    }

    /// Get global value
    pub fn get_global_val(&self, name: &str) -> Option<Value> {
        derive::Instance::get_global_val(&self.0, name)
    }
}
