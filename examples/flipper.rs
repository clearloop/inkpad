//! POC
//!
//! Test invoking ink! functions in wasmtime
use ceres_ri::Instance;
use ceres_runtime::Runtime;
use parity_scale_codec::Encode;

fn main() {
    let mut rt = Runtime::from_contract(
        include_bytes!("../contracts/flipper.contract"),
        Some(Instance),
    )
    .expect("Create runtime failed");

    rt.deploy("default", vec![], None).expect("Deploy failed");
    assert_eq!(
        &rt.call("get", vec![], None).expect("Call contract failed"),
        &[0]
    );

    rt.deploy("new", vec![true.encode()], None)
        .expect("Deploy failed");
    assert_eq!(
        &rt.call("get", vec![], None).expect("Call contract failed"),
        &[1]
    );

    rt.call("flip", vec![], None).expect("Call contract failed");
    assert_eq!(
        &rt.call("get", vec![], None).expect("Call contract failed"),
        &[0]
    );
}
