//! POC
//!
//! Test invoking ink! functions in wasmtime
use ceres_runtime::Runtime;

// #[test]
// fn test_flipper_trap() {
//     let mut rt = Runtime::from_contract(include_bytes!("../fixtures/flipper_trap.contract"))
//         .expect("Create runtime failed");
//
//     rt.deploy("default", &[], None).expect("Deploy failed");
//     assert_eq!(
//         &rt.call("get", &[], None).expect("Call contract failed"),
//         &[0]
//     );
//
//     rt.call("flip", &[], None).expect("Call contract failed");
// }
