//! ## Call contracts
//!
//! - deploy callee contracts
//!   - deploy `accumulator.contract`
//!   - deploy `adder.contract`
//!   - deploy `subber.contract`
//!
//! - deploy `delegator.contract`
//!
//! - test `delegator`
//!   - call `get`
//!   - call `change`
//!   - call `switch`
use ceres_runtime::{MemoryStorage, Runtime};
use std::{cell::RefCell, rc::Rc};

#[test]
fn test_call_contracts() {
    let shared = Rc::new(RefCell::new(MemoryStorage::new()));

    let hashes = [
        include_bytes!("../contracts/accumulator.contract.debug").to_vec(),
        include_bytes!("../contracts/adder.contract.debug").to_vec(),
        include_bytes!("../contracts/subber.contract.debug").to_vec(),
    ]
    .iter()
    .map(|contract| {
        let mut rt = Runtime::from_contract_and_storage(contract, shared.clone()).unwrap();
        rt.flush().unwrap();
        rt.metadata.source.hash
    })
    .collect::<Vec<String>>();

    // init delegator
    let mut delegator = Runtime::from_contract_and_storage(
        include_bytes!("../contracts/delegator.contract.debug"),
        shared.clone(),
    )
    .unwrap();
    delegator
        .deploy(
            "new",
            &["00", "00", &hashes[0], &hashes[1], &hashes[2]],
            None,
        )
        .unwrap();

    // println!("{:?}", delegator.call("get", &[], None));
}
