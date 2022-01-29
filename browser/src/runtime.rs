//! Inkpad Runtime interfaces
use crate::{result::err_check, ri::Interface, ti::Transaction, Tree};
use inkpad_runtime::Runtime as RuntimeInner;
use wasm_bindgen::prelude::wasm_bindgen;

const CERES_BROWSER_CACHE: &str = "CERES_BROWSER_CACHE";

/// Inkpad browser runtime
#[wasm_bindgen]
pub struct Runtime(inkpad_runtime::Runtime);

#[wasm_bindgen]
impl Runtime {
    /// New runtime
    #[wasm_bindgen(constructor)]
    pub fn new(contract: &str) -> Runtime {
        Runtime(err_check(RuntimeInner::from_metadata(
            err_check(serde_json::from_str(contract)),
            Tree::new(CERES_BROWSER_CACHE),
            Some(Interface),
        )))
    }

    /// Deploy contract
    pub fn deploy(&mut self, method: &str, args_json: &str, tx_json: Option<String>) {
        Self::parse_args_and_then(args_json, tx_json, move |args, tx| {
            err_check(self.0.deploy(method, args, tx.map(|v| v.into())));
        })
    }

    /// Deploy contract
    pub fn call(&mut self, method: &str, args_json: &str, tx_json: Option<String>) -> String {
        hex::encode(
            &Self::parse_args_and_then(args_json, tx_json, move |args, tx| {
                err_check(self.0.call(method, args, tx.map(|v| v.into())))
            })
            .unwrap_or_default(),
        )
    }

    /// Parse js arguments
    fn parse_args_and_then<F, T>(args_json: &str, tx_json: Option<String>, mut f: F) -> T
    where
        F: FnMut(Vec<Vec<u8>>, Option<Transaction>) -> T,
    {
        let args: Vec<String> = err_check(serde_json::from_str(args_json));
        let tx = tx_json.map(|v| err_check(serde_json::from_str(&v)));
        let mut args_bytes: Vec<Vec<u8>> = Default::default();

        for arg in args {
            args_bytes.push(err_check(hex::decode(arg)));
        }
        f(args_bytes, tx)
    }
}
