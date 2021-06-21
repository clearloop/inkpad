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
use ceres_runtime::Runtime;
use ceres_sandbox::Transaction;
use ceres_support::types::MemoryStorage;
use parity_scale_codec::Encode;
use std::{cell::RefCell, rc::Rc};

#[test]
fn test_call_contracts() {
    env_logger::init();
    let cache = Rc::new(RefCell::new(MemoryStorage::default()));
    let state = Rc::new(RefCell::new(MemoryStorage::default()));
    let hashes = [
        include_bytes!("../contracts/accumulator.contract").to_vec(),
        include_bytes!("../contracts/adder.contract").to_vec(),
        include_bytes!("../contracts/subber.contract").to_vec(),
    ]
    .iter()
    .map(|contract| {
        let rt = Runtime::from_contract_and_storage(
            contract,
            cache.clone(),
            state.clone(),
            Some(Instance),
        )
        .unwrap();
        rt.metadata.source.hash
    })
    .collect::<Vec<String>>();

    // init delegator
    let mut delegator = Runtime::from_contract_and_storage(
        include_bytes!("../contracts/delegator.contract"),
        cache,
        state,
        Some(Instance),
    )
    .unwrap();

    assert!(delegator
        .deploy(
            "new",
            vec![
                42.encode(),
                0.encode(),
                hex::decode(&hashes[0][2..]).unwrap(),
                hex::decode(&hashes[1][2..]).unwrap(),
                hex::decode(&hashes[2][2..]).unwrap(),
            ],
            Some(Transaction {
                balance: 100_000,
                ..Default::default()
            }),
        )
        .is_err());
}
