//! The runtime of ink! machine
use crate::{Error, Result, StorageKey};
use alloc::vec::Vec;
use hashbrown::HashMap;
use parity_scale_codec::{Decode, DecodeAll, Encode};
use wasmi::MemoryRef;

/// The runtime of ink! machine
pub struct Sandbox {
    /// input data
    pub input: Option<Vec<u8>>,
    store: HashMap<StorageKey, Vec<u8>>,
    memory: MemoryRef,
}

impl Sandbox {
    /// New sandbox
    pub fn new(memory: MemoryRef) -> Sandbox {
        Sandbox {
            input: None,
            store: HashMap::new(),
            memory,
        }
    }

    /// Get memory ref
    pub fn mem(&self) -> MemoryRef {
        self.memory.clone()
    }

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
        let mut buf = vec![0u8; len as usize];
        self.read_sandbox_memory_into_buf(ptr, &mut buf)?;
        Ok(buf)
    }

    /// Read designated chunk from the sandbox into the supplied buffer
    pub fn read_sandbox_memory_into_buf(&self, ptr: u32, buf: &mut [u8]) -> Result<()> {
        self.memory
            .get_into(ptr, buf)
            .map_err(|_| Error::OutOfBounds)?;
        Ok(())
    }

    /// Read designated chunk from the sandbox memory and attempt to decode into the specified type.
    pub fn read_sandbox_memory_as<D: Decode>(&mut self, ptr: u32, len: u32) -> Result<D> {
        let buf = self.read_sandbox_memory(ptr, len)?;
        let decoded = D::decode_all(&mut &buf[..]).map_err(|_| Error::DecodeRuntimeValueFailed)?;
        Ok(decoded)
    }

    /// Write the given buffer to the designated location in the sandbox memory.
    pub fn write_sandbox_memory(&mut self, ptr: u32, buf: &[u8]) -> Result<()> {
        Ok(self.memory.set(ptr, buf).map_err(|_| Error::OutOfBounds)?)
    }

    /// Write the given buffer and its length to the designated locations in sandbox memory
    ///
    /// buf -> memory
    pub fn write_sandbox_output(
        &mut self,
        out_ptr: u32,
        out_len_ptr: u32,
        buf: &[u8],
    ) -> Result<()> {
        let buf_len = buf.len() as u32;
        let len: u32 = self.read_sandbox_memory_as(out_len_ptr, 4)?;
        if len < buf_len {
            Err(Error::OutputBufferTooSmall)?
        }

        self.memory
            .set(out_ptr, buf)
            .and_then(|_| self.memory.set(out_len_ptr, &buf_len.encode()))
            .map_err(|_| Error::OutOfBounds)?;

        Ok(())
    }
}
