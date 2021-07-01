//! Memory Cache
use crate::{
    traits::{self, Frame, Storage},
    types::State,
};
use ceres_std::{BTreeMap, Vec};

/// Memory cache implementation
pub struct Cache {
    storage: BTreeMap<Vec<u8>, Vec<u8>>,
    frame: Vec<State>,
}

impl Default for Cache {
    fn default() -> Self {
        Self {
            storage: BTreeMap::new(),
            frame: Vec::new(),
        }
    }
}

impl Storage for Cache {
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

impl Frame for Cache {
    fn active(&self) -> Option<[u8; 32]> {
        Some(self.frame.last()?.hash)
    }

    fn state(&self) -> Option<&State> {
        self.frame.last()
    }

    fn state_mut(&mut self) -> Option<&mut State> {
        self.frame.last_mut()
    }

    fn push(&mut self, s: State) {
        self.frame.push(s)
    }

    fn pop(&mut self) -> Option<State> {
        self.frame.pop()
    }
}

impl traits::Cache for Cache {}
