//! Browser storage
use crate::{Error, Result};
use ceres_runtime::{Result as RuntimeResult, Storage};
use ceres_sandbox::StorageKey;
use ceres_std::{BTreeMap, Vec};

/// Browser storage
pub struct BrowserStorage(web_sys::Storage);

impl BrowserStorage {
    pub fn new() -> Result<Self> {
        let window = web_sys::window().ok_or(Error::WindowNotExists)?;
        Ok(Self(
            window
                .local_storage()
                .map_err(|_js_value| {
                    // TODO:
                    //
                    // Display js_value
                    Error::WebSysError
                })?
                .ok_or(Error::LocalStorageNotExists)?,
        ))
    }
}

impl Storage for BrowserStorage {
    fn set(
        &mut self,
        code_hash: StorageKey,
        data: BTreeMap<StorageKey, Vec<u8>>,
    ) -> RuntimeResult<()> {
        let data_str =
            serde_json::to_string(&data).map_err(|_| ceres_runtime::Error::SerdeError)?;
        self.0
            .set(&hex::encode(code_hash), &data_str)
            .map_err(|_| ceres_runtime::Error::CouldNotSetStorage)?;
        Ok(())
    }

    fn get(&self, code_hash: StorageKey) -> Option<BTreeMap<StorageKey, Vec<u8>>> {
        if let Ok(Some(data)) = self.0.get(&hex::encode(code_hash)) {
            Some(serde_json::from_str(&data).ok()?)
        } else {
            None
        }
    }

    fn new_state(&self) -> BTreeMap<StorageKey, Vec<u8>> {
        BTreeMap::new()
    }
}
