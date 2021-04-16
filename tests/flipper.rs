//! POC
//!
//! Test invoking ink! functions in wasmtime
use ceres_runtime::Runtime;

#[test]
fn test_flipper() {
    let mut rt = Runtime::from_contract(include_bytes!("../fixtures/flipper.contract"))
        .expect("Create runtime failed");

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
