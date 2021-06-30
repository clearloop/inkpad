//! Contract State
use crate::traits::Storage;

/// Contract State
pub trait State<Memory>: Storage {
    /// Get memory mut
    fn memory_mut(&mut self) -> &mut Memory;
}
