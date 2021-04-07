#![cfg_attr(not(feature = "std"), no_std)]
use ceres_executor::{Error, Memory, Result};
use ceres_std::{vec, BTreeMap, Vec};
use parity_scale_codec::{Decode, DecodeAll, Encode};

/// Custom storage key
pub type StorageKey = [u8; 32];

/// The runtime of ink! machine
pub struct Sandbox {
    /// input data
    pub input: Option<Vec<u8>>,
    pub ret: Option<Vec<u8>>,
    state: BTreeMap<StorageKey, Vec<u8>>,
    memory: Memory,
}

impl Sandbox {
    /// New sandbox
    pub fn new(memory: Memory, state: BTreeMap<StorageKey, Vec<u8>>) -> Sandbox {
        Sandbox {
            input: None,
            ret: None,
            state,
            memory,
        }
    }

    /// Get memory ref
    pub fn mem(&self) -> Memory {
        self.memory.clone()
    }

    /// Get storage
    pub fn get_storage(&self, key: &StorageKey) -> Result<Option<Vec<u8>>> {
        Ok(self.state.get(key).map(|v| v.clone()))
    }

    /// Get storage
    pub fn set_storage(&mut self, key: &StorageKey, value: Vec<u8>) -> Result<()> {
        self.state.insert(*key, value);
        Ok(())
    }

    /// Read designated chunk from the sandbox memory.
    pub fn read_sandbox_memory(&self, ptr: u32, len: u32) -> Result<Vec<u8>> {
        let mut buf = vec![0u8; len as usize];
        self.read_sandbox_memory_into_buf(ptr, &mut buf)?;
        Ok(buf.to_vec())
    }

    /// Read designated chunk from the sandbox into the supplied buffer
    pub fn read_sandbox_memory_into_buf(&self, ptr: u32, buf: &mut [u8]) -> Result<()> {
        self.memory.get(ptr, buf).map_err(|_| Error::OutOfBounds)?;
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
