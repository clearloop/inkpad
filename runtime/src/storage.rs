//! Storage interfaces
use crate::{Error, Result, StorageKey};
use ceres_std::BTreeMap;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

/// Storage interfaces
pub trait Storage {
    // Set storage by code hash
    fn set(&mut self, code_hash: StorageKey, data: BTreeMap<StorageKey, Vec<u8>>) -> Result<()>;

    /// Get storage by code hash
    fn get(&self, code_hash: StorageKey) -> Option<&BTreeMap<StorageKey, Vec<u8>>>;

    /// New state
    fn new_state(&self) -> BTreeMap<StorageKey, Vec<u8>>;
}

/// Memory storage
pub struct MemoryStorage(BTreeMap<StorageKey, BTreeMap<StorageKey, Vec<u8>>>);

impl MemoryStorage {
    /// New memory storage
    pub fn new() -> MemoryStorage {
        MemoryStorage(BTreeMap::new())
    }
}

impl Storage for MemoryStorage {
    fn set(&mut self, code_hash: StorageKey, data: BTreeMap<StorageKey, Vec<u8>>) -> Result<()> {
        if let Some(_) = self.0.insert(code_hash, data) {
            Ok(())
        } else {
            Err(Error::CouldNotSetStorage)
        }
    }

    fn get(&self, code_hash: StorageKey) -> Option<&BTreeMap<StorageKey, Vec<u8>>> {
        self.0.get(&code_hash)
    }

    /// New state
    fn new_state(&self) -> BTreeMap<StorageKey, Vec<u8>> {
        BTreeMap::new()
    }
}
