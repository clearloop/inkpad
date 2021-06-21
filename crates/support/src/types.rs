//! support types
use crate::traits::Storage;
use ceres_std::{BTreeMap, Vec};

/// Custom storage key
pub type StorageKey = [u8; 32];

/// Memory storage
#[derive(Default)]
pub struct MemoryStorage(BTreeMap<StorageKey, Vec<u8>>);

impl Storage for MemoryStorage {
    fn get(&self, key: StorageKey) -> Option<Vec<u8>> {
        self.0.get(&key).map(|v| v.clone())
    }

    fn set(&mut self, key: StorageKey, value: Vec<u8>) -> Option<StorageKey> {
        self.0.insert(key, value).map(|_| key)
    }
}
