//! The runtime of ink! machine
use crate::{Result, StorageKey};
use alloc::vec::Vec;
use hashbrown::HashMap;
use wasmi::MemoryRef;

/// The runtime of ink! machine
pub struct Sandbox {
    /// input data
    pub input_data: Option<Vec<u8>>,
    store: HashMap<StorageKey, Vec<u8>>,
    memory: Option<MemoryRef>,
}

impl Sandbox {
    /// Get storage
    pub fn get_storage(&self, key: &StorageKey) -> Result<Option<Vec<u8>>> {
        Ok(self.store.get(key).map(|v| v.clone()))
    }

    /// Get storage
    pub fn set_storage(&mut self, key: &StorageKey, value: Vec<u8>) -> Result<()> {
        self.store.insert(*key, value);
        Ok(())
    }

    /// Read designated chunk from the sandbox memory.
    pub fn read_sandbox_memory(&self, ptr: u32, len: u32) -> Result<Vec<u8>> {
        todo!()
    }

    /// Read designated chunk from the sandbox into the supplied buffer
    pub fn read_sandbox_memory_into_buf(&self, ptr: u32, buf: &mut [u8]) -> Result<()> {
        todo!()
    }

    /// Write the given buffer to the designated location in the sandbox memory.
    pub fn write_sandbox_memory(&mut self, ptr: u32, buf: &[u8]) -> Result<()> {
        todo!()
    }

    /// Write the given buffer and its length to the designated locations in sandbox memory
    pub fn write_sandbox_output(
        &mut self,
        out_ptr: u32,
        out_len_ptr: u32,
        buf: &[u8],
    ) -> Result<()> {
        todo!()
    }
}
