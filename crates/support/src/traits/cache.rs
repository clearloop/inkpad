//! Cache trait
use crate::traits::{Frame, State};
use core::iter::Iterator;

/// Cache traits
pub trait Cache<Memory>: State<Memory> + Frame + Iterator {
    /// Active set
    fn active_set(&mut self, key: Vec<u8>, value: Vec<u8>) -> Option<Vec<u8>> {
        self.prefix_set(&self.active()?.to_vec(), key, value)
    }

    /// Active get
    fn active_get(&self, key: &[u8]) -> Option<&[u8]> {
        self.prefix_get(self.active()?, key)
    }
}
