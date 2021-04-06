//! Ceres executor instance
use crate::{Builder, Result, Value};

/// Ceres executor instance
pub trait Instance<T>: Sized {
    /// Instantiate a module with the given env builder
    fn new(code: &[u8], builder: &impl Builder<T>, state: &mut T) -> Result<Self>;

    /// invoke an exported function
    fn invoke(&mut self, name: &str, args: &[Value], state: &mut T) -> Option<Value>;
}
