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
