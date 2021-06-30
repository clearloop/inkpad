//! frame trait
use crate::traits::Storage;

/// WASM execution frame
pub trait Frame: Storage {
    /// Current id
    fn id(&self) -> usize;

    /// active frame
    fn active(&self) -> Option<Vec<u8>>;

    /// Pop frame
    fn pop_frame(&mut self) -> Option<Vec<u8>>;

    /// Push frame
    fn push_frame(&mut self, key: &[u8]) -> Option<Vec<u8>>;
}
