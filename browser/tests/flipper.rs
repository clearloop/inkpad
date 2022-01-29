//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

// tests
use inkpad_browser::Runtime;
use inkpad_support::types::Metadata;

#[wasm_bindgen_test]
fn test_flipper() {
    let contract: Metadata =
        serde_json::from_slice(include_bytes!("../../contracts/flipper.contract")).unwrap();
    let mut rt = Runtime::new(&serde_json::to_string(&contract).unwrap());

    rt.deploy("default", "[]", None);
    assert_eq!(&rt.call("get", "[]", None), "00");

    rt.deploy("new", r#"["01"]"#, None);
    assert_eq!(&rt.call("get", "[]", None), "01");

    rt.call("flip", "[]", None);
    assert_eq!(&rt.call("get", "[]", None), "00");
}
