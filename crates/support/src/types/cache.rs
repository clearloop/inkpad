//! Memory Cache
use crate::{
    traits::{self, Frame, Storage},
    types::State,
};
use ceres_std::{BTreeMap, Vec};

/// Memory cache implementation
pub struct Cache<Memory: Clone> {
    storage: BTreeMap<Vec<u8>, Vec<u8>>,
    frame: Vec<State<Memory>>,
}

impl<Memory: Clone> Default for Cache<Memory> {
    fn default() -> Self {
        Self {
            storage: BTreeMap::new(),
            frame: Vec::new(),
        }
    }
}

impl<Memory: Clone> Storage for Cache<Memory> {
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

impl<Memory: 'static + Clone> Frame<Memory> for Cache<Memory> {
    fn active(&self) -> Option<[u8; 32]> {
        Some(self.frame.last()?.hash)
    }

    fn state(&self) -> Option<&State<Memory>> {
        self.frame.last()
    }

    fn state_mut(&mut self) -> Option<&mut State<Memory>> {
        self.frame.last_mut()
    }

    fn push(&mut self, s: State<Memory>) {
        self.frame.push(s)
    }

    fn pop(&mut self) -> Option<State<Memory>> {
        self.frame.pop()
    }
}

impl<Memory: 'static + Clone> traits::Cache<Memory> for Cache<Memory> {}
