//! Memory Cache
use crate::traits;
use ceres_std::{BTreeMap, Vec};

/// Memory cache implementation
pub struct Cache<Memory: Clone> {
    storage: BTreeMap<Vec<u8>, Vec<u8>>,
    frame: Vec<Vec<u8>>,
    memory: Vec<Memory>,
}

impl<Memory: Clone> Default for Cache<Memory> {
    fn default() -> Self {
        Self {
            storage: BTreeMap::new(),
            memory: Vec::new(),
            frame: Vec::new(),
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
        self.storage.get(key).cloned()
    }
}

impl<Memory: Clone> traits::Frame for Cache<Memory> {
    /// Current id
    fn id(&self) -> usize {
        self.frame.len()
    }

    /// active frame
    fn active(&self) -> Option<Vec<u8>> {
        self.frame.last().cloned()
    }

    /// Pop frame
    fn pop_frame(&mut self) -> Option<Vec<u8>> {
        self.frame.pop()
    }

    /// Push frame
    fn push_frame(&mut self, key: &[u8]) -> Option<Vec<u8>> {
        self.frame.push(key.to_vec());
        Some(key.to_vec())
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
