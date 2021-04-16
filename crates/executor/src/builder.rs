//! Environment builder
use crate::{func::HostFuncType, memory::Memory};
use ceres_std::Vec;

/// Ceres environment builder
pub trait Builder<T>: Sized {
    type Memory: Memory;

    /// New builder
    fn new() -> Self;

    /// Register a host function in this environment definition
    fn add_host_func<M, F>(&mut self, module: M, field: F, f: HostFuncType<T>)
    where
        F: Into<Vec<u8>>,
        M: Into<Vec<u8>>;

    /// Register a memory in this environment definition.
    fn add_memory<M, F>(&mut self, module: M, field: F, mem: Self::Memory)
    where
        M: Into<Vec<u8>>,
        F: Into<Vec<u8>>;
}
