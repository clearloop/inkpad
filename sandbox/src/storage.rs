use crate::{Sandbox, StorageKey};
use ceres_executor::Result;
use ceres_std::Vec;

impl Sandbox {
    /// Get storage
    pub fn get_storage(&self, key: &StorageKey) -> Result<Option<Vec<u8>>> {
        Ok(self.state.get(key).map(|v| v.clone()))
    }

    /// Get storage
    pub fn set_storage(&mut self, key: &StorageKey, value: Vec<u8>) -> Result<()> {
        self.state.insert(*key, value);
        Ok(())
    }
}
