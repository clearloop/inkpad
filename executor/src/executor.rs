//! This mod includes executor instances
//!
//! * Memory
//! * Builder
//! * Instance
//!
//! Which have seem methods like the matching trait.
use crate::{
    derive::{self, HostFuncType, ReturnValue, Value},
    Result,
};
use ceres_std::Vec;
use core::ops;

#[cfg(not(feature = "std"))]
use crate::wasmi as e;
#[cfg(feature = "std")]
use crate::wasmtime as e;

/// WASM executor liner memory
#[derive(Clone)]
pub struct Memory(pub e::Memory);

impl ops::Deref for Memory {
    type Target = e::Memory;

    fn deref(&self) -> &e::Memory {
        &self.0
    }
}

impl Memory {
    /// New liner memory
    pub fn new(initial: u32, maximum: Option<u32>) -> Result<Self> {
        Ok(Self(<e::Memory as derive::Memory>::new(initial, maximum)?))
    }

    /// Read a memory area at the address `ptr` with the size of the provided slice `buf`.
    pub fn get(&self, ptr: u32, buf: &mut [u8]) -> Result<()> {
        Ok(derive::Memory::get(&self.0, ptr, buf)?)
    }

    /// Write a memory area at the address `ptr` with contents of the provided slice `buf`.
    pub fn set(&self, ptr: u32, value: &[u8]) -> Result<()> {
        Ok(derive::Memory::set(&self.0, ptr, value)?)
    }
}

/// Ceres environment builder
pub struct Builder<T>(e::Builder<T>);

impl<T> ops::Deref for Builder<T> {
    type Target = e::Builder<T>;

    fn deref(&self) -> &e::Builder<T> {
        &self.0
    }
}

impl<T> Builder<T> {
    /// New env builder
    pub fn new() -> Self {
        Builder(<e::Builder<T> as derive::Builder<T>>::new())
    }

    /// Register a host function in this environment definition
    pub fn add_host_func<M, F>(&mut self, module: M, field: F, f: HostFuncType<T>)
    where
        F: Into<Vec<u8>>,
        M: Into<Vec<u8>>,
    {
        derive::Builder::add_host_func(&mut self.0, module, field, f);
    }

    /// Register a memory in this environment definition.
    pub fn add_memory<M, F>(&mut self, module: M, field: F, mem: Memory)
    where
        M: Into<Vec<u8>>,
        F: Into<Vec<u8>>,
    {
        derive::Builder::add_memory(&mut self.0, module, field, mem.0);
    }
}

/// Instance instance
pub struct Instance<T>(e::Instance<T>);

impl<T> Instance<T> {
    /// Instantiate a module with the given env builder
    pub fn new(code: &[u8], builder: &Builder<T>, state: &mut T) -> Result<Self> {
        Ok(Instance(<e::Instance<T> as derive::Instance<T>>::new(
            code, &builder, state,
        )?))
    }

    /// invoke an exported function
    pub fn invoke(&mut self, name: &str, args: &[Value], state: &mut T) -> Result<ReturnValue> {
        Ok(derive::Instance::invoke(&mut self.0, name, args, state)?)
    }

    /// Get global value
    pub fn get_global_val(&self, name: &str) -> Option<Value> {
        Some(derive::Instance::get_global_val(&self.0, name)?)
    }
}
