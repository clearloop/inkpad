use crate::Sandbox;
use ceres_executor::{Error, Result};
use ceres_std::{vec, Vec};
use parity_scale_codec::{Decode, DecodeAll, Encode};

impl Sandbox {
    /// Read designated chunk from the sandbox memory.
    pub fn read_sandbox_memory(&self, ptr: u32, len: u32) -> Result<Vec<u8>> {
        let mut buf = vec![0u8; len as usize];
        self.read_sandbox_memory_into_buf(ptr, &mut buf)?;
        Ok(buf.to_vec())
    }

    /// Read designated chunk from the sandbox into the supplied buffer
    pub fn read_sandbox_memory_into_buf(&self, ptr: u32, buf: &mut [u8]) -> Result<()> {
        self.cache
            .borrow_mut()
            .memory_mut()
            .ok_or(Error::CouldNotFindMemory)?
            .get(ptr, buf)
            .map_err(|_| Error::OutOfBounds)?;
        Ok(())
    }

    /// Read designated chunk from the sandbox memory and attempt to decode into the specified type.
    pub fn read_sandbox_memory_as<D: Decode>(&mut self, ptr: u32, len: u32) -> Result<D> {
        let buf = self.read_sandbox_memory(ptr, len)?;
        let decoded = D::decode_all(&buf[..]).map_err(|_| Error::DecodeRuntimeValueFailed)?;
        Ok(decoded)
    }

    /// Write the given buffer to the designated location in the sandbox memory.
    pub fn write_sandbox_memory(&mut self, ptr: u32, buf: &[u8]) -> Result<()> {
        let mut cache = self.cache.borrow_mut();
        cache
            .memory_mut()
            .ok_or(Error::CouldNotFindMemory)?
            .set(ptr, buf)
            .map_err(|_| Error::OutOfBounds)
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
            return Err(Error::OutputBufferTooSmall);
        }

        let mut cache = self.cache.borrow_mut();
        let mem_mut = cache.memory_mut().ok_or(Error::CouldNotFindMemory)?;

        mem_mut
            .set(out_ptr, buf)
            .and_then(|_| mem_mut.set(out_len_ptr, &buf_len.encode()))
            .map_err(|_| Error::OutOfBounds)?;

        Ok(())
    }
}
