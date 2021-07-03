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
use ceres_support::types::Cache;
use parity_scale_codec::Encode;

#[test]
fn test_call_contracts() {
    env_logger::init();
    let mut delegator = Runtime::from_contract(
        include_bytes!("../contracts/delegator.contract"),
        Cache::default(),
        Some(Instance),
    )
    .unwrap();

    // Get hashes
    let mut hashes: Vec<[u8; 32]> = Vec::new();
    for contract in [
        include_bytes!("../contracts/accumulator.contract").to_vec(),
        include_bytes!("../contracts/adder.contract").to_vec(),
        include_bytes!("../contracts/subber.contract").to_vec(),
    ]
    .iter()
    {
        hashes.push(delegator.load(contract).unwrap())
    }

    // deploy
    delegator
        .deploy(
            "new",
            vec![
                42.encode(),
                1.encode(),
                hashes[0].encode(),
                hashes[1].encode(),
                hashes[2].encode(),
            ],
            None,
        )
        .unwrap();

    // call
    assert_eq!(delegator.call("get", vec![], None), Ok(Some(42.encode())));
}
