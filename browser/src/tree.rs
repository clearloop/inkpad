//! Browser storage
use ceres_sandbox::StorageKey;
use ceres_std::Vec;
use ceres_support::traits::Storage;
use wasm_bindgen::prelude::wasm_bindgen;

/// Browser storage
#[wasm_bindgen]
pub struct Tree {
    name: String,
    storage: web_sys::Storage,
}

#[wasm_bindgen]
impl Tree {
    #[wasm_bindgen(constructor)]
    pub fn new(name: &str) -> Self {
        let window = web_sys::window().expect("Could not find window");
        Self {
            name: name.to_string(),
            storage: window
                .local_storage()
                .expect("Could not find local_storage")
                .expect("Could not find local_storage"),
        }
    }
}

fn browser_key(mut name: String, code_hash: StorageKey) -> String {
    name.push_str(&hex::encode(code_hash));
    name
}

impl Storage for Tree {
    fn set(&mut self, code_hash: StorageKey, data: Vec<u8>) -> Option<StorageKey> {
        let data_str = serde_json::to_string(&data).ok()?;
        self.storage
            .set(&browser_key(self.name.to_string(), code_hash), &data_str)
            .map(|_| code_hash)
            .ok()
    }

    fn get(&self, code_hash: StorageKey) -> Option<Vec<u8>> {
        if let Ok(Some(data)) = self
            .storage
            .get(&browser_key(self.name.to_string(), code_hash))
        {
            Some(serde_json::from_str(&data).ok()?)
        } else {
            None
        }
    }
}
