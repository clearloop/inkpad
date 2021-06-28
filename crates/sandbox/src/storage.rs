use crate::{Sandbox, StorageKey};
use ceres_executor::{Error, Result};
use ceres_std::{BTreeMap, Rc, Vec};
use core::cell::RefCell;
use parity_scale_codec::{Decode, Encode};

impl Sandbox {
    /// Flush current bucket
    pub fn flush_bucket(&mut self) -> Result<()> {
        let active = self.stack.last().ok_or(Error::ExitedAllFrames)?;

        // Flush current bucket
        self.state
            .borrow_mut()
            .set(*active, self.bucket.borrow().encode())
            .ok_or(Error::SetStorageFailed)?;

        Ok(())
    }

    // Set new bucket
    pub fn set_bucket(&mut self) -> Result<()> {
        let active = self.stack.last().ok_or(Error::ExitedAllFrames)?;
        log::debug!("active bucket: {:?}", active);

        // Set new bucket
        if let Some(bucket) = self.state.borrow().get(*active) {
            self.bucket = Rc::new(RefCell::new(
                <BTreeMap<StorageKey, Vec<u8>>>::decode(&mut bucket.as_ref())
                    .map_err(|_| Error::DecodeBucketFailed(*active))?,
            ));
        }
        Ok(())
    }

    /// Get storage
    pub fn get_storage(&self, key: &StorageKey) -> Result<Option<Vec<u8>>> {
        let v = self.bucket.borrow().get(key).map(|v| v.to_vec());
        log::debug!("sandbox get storage {:?}, {:?}", key, v);
        Ok(v)
    }

    /// Get storage
    pub fn set_storage(&mut self, key: &StorageKey, value: Vec<u8>) -> Result<()> {
        log::debug!("sandbox set storage {:?}, {:?}", key, value);
        self.bucket.borrow_mut().insert(*key, value);
        Ok(())
    }
}
