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

/// Cache with executing feature
pub trait Cache: Storage {
    // call methods from a contract
    fn call(&self, code_hash: StorageKey, method: &str, data: Vec<u8>) -> Option<Vec<u8>>;
}
