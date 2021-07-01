//! Contract State
use crate::traits::Storage;

/// Contract State
pub trait State: Storage {
    /// Get hash
    fn hash(&self) -> [u8; 32];

    /// Input
    fn input(&mut self) -> Option<&mut Vec<u8>>;

    /// Return data
    fn output(&mut self) -> Option<&mut Vec<u8>>;
}
