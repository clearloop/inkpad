//! Derive traits
pub use crate::{
    func::HostFuncType,
    value::{Type, Value},
    Result,
};
use ceres_std::Vec;

/// Host function parcel
pub type HostCall<M, F, T> = (M, F, HostFuncType<T>);

/// Custom SealCall
pub type SealCall<T> = HostCall<&'static str, &'static str, T>;

/// Ceres wasm executor memory
pub trait Memory: Sized + Clone {
    /// Construct a new linear memory instance
    fn new(initial: u32, maximum: Option<u32>) -> Result<Self>;

    /// Read a memory area at the address `ptr` with the size of the provided slice `buf`.
    fn get(&self, ptr: u32, buf: &mut [u8]) -> Result<()>;

    /// Write a memory area at the address `ptr` with contents of the provided slice `buf`.
    fn set(&self, ptr: u32, value: &[u8]) -> Result<()>;
}

/// Ceres executor instance
pub trait Instance<T>: Sized {
    type Builder: Builder<T>;

    /// Instantiate a module with the given env builder
    fn new(code: &[u8], builder: &Self::Builder, state: &mut T) -> Result<Self>;

    /// invoke an exported function
    fn invoke(&mut self, name: &str, args: &[Value], state: &mut T) -> Result<Value>;

    /// Get global value
    fn get_global_val(&self, name: &str) -> Option<Value>;
}

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
