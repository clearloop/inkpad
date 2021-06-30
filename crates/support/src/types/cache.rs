//! Memory Cache
use crate::traits;
use ceres_std::{BTreeMap, Vec};

/// Memory cache implementation
pub struct Cache<Memory> {
    storage: BTreeMap<Vec<u8>, Vec<u8>>,
    memory: BTreeMap<Vec<u8>, Memory>,
}

impl<Memory> traits::Storage for Cache<Memory> {
    fn set(&mut self, key: Vec<u8>, value: Vec<u8>) -> Option<Vec<u8>> {
        self.storage.insert(key, value)
    }

    fn remove(&mut self, key: &[u8]) -> Option<Vec<u8>> {
        self.storage.remove(key)
    }

    fn get(&self, key: &[u8]) -> Option<&[u8]> {
        self.storage.get(key).map(|v| v.as_ref())
    }
}

impl<Memory> traits::Frame for Cache<Memory> {
    // const PREFIX: [u8; 4] = [0; 4];
    fn frame_prefix(&self) -> &[u8] {
        &[0, 0, 0, 0]
    }
}

impl<Memory> traits::State<Memory> for Cache<Memory> {
    fn memory_mut(&mut self) -> Option<&mut Memory> {
        self.memory.get_mut(&traits::Frame::active(self)?.to_vec())
    }
    /// Get memory mut
    fn pop_memory(&mut self) -> Option<Memory> {
        self.memory.remove(&traits::Frame::active(self)?.to_vec())
    }

    /// Get memory mut
    fn push_memory(&mut self, memory: Memory) -> Option<()> {
        self.memory
            .insert(traits::Frame::active(self)?.to_vec(), memory);
        Some(())
    }
}

impl<Memory> traits::Cache<Memory> for Cache<Memory> {}
