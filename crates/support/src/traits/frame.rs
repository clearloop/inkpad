//! frame trait
use crate::{traits::Cache, types::State};
use ceres_std::Rc;
use core::cell::RefCell;

/// WASM execution frame
pub trait Frame<Memory: 'static + Clone>: Cache<Memory> {
    fn active(&self) -> Option<[u8; 32]> {
        Some(self.frame().last()?.borrow().hash)
    }

    fn active_set(&self, key: [u8; 32], value: Vec<u8>) -> Option<Vec<u8>> {
        self.frame()
            .last()
            .map(|state| state.borrow_mut().set(key.to_vec(), value))?
    }

    fn active_get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.frame()
            .last()
            .map(|state| state.borrow().get(key).map(|v| v.to_vec()))?
    }

    fn push(&mut self, code_hash: [u8; 32], memory: Memory) {
        self.frame_mut()
            .push(Rc::new(RefCell::new(State::new(code_hash, memory))));
    }

    fn switch(&mut self, code_hash: [u8; 32]) -> Option<()> {
        let frames = self.frame().clone();
        self.frame_mut().push(
            frames
                .iter()
                .filter(|v| v.borrow().hash == code_hash)
                .last()?
                .clone(),
        );
        Some(())
    }

    fn back(&mut self) -> Option<()> {
        let frame_mut = self.frame_mut();
        if frame_mut.len() < 2 {
            None
        } else {
            frame_mut.push(frame_mut[frame_mut.len() - 2].clone());
            Some(())
        }
    }

    fn top(&mut self) -> Option<()> {
        let frame_mut = self.frame_mut();
        frame_mut.push(frame_mut[0].clone());
        Some(())
    }
}
