//! Memory Cache
use crate::{
    traits::{self, Frame, Storage},
    types::State,
};
use ceres_std::{BTreeMap, Rc, Vec};
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

impl<Memory: 'static + Clone> Frame<Memory> for Cache<Memory> {
    fn active(&self) -> Option<[u8; 32]> {
        Some(self.frame.last()?.borrow().hash)
    }

    fn active_set(&self, key: [u8; 32], value: Vec<u8>) -> Option<Vec<u8>> {
        self.frame
            .last()
            .map(|state| state.borrow_mut().set(key.to_vec(), value))?
    }

    fn active_get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.frame
            .last()
            .map(|state| state.borrow().get(key).map(|v| v.to_vec()))?
    }

    fn push(&mut self, code_hash: [u8; 32], memory: Memory) {
        self.frame
            .push(Rc::new(RefCell::new(State::new(code_hash, memory))));
    }

    #[allow(mutable_borrow_reservation_conflict)]
    fn switch(&mut self, code_hash: [u8; 32]) -> Option<()> {
        for frame in &self.frame {
            if frame.borrow().hash != code_hash {
                continue;
            }

            self.frame.push(frame.clone());
            return Some(());
        }

        None
    }

    fn back(&mut self) -> Option<()> {
        if self.frame.len() < 2 {
            None
        } else {
            self.frame.push(self.frame[self.frame.len() - 2].clone());
            Some(())
        }
    }

    fn top(&mut self) -> Option<()> {
        self.frame.push(self.frame[0].clone());
        Some(())
    }

    fn memory(&self) -> Option<Memory> {
        Some(self.frame.last()?.borrow().memory.clone())
    }
}

impl<Memory: 'static + Clone> traits::Cache<Memory> for Cache<Memory> {}
