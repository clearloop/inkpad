//! Memory frame implementation
use inkpad_std::{BTreeMap, Vec};

/// Memory frame implementation
#[derive(Clone, Debug)]
pub struct State<Memory> {
    pub hash: [u8; 32],
    pub memory: Memory,
    pub state: BTreeMap<Vec<u8>, Vec<u8>>,
}

impl<Memory: Clone> State<Memory> {
    /// New state
    pub fn new(hash: [u8; 32], memory: Memory) -> Self {
        Self {
            hash,
            memory,
            state: BTreeMap::new(),
        }
    }

    /// Set storage
    pub fn set(&mut self, key: Vec<u8>, value: Vec<u8>) -> Option<Vec<u8>> {
        self.state.insert(key, value)
    }

    /// Get storage
    pub fn get(&self, key: &[u8]) -> Option<&[u8]> {
        self.state.get(key).map(|v| v.as_ref())
    }
}
