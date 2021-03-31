//! POC
//!
//! Test invoking ink! functions in wasmtime
use ceres_runtime::Runtime;

#[test]
fn test_flipper() {
    let mut rt = Runtime::from_contract(include_bytes!("../flipper.contract"))
        .expect("Create runtime failed");

    // rt.deploy("default", &[]).expect("Deploy failed");
    rt.deploy("new", &["true"]).expect("Deploy failed");

    rt.call("flip", &[]).expect("Deploy failed");
    // rt.call("get", &[]).expect("Deploy failed");
}
