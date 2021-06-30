//! Browser storage
use ceres_executor::Memory;
use ceres_std::Vec;
use ceres_support::traits;
use wasm_bindgen::prelude::wasm_bindgen;

/// Browser storage
#[wasm_bindgen]
pub struct Tree {
    name: String,
    storage: web_sys::Storage,
    memory: Vec<Memory>,
    frame: Vec<Vec<u8>>,
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
            memory: Vec::new(),
            frame: Vec::new(),
        }
    }
}

fn browser_key(mut name: String, code_hash: Vec<u8>) -> String {
    name.push_str(&hex::encode(code_hash));
    name
}

impl traits::Storage for Tree {
    fn set(&mut self, key: Vec<u8>, value: Vec<u8>) -> Option<Vec<u8>> {
        let data_str = serde_json::to_string(&value).ok()?;
        self.storage
            .set(&browser_key(self.name.to_string(), key.clone()), &data_str)
            .map(|_| key)
            .ok()
    }

    fn remove(&mut self, key: &[u8]) -> Option<Vec<u8>> {
        let data_str = serde_json::to_string(&key).ok()?;
        self.storage
            .remove_item(&data_str)
            .ok()
            .map(|_| key.to_vec())
    }

    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        if let Ok(Some(data)) = self
            .storage
            .get(&browser_key(self.name.to_string(), key.to_vec()))
        {
            Some(serde_json::from_str(&data).ok()?)
        } else {
            None
        }
    }
}

impl traits::Frame for Tree {
    /// Current id
    fn id(&self) -> usize {
        self.frame.len()
    }

    /// active frame
    fn active(&self) -> Option<Vec<u8>> {
        self.frame.last().map(|v| v.clone())
    }

    /// Pop frame
    fn pop_frame(&mut self) -> Option<Vec<u8>> {
        self.frame.pop()
    }

    /// Push frame
    fn push_frame(&mut self, key: &[u8]) -> Option<Vec<u8>> {
        self.frame.push(key.to_vec());
        Some(key.to_vec())
    }
}

impl traits::State<Memory> for Tree {
    fn memory(&self) -> Option<Memory> {
        Some(self.memory[self.memory.len() - 1].clone())
    }

    fn memory_mut(&mut self) -> Option<&mut Memory> {
        self.memory.last_mut()
    }
    /// Get memory mut
    fn pop_memory(&mut self) -> Option<Memory> {
        self.memory.pop()
    }

    /// Get memory mut
    fn push_memory(&mut self, memory: Memory) -> Option<()> {
        self.memory.push(memory);
        Some(())
    }
}

impl traits::Cache<Memory> for Tree {}
