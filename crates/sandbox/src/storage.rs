use crate::{Sandbox, StorageKey};
use ceres_executor::Result;
use ceres_std::Vec;

impl Sandbox {
    /// Get storage
    pub fn get_storage(&self, key: &StorageKey) -> Result<Option<Vec<u8>>> {
        let v = self.cache.borrow().active_get(key).map(|v| v.to_vec());
        Ok(v)
    }

    /// Get storage
    pub fn set_storage(&mut self, key: StorageKey, value: Vec<u8>) -> Result<()> {
        self.cache.borrow_mut().active_set(key.to_vec(), value);
        Ok(())
    }
}
