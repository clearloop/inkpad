//! Ceres executor instance
use crate::{
    builder::Builder,
    value::{ReturnValue, Value},
    Result,
};

/// Ceres executor instance
pub trait Instance<T>: Sized {
    type Builder: Builder<T>;

    /// Instantiate a module with the given env builder
    fn new(code: &[u8], builder: &Self::Builder, state: &mut T) -> Result<Self>;

    /// invoke an exported function
    fn invoke(&mut self, name: &str, args: &[Value], state: &mut T) -> Result<ReturnValue>;

    /// Get global value
    fn get_global_val(&self, name: &str) -> Option<Value>;
}
