//! Ceres executor memory
use crate::Result;

/// Ceres wasm executor memory
pub trait Memory: Sized {
    /// Construct a new linear memory instance
    fn new(initial: u32, maximum: Option<u32>) -> Result<Self>;

    /// Read a memory area at the address `ptr` with the size of the provided slice `buf`.
    fn get(&self, ptr: u32, buf: &mut [u8]) -> Result<()>;

    /// Write a memory area at the address `ptr` with contents of the provided slice `buf`.
    fn set(&self, ptr: u32, value: &[u8]) -> Result<()>;
}
