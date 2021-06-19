//! POC
//!
//! Test invoking ink! functions in wasmtime
use ceres_executor::{Trap, TrapCode};
use ceres_ri::Instance;
use ceres_runtime::Runtime;

#[test]
fn test_flipper_trap() {
    let mut rt = Runtime::from_contract(
        include_bytes!("../contracts/flipper_trap.contract"),
        Some(Instance),
    )
    .expect("Create runtime failed");

    rt.deploy("default", &[], None).expect("Deploy failed");
    assert_eq!(
        &rt.call("get", &[], None).expect("Call contract failed"),
        &[0]
    );

    if let Some(ceres_runtime::Error::CallContractFailed {
        error: ceres_executor::Error::Trap(Trap { code, .. }),
    }) = rt.call("flip", &[], None).err()
    {
        assert_eq!(code, TrapCode::UnreachableCodeReached);
    } else {
        panic!("Call flipper_trap with unexpected error");
    }
}
