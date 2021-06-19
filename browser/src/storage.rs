//! Browser storage
use ceres_runtime::{Result as RuntimeResult, Storage};
use ceres_sandbox::StorageKey;
use ceres_std::{BTreeMap, Vec};
use wasm_bindgen::prelude::wasm_bindgen;

/// Browser storage
#[wasm_bindgen]
pub struct BrowserStorage(web_sys::Storage);

#[wasm_bindgen]
impl BrowserStorage {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let window = web_sys::window().expect("Could not find window");
        Self(
            window
                .local_storage()
                .expect("Could not find local_storage")
                .expect("Could not find local_storage"),
        )
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
