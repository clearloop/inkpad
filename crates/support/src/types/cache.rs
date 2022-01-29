//! Memory Cache
use crate::{
    traits::{self, Frame, Storage},
    types::State,
};
use inkpad_std::{BTreeMap, Rc, Vec};
use core::cell::RefCell;

/// Memory cache implementation
pub struct Cache<Memory: Clone> {
    storage: BTreeMap<Vec<u8>, Vec<u8>>,
    frame: Vec<Rc<RefCell<State<Memory>>>>,
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

impl<Memory: 'static + Clone> traits::Cache<Memory> for Cache<Memory> {
    /// Get frame
    fn frame(&self) -> &Vec<Rc<RefCell<State<Memory>>>> {
        &self.frame
    }

    /// Get frame mut
    fn frame_mut(&mut self) -> &mut Vec<Rc<RefCell<State<Memory>>>> {
        &mut self.frame
    }

    fn memory(&self) -> Option<Memory> {
        Some(self.frame.last()?.borrow().memory.clone())
    }
}

impl<Memory: 'static + Clone> Frame<Memory> for Cache<Memory> {}
