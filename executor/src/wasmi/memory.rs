//! WASMi memory implementation
use crate::{derive, Error, Result};
use wasmi::{memory_units::Pages, MemoryInstance, MemoryRef};

/// WASMi memory implementation
pub struct Memory(pub MemoryRef);

impl derive::Memory for Memory {
    fn new(initial: u32, maximum: Option<u32>) -> Result<Memory> {
        Ok(Memory(
            MemoryInstance::alloc(Pages(initial as usize), maximum.map(|m| Pages(m as usize)))
                .map_err(|_| Error::InitMemoryFailed)?,
        ))
    }

    fn get(&self, ptr: u32, buf: &mut [u8]) -> Result<()> {
        self.0
            .get_into(ptr, buf)
            .map_err(|_| Error::MemoryOutOfBonds)?;
        Ok(())
    }

    fn set(&self, ptr: u32, value: &[u8]) -> Result<()> {
        self.0
            .set(ptr, value)
            .map_err(|_| Error::MemoryOutOfBonds)?;
        Ok(())
    }
}
