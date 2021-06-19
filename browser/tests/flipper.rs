//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

// tests
use ceres_browser::Runtime;

#[wasm_bindgen_test]
fn test_flipper() {
    let mut rt = Runtime::new(hex::encode(include_bytes!("../contracts/flipper.contract")));

    rt.deploy("default", &[], None).expect("Deploy failed");
    assert_eq!(
        &rt.call("get", &[], None).expect("Call contract failed"),
        &[0]
    );

    rt.deploy("new", &["true"], None).expect("Deploy failed");
    assert_eq!(
        &rt.call("get", &[], None).expect("Call contract failed"),
        &[1]
    );

    rt.call("flip", &[], None).expect("Call contract failed");
    assert_eq!(
        &rt.call("get", &[], None).expect("Call contract failed"),
        &[0]
    );
}
