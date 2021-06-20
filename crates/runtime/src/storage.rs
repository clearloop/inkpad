//! Storage interfaces
use crate::{Result, StorageKey};
use ceres_std::BTreeMap;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

/// Storage interfaces
pub trait Storage {
    /// Set storage by code hash
    fn set(&mut self, code_hash: StorageKey, data: BTreeMap<StorageKey, Vec<u8>>) -> Result<()>;

    /// Get storage by code hash
    fn get(&self, code_hash: StorageKey) -> Option<BTreeMap<StorageKey, Vec<u8>>>;

    /// New state
    fn new_state(&self) -> BTreeMap<StorageKey, Vec<u8>>;
}

/// Memory storage
#[derive(Default)]
pub struct MemoryStorage(pub BTreeMap<StorageKey, BTreeMap<StorageKey, Vec<u8>>>);

impl MemoryStorage {
    /// New memory storage
    pub fn new() -> MemoryStorage {
        Self::default()
    }
}

impl Storage for MemoryStorage {
    fn set(&mut self, code_hash: StorageKey, data: BTreeMap<StorageKey, Vec<u8>>) -> Result<()> {
        log::debug!("set {:?}", code_hash);
        self.0.insert(code_hash, data);
        Ok(())
    }

    #[allow(clippy::map_clone)]
    fn get(&self, code_hash: StorageKey) -> Option<BTreeMap<StorageKey, Vec<u8>>> {
        log::debug!("get {:?}", code_hash);
        self.0.get(&code_hash).map(|v| v.clone())
    }

    /// New state
    fn new_state(&self) -> BTreeMap<StorageKey, Vec<u8>> {
        BTreeMap::new()
    }
}
