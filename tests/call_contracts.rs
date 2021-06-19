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
use ceres_ri::Instance;
use ceres_runtime::{MemoryStorage, Runtime};
use std::{cell::RefCell, rc::Rc};

#[test]
fn test_call_contracts() {
    let shared = Rc::new(RefCell::new(MemoryStorage::new()));

    let hashes = [
        include_bytes!("../contracts/accumulator.contract").to_vec(),
        include_bytes!("../contracts/adder.contract").to_vec(),
        include_bytes!("../contracts/subber.contract").to_vec(),
    ]
    .iter()
    .map(|contract| {
        let mut rt =
            Runtime::from_contract_and_storage(contract, shared.clone(), Some(Instance)).unwrap();
        rt.flush().unwrap();
        rt.metadata.source.hash
    })
    .collect::<Vec<String>>();

    // init delegator
    let mut delegator = Runtime::from_contract_and_storage(
        include_bytes!("../contracts/delegator.contract"),
        shared.clone(),
        Some(Instance),
    )
    .unwrap();

    assert!(delegator
        .deploy(
            "new",
            &["00", "00", &hashes[0], &hashes[1], &hashes[2]],
            None,
        )
        .is_err());
}
