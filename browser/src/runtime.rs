//! Ceres Runtime interfaces
use crate::{result::err_check, ri::Interface, ti::Transaction, BrowserStorage};
use ceres_runtime::{Metadata, Runtime as RuntimeInner};
use ceres_std::Rc;
use core::cell::RefCell;
use wasm_bindgen::prelude::wasm_bindgen;

/// Ceres browser runtime
#[wasm_bindgen]
pub struct Runtime(ceres_runtime::Runtime);

#[wasm_bindgen]
impl Runtime {
    /// New runtime
    #[wasm_bindgen(constructor)]
    pub fn new(b: &str, metadata: String) -> Runtime {
        let bytes = err_check(hex::decode(b));
        let metadata: Metadata = err_check(serde_json::from_str(&metadata));
        let storage = BrowserStorage::new();
        Runtime(err_check(RuntimeInner::new(
            &bytes,
            metadata,
            Rc::new(RefCell::new(storage)),
            Some(Interface),
        )))
    }

    /// Create runtime from contract
    pub fn from_contract(contract: &str) -> Runtime {
        let bytes = err_check(hex::decode(contract));
        let storage = BrowserStorage::new();
        Runtime(err_check(RuntimeInner::from_contract_and_storage(
            &bytes,
            Rc::new(RefCell::new(storage)),
            Some(Interface),
        )))
    }

    /// Deploy contract
    pub fn deploy(&mut self, method: &str, args_json: &str, tx_json: &str) {
        Self::parse_args_and_then(args_json, tx_json, move |args, tx| {
            err_check(self.0.deploy(&method, &args, tx.map(|v| v.into())));
        })
    }

    /// Deploy contract
    pub fn call(&mut self, method: &str, args_json: &str, tx_json: &str) -> String {
        hex::encode(&Self::parse_args_and_then(
            args_json,
            tx_json,
            move |args, tx| err_check(self.0.call(&method, &args, tx.map(|v| v.into()))),
        ))
    }

    /// Flush storage
    pub fn flush(&mut self) {
        err_check(self.0.flush());
    }

    /// Parse js arguments
    fn parse_args_and_then<F, T>(args_json: &str, tx_json: &str, mut f: F) -> T
    where
        F: FnMut(Vec<&str>, Option<Transaction>) -> T,
    {
        let args: Vec<String> = err_check(serde_json::from_str(&args_json));
        let tx = err_check(serde_json::from_str(&tx_json));
        let mut args_ref: Vec<&str> = Default::default();

        args.iter().for_each(|v| args_ref.push(v.as_str()));
        f(args_ref, tx)
    }
}
