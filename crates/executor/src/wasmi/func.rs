//! wasmi functions
use crate::derive;
use inkpad_std::Vec;

/// function id struct
pub struct HostFuncIndex(pub usize);

/// Defined host functions
#[derive(Default)]
pub struct DefinedHostFunctions<T>(pub Vec<derive::HostFuncType<T>>);

impl<T> Clone for DefinedHostFunctions<T> {
    fn clone(&self) -> DefinedHostFunctions<T> {
        DefinedHostFunctions(self.0.clone())
    }
}

impl<T> DefinedHostFunctions<T> {
    /// New defined host functions
    pub fn new() -> DefinedHostFunctions<T> {
        DefinedHostFunctions(Vec::new())
    }

    /// Get func by id
    pub fn func(&self, idx: usize) -> derive::HostFuncType<T> {
        self.0[idx]
    }

    /// define host function
    pub fn define(&mut self, f: derive::HostFuncType<T>) -> HostFuncIndex {
        let idx = self.0.len();
        self.0.push(f);

        HostFuncIndex(idx)
    }
}
