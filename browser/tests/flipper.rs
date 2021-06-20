//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

// tests
use ceres_browser::Runtime;

#[wasm_bindgen_test]
fn test_flipper() {
    let mut rt = Runtime::new(&hex::encode(include_bytes!(
        "../../contracts/flipper.contract"
    )));

    rt.deploy("default", "[]", "null");
    assert_eq!(&rt.call("get", "[]", "null"), "00");

    rt.deploy("new", r#"["true"]"#, "null");
    assert_eq!(&rt.call("get", "[]", "null"), "01");

    rt.call("flip", "[]", "null");
    assert_eq!(&rt.call("get", "[]", "null"), "00");
}
