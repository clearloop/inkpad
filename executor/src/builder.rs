//! Environment builder
use crate::{HostFuncType, Memory};

/// Ceres environment builder
pub trait Builder<T>: Sized {
    /// Register a host function in this environment definition
    fn add_host_func<M, F>(&mut self, module: M, field: F, f: HostFuncType<T>)
    where
        F: Into<Vec<u8>>,
        M: Into<Vec<u8>>;

    /// Register a memory in this environment definition.
    fn add_memory<M, F>(&mut self, module: M, field: F, mem: impl Memory)
    where
        M: Into<Vec<u8>>,
        F: Into<Vec<u8>>;
}
