//! Cache trait
use crate::traits::{Frame, Storage};

/// Cache traits
pub trait Cache<Memory: 'static + Clone>: Frame<Memory> + Storage {
    fn active_set(&mut self, key: Vec<u8>, value: Vec<u8>) -> Option<Vec<u8>> {
        self.state_mut()?.set(key, value)
    }

    fn active_get(&self, key: &[u8]) -> Option<&[u8]> {
        self.state()?.get(key)
    }
}
