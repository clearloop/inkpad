//! POC
//!
//! Test invoking ink! functions in wasmtime
use ceres_executor::{Trap, TrapCode};
use ceres_runtime::Runtime;

#[test]
fn test_flipper_trap() {
    let mut rt = Runtime::from_contract(include_bytes!("../fixtures/flipper_trap.contract"))
        .expect("Create runtime failed");

    rt.deploy("default", &[], None).expect("Deploy failed");
    assert_eq!(
        &rt.call("get", &[], None).expect("Call contract failed"),
        &[0]
    );

    assert_eq!(
        rt.call("flip", &[], None).err(),
        Some(ceres_runtime::Error::CallContractFailed {
            error: ceres_executor::Error::Trap(Trap {
                code: TrapCode::UnreachableCodeReached,
                trace: vec!["wasm trap: unreachable".to_string()],
            })
        })
    )
}
