//! Environment builder
#[cfg(not(feature = "std"))]
use crate::wasmi as e;
#[cfg(feature = "std")]
use crate::wasmtime as e;
use crate::{
    derive::{self, HostCall, HostFuncType},
    memory::Memory,
};
use ceres_std::Vec;
use core::ops;

/// Ceres environment builder
pub struct Builder<T>(e::Builder<T>);

impl<T> Default for Builder<T> {
    fn default() -> Self {
        Self::new()
    }
}

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

    /// Shortcut of `add_host_func`
    pub fn add_host_parcel<M, F>(&mut self, call: HostCall<M, F, T>)
    where
        F: Into<Vec<u8>>,
        M: Into<Vec<u8>>,
    {
        self.add_host_func(call.0, call.1, call.2)
    }

    /// Shortcut of `add_host_func`
    pub fn add_host_parcels<M, F>(mut self, calls: Vec<HostCall<M, F, T>>) -> Self
    where
        F: Into<Vec<u8>>,
        M: Into<Vec<u8>>,
    {
        for call in calls {
            self.add_host_func(call.0, call.1, call.2)
        }

        self
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
