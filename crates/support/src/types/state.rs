//! Memory frame implementation
use ceres_std::{BTreeMap, Vec};

/// Memory frame implementation
#[derive(Clone, Debug)]
pub struct State {
    pub hash: [u8; 32],
    pub input: Option<Vec<u8>>,
    pub output: Option<Vec<u8>>,
    state: BTreeMap<Vec<u8>, Vec<u8>>,
}

impl State {
    /// New state
    pub fn new(hash: [u8; 32]) -> Self {
        Self {
            hash,
            input: None,
            output: None,
            state: BTreeMap::new(),
        }
    }

    pub fn set(&mut self, key: Vec<u8>, value: Vec<u8>) -> Option<Vec<u8>> {
        self.state.insert(key, value)
    }

    pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.state.get(key).cloned()
    }
}
