//! Memory Cache
use crate::traits;
use ceres_std::{BTreeMap, Vec};

/// Memory cache implementation
pub struct Cache<Memory: Clone> {
    storage: BTreeMap<Vec<u8>, Vec<u8>>,
    memory: Vec<Memory>,
}

impl<Memory: Clone> Default for Cache<Memory> {
    fn default() -> Self {
        Self {
            storage: BTreeMap::new(),
            memory: Vec::new(),
        }
    }
}

impl<Memory: Clone> traits::Storage for Cache<Memory> {
    fn set(&mut self, key: Vec<u8>, value: Vec<u8>) -> Option<Vec<u8>> {
        self.storage.insert(key, value)
    }

    fn remove(&mut self, key: &[u8]) -> Option<Vec<u8>> {
        self.storage.remove(key)
    }

    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.storage.get(key).map(|v| v.clone())
    }
}

impl<Memory: Clone> traits::Frame for Cache<Memory> {
    // const PREFIX: [u8; 4] = [0; 4];
    fn frame_prefix(&self) -> &[u8] {
        &[0, 0, 0, 0]
    }
}

impl<Memory: Clone> traits::State<Memory> for Cache<Memory> {
    fn memory(&self) -> Option<Memory> {
        Some(self.memory[self.memory.len() - 1].clone())
    }

    fn memory_mut(&mut self) -> Option<&mut Memory> {
        self.memory.last_mut()
    }
    /// Get memory mut
    fn pop_memory(&mut self) -> Option<Memory> {
        self.memory.pop()
    }

    /// Get memory mut
    fn push_memory(&mut self, memory: Memory) -> Option<()> {
        self.memory.push(memory);
        Some(())
    }
}

impl<Memory: Clone> traits::Cache<Memory> for Cache<Memory> {}
