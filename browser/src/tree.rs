//! Browser storage
use inkpad_executor::Memory;
use inkpad_std::{Rc, Vec};
use inkpad_support::{
    traits::{Cache, Frame, Storage},
    types::State,
};
use core::cell::RefCell;
use wasm_bindgen::prelude::wasm_bindgen;

/// Browser storage
#[wasm_bindgen]
pub struct Tree {
    name: String,
    storage: web_sys::Storage,
    frame: Vec<Rc<RefCell<State<Memory>>>>,
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
            frame: Vec::new(),
        }
    }
}

fn browser_key(mut name: String, code_hash: Vec<u8>) -> String {
    name.push_str(&hex::encode(code_hash));
    name
}

impl Storage for Tree {
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

impl Frame<Memory> for Tree {}

impl Cache<Memory> for Tree {
    fn frame(&self) -> &Vec<Rc<RefCell<State<Memory>>>> {
        &self.frame
    }

    fn frame_mut(&mut self) -> &mut Vec<Rc<RefCell<State<Memory>>>> {
        &mut self.frame
    }

    fn memory(&self) -> Option<Memory> {
        Some(self.frame.last()?.borrow().memory.clone())
    }
}
