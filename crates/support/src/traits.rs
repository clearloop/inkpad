//! support traits
use crate::types::StorageKey;
use ceres_std::Vec;

/// Custom storage
pub trait Storage {
    /// Get bytes by StorageKey
    fn get(&self, key: StorageKey) -> Option<Vec<u8>>;

    /// Set bytes by StorageKey
    fn set(&mut self, key: StorageKey, value: Vec<u8>) -> Option<StorageKey>;
}

/// Contract exectuor
pub trait Executor<T, S, R, E> {
    /// build instance
    fn build(&mut self, b: &[u8], sandbox: &mut T, ri: Vec<S>) -> Result<(), E>;

    /// Invoke contract method
    fn invoke(&mut self, method: &str, data: Vec<u8>, sandbox: &mut T) -> Result<(Vec<u8>, R), E>;
}
