//! Wasmtime memory
use super::util;
use crate::{derive, Error};
use core::ops::Range;
use wasmtime::{Limits, Memory as MemoryRef, MemoryType, Store};

/// Construct a range from an offset to a data length after the offset.
/// Returns None if the end of the range would exceed some maximum offset.
pub fn checked_range(offset: usize, len: usize, max: usize) -> Option<Range<usize>> {
    let end = offset.checked_add(len)?;
    if end <= max {
        Some(offset..end)
    } else {
        None
    }
}

/// Wasmtime memory
#[derive(Clone)]
pub struct Memory {
    store: Store,
    inner: MemoryRef,
}

impl Memory {
    pub fn store(&self) -> &Store {
        &self.store
    }

    /// Get the inner memory
    pub fn cast(self) -> MemoryRef {
        self.inner
    }
}

impl derive::Memory for Memory {
    /// New memory with config
    fn new(initial: u32, maximum: Option<u32>) -> Result<Memory, Error> {
        let store = util::store_with_dwarf()?;
        Ok(Memory {
            inner: MemoryRef::new(&store, MemoryType::new(Limits::new(initial, maximum))),
            store,
        })
    }

    fn get(&self, ptr: u32, buf: &mut [u8]) -> Result<(), Error> {
        // This should be safe since we don't grow up memory while caching this reference and
        // we give up the reference before returning from this function.
        let memory = unsafe { self.inner.data_unchecked() };
        let range =
            checked_range(ptr as usize, buf.len(), memory.len()).ok_or(Error::OutOfBounds)?;
        buf.copy_from_slice(&memory[range]);
        Ok(())
    }

    fn set(&self, ptr: u32, buf: &[u8]) -> Result<(), Error> {
        let memory = unsafe { self.inner.data_unchecked_mut() };
        let range =
            checked_range(ptr as usize, buf.len(), memory.len()).ok_or(Error::OutOfBounds)?;
        memory[range].copy_from_slice(buf);
        Ok(())
    }
}
