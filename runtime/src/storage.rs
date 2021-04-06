//! Storage interfaces
use crate::{Error, Result, StorageKey};
use hashbrown::HashMap;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

/// Storage interfaces
pub trait Storage {
    // Set storage by code hash
    fn set(&mut self, code_hash: StorageKey, data: HashMap<StorageKey, Vec<u8>>) -> Result<()>;

    /// Get storage by code hash
    fn get(&self, code_hash: StorageKey) -> Option<&HashMap<StorageKey, Vec<u8>>>;

    /// New state
    fn new_state(&self) -> HashMap<StorageKey, Vec<u8>>;
}

/// Memory storage
pub struct Memory(HashMap<StorageKey, HashMap<StorageKey, Vec<u8>>>);

impl Memory {
    /// New memory storage
    pub fn new() -> Memory {
        Memory(HashMap::new())
    }
}

impl Storage for Memory {
    fn set(&mut self, code_hash: StorageKey, data: HashMap<StorageKey, Vec<u8>>) -> Result<()> {
        if let Some(_) = self.0.insert(code_hash, data) {
            Ok(())
        } else {
            Err(Error::CouldNotSetStorage)
        }
    }

    fn get(&self, code_hash: StorageKey) -> Option<&HashMap<StorageKey, Vec<u8>>> {
        self.0.get(&code_hash)
    }

    /// New state
    fn new_state(&self) -> HashMap<StorageKey, Vec<u8>> {
        HashMap::new()
    }
}
