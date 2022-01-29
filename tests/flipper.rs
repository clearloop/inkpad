//! POC
//!
//! Test invoking ink! functions in wasmtime
use inkpad_executor::{Trap, TrapCode};
use inkpad_ri::Instance;
use inkpad_runtime::Runtime;
use parity_scale_codec::Encode;

#[test]
fn test_flipper() {
    let mut rt = Runtime::contract(
        include_bytes!("../contracts/flipper.contract"),
        Some(Instance),
    )
    .expect("Create runtime failed");

    rt.deploy("default", vec![], None).expect("Deploy failed");
    assert_eq!(rt.call("get", vec![], None), Ok(Some(vec![0])));

    rt.deploy("new", vec![true.encode()], None)
        .expect("Deploy failed");
    assert_eq!(rt.call("get", vec![], None), Ok(Some(vec![1])));

    rt.call("flip", vec![], None).expect("Call contract failed");
    assert_eq!(rt.call("get", vec![], None), Ok(Some(vec![0])));
}

#[test]
fn test_flipper_trap() {
    let mut rt = Runtime::contract(
        include_bytes!("../contracts/flipper_trap.contract"),
        Some(Instance),
    )
    .expect("Create runtime failed");

    rt.deploy("default", vec![], None).expect("Deploy failed");
    assert_eq!(rt.call("get", vec![], None), Ok(Some(vec![0])));

    if let Some(inkpad_runtime::Error::CallContractFailed {
        error: inkpad_executor::Error::Trap(Trap { code, .. }),
    }) = rt.call("flip", vec![], None).err()
    {
        assert_eq!(code, TrapCode::UnreachableCodeReached);
    } else {
        panic!("Call flipper_trap with unexpected error");
    }
}
